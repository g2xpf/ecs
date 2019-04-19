use crate::types::{component_collection, component_type};
use glsl_linalg::float;

pub struct ComponentData<F>
where
    F: float::Float,
{
    pub rigid: component_collection::ComponentCollection<component_type::Rigid<F>>,
    pub shape: component_collection::ComponentCollection<component_type::Shape<F>>,
    pub renderer: Option<Box<component_type::Renderer>>,
    pub event_handler: Option<Box<component_type::EventHandler>>,
}

impl<F> ComponentData<F>
where
    F: float::Float,
{
    pub fn new() -> Self {
        ComponentData {
            rigid: component_collection::ComponentCollection::new(),
            shape: component_collection::ComponentCollection::new(),
            renderer: None,
            event_handler: None,
        }
    }
}
