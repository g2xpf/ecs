use crate::types::component::Component;
use crate::types::{component_data, component_store, component_type, entity, system};

pub struct World<'a> {
    pub entity: entity::Entity,
    pub component_data: component_data::ComponentData<'a>,
    pub before: system::SystemCollection,
    pub system: system::SystemCollection,
    pub after: system::SystemCollection,
    pub init_system: system::SystemCollection,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        World {
            entity: entity::Entity::new(),
            component_data: component_data::ComponentData::new(),
            before: system::SystemCollection::new(),
            system: system::SystemCollection::new(),
            after: system::SystemCollection::new(),
            init_system: system::SystemCollection::new(),
        }
    }

    pub fn add_system(&mut self, system: system::System) {
        self.system.push(system);
    }

    pub fn add_before(&mut self, system: system::System) {
        self.before.push(system);
    }

    pub fn add_after(&mut self, system: system::System) {
        self.after.push(system);
    }

    pub fn add_init_system(&mut self, system: system::System) {
        self.init_system.push(system);
    }

    pub fn remove_component_data(&mut self, index: usize) {
        self.component_data.rigid[index] = None;
        self.component_data.shape[index] = None;
        self.entity.remove(index);
    }

    pub fn init(&mut self) {
        for system in self.init_system.iter() {
            for (i, e) in self.entity.entity.iter().enumerate() {
                if (*e).contains(system.requirements) {
                    (system.body)(&mut self.component_data, i);
                }
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            for before in self.before.iter() {
                (before.body)(&mut self.component_data, 0);
            }
            for system in self.system.iter() {
                for (i, e) in self.entity.entity.iter().enumerate() {
                    if (*e).contains(system.requirements) {
                        (system.body)(&mut self.component_data, i);
                    }
                }
            }
            for after in self.after.iter() {
                (after.body)(&mut self.component_data, 0);
            }
        }
    }
}

impl<'a> component_store::ComponentStore<component_type::Rigid> for World<'a> {
    fn push(&mut self, d: component_type::Rigid, index: usize) {
        if !self.entity.entity[index].contains(Component::RIGID) {
            return;
        }

        let rigid = &mut self.component_data.rigid;
        if rigid.len() <= index {
            rigid.resize_with((1 + index).next_power_of_two(), Default::default);
        }
        rigid[index] = Some(Box::new(d));
    }

    fn get(&self, c: Component, index: usize) -> Option<&Option<Box<component_type::Rigid>>> {
        if c == Component::RIGID {
            Some(&self.component_data.rigid[index])
        } else {
            None
        }
    }

    fn get_mut(
        &mut self,
        c: Component,
        index: usize,
    ) -> Option<&mut Option<Box<component_type::Rigid>>> {
        if c == Component::RIGID {
            Some(&mut self.component_data.rigid[index])
        } else {
            None
        }
    }
}

impl<'a> component_store::ComponentStore<component_type::Shape> for World<'a> {
    fn push(&mut self, d: component_type::Shape, index: usize) {
        if !self.entity.entity[index].contains(Component::SHAPE) {
            return;
        }

        let shape = &mut self.component_data.shape;
        if shape.len() <= index {
            shape.resize_with((1 + index).next_power_of_two(), Default::default);
        }
        shape[index] = Some(Box::new(d));
    }

    fn get(&self, c: Component, index: usize) -> Option<&Option<Box<component_type::Shape>>> {
        if c == Component::RIGID {
            Some(&self.component_data.shape[index])
        } else {
            None
        }
    }

    fn get_mut(
        &mut self,
        c: Component,
        index: usize,
    ) -> Option<&mut Option<Box<component_type::Shape>>> {
        if c == Component::RIGID {
            Some(&mut self.component_data.shape[index])
        } else {
            None
        }
    }
}
