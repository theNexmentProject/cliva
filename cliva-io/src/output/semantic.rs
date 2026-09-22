use super::print::{Style, get_text, print};

pub enum Semantics {
	Success,
	Error,
	Info,
	Warning,
}

pub fn semantics(text: &str, semantic: Semantics) {
	match semantic {
		Semantics::Success => println!(
			"{} {}",
			get_text("✓ SUCCESS", Style::Multiple(vec!["bold", "green"])),
			text
		),
		Semantics::Info => println!(
			"{} {}",
			get_text("! INFO", Style::Multiple(vec!["bold", "blue"])),
			text
		),
		Semantics::Warning => println!(
			"{} {}",
			get_text("⚠ WARNING", Style::Multiple(vec!["bold", "yellow"])),
			text
		),
		Semantics::Error => println!(
			"{} {}",
			get_text("✗ ERROR", Style::Multiple(vec!["bold", "red"])),
			text
		),
	}
}

pub fn success(text: &str) {
	print(text, Style::Multiple(vec!["bold", "green"]));
}
pub fn info(text: &str) {
	print(text, Style::Multiple(vec!["bold", "blue"]));
}
pub fn warning(text: &str) {
	print(text, Style::Multiple(vec!["bold", "yellow"]));
}
pub fn error(text: &str) {
	print(text, Style::Multiple(vec!["bold", "red"]));
}
