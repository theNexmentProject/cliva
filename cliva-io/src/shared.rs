use std::collections::HashMap;
use std::sync::LazyLock;

use crate::output::Style;

static ANSI_CODES: LazyLock<HashMap<&'static str, u8>> = LazyLock::new(|| {
	HashMap::from([
		// Text styles
		("reset", 0),
		("bold", 1),
		("dim", 2),
		("italic", 3),
		("underline", 4),
		("hidden", 8),
		("strikethrough", 9),
		// Standard foreground colors
		("black", 30),
		("red", 31),
		("green", 32),
		("yellow", 33),
		("blue", 34),
		("magenta", 35),
		("cyan", 36),
		("white", 37),
		// Bright foreground colors
		("bright_black", 90),
		("bright_red", 91),
		("bright_green", 92),
		("bright_yellow", 93),
		("bright_blue", 94),
		("bright_magenta", 95),
		("bright_cyan", 96),
		("bright_white", 97),
		// Standard background colors
		("bg_black", 40),
		("bg_red", 41),
		("bg_green", 42),
		("bg_yellow", 43),
		("bg_blue", 44),
		("bg_magenta", 45),
		("bg_cyan", 46),
		("bg_white", 47),
		// Bright background colors
		("bg_bright_black", 100),
		("bg_bright_red", 101),
		("bg_bright_green", 102),
		("bg_bright_yellow", 103),
		("bg_bright_blue", 104),
		("bg_bright_magenta", 105),
		("bg_bright_cyan", 106),
		("bg_bright_white", 107),
	])
});

pub fn get_ansi(style: Style) -> String {
	match style {
		Style::Single(name) => match ANSI_CODES.get(name) {
			Some(code) => format!("\x1b[{}m", code),
			None => String::new(),
		},

		Style::Multiple(names) => {
			let codes: Vec<String> = names
				.iter()
				.filter_map(|name| ANSI_CODES.get(*name))
				.map(u8::to_string)
				.collect();

			if codes.is_empty() {
				String::new()
			} else {
				format!("\x1b[{}m", codes.join(";"))
			}
		}
	}
}
