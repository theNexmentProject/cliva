use crate::output::{Style, get_text};

pub enum Semantics {
    Success,
    Error,
    Info,
    Warning,
}

pub fn semantic(text: &str, look: Semantics) {
    match look {
        Semantics::Success => println!(
            "{} : {}",
            get_text("✓ SUCCESS", Style::Multiple(vec!["bright_green", "bold"])),
            get_text(text, Style::Multiple(vec!["bold", "green"]))
        ),
        Semantics::Error => println!(
            "{} {}",
            get_text("✖ ERROR", Style::Multiple(vec!["bold", "red"])),
            get_text(text, Style::Error)
        ),
        Semantics::Info => println!(
            "{} {}",
            get_text("!INFO", Style::Multiple(vec!["bold", "blue"])),
            get_text(text, Style::Info)
        ),
        Semantics::Warning => println!(
            "{} {}",
            get_text("⚠ WARN", Style::Multiple(vec!["yellow", "bold"])),
            get_text(text, Style::Warning)
        ),
    }
}
