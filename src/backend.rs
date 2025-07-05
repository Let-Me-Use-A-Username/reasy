use winit::{window::Window};
use wgpu::{Device, Instance, Queue, Surface, SurfaceConfiguration, SurfaceTargetUnsafe};
use egui_wgpu::Renderer;


///WGPU Rendering backend.
/// 
/// Handles:
///     - Rendering target:  surface, surface texture, view
///     - Rendering tools: device, queue, encoder
///     - Configuration: config, descriptors
pub(crate) struct WgpuState{
    surface: Surface<'static>,
    pub(crate) device: Device,
    queue: Queue,
    pub(crate) config: SurfaceConfiguration,
    pub(crate) size: winit::dpi::PhysicalSize<u32>
}
impl WgpuState{
    pub(crate) async fn new(window: &Window) -> WgpuState{
        //WGPU entry point.
        let instance = Instance::new(&wgpu::InstanceDescriptor{
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        //Rendering target
        let surface = unsafe{ 
            let inst = SurfaceTargetUnsafe::from_window(window).unwrap();
            instance.create_surface_unsafe(inst).unwrap()
        };
        //Abstraction to rendering device (GPU)
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptionsBase { 
                power_preference: wgpu::PowerPreference::HighPerformance, 
                force_fallback_adapter: false, 
                compatible_surface: Some(&surface) 
            }
        ).await.unwrap();

        //Retrieve the device and queue
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor{
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            }, 
            None,
        ).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats.iter()
            .find(|form| form.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]
        );
        
        let inner_size = window.inner_size();
        
        //Surface descriptor regarding graphical presentation specifics, of provided `Window`. 
        let config = SurfaceConfiguration{
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: inner_size.width,
            height: inner_size.height,
            present_mode: surface_capabilities.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        return WgpuState { 
            surface: surface,
            device: device,
            queue: queue,
            config: config,
            size: inner_size
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.height = new_size.height;
            self.config.width = new_size.width;
            self.surface.configure(&self.device, &self.config);
        }
    }
    
    pub(crate) fn render(&mut self, egui_renderer: &mut Renderer, paint_jobs: Vec<egui::ClippedPrimitive>, textures_delta: egui::TexturesDelta) -> Result<(), wgpu::SurfaceError> {
        //Get the next surface texture
        let output = self.surface.get_current_texture()?;
        //Create view from texture
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        //Create GPU operation encoder.
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        // Update EGUI textures
        for (id, image_delta) in &textures_delta.set {
            egui_renderer.update_texture(&self.device, &self.queue, *id, image_delta);
        }
        
        // Create screen descriptor for EGUI
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: 1.0,
        };
        
        // Update EGUI buffers with next operations
        egui_renderer.update_buffers(&self.device, &self.queue, &mut encoder, &paint_jobs, &screen_descriptor);
        
        // Create render pass descriptor
        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };
        
        // Render pass. Block is needed to drop the reference to the encoder.
        {
            let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor).forget_lifetime();
            // Render egui
            egui_renderer.render(&mut render_pass, &paint_jobs, &screen_descriptor);
        }
        
        // Submit commandto GPU for execution
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        // Free egui textures
        for id in &textures_delta.free {
            egui_renderer.free_texture(id);
        }
        
        Ok(())
    }
}

