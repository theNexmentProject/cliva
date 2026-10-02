mod prompt;
mod selection;

pub use prompt::{InputType, InputValue, confirm, password, prompt};
pub use selection::{checkbox_selection, inline_selection, vertical_selection};
