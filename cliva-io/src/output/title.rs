pub enum Title {
    Simple(char),
    BottomBar(char),
    Bar(char),
}

pub enum Align {
    Center,
    Left,
    Right,
}

pub fn title(text: &str, look: Title, width: usize, align: Option<Align>) {
    match look {
        Title::Simple(style) => match align {
            None => print_simple_title(text, width, style),
            Some(align_p) => match align_p {
                Align::Center => print_simple_title(text, width, style),
                Align::Left => print_simple_title_left(text, width, style),
                Align::Right => print_simple_text_right(text, width, style),
            },
        },
        Title::BottomBar(style) => match align {
            None => print_bottombar_title(text, width, style),
            Some(align_p) => match align_p {
                Align::Center => print_bottombar_title(text, width, style),
                Align::Left => print_bottombar_title_left(text, width, style),
                Align::Right => print_bottombar_title_right(text, width, style),
            },
        },
        Title::Bar(style) => match align {
            None => print_bar_title(text, width, style),
            Some(align_p) => match align_p {
                Align::Center => print_bar_title(text, width, style),
                Align::Left => print_bar_title_left(text, width, style),
                Align::Right => print_bar_title_right(text, width, style),
            },
        },
    }
}

fn print_simple_title(text: &str, width: usize, style: char) {
    if text.len() < width {
        let total_len = width - text.len();
        let left = total_len / 2;
        let right = total_len - left;
        let final_text = format!(
            "{}{}{}",
            style.to_string().repeat(left),
            text,
            style.to_string().repeat(right)
        );
        println!("{final_text}");
    } else {
        println!("{text}");
    }
}

fn print_simple_title_left(text: &str, width: usize, style: char) {
    if text.len() < width {
        let total_len = width - text.len();
        let final_text = format!("{}{}", text, style.to_string().repeat(total_len));
        println!("{final_text}");
    } else {
        println!("{text}");
    }
}

fn print_simple_text_right(text: &str, width: usize, style: char) {
    if text.len() < width {
        let total_len = width - text.len();
        println!("{}{}", style.to_string().repeat(total_len), text);
    } else {
        println!("{text}");
    }
}

fn get_centered_string(text: &str, width: usize) -> String {
    if text.len() < width {
        let total_len = width - text.len();
        let left = total_len / 2;
        let right = total_len - left;
        format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
    } else {
        text.to_string()
    }
}

fn print_bottombar_title(text: &str, width: usize, style: char) {
    println!(
        "{}\n{}",
        get_centered_string(text, width),
        style.to_string().repeat(width)
    );
}

fn print_bottombar_title_right(text: &str, width: usize, style: char) {
    if text.len() < width {
        let total_len = width - text.len();
        let final_text = format!("{}{}", " ".repeat(total_len), text);
        println!("{}\n{}", final_text, style.to_string().repeat(width));
    } else {
        println!("{text}\n{}", style.to_string().repeat(width));
    }
}

fn print_bottombar_title_left(text: &str, width: usize, style: char) {
    println!("{text}\n{}", style.to_string().repeat(width));
}

fn print_bar_title(text: &str, width: usize, style: char) {
    println!(
        "{}\n{}\n{}",
        style.to_string().repeat(width),
        get_centered_string(text, width),
        style.to_string().repeat(width)
    );
}

fn print_bar_title_left(text: &str, width: usize, style: char) {
    println!(
        "{}\n{}\n{}",
        style.to_string().repeat(width),
        text,
        style.to_string().repeat(width)
    );
}

fn print_bar_title_right(text: &str, width: usize, style: char) {
    if text.len() < width {
        let total_len = width - text.len();
        let final_text = format!("{}{}", " ".repeat(total_len), text);
        println!(
            "{}\n{}\n{}",
            style.to_string().repeat(width),
            final_text,
            style.to_string().repeat(width)
        );
    } else {
        println!(
            "{}\n{}\n{}",
            style.to_string().repeat(width),
            text,
            style.to_string().repeat(width)
        );
    }
}
