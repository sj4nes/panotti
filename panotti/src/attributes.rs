#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}
impl Default for Attribute {
    fn default() -> Self {
        Self {
            name: String::from("default"),
            value: String::from("default"),
        }
    }
}
impl Attribute {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
