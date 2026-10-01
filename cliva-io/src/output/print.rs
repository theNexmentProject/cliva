use crate::{output::Style, shared::get_ansi};

const RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Copy)]
pub enum Semantics {
    Success,
    Error,
    Info,
    Warning,
}

pub fn print(text: &str, style: Style<'_>) {
    println!("{}{text}{RESET}", get_ansi(style));
}

pub fn get_text(text: &str, style: Style<'_>) -> String {
    format!("{}{text}{RESET}", get_ansi(style))
}

pub fn semantic(text: &str, kind: Semantics) {
    let (label, label_style, text_style) = match kind {
        Semantics::Success => (
            "✓ SUCCESS",
            Style::Multiple(vec!["bright_green", "bold"]),
            Style::Multiple(vec!["bold", "green"]),
        ),
        Semantics::Error => (
            "✖ ERROR",
            Style::Multiple(vec!["bold", "red"]),
            Style::Error,
        ),
        Semantics::Info => ("!INFO", Style::Multiple(vec!["bold", "blue"]), Style::Info),
        Semantics::Warning => (
            "⚠ WARN",
            Style::Multiple(vec!["yellow", "bold"]),
            Style::Warning,
        ),
    };

    println!(
        "{} {}",
        get_text(label, label_style),
        get_text(text, text_style)
    );
}
