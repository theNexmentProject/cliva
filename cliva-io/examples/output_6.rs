use cliva_io::output::{Progress, progress};
use std::{thread, time::Duration};

fn main() {
	println!("=== Progress Examples ===\n");

	// 1. Bar
	for current in 0..=100 {
		progress("Downloading", Progress::Bar, current, 100);
		thread::sleep(Duration::from_millis(30));
	}

	println!();

	// 2. Percentage
	for current in (0..=100).step_by(10) {
		progress("Installing", Progress::Percentage, current, 100);
		thread::sleep(Duration::from_millis(200));
	}

	println!();

	// 3. Counter
	for current in 0..=10 {
		progress("Processing files", Progress::Counter, current, 10);
		thread::sleep(Duration::from_millis(200));
	}

	println!();

	// 4. Dots
	for current in (0..=100).step_by(10) {
		progress("Processing", Progress::Dots, current, 100);
		thread::sleep(Duration::from_millis(200));
	}

	println!();

	// 5. Bar + Percentage
	for current in 0..=100 {
		progress(
			"Building project",
			Progress::BarWithPercentage,
			current,
			100,
		);

		thread::sleep(Duration::from_millis(30));
	}
}
