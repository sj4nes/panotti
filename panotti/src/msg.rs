use crate::attributes::Attribute;

pub enum Msg {
    Started,
    Stopping,
    ContextAdded { attributes: Vec<Attribute> },
    ContextRemoved { attributes: Vec<Attribute> },
    ClearMessages,
}
