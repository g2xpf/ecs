use crate::types::component::Component;
use crate::types::component_type;
use crate::types::system::System;
use glium;
use glium::glutin;

pub fn init_gl() -> System {
    System {
        requirements: Component::NONE,
        body: |component_data, _index| {
            let events_loop = glium::glutin::EventsLoop::new();
            let window = glium::glutin::WindowBuilder::new()
                .with_title("GLWindow")
                .with_dimensions(glutin::dpi::LogicalSize::new(640.0, 640.0));
            let context = glium::glutin::ContextBuilder::new().with_vsync(true);
            let display = glium::Display::new(window, context, &events_loop).unwrap();
            component_data.event_handler =
                Some(Box::new(component_type::EventHandler { events_loop }));
            component_data.renderer = Some(Box::new(component_type::Renderer::new(display)));
        },
    }
}
