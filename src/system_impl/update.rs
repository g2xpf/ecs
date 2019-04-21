use crate::types::component::Component;
use crate::types::system::System;
use glium::Surface;

pub fn update_system() -> System {
    System {
        requirements: Component::RIGID,
        body: |component_data, index| match component_data.global_config {
            Some(ref gc) => match component_data.rigid[index] {
                Some(ref mut rigid) => {
                    rigid.a = gc.gravity;

                    rigid.v += rigid.a * gc.dt;
                    rigid.r += rigid.v * gc.dt;
                }
                None => {}
            },
            None => {}
        },
    }
}
