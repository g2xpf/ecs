use crate::types::component::Component;
use crate::types::component_data;

pub struct System {
    pub requirements: Component,
    pub body: fn(&mut component_data::ComponentData, index: usize),
}

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
