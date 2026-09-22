use std::io::{self, Write};

pub enum Progress {
	Bar,
	Percentage,
	Counter,
	Dots,
	BarWithPercentage,
}

pub fn progress(text: &str, style: Progress, current: usize, total: usize) {
	let percentage = if total == 0 {
		0
	} else {
		((current as f64 / total as f64) * 100.0).min(100.0) as usize
	};

	let output = match style {
		Progress::Bar => {
			let width = 30;
			let filled = if total == 0 {
				0
			} else {
				(current * width / total).min(width)
			};

			let bar = format!("{}{}", "█".repeat(filled), "░".repeat(width - filled));

			format!("{} [{}]", text, bar)
		}

		Progress::Percentage => {
			format!("{} {}%", text, percentage)
		}

		Progress::Counter => {
			format!("{} {}/{}", text, current, total)
		}

		Progress::Dots => {
			let dots = match percentage {
				0..=24 => ".",
				25..=49 => "..",
				50..=74 => "...",
				75..=99 => "....",
				_ => ".....",
			};

			format!("{} {}", text, dots)
		}

		Progress::BarWithPercentage => {
			let width = 30;
			let filled = if total == 0 {
				0
			} else {
				(current * width / total).min(width)
			};

			let bar = format!("{}{}", "█".repeat(filled), "░".repeat(width - filled));

			format!("{} [{}] {}%", text, bar, percentage)
		}
	};

	print!("\r{}", output);
	io::stdout().flush().unwrap();

	if current >= total {
		println!();
	}
}
