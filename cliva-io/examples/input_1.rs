use cliva_io::input::{InputType, InputValue, prompt};

fn main() {
	let name = prompt("Name", "Shisui", InputType::String);
	let age = prompt("Age", "18", InputType::Int);
	let height = prompt("Height", "175.5", InputType::Float);

	match name {
		InputValue::String(value) => println!("Name = {value}"),
		_ => {}
	}

	match age {
		InputValue::Int(value) => println!("Age = {value}"),
		_ => {}
	}

	match height {
		InputValue::Float(value) => println!("Height = {value}"),
		_ => {}
	}
}
