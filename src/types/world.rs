use crate::types::{component, component_data, component_store, component_type, entity, system};
use glsl_linalg::float;

pub struct World<F>
where
    F: float::Float,
{
    pub entity: entity::Entity,
    pub component_data: component_data::ComponentData<F>,
    pub system: system::SystemCollection<F>,
    pub init_system: system::SystemCollection<F>,
}

impl<F> World<F>
where
    F: float::Float,
{
    pub fn new() -> Self {
        World {
            entity: entity::Entity::new(),
            component_data: component_data::ComponentData::new(),
            system: system::SystemCollection::new(),
            init_system: system::SystemCollection::new(),
        }
    }

    pub fn add_system(&mut self, system: system::System<F>) {
        self.system.push(system);
    }

    pub fn add_init_system(&mut self, system: system::System<F>) {
        self.init_system.push(system)
    }

    pub fn remove_component_data(&mut self, index: usize) {
        self.component_data.rigid.0[index] = None;
        self.component_data.shape.0[index] = None;
        self.entity.remove(index);
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

impl<F> component_store::ComponentStore<component_type::Rigid<F>> for World<F>
where
    F: float::Float,
{
    fn push(&mut self, d: component_type::Rigid<F>, index: usize) {
        if !self.entity.entity[index].contains(component::Component::RIGID) {
            return;
        }

        let rigid = &mut self.component_data.rigid.0;
        if rigid.len() <= index {
            rigid.resize_with((1 + index).next_power_of_two(), Default::default);
        }
        rigid[index] = Some(Box::new(d));
    }

    fn get(
        &self,
        c: component::Component,
        index: usize,
    ) -> Option<&Option<Box<component_type::Rigid<F>>>> {
        if c == component::Component::RIGID {
            Some(&self.component_data.rigid.0[index])
        } else {
            None
        }
    }

    fn get_mut(
        &mut self,
        c: component::Component,
        index: usize,
    ) -> Option<&mut Option<Box<component_type::Rigid<F>>>> {
        if c == component::Component::RIGID {
            Some(&mut self.component_data.rigid.0[index])
        } else {
            None
        }
    }
}

impl<F> component_store::ComponentStore<component_type::Shape<F>> for World<F>
where
    F: float::Float,
{
    fn push(&mut self, d: component_type::Shape<F>, index: usize) {
        if !self.entity.entity[index].contains(component::Component::SHAPE) {
            return;
        }

        let shape = &mut self.component_data.shape.0;
        if shape.len() <= index {
            shape.resize_with((1 + index).next_power_of_two(), Default::default);
        }
        shape[index] = Some(Box::new(d));
    }

    fn get(
        &self,
        c: component::Component,
        index: usize,
    ) -> Option<&Option<Box<component_type::Shape<F>>>> {
        if c == component::Component::RIGID {
            Some(&self.component_data.shape.0[index])
        } else {
            None
        }
    }

    fn get_mut(
        &mut self,
        c: component::Component,
        index: usize,
    ) -> Option<&mut Option<Box<component_type::Shape<F>>>> {
        if c == component::Component::RIGID {
            Some(&mut self.component_data.shape.0[index])
        } else {
            None
        }
    }
}
