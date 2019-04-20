use crate::types::component::Component;
use crate::types::system::System;
use glium::Surface;

pub fn clear() -> System {
    System {
        requirements: Component::NONE,
        body: |component_data, _index| match component_data.renderer {
            Some(ref mut renderer) => {
                let mut target = renderer.display.draw();
                target.clear_color(0.01, 0.01, 0.01, 1.0);
                (*renderer).target = Some(target);
            }
            None => {
                panic!("Renderer not created");
            }
        },
    }
}

pub fn flush() -> System {
    System {
        requirements: Component::NONE,
        body: |component_data, _index| match component_data.renderer {
            Some(ref mut renderer) => renderer.target.as_mut().unwrap().set_finish().unwrap(),
            None => {
                panic!("Renderer not created");
            }
        },
    }
}

pub fn render_system() -> System {
    System {
        requirements: Component::RIGID | Component::SHAPE,
        body: |component_data, index| match component_data.renderer {
            Some(ref mut renderer) => {
                let rigid = component_data.rigid[index].as_ref().unwrap();
                let shape = component_data.shape[index].as_ref().unwrap();
                renderer.draw(rigid, shape);
            }
            None => {
                panic!("Renderer not initialized!");
            }
        },
    }
}
