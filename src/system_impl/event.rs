use crate::types::component::Component;
use crate::types::system::System;
use glium;
use glium::glutin;

pub fn event_hook() -> System {
    System {
        requirements: Component::NONE,
        body: |component_data, _index| match component_data.event_handler {
            Some(ref mut handler) => {
                handler.events_loop.poll_events(|ev| match ev {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested
                        | glutin::WindowEvent::KeyboardInput {
                            input:
                                glutin::KeyboardInput {
                                    scancode: 16,
                                    state: glutin::ElementState::Pressed,
                                    ..
                                },
                            ..
                        } => std::process::exit(0),
                        _ => (),
                    },
                    _ => (),
                });
            }
            None => (),
        },
    }
}
