mod print;
mod semantic;
mod title;

pub use print::{get_text, print};
pub use semantic::{Semantics, semantic};
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
