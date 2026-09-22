mod print;
mod semantic;
mod title;

use crate::shared::get_ansi;

pub use print::{Style, get_text, print};
pub use semantic::{Semantics, error, info, semantics, success, warning};
pub use title::{Title, TitleAlign, title};
