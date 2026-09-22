use std::io::{self, Write};
use std::process;

use super::output::{Semantics, semantics};

use crossterm::{
	cursor,
	event::{self, Event, KeyCode, KeyModifiers},
	execute,
	style::{Color, Print, ResetColor, SetForegroundColor},
	terminal::{disable_raw_mode, enable_raw_mode},
};

#[derive(Debug, Clone, Copy)]
pub enum InputType {
	String,
	Int,
	Float,
}

#[derive(Debug, Clone)]
pub enum InputValue {
	String(String),
	Int(i64),
	Float(f64),
}

pub fn prompt(text: &str, default: &str, input_type: InputType) -> InputValue {
	loop {
		match read_input(text, default, input_type) {
			Some(value) => return value,
			None => {
				semantics(
					"Invalid input. Please enter a valid value.",
					Semantics::Error,
				);
			}
		}
	}
}

fn read_input(text: &str, default: &str, input_type: InputType) -> Option<InputValue> {
	let mut input = String::new();
	let mut showing_default = true;

	print_prompt(text, default);

	enable_raw_mode().unwrap();

	loop {
		if let Event::Key(key) = event::read().unwrap() {
			// Ctrl + C
			if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
				disable_raw_mode().unwrap();

				semantics("Input cancelled. Exiting.", Semantics::Error);

				process::exit(1);
			}

			match key.code {
				KeyCode::Char(c) => {
					if showing_default {
						clear_default(default.len());
						showing_default = false;
					}

					input.push(c);
					print!("{c}");
					io::stdout().flush().unwrap();
				}

				KeyCode::Backspace => {
					if !input.is_empty() {
						input.pop();

						print!("\x08 \x08");

						if input.is_empty() {
							showing_default = true;
							print_dimmed(default);
						}

						io::stdout().flush().unwrap();
					}
				}

				KeyCode::Enter => {
					disable_raw_mode().unwrap();
					println!();

					let value = if showing_default || input.is_empty() {
						default.to_string()
					} else {
						input
					};

					return parse_value(value, input_type);
				}

				KeyCode::Esc => {
					disable_raw_mode().unwrap();
					println!();

					return parse_value(default.to_string(), input_type);
				}

				_ => {}
			}
		}
	}
}

fn parse_value(value: String, input_type: InputType) -> Option<InputValue> {
	match input_type {
		InputType::String => Some(InputValue::String(value)),

		InputType::Int => value.parse::<i64>().ok().map(InputValue::Int),

		InputType::Float => value.parse::<f64>().ok().map(InputValue::Float),
	}
}

fn print_prompt(text: &str, default: &str) {
	execute!(
		io::stdout(),
		SetForegroundColor(Color::Cyan),
		Print("❯"),
		ResetColor,
	)
	.unwrap();

	print!(" {text}");

	execute!(
		io::stdout(),
		SetForegroundColor(Color::DarkGrey),
		Print(":"),
		ResetColor,
	)
	.unwrap();

	print!(" ");
	print_dimmed(default);

	io::stdout().flush().unwrap();
}

fn print_dimmed(text: &str) {
	execute!(
		io::stdout(),
		SetForegroundColor(Color::DarkGrey),
		Print(text),
		ResetColor,
	)
	.unwrap();
}

fn clear_default(len: usize) {
	execute!(
		io::stdout(),
		cursor::MoveLeft(len as u16),
		Print(" ".repeat(len)),
		cursor::MoveLeft(len as u16),
	)
	.unwrap();
}
