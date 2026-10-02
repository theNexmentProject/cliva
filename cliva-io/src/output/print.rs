use crate::{output::Style, shared::get_ansi};
use std::collections::HashMap;
use std::sync::LazyLock;

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

static ICONS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../data/icons.json")).expect("Invalid Cliva icon registry")
});

pub fn icon(slug: &str) -> String {
    ICONS
        .get(&slug.trim().to_lowercase())
        .cloned()
        .unwrap_or_else(|| slug.to_string())
}
