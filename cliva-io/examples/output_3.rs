use cliva_io::output::{Semantics, semantics};

fn main() {
	//these prints the text with prefix and respective color

	//this is an error
	semantics("This is an error", Semantics::Error);

	//this is a info
	semantics("This is an info", Semantics::Info);

	//this is a warning
	semantics("This is an warning", Semantics::Warning);

	//this is a success
	semantics("This is an success", Semantics::Success);
}
