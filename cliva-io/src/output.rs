mod loader;
mod print;
mod progress;
mod title;

pub use loader::{Loader, loader};
pub use print::{Semantics, get_text, print, semantic};
pub use progress::{Progress, progress};
pub use title::{Align, Title, title};

#[derive(Debug, Clone)]
pub enum Style<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
    Success,
    Warning,
    Info,
    Error,
}
