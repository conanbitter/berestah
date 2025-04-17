use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

#[derive(Default)]
struct AppState {
    window: Option<Window>,
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
        // center window
        let window = self.window.as_ref().unwrap();
        let mon_size = window.primary_monitor().unwrap().size();
        let window_size = window.outer_size();
        window.set_outer_position(PhysicalPosition::new(
            (mon_size.width - window_size.width) / 2,
            (mon_size.height - window_size.height) / 2,
        ));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }

            _ => {}
        }
    }
}
fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = AppState::default();
    event_loop.run_app(&mut app).unwrap();
}
