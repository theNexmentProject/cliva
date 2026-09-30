use crate::output::Style;
use crate::shared::get_ansi;

const RETURN: &str = "\x1b[0m";

pub fn print(text: &str, style: Style<'_>) {
    println!("{}{}{}", get_ansi(style), text, RETURN);
}

pub fn get_text(text: &str, style: Style<'_>) -> String {
    format!("{}{}{}", get_ansi(style), text, RETURN)
}
