use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::{event_loop::EventLoop, window::Window};
use winit::application::ApplicationHandler;

fn main() {
    match EventLoop::new(){
        Ok(event_loop) => {
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            event_loop.set_control_flow(ControlFlow::Poll);

            // ControlFlow::Wait pauses the event loop if no events are available to process.
            // This is ideal for non-game applications that only update in response to user
            // input, and uses significantly less power/CPU time than ControlFlow::Poll.
            //event_loop.set_control_flow(ControlFlow::Wait);

            let mut app = EditorWindow::default();
            let result = event_loop.run_app(&mut app);

            if result.is_err(){
                eprintln!("Error running app: {:?}", result.unwrap_err());
            }
        },
        Err(err) => {
            eprintln!("Error creatin event loop: {:?}", err)
        },
    }
}

#[derive(Default)]
struct EditorWindow{
    window: Option<Window>
}

impl ApplicationHandler for EditorWindow{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match event_loop.create_window(Window::default_attributes()){
            Ok(window) => {
                self.window = Some(window);
            },
            Err(err) => {
                eprintln!("Error creating window: {:?}", err);
            },
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
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
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}