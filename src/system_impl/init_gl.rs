use crate::types::component_type;
use crate::types::system::System;
use glium;

pub fn init_gl() -> System {
    |_entity, component_data| {
        let events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new();
        let context = glium::glutin::ContextBuilder::new().with_vsync(true);
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        component_data.event_handler = Some(Box::new(component_type::EventHandler { events_loop }));
        component_data.renderer = Some(Box::new(component_type::Renderer { display }));
    }
}
