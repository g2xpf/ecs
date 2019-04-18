use crate::types::{component_data, entity};

struct World {
    entity: entity::Entity,
    component_data: component_data::ComponentData,
    system: Vec<Box<Fn(&mut component_data::ComponentData)>>,
}
