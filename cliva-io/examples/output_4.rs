use cliva_io::output::{Title, TitleAlign, title};

fn main() {
	println!("");
	println!("");
	//these print text with a style that look likes a title

	//this is an simple title
	title("Simple Title", Title::Simple, TitleAlign::Center, 40);
	println!("");
	println!("");

	//this is a heavy title
	title("Heavy Title", Title::Heavy, TitleAlign::Center, 40);
	println!("");
	println!("");

	//this is a boxed title
	title("Boxed Title", Title::Boxed, TitleAlign::Center, 40);
	println!("");
	println!("");

	//this is a bracket title
	title("Bracket Title", Title::Bracket, TitleAlign::Center, 20);
	println!("");
	println!("");

	// this is a marker title
	title("Marker Title", Title::Marker, TitleAlign::Left, 40);
}
