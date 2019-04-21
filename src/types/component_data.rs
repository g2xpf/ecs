use crate::types::{component_collection, component_type};

pub struct ComponentData<'a> {
    pub rigid: component_collection::ComponentCollection<component_type::Rigid>,
    pub shape: component_collection::ComponentCollection<component_type::Shape>,
    pub global_config: Option<Box<component_type::GlobalConfig>>,
    pub renderer: Option<Box<component_type::Renderer<'a>>>,
    pub event_handler: Option<Box<component_type::EventHandler>>,
}

impl<'a> ComponentData<'a> {
    pub fn new() -> Self {
        ComponentData {
            rigid: component_collection::ComponentCollection::new(),
            shape: component_collection::ComponentCollection::new(),
            global_config: None,
            renderer: None,
            event_handler: None,
        }
    }
}
