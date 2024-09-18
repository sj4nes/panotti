use crate::context::Context;

/// The mode of the app determines what kinds of input the app is listening for
#[derive(Debug)]
pub enum Mode {
    /// Locked mode, listening for a passphrase to unlock the assistant
    Locked,

    /// Idle mode, listening for a wake word to switch to normal mode
    Idle,

    // Listening for commands or dictation
    Normal,

    /// Dictating text with speech-to-text with modal audio macros that are contextual
    Dictating,

    /// Listening for a command to apply in a context, such as a command to apply to a selected
    /// text or starting of dictation
    Command,
}

struct Module {}

struct Reaction {
    name: String,
    context: Context,
}

#[derive(Debug)]
pub struct App {
    pub current_context: Context,
    pub mode: Mode,
    pub exit: bool,
}
impl Default for App {
    fn default() -> Self {
        Self {
            current_context: Context::default(),
            mode: Mode::Normal,
            exit: false,
        }
    }
}
