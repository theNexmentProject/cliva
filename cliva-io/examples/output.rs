use cliva_io::output::{Semantics, Style, get_text, print, semantic};

fn main() {
    //print coloured text
    print("Red : Hello World", Style::Single("red"));

    //printing text with multiple colours
    print(
        "Bold blue: Hello World!",
        Style::Multiple(vec!["blue", "bold"]),
    );

    //get text with the ansi sequence to use it with your own needs
    println!(
        "{}",
        get_text(
            "Hello World",
            Style::Multiple(vec!["bold", "bg_red", "white"])
        )
    );

    println!();
    println!();

    //some pre configured styles
    print("This is a success message", Style::Success);
    print("This is an error message", Style::Error);
    print("This is an info message", Style::Info);
    print("This is a warning message", Style::Warning);

    println!();
    println!();

    //semantics - you don't have to do much to print errors
    semantic("This command got an error", Semantics::Error);
    semantic("This is a success message", Semantics::Success);
    semantic("You are not using the latest version", Semantics::Warning);
    semantic("Please update to latest version", Semantics::Info);
}
