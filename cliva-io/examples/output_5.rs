use cliva_io::output::{Loader, loader};
use std::thread;
use std::time::Duration;

fn main() {
	// Spinner
	loader("Building project", Loader::Spinner, || {
		thread::sleep(Duration::from_secs(5));
	});

	// Line
	loader("Compiling project", Loader::Line, || {
		thread::sleep(Duration::from_secs(5));
	});

	// Dots
	loader("Downloading files", Loader::Dots, || {
		thread::sleep(Duration::from_secs(20));
	});
}
