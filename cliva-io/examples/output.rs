use cliva_io::output::{Align, Semantics, Style, Title, get_text, print, semantic, title, icon};

fn main() {
    // Print normal text.
    println!("Hello, Cliva!");

    // Print text with a style.
    print("This is red text", Style::Single("red"));
    print("This is bold text", Style::Single("bold"));

    // Use multiple styles together.
    print("Bold green text", Style::Multiple(vec!["bold", "green"]));

    // Get styled text without printing it.
    let text = get_text("Styled text", Style::Single("cyan"));
    println!("Here is some {text}");

    // Print a centered title.
    title("Cliva IO", Title::Simple('='), 40, None);

    // Print a left-aligned title.
    title("Getting Started", Title::Simple('-'), 40, Some(Align::Left));

    // Print a right-aligned title.
    title("Configuration", Title::Simple('-'), 40, Some(Align::Right));

    // Print a title with a bottom bar.
    title("Output Example", Title::BottomBar('='), 40, None);

    // Print a title with bars above and below.
    title("Welcome", Title::Bar('='), 40, None);

    // Print a success message.
    semantic("Task completed", Semantics::Success);

    // Print an error message.
    semantic("Something went wrong", Semantics::Error);

    // Print an information message.
    semantic("Starting the application", Semantics::Info);

    // Print a warning message.
    semantic("Check your configuration", Semantics::Warning);

    //icons usage
    println!("{} Operation completed", icon("check"));
    println!("{} Operation failed", icon("cross"));
    println!("{} Information message", icon("info"));
    println!("{} Warning message", icon("warning"));
    println!("{} Favorite item", icon("heart"));
}
