use std::io::{self, Write};
use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

pub enum Loader {
	Spinner,
	Dots,
	Line,
}

pub fn loader<F, T>(text: &str, style: Loader, task: F) -> T
where
	F: FnOnce() -> T,
{
	let running = Arc::new(AtomicBool::new(true));
	let loader_running = Arc::clone(&running);

	let text = text.to_string();

	let handle = thread::spawn(move || match style {
		Loader::Spinner => {
			let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
			let mut index = 0;

			while loader_running.load(Ordering::Relaxed) {
				print!("\r{} {}", frames[index], text);
				io::stdout().flush().unwrap();

				index = (index + 1) % frames.len();
				thread::sleep(Duration::from_millis(80));
			}
		}

		Loader::Dots => {
			let width = terminal_width();
			let text_width = text.chars().count();

			let target = width.saturating_sub(10);
			let start = text_width + 1;

			let mut position = start;

			while loader_running.load(Ordering::Relaxed) {
				let spaces = position.saturating_sub(text_width);

				print!("\r{}{}...", text, " ".repeat(spaces));
				io::stdout().flush().unwrap();

				if position < target {
					position += 1;
				} else {
					position = start;
				}

				thread::sleep(Duration::from_millis(100));
			}
		}

		Loader::Line => {
			let frames = ['-', '\\', '|', '/'];
			let mut index = 0;

			while loader_running.load(Ordering::Relaxed) {
				print!("\r{} {}", frames[index], text);
				io::stdout().flush().unwrap();

				index = (index + 1) % frames.len();
				thread::sleep(Duration::from_millis(100));
			}
		}
	});

	let result = task();

	running.store(false, Ordering::Relaxed);
	handle.join().unwrap();

	print!("\r{}\r", " ".repeat(terminal_width()));
	io::stdout().flush().unwrap();

	result
}

fn terminal_width() -> usize {
	std::env::var("COLUMNS")
		.ok()
		.and_then(|value| value.parse().ok())
		.unwrap_or(80)
}
