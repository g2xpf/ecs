use crate::types::{component_data, entity, system};

pub struct World {
    pub entity: entity::Entity,
    pub component_data: component_data::ComponentData,
    pub system: system::SystemCollection,
    pub init_system: system::SystemCollection,
}

impl World {
    pub fn new() -> Self {
        World {
            entity: entity::Entity::new(),
            component_data: component_data::ComponentData::new(),
            system: system::SystemCollection::new(),
            init_system: system::SystemCollection::new(),
        }
    }

    pub fn add_system(&mut self, system: system::System) {
        self.system.push(system);
    }

    pub fn add_init_system(&mut self, system: system::System) {
        self.init_system.push(system)
    }

    pub fn init(&mut self) {
        for system in self.init_system.iter() {
            system(&mut self.entity, &mut self.component_data);
        }
    }

    pub fn run(&mut self) {
        loop {
            for system in self.system.iter() {
                system(&mut self.entity, &mut self.component_data);
            }
        }
    }
}
