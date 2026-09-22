use std::fmt::Display;

pub fn table<T: Display>(headers: &[T], rows: &[Vec<T>]) {
	if headers.is_empty() {
		return;
	}

	let columns = headers.len();

	let mut widths: Vec<usize> = headers
		.iter()
		.map(|header| header.to_string().chars().count())
		.collect();

	for row in rows {
		for (index, value) in row.iter().enumerate().take(columns) {
			let width = value.to_string().chars().count();

			if width > widths[index] {
				widths[index] = width;
			}
		}
	}

	print_separator(&widths);

	print!("│");
	for (index, header) in headers.iter().enumerate() {
		print!(" {:width$} │", header, width = widths[index]);
	}
	println!();

	print_separator(&widths);

	for row in rows {
		print!("│");

		for index in 0..columns {
			let value = row
				.get(index)
				.map(|value| value.to_string())
				.unwrap_or_default();

			print!(" {:width$} │", value, width = widths[index]);
		}

		println!();
	}

	print_separator(&widths);
}

fn print_separator(widths: &[usize]) {
	print!("├");

	for (index, width) in widths.iter().enumerate() {
		print!("{}", "─".repeat(width + 2));

		if index + 1 == widths.len() {
			print!("┤");
		} else {
			print!("┼");
		}
	}

	println!();
}
