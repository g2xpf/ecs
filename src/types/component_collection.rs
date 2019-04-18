#[derive(Default, Debug)]
pub struct ComponentCollection<T>(Vec<Option<Box<T>>>);

impl<T> ComponentCollection<T> {
    pub fn new() -> Self {
        ComponentCollection(Vec::new())
    }
}
