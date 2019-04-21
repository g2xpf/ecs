use crate::types::component::Component;
use crate::types::component_type::GlobalConfig;
use crate::types::system::System;
use cgmath::Vector2;

pub fn init() -> System {
    System {
        requirements: Component::NONE,
        body: |component_data, _index| {
            component_data.global_config = Some(Box::new(GlobalConfig {
                gravity: Vector2::new(0.0, -9.8),
                dt: 0.016,
                scale: 0.1,
            }));
        },
    }
}
