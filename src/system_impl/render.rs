use crate::types::system::System;
use glium::Surface;
use glsl_linalg::float;

pub fn render_system<F>() -> System<F>
where
    F: float::Float + std::fmt::Debug,
{
    |_entity, component_data| match component_data.renderer {
        Some(ref render) => {
            let mut target = render.display.draw();
            target.clear_color(0.01, 0.01, 0.01, 1.0);
            target.finish().unwrap();
        }
        None => (),
    }
}
