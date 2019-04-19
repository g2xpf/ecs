use crate::types::{component_data, entity};
use glsl_linalg::float;

pub type System<F> = fn(&mut entity::Entity, &mut component_data::ComponentData<F>);

#[derive(Default)]
pub struct SystemCollection<F>(Vec<System<F>>)
where
    F: float::Float;

impl<F> SystemCollection<F>
where
    F: float::Float,
{
    pub fn new() -> Self {
        SystemCollection(Vec::new())
    }

    pub fn push(&mut self, system: System<F>) {
        self.0.push(system);
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, System<F>> {
        self.0.iter()
    }
}
