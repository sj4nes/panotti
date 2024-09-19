use crate::attributes::Attribute;

pub enum Msg {
    NoOp,
    Started,
    Stopping,
    ContextAdded {
        attributes: Vec<Attribute>,
    },
    ContextRemoved {
        attributes: Vec<Attribute>,
    },
    ClearMessages,
    /// Debug the cpal->whisper wav2txt pipeline
    ToggleTranscriber,
}
