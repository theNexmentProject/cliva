mod print;
mod semantic;

pub use print::{get_text, print};
pub use semantic::{Semantics, semantic};

#[derive(Debug, Clone)]
pub enum Style<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
    Success,
    Warning,
    Info,
    Error,
}
