use crate::types::{component_collection, component_type};

type Float = f32;

pub struct ComponentData {
    pub rigid: component_collection::ComponentCollection<component_type::Rigid<Float>>,
    pub shape: component_collection::ComponentCollection<component_type::Shape<Float>>,
    pub renderer: Option<Box<component_type::Renderer>>,
    pub event_handler: Option<Box<component_type::EventHandler>>,
}

impl ComponentData {
    pub fn new() -> Self {
        ComponentData {
            rigid: component_collection::ComponentCollection::new(),
            shape: component_collection::ComponentCollection::new(),
            renderer: None,
            event_handler: None,
        }
    }
}
