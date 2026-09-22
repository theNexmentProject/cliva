use cliva_io::output::{Style, get_text, print};

fn main() {
	//single style
	print("Hello, World", Style::Single("magenta"));

	// Combine multiple styles.
	print("Hello World", Style::Multiple(vec!["bold", "magenta"]));

	// Get a colured String without printing
	let colored_string: String = get_text("Hello, World", Style::Single("red"));
	println!("{}", colored_string);
}
