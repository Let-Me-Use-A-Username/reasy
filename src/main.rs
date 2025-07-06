use once_cell::sync::OnceCell;
use std::sync::Arc;

use winit::event_loop::{ControlFlow, EventLoopProxy};
use winit::{event_loop::EventLoop};

mod core;
mod utils;
mod event;

use crate::core::editor::editor::EditorWindow;
use crate::event::UserEvent;


static USER_EVENT_PROXY: OnceCell<Arc<EventLoopProxy<UserEvent>>> = OnceCell::new();

fn main() {
    //Main event dispatcher for OS calls
    let event_loop_status = EventLoop::<UserEvent>::with_user_event().build();
    
    match event_loop_status{
        Ok(event_loop) => {
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            event_loop.set_control_flow(ControlFlow::Poll);

            // ControlFlow::Wait pauses the event loop if no events are available to process.
            // This is ideal for non-game applications that only update in response to user
            // input, and uses significantly less power/CPU time than ControlFlow::Poll.
            //event_loop.set_control_flow(ControlFlow::Wait);
            let _ = USER_EVENT_PROXY.set(Arc::new(event_loop.create_proxy()));

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