use crate::types::component_type;

type Float = f32;

pub struct ComponentData {
    pub rigid: Vec<Option<Box<component_type::Rigid<Float>>>>,
    pub shape: Vec<Option<Box<component_type::Shape<Float>>>>,
}
