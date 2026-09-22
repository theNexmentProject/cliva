pub enum Title {
	Boxed,
	Heavy,
	Simple,
	Bracket,
	Marker,
}

pub enum TitleAlign {
	Left,
	Center,
	Right,
}

pub fn title(text: &str, style: Title, align: TitleAlign, width: usize) {
	match style {
		Title::Boxed => {
			let border = "─".repeat(width.saturating_sub(2));
			println!("┌{}┐", border);
			println!("│{}│", align_text(text, align, width.saturating_sub(2)));
			println!("└{}┘", border);
		}

		Title::Heavy => {
			println!("{}", "═".repeat(width));
			println!("{}", align_text(text, align, width));
			println!("{}", "═".repeat(width));
		}

		Title::Simple => {
			println!("{}", align_text(text, align, width));
			println!("{}", "─".repeat(width));
		}

		Title::Bracket => {
			let content_width = width.saturating_sub(4);
			println!("[ {} ]", align_text(text, align, content_width));
		}

		Title::Marker => {
			let content_width = width.saturating_sub(2);
			println!("◆ {}", align_text(text, align, content_width));
		}
	}
}

fn align_text(text: &str, align: TitleAlign, width: usize) -> String {
	let text_width = text.chars().count();

	if text_width >= width {
		return text.chars().take(width).collect();
	}

	let remaining = width - text_width;

	match align {
		TitleAlign::Left => {
			format!("{}{}", text, " ".repeat(remaining))
		}

		TitleAlign::Center => {
			let left = remaining / 2;
			let right = remaining - left;

			format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
		}

		TitleAlign::Right => {
			format!("{}{}", " ".repeat(remaining), text)
		}
	}
}
