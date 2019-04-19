use crate::types::component;

#[derive(Default, Debug)]
pub struct Entity {
    pub entity: Vec<component::Component>,
    pub garbage_stack: Vec<usize>,
}

impl Entity {
    pub fn new() -> Self {
        Entity {
            entity: Vec::new(),
            garbage_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, c: component::Component) -> usize {
        match self.garbage_stack.pop() {
            Some(index) => {
                self.entity[index] = c;
                return index;
            }
            None => {
                self.entity.push(c);
                return self.entity.len() - 1;
            }
        }
    }

    pub fn remove_component(&mut self, c: component::Component, index: usize) {
        self.entity[index] &= !c;
    }

    pub fn set_component(&mut self, c: component::Component, index: usize) {
        self.entity[index] = c;
    }

    pub fn reset_component(&mut self, index: usize) {
        self.set_component(component::Component::NONE, index);
    }

    pub fn remove(&mut self, index: usize) {
        self.reset_component(index);
        self.garbage_stack.push(index);
    }
}
