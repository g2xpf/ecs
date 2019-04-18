use crate::types::system::System;
use glium::Surface;

pub fn render_system() -> System {
    |_entity, component_data| match component_data.renderer {
        Some(ref render) => {
            let mut target = render.display.draw();
            target.clear_color(0.01, 0.01, 0.01, 1.0);
            target.finish().unwrap();
        }
        None => (),
    }
}
