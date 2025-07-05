use egui::Context;
use egui_winit::State;
use winit::event::WindowEvent;
use winit::{window::Window};
use winit::application::ApplicationHandler;
use egui_wgpu::Renderer;

use crate::backend::WgpuState;
use crate::event::{self, UserEvent};



/// Abstraction over editor GUI window.
/// 
/// Bridges EGUI's immediate mode and GPU rendering backend.
/// Orcherstrates rendering and UI logic.
#[derive(Default)]
pub(crate) struct EditorWindow{
    window: Option<Window>,
    wgpu_state: Option<WgpuState>,
    egui_winit_state: Option<State>,
    egui_context: Option<Context>,
    egui_renderer: Option<Renderer>
}

impl ApplicationHandler<UserEvent> for EditorWindow{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        //Creates a window if not exists.
        if self.window.is_none(){

            match event_loop.create_window(Window::default_attributes()){
                Ok(window) => {
                    window.set_title("Reasy");
                    //Create underlying rendering interface.
                    let wgpu_state = pollster::block_on(WgpuState::new(&window));

                    let egui_context = Context::default();
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
                    
                    self.window = Some(window);
                    self.wgpu_state = Some(wgpu_state);
                    self.egui_winit_state = Some(state);
                    self.egui_context = Some(egui_context);
                    self.egui_renderer = Some(egui_renderer);
                },
                Err(err) => {
                    eprintln!("Error creating window: {:?}", err);
                },
            }

        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let (Some(egui_state), Some(window)) = (&mut self.egui_winit_state, &self.window){
            let _ = egui_state.on_window_event(window, &event);
        }

        match event {
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
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
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
        let (egui_context, egui_state, egui_renderer, wgpu_state, window) = match (
            self.egui_context.as_ref(),
            self.egui_winit_state.as_mut(),
            self.egui_renderer.as_mut(),
            self.wgpu_state.as_mut(),
            self.window.as_ref()
        ) {
            (Some(ctx), Some(state), Some(renderer), Some(wgpu), Some(win)) => {
                (ctx, state, renderer, wgpu, win)
            },
            _ => return,
        };
        
        //Retrieve UI input via egui
        let raw_input = egui_state.take_egui_input(window);
        //Render UI for one frame.
        let full_output = egui_context.run(raw_input, |ctx| {
            // Build the UI directly here to avoid borrowing issues
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Game Editor");
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    if ui.button("New Project").clicked() {
                        println!("New Project clicked");
                    }
                    if ui.button("Open Project").clicked() {
                        println!("Open Project clicked");
                    }
                    if ui.button("Save Project").clicked() {
                        println!("Save Project clicked");
                    }
                });
                
                ui.separator();
                
                ui.columns(2, |columns| {
                    columns[0].group(|ui| {
                        ui.label("Scene Hierarchy");
                        ui.separator();
                        
                        if ui.button("+ Add GameObject").clicked() {
                            println!("Add GameObject clicked");
                        }
                        
                        // Example hierarchy items
                        ui.label("📁 Root");
                        ui.indent("hierarchy", |ui| {
                            ui.label("🎮 Player");
                            ui.label("🌍 Environment");
                            ui.label("💡 Lighting");
                        });
                    });
                    
                    columns[1].group(|ui| {
                        ui.label("Properties");
                        ui.separator();
                        
                        ui.label("Transform");
                        ui.horizontal(|ui| {
                            ui.label("Position:");
                            ui.add(egui::DragValue::new(&mut 0.0).prefix("X: "));
                            ui.add(egui::DragValue::new(&mut 0.0).prefix("Y: "));
                            ui.add(egui::DragValue::new(&mut 0.0).prefix("Z: "));
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Rotation:");
                            ui.add(egui::DragValue::new(&mut 0.0).prefix("X: "));
                            ui.add(egui::DragValue::new(&mut 0.0).prefix("Y: "));
                            ui.add(egui::DragValue::new(&mut 0.0).prefix("Z: "));
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Scale:");
                            ui.add(egui::DragValue::new(&mut 1.0).prefix("X: "));
                            ui.add(egui::DragValue::new(&mut 1.0).prefix("Y: "));
                            ui.add(egui::DragValue::new(&mut 1.0).prefix("Z: "));
                        });
                    });
                });
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.label("Status: Ready");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("FPS: {:.1}", ctx.input(|i| 1.0 / i.stable_dt)));
                    });
                });
            });
        });
        
        //Handle UI output via Egui
        egui_state.handle_platform_output(window, full_output.platform_output);
        
        //Converts shapes into triangles meshes
        let paint_jobs = egui_context.tessellate(full_output.shapes, full_output.pixels_per_point);
        
        // Render via WGPU using EGUI renderer
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
}

// pub(crate) struct Panel{
//     pub(crate) num: usize,
// }

// pub(crate) struct TreeBehavior{}

// impl egui_tiles::Behavior<Panel> for TreeBehavior {
//     fn tab_title_for_pane(&mut self, pane: &Panel) -> egui::WidgetText {
//         format!("Pane {}", pane.num).into()
//     }

//     fn pane_ui(&mut self, ui: &mut egui::Ui, _tile_id: egui_tiles::TileId, panel: &mut Panel) -> egui_tiles::UiResponse {
//         // Give each pane a unique color:
//         let color = egui::epaint::Hsva::new(0.103 * panel.num as f32, 0.5, 0.5, 1.0);
//         ui.painter().rect_filled(ui.max_rect(), 0.0, color);

//         ui.label(format!("The contents of pane {}.", panel.num));

//         // You can make your pane draggable like so:
//         if ui
//             .add(egui::Button::new("Drag me!").sense(egui::Sense::drag()))
//             .drag_started()
//         {
//             egui_tiles::UiResponse::DragStarted
//         } else {
//             egui_tiles::UiResponse::None
//         }
//     }
// }