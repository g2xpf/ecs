use crate::types::{component_data, entity};

pub type System = fn(&mut entity::Entity, &mut component_data::ComponentData);

#[derive(Default)]
pub struct SystemCollection(Vec<System>);

impl SystemCollection {
    pub fn new() -> Self {
        SystemCollection(Vec::new())
    }

    pub fn push(&mut self, system: System) {
        self.0.push(system);
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, System> {
        self.0.iter()
    }
}
