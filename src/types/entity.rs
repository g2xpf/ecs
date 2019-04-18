use crate::types::component;

#[derive(Default, Debug)]
pub struct Entity(pub Vec<component::Component>);

impl Entity {
    pub fn new() -> Self {
        Entity(Vec::new())
    }
}
