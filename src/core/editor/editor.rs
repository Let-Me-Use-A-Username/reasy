use std::sync::{Arc, RwLock};

use egui_winit::State;
use winit::dpi::PhysicalPosition;
use winit::event::WindowEvent;
use winit::{window::Window};
use winit::application::ApplicationHandler;
use egui_wgpu::Renderer;

use crate::core::editor::menu::EditorMenu;
use crate::core::editor::objects;
use crate::core::editor::objects::settings::{EditorSettings};
use crate::core::renderer::backend::WgpuState;
use crate::event::{self, UserEvent};
use crate::core::editor::layout::EditorLayout;



/// Abstraction over editor GUI window.
/// 
/// Bridges EGUI's immediate mode and GPU rendering backend.
/// Orcherstrates rendering and UI logic.
#[derive(Default)]
pub(crate) struct EditorWindow{
    //Window fields
    window: Option<Window>,
    wgpu_state: Option<WgpuState>,
    egui_winit_state: Option<State>,
    //Rendering fields
    egui_context: Option<egui::Context>,
    egui_renderer: Option<Renderer>,
    //UI Fields
    egui_layout: Option<EditorLayout>,
    //Settings fields
    editor_settings: Option<Arc<RwLock<EditorSettings>>>,
    //
    user_pointer_pos: Option<PhysicalPosition<f64>>
}

impl ApplicationHandler<UserEvent> for EditorWindow{
    //Note: Resume isn't called on every frame. Rather, it is called quite rarely, most often on mobile.
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        //Creates a window if not exists.
        if self.window.is_none(){

            match event_loop.create_window(Window::default_attributes()){
                Ok(window) => {
                    window.set_title("Reasy");
                    //Create underlying rendering interface.
                    let wgpu_state = pollster::block_on(WgpuState::new(&window));

                    let egui_context = egui::Context::default();
                    let viewport_id = egui_context.viewport_id();
                    let pixels_per_point = egui_context.pixels_per_point();

                    //Create State that translates winit events into egui.
                    let state = State::new(
                        egui_context.clone(), 
                        viewport_id, 
                        &window, 
                        Some(pixels_per_point), 
                        None, 
                        None
                    );

                    //Create Renderer for EGUI UI.
                    let egui_renderer = Renderer::new(
                        &wgpu_state.device,
                        wgpu_state.config.format,
                        None,
                        1,
                        false
                    );

                    //Load editor settings or default to default
                    let settings = objects::settings::load_settings().unwrap_or_default();
                    
                    self.window = Some(window);
                    self.wgpu_state = Some(wgpu_state);
                    self.egui_winit_state = Some(state);
                    self.egui_context = Some(egui_context);
                    self.egui_renderer = Some(egui_renderer);
                
                    //Create Egui Editor layout
                    if let Ok(layout) = EditorLayout::new(settings.clone()){
                        self.egui_layout = Some(layout);
                        self.editor_settings = Some(Arc::new(RwLock::new(settings)))
                    }
                    else{
                        event_loop.exit();
                    }
                },
                Err(err) => {
                    eprintln!("Error creating window: {:?}", err);
                },
            }

        }
    }

    fn window_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _window_id: winit::window::WindowId, event: winit::event::WindowEvent) {
        if let (Some(egui_state), Some(window)) = (&mut self.egui_winit_state, &self.window){
            let _ = egui_state.on_window_event(window, &event);
        }

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.user_pointer_pos = Some(position)
            },
            WindowEvent::Resized(physical_size) => {
                if let Some(wgpu_state) = &mut self.wgpu_state{
                    wgpu_state.resize(physical_size);
                }
            },
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.render();
            },
            //DEPRECATED: Not used due to lack of WINIT functionality.
            WindowEvent::HoveredFile(path) => {
                //println!("File hovered: {}", path.display());
                if let Some(layout) = &mut self.egui_layout{
                    layout.file_hovered(path);
                }
            },
            //DEPRECATED: Not used due to lack of WINIT functionality.
            WindowEvent::HoveredFileCancelled => {
                //println!("File hovered cancelled. Dropping files.");
                if let Some(layout) = &mut self.egui_layout{
                    layout.clear_dropped_list();
                }
            },
            //DEPRECATED: Not used due to lack of WINIT functionality.
            WindowEvent::DroppedFile(_) => {
                //If `dnd`, handle in layout;
                let egui_pos = &self.convert_to_egui_pos();

                if let Some(layout) = &mut self.egui_layout{
                    layout.handle_file_drop(egui_pos);
                }
                self.render();
            }
            _ => (),
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: event::UserEvent) {
        match event{
            UserEvent::KeyPress(key) => {
                println!("User key press: {}", key)
            }
            _ => println!("Unknown event: {:?}", event)
        }
    }
}

impl EditorWindow{
    fn render(&mut self) {
        // Extract components from Option
        let (egui_context, egui_state, egui_renderer, wgpu_state, window, egui_layout, editor_settings) = match (
            self.egui_context.as_ref(),
            self.egui_winit_state.as_mut(),
            self.egui_renderer.as_mut(),
            self.wgpu_state.as_mut(),
            self.window.as_ref(),
            self.egui_layout.as_mut(),
            self.editor_settings.as_mut()
        ) {
            (Some(ctx), Some(state), Some(renderer), Some(wgpu), Some(win), Some(layout), Some(editor_settings)) => {
                (ctx, state, renderer, wgpu, win, layout, editor_settings)
            },
            _ => return,
        };

        //Retrieve UI input via egui
        let raw_input = egui_state.take_egui_input(window);
        //Render UI for one frame.
        let full_output = egui_context.run(raw_input.clone(), |ctx| {
            //Top Panel must be build first and seperately from others.
            //Collect *UI* Changes performed in top menu
            let ui_changes = egui::TopBottomPanel::top("MenuBar").show(ctx, |ui| {
                if let Ok(mut settings) = editor_settings.try_write(){ 
                    let mut menu = EditorMenu{};
                    menu.ui(ui, &mut settings)
                }
                else {
                    eprintln!("Error: Could not acquire settings");
                    None
                }
            });
            //If UI changes, reload layout
            if ui_changes.inner.is_some(){
                if let Ok(settings) = editor_settings.try_read(){
                    egui_layout.reload(
                        ui_changes.inner.unwrap(),
                        &settings
                    );
                }
            }
            //Central panel must be build last. 
            // Build layout UI here to avoid borrowing issues.
            // This consists of *ALL* the panels/tiles that exist inside the layout.
            egui::CentralPanel::default().show(ctx, |ui| {
                egui_layout.ui(ui);
            });
        });
        
        //Handle UI output via Egui
        egui_state.handle_platform_output(window, full_output.platform_output);
        
        //Converts shapes into triangles meshes
        let paint_jobs = egui_context.tessellate(full_output.shapes, full_output.pixels_per_point);
        
        // Render via WGPU using EGUI-WGPU renderer
        match wgpu_state.render(egui_renderer, paint_jobs, full_output.textures_delta) {
            Ok(_) => {},
            Err(wgpu::SurfaceError::Lost) => wgpu_state.resize(wgpu_state.size),
            Err(wgpu::SurfaceError::OutOfMemory) => {
                eprintln!("Out of memory!");
                std::process::exit(1);
            },
            Err(e) => eprintln!("Surface error: {:?}", e),
        }
        
        // Request next frame
        window.request_redraw();
    }

    fn convert_to_egui_pos(&self) -> Option<egui::Pos2>{
        let physical_pos = self.user_pointer_pos?;
        let window = self.window.as_ref()?;
        let ctx = self.egui_context.as_ref()?;

        let scale = window.scale_factor();
        let ppp = ctx.pixels_per_point();

        let logic_x = physical_pos.x / scale;
        let logic_y = physical_pos.y / scale;

        let egui_x = (logic_x as f32) / ppp;
        let egui_y = (logic_y as f32) / ppp;

        return Some(egui::Pos2 { x: egui_x, y: egui_y })
    }
}