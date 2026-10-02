use std::io::{self, Write};

use crossterm::{
    cursor::MoveToColumn,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, Clear, ClearType},
};

use crate::output::{Style, get_text, print};

pub fn inline_selection<'a>(
    prompt: &str,
    options: Vec<&'a str>,
    default: Option<usize>,
) -> io::Result<&'a str> {
    if options.is_empty() {
        print(
            "Selection error: no options provided.",
            Style::Single("red"),
        );
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no options provided",
        ));
    }

    if default.is_some_and(|i| i >= options.len()) {
        print(
            "Selection error: default index is out of range.",
            Style::Single("red"),
        );
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "default index out of range",
        ));
    }

    let mut selected = default;
    let mut error: Option<&str> = None;

    terminal::enable_raw_mode()?;

    let result = (|| -> io::Result<&'a str> {
        let mut stdout = io::stdout();

        loop {
            execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine))?;

            write!(stdout, "\x1b[36m❯\x1b[0m {} : [ ", prompt)?;

            for (i, option) in options.iter().enumerate() {
                if i > 0 {
                    write!(stdout, " / ")?;
                }

                if selected == Some(i) {
                    write!(
                        stdout,
                        "{}",
                        get_text(option, Style::Multiple(vec!["cyan", "underline"]),)
                    )?;
                } else {
                    write!(stdout, "{}", option)?;
                }
            }

            write!(stdout, " ] ")?;

            if let Some(message) = error {
                write!(stdout, "{}", get_text(message, Style::Single("red")))?;
            }

            stdout.flush()?;

            let Event::Key(key) = event::read()? else {
                continue;
            };

            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Left => {
                    selected = Some(match selected {
                        Some(0) => options.len() - 1,
                        Some(i) => i - 1,
                        None => options.len() - 1,
                    });
                    error = None;
                }

                KeyCode::Right => {
                    selected = Some(match selected {
                        Some(i) if i + 1 < options.len() => i + 1,
                        _ => 0,
                    });
                    error = None;
                }

                KeyCode::Enter => {
                    if let Some(i) = selected {
                        writeln!(stdout)?;
                        return Ok(options[i]);
                    }

                    error = Some("Please select an option before pressing Enter.");
                }

                KeyCode::Esc => {
                    writeln!(stdout)?;
                    return Err(io::Error::new(
                        io::ErrorKind::Interrupted,
                        "selection cancelled",
                    ));
                }

                _ => {}
            }
        }
    })();

    let restore = terminal::disable_raw_mode();
    let clear = execute!(io::stdout(), MoveToColumn(0), Clear(ClearType::CurrentLine));

    restore?;
    clear?;

    result
}

pub fn vertical_selection<'a>(
    prompt: &str,
    options: Vec<&'a str>,
    default: Option<usize>,
    bg_style: Option<&str>,
) -> io::Result<&'a str> {
    if options.is_empty() {
        print(
            "Selection error: no options provided.",
            Style::Single("red"),
        );

        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no options provided",
        ));
    }

    let mut selected = default.unwrap_or(usize::from(options.len() > 1));

    if selected >= options.len() {
        print(
            "Selection error: default index is out of range.",
            Style::Single("red"),
        );

        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "default index out of range",
        ));
    }

    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    let result = (|| -> io::Result<&'a str> {
        write!(stdout, "\x1b[?25l")?;
        stdout.flush()?;

        let mut rendered_lines = 0;

        loop {
            if rendered_lines > 0 {
                write!(stdout, "\x1b[{}A\r\x1b[J", rendered_lines)?;
            }

            write!(stdout, "\x1b[36m❯\x1b[0m {} :\r\n", prompt)?;

            for (i, option) in options.iter().enumerate() {
                if i == selected {
                    let mut styles = vec!["bold"];

                    if let Some(bg) = bg_style {
                        styles.push(bg);
                    }

                    write!(
                        stdout,
                        "  \x1b[36m❯\x1b[0m {}\r\n",
                        get_text(option, Style::Multiple(styles))
                    )?;
                } else {
                    write!(stdout, "    {}\r\n", option)?;
                }
            }

            stdout.flush()?;
            rendered_lines = options.len() + 1;

            let Event::Key(key) = event::read()? else {
                continue;
            };

            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Up => {
                    selected = if selected == 0 {
                        options.len() - 1
                    } else {
                        selected - 1
                    };
                }

                KeyCode::Down => {
                    selected = (selected + 1) % options.len();
                }

                KeyCode::Enter => {
                    return Ok(options[selected]);
                }

                KeyCode::Esc => {
                    return Err(io::Error::new(
                        io::ErrorKind::Interrupted,
                        "selection cancelled",
                    ));
                }

                _ => {}
            }
        }
    })();

    let cleanup = (|| -> io::Result<()> {
        write!(stdout, "\x1b[?25h")?;
        stdout.flush()?;
        terminal::disable_raw_mode()?;
        writeln!(stdout)?;
        stdout.flush()?;
        Ok(())
    })();

    cleanup?;
    result
}

pub fn checkbox_selection<'a>(
    prompt: &str,
    options: Vec<&'a str>,
    defaults: Option<Vec<usize>>,
    bg_style: Option<&str>,
) -> io::Result<Vec<&'a str>> {
    if options.is_empty() {
        print(
            "Selection error: no options provided.",
            Style::Single("red"),
        );

        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no options provided",
        ));
    }

    let mut checked = vec![false; options.len()];

    if let Some(defaults) = defaults {
        for index in defaults {
            if index >= options.len() {
                print(
                    "Selection error: default index is out of range.",
                    Style::Single("red"),
                );

                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "default index out of range",
                ));
            }

            checked[index] = true;
        }
    }

    let mut selected = 0;

    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    let result = (|| -> io::Result<Vec<&'a str>> {
        write!(stdout, "\x1b[?25l")?;
        stdout.flush()?;

        let mut rendered_lines = 0;

        loop {
            if rendered_lines > 0 {
                write!(stdout, "\x1b[{}A\r\x1b[J", rendered_lines)?;
            }

            write!(stdout, "\x1b[36m❯\x1b[0m {} :\r\n", prompt)?;

            for (i, option) in options.iter().enumerate() {
                let mark = if checked[i] { "✓" } else { " " };

                if i == selected {
                    let mut styles = vec!["bold"];

                    if let Some(bg) = bg_style {
                        styles.push(bg);
                    }

                    write!(
                        stdout,
                        "  \x1b[36m❯\x1b[0m [{}] {}\r\n",
                        get_text(mark, Style::Multiple(styles.clone())),
                        get_text(option, Style::Multiple(styles)),
                    )?;
                } else {
                    write!(stdout, "    [{}] {}\r\n", mark, option,)?;
                }
            }

            write!(
                stdout,
                "\x1b[90m↑/↓ navigate • Space toggle • Enter confirm • Esc cancel\x1b[0m\r\n"
            )?;

            stdout.flush()?;
            rendered_lines = options.len() + 2;

            let Event::Key(key) = event::read()? else {
                continue;
            };

            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Up => {
                    selected = if selected == 0 {
                        options.len() - 1
                    } else {
                        selected - 1
                    };
                }

                KeyCode::Down => {
                    selected = (selected + 1) % options.len();
                }

                KeyCode::Char(' ') => {
                    checked[selected] = !checked[selected];
                }

                KeyCode::Enter => {
                    if !checked.iter().any(|value| *value) {
                        continue;
                    }

                    return Ok(options
                        .iter()
                        .enumerate()
                        .filter_map(|(i, option)| checked[i].then_some(*option))
                        .collect());
                }

                KeyCode::Esc => {
                    return Err(io::Error::new(
                        io::ErrorKind::Interrupted,
                        "selection cancelled",
                    ));
                }

                _ => {}
            }
        }
    })();

    let cleanup = (|| -> io::Result<()> {
        write!(stdout, "\x1b[?25h")?;
        stdout.flush()?;
        terminal::disable_raw_mode()?;
        writeln!(stdout)?;
        stdout.flush()?;
        Ok(())
    })();

    cleanup?;
    result
}
