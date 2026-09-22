use super::get_ansi;

pub enum Style<'a> {
	Single(&'a str),
	Multiple(Vec<&'a str>),
}

const RETURN: &str = "\x1b[0m";

pub fn print(text: &str, style: Style) {
	let ansi_code: String = get_ansi(style);
	println!("{ansi_code}{text}{RETURN}");
}

pub fn get_text(text: &str, style: Style) -> String {
	let ansi_code: String = get_ansi(style);
	format!("{ansi_code}{text}{RETURN}")
}
