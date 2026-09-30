use crate::output::Style;
use std::collections::HashMap;
use std::sync::LazyLock;

static ANSI_STYLES: LazyLock<HashMap<&'static str, u8>> = LazyLock::new(|| {
    HashMap::from([
        ("bold", 1),
        ("dim", 2),
        ("italic", 3),
        ("underline", 4),
        ("blink", 5),
        ("rapid_blink", 6),
        ("reverse", 7),
        ("hidden", 8),
        ("strikethrough", 9),
        // Foreground colors
        ("black", 30),
        ("red", 31),
        ("green", 32),
        ("yellow", 33),
        ("blue", 34),
        ("magenta", 35),
        ("cyan", 36),
        ("white", 37),
        ("default", 39),
        // Bright foreground colors
        ("bright_black", 90),
        ("bright_red", 91),
        ("bright_green", 92),
        ("bright_yellow", 93),
        ("bright_blue", 94),
        ("bright_magenta", 95),
        ("bright_cyan", 96),
        ("bright_white", 97),
        // Background colors
        ("bg_black", 40),
        ("bg_red", 41),
        ("bg_green", 42),
        ("bg_yellow", 43),
        ("bg_blue", 44),
        ("bg_magenta", 45),
        ("bg_cyan", 46),
        ("bg_white", 47),
        ("bg_default", 49),
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

pub fn get_ansi(style: Style<'_>) -> String {
    match style {
        Style::Single(look) => match ANSI_STYLES.get(look) {
            Some(code) => format!("\x1b[{}m", code),
            None => String::new(),
        },

        Style::Multiple(looks) => {
            let codes: Vec<String> = looks
                .iter()
                .filter_map(|look| ANSI_STYLES.get(look).map(|code| code.to_string()))
                .collect();

            if codes.is_empty() {
                String::new()
            } else {
                format!("\x1b[{}m", codes.join(";"))
            }
        }

        Style::Success => String::from("\x1b[32m"),

        Style::Warning => String::from("\x1b[33m"),

        Style::Info => String::from("\x1b[34m"),

        Style::Error => String::from("\x1b[31m"),
    }
}
