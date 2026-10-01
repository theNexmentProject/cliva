#[derive(Debug, Clone, Copy)]
pub enum Title {
    Simple(char),
    BottomBar(char),
    Bar(char),
}

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Center,
    Left,
    Right,
}

pub fn title(text: &str, style: Title, width: usize, align: Option<Align>) {
    let align = align.unwrap_or(Align::Center);
    let width = width.max(1);
    let text_width = text.chars().count();
    let padding = width.saturating_sub(text_width);

    let content = if padding == 0 {
        text.to_string()
    } else {
        match align {
            Align::Left => format!("{text}{}", " ".repeat(padding)),
            Align::Right => format!("{}{text}", " ".repeat(padding)),
            Align::Center => {
                let left = padding / 2;
                format!("{}{text}{}", " ".repeat(left), " ".repeat(padding - left))
            }
        }
    };

    let line = |ch: char| ch.to_string().repeat(width);

    match style {
        Title::Simple(ch) => {
            let padding = width.saturating_sub(text_width);

            let content = match align {
                Align::Left => format!("{text}{}", ch.to_string().repeat(padding)),
                Align::Right => format!("{}{text}", ch.to_string().repeat(padding)),
                Align::Center => {
                    let left = padding / 2;
                    format!(
                        "{}{text}{}",
                        ch.to_string().repeat(left),
                        ch.to_string().repeat(padding - left)
                    )
                }
            };

            println!("{content}");
        }
        Title::BottomBar(ch) => {
            println!("{content}\n{}", line(ch));
        }
        Title::Bar(ch) => {
            println!("{}\n{content}\n{}", line(ch), line(ch));
        }
    }
}
