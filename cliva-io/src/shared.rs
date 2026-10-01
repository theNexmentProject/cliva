use crate::output::Style;

fn ansi_code(style: &str) -> Option<u8> {
    Some(match style {
        "bold" => 1,
        "dim" => 2,
        "italic" => 3,
        "underline" => 4,
        "blink" => 5,
        "rapid_blink" => 6,
        "reverse" => 7,
        "hidden" => 8,
        "strikethrough" => 9,

        "black" => 30,
        "red" => 31,
        "green" => 32,
        "yellow" => 33,
        "blue" => 34,
        "magenta" => 35,
        "cyan" => 36,
        "white" => 37,
        "default" => 39,

        "bright_black" => 90,
        "bright_red" => 91,
        "bright_green" => 92,
        "bright_yellow" => 93,
        "bright_blue" => 94,
        "bright_magenta" => 95,
        "bright_cyan" => 96,
        "bright_white" => 97,

        "bg_black" => 40,
        "bg_red" => 41,
        "bg_green" => 42,
        "bg_yellow" => 43,
        "bg_blue" => 44,
        "bg_magenta" => 45,
        "bg_cyan" => 46,
        "bg_white" => 47,
        "bg_default" => 49,

        "bg_bright_black" => 100,
        "bg_bright_red" => 101,
        "bg_bright_green" => 102,
        "bg_bright_yellow" => 103,
        "bg_bright_blue" => 104,
        "bg_bright_magenta" => 105,
        "bg_bright_cyan" => 106,
        "bg_bright_white" => 107,

        _ => return None,
    })
}

pub fn get_ansi(style: Style<'_>) -> String {
    match style {
        Style::Single(name) => ansi_code(name)
            .map(|code| format!("\x1b[{code}m"))
            .unwrap_or_default(),

        Style::Multiple(names) => {
            let mut ansi = String::from("\x1b[");

            for name in names {
                if let Some(code) = ansi_code(name) {
                    if ansi.len() > 2 {
                        ansi.push(';');
                    }
                    ansi.push_str(&code.to_string());
                }
            }

            if ansi.len() == 2 {
                String::new()
            } else {
                ansi.push('m');
                ansi
            }
        }

        Style::Success => "\x1b[32m".into(),
        Style::Warning => "\x1b[33m".into(),
        Style::Info => "\x1b[34m".into(),
        Style::Error => "\x1b[31m".into(),
    }
}
