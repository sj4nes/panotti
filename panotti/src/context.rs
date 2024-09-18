use crate::attributes::Attribute;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Context {
    pub name: String,
    pub attributes: HashSet<Attribute>,
}
impl Default for Context {
    fn default() -> Self {
        Self {
            name: String::from("default"),
            attributes: HashSet::new(),
        }
    }
}
impl Context {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }
    pub fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.insert(attribute);
    }
    pub fn remove_attribute(&mut self, attribute: Attribute) {
        self.attributes.remove(&attribute);
    }
}
