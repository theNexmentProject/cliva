use cliva_io::output::{error, info, success, warning};

fn main() {
	//these prints the text with respective color

	//this is an error
	error("This is an error");

	//this is a info
	info("This is a info");

	//this is a warning
	warning("This is a warning");

	//this is a success
	success("This is a success");
}
