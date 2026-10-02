use std::{
    io::{self, Read, Write},
    str::FromStr,
};

use crate::output::{Style, print};

pub enum InputType {
    Str,
    Int,
    Float,
}

pub enum InputValue {
    Str(String),
    Int(i64),
    Float(f64),
}

struct RawMode(libc::termios);

impl RawMode {
    fn new() -> io::Result<Self> {
        unsafe {
            let mut original = std::mem::zeroed();

            if libc::tcgetattr(libc::STDIN_FILENO, &mut original) != 0 {
                return Err(io::Error::last_os_error());
            }

            let mut raw = original;
            raw.c_lflag &= !(libc::ICANON | libc::ECHO);
            raw.c_cc[libc::VMIN] = 1;
            raw.c_cc[libc::VTIME] = 0;

            if libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &raw) != 0 {
                return Err(io::Error::last_os_error());
            }

            Ok(Self(original))
        }
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.0);
        }
    }
}

pub fn prompt(
    question: &str,
    input_type: InputType,
    default: Option<&str>,
) -> io::Result<InputValue> {
    let _raw = RawMode::new()?;
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    let mut cursor = 0;

    loop {
        write!(stdout, "\r\x1b[2K\x1b[36m❯\x1b[0m {} ", question)?;

        if input.is_empty() {
            if let Some(value) = default {
                write!(stdout, "\x1b[2m{}\x1b[0m", value)?;
                write!(stdout, "\x1b[{}D", value.chars().count())?;
            }
        } else {
            write!(stdout, "{}", input)?;

            let remaining = input[cursor..].chars().count();

            if remaining > 0 {
                write!(stdout, "\x1b[{}D", remaining)?;
            }
        }

        stdout.flush()?;

        let mut byte = [0u8; 1];

        if reader.read_exact(&mut byte).is_err() {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Operation cancelled by user",
            ));
        }

        match byte[0] {
            3 | 4 => {
                writeln!(stdout)?;
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Operation cancelled by user",
                ));
            }

            13 | 10 => {
                let value = if input.is_empty() {
                    default.unwrap_or("").to_string()
                } else {
                    input.clone()
                };

                match &input_type {
                    InputType::Str => {
                        writeln!(stdout)?;
                        return Ok(InputValue::Str(value));
                    }

                    InputType::Int => {
                        if let Ok(number) = i64::from_str(&value) {
                            writeln!(stdout)?;
                            return Ok(InputValue::Int(number));
                        }
                    }

                    InputType::Float => {
                        if let Ok(number) = f64::from_str(&value) {
                            if number.is_finite() {
                                writeln!(stdout)?;
                                return Ok(InputValue::Float(number));
                            }
                        }
                    }
                }

                writeln!(stdout)?;
                print("Invalid input. Please enter a valid value.", Style::Error);

                input.clear();
                cursor = 0;
            }

            127 | 8 => {
                if cursor > 0 {
                    let previous = input[..cursor]
                        .char_indices()
                        .last()
                        .map(|(index, _)| index)
                        .unwrap_or(0);

                    input.drain(previous..cursor);
                    cursor = previous;
                }
            }

            27 => {
                let mut sequence = [0u8; 2];

                if reader.read_exact(&mut sequence).is_ok() && sequence[0] == b'[' {
                    match sequence[1] {
                        b'D' if cursor > 0 => {
                            cursor = input[..cursor]
                                .char_indices()
                                .last()
                                .map(|(index, _)| index)
                                .unwrap_or(0);
                        }

                        b'C' if cursor < input.len() => {
                            cursor += input[cursor..].chars().next().unwrap().len_utf8();
                        }

                        _ => {}
                    }
                }
            }

            byte if byte.is_ascii_graphic() || byte == b' ' => {
                input.insert(cursor, byte as char);
                cursor += 1;
            }

            _ => {}
        }
    }
}

pub fn confirm(text: &str, default: Option<bool>) -> io::Result<bool> {
    let _raw = RawMode::new()?;
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    loop {
        let hint = match default {
            Some(true) => "[Y/n]",
            Some(false) => "[y/N]",
            None => "[y/n]",
        };

        write!(
            stdout,
            "\r\x1b[2K\x1b[36m❯\x1b[0m {} \x1b[1;35m{}\x1b[0m : ",
            text, hint
        )?;

        write!(stdout, "{}", input)?;
        stdout.flush()?;

        let mut byte = [0u8; 1];

        if reader.read_exact(&mut byte).is_err() {
            writeln!(stdout)?;
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Operation cancelled by user",
            ));
        }

        match byte[0] {
            3 | 4 => {
                writeln!(stdout)?;
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Operation cancelled by user",
                ));
            }

            13 | 10 => {
                writeln!(stdout)?;

                match input.trim().to_ascii_lowercase().as_str() {
                    "y" | "yes" => return Ok(true),
                    "n" | "no" => return Ok(false),
                    "" => {
                        if let Some(value) = default {
                            return Ok(value);
                        }

                        print("Please provide an input.", Style::Error);
                    }
                    _ => print("Invalid input. Please answer with y or n.", Style::Error),
                }

                input.clear();
            }

            127 | 8 => {
                input.pop();
            }

            byte if byte.is_ascii_graphic() || byte == b' ' => {
                input.push(byte as char);
            }

            _ => {}
        }
    }
}

pub fn password(text: &str, default: Option<&str>) -> io::Result<String> {
    let _raw = RawMode::new()?;
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    loop {
        write!(
            stdout,
            "\r\x1b[2K\x1b[36m❯\x1b[0m {} ",
            text
        )?;

        write!(stdout, "{}", "*".repeat(input.chars().count()))?;
        stdout.flush()?;

        let mut byte = [0u8; 1];

        if reader.read_exact(&mut byte).is_err() {
            writeln!(stdout)?;
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Operation cancelled by user",
            ));
        }

        match byte[0] {
            3 | 4 => {
                writeln!(stdout)?;
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Operation cancelled by user",
                ));
            }

            13 | 10 => {
                writeln!(stdout)?;

                if !input.is_empty() {
                    return Ok(input);
                }

                if let Some(value) = default {
                    return Ok(value.to_string());
                }

                print("Please provide a password.", Style::Error);
            }

            127 | 8 => {
                input.pop();
            }

            byte if byte.is_ascii_graphic() || byte == b' ' => {
                input.push(byte as char);
            }

            _ => {}
        }
    }
}