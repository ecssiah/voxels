use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod interface;
mod simulation;

use crate::interface::Interface;
use crate::simulation::Simulation;

#[derive(Default)]
struct App {
    interface: Option<Interface>,
    simulation: Option<Simulation>,
}

impl App {
    fn resume_simulation(&mut self) {
        let simulation = pollster::block_on(Simulation::new());
        self.simulation = Some(simulation);
    }

    fn resume_interface(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let interface = pollster::block_on(Interface::new(window.clone()));
        self.interface = Some(interface);

        window.request_redraw();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.resume_simulation();
        self.resume_interface(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let interface = self.interface.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                interface.render();
                interface.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                interface.resize(size);
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                interface.handle_keyboard_input(device_id, event, is_synthetic);
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                interface.handle_mouse_input(device_id, state, button);
            }
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => {
                interface.handle_mouse_wheel(device_id, delta, phase);
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                interface.handle_cursor_moved(device_id, position);
            }
            _ => (),
        }
    }
}

pub fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
