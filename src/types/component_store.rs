use crate::types::component;

pub trait ComponentStore<D> {
    fn push(&mut self, d: D, index: usize);
    fn get(&self, c: component::Component, index: usize) -> Option<&Option<Box<D>>>;
    fn get_mut(&mut self, c: component::Component, index: usize) -> Option<&mut Option<Box<D>>>;
}
