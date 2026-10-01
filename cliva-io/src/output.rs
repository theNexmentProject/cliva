mod list;
mod loader;
mod print;
mod progress;
mod table;
mod title;

pub use list::{List, list, nested_list};
pub use loader::{Loader, loader};
pub use print::{Semantics, get_text, print, semantic, icon};
pub use progress::{Progress, progress};
pub use table::Table;
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
