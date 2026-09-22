//! Cliva Output API — basic usage examples.
//!
//! This example demonstrates:
//! - Basic styled output
//! - Semantic output helpers
//! - Custom semantic output
//! - Title styles and alignment

use cliva_io::output::{
    Semantics, Style, Title, TitleAlign, error, get_text, info, print, semantics, success, title,
    warning,
};

fn main() {
    // ---------------------------------------------------------
    // Basic styled output
    // ---------------------------------------------------------

    // Apply a single style to the text.
    print("Hello, World", Style::Single("magenta"));

    // Combine multiple styles.
    print("Hello World", Style::Multiple(vec!["bold", "magenta"]));

    // Get a colured String without printing
    let colored_string: String = get_text("Hello, World", Style::Single("red"));
    println!("{}", colored_string);

    // ---------------------------------------------------------
    // Semantic output
    // ---------------------------------------------------------

    // Common CLI messages with predefined semantics.
    success("Operation completed successfully");
    info("Compiling project...");
    warning("This feature is deprecated");
    error("Failed to compile project");

    // ---------------------------------------------------------
    // Custom semantic output
    // ---------------------------------------------------------

    // Use `semantics` when you want to explicitly choose
    // the semantic type instead of using a helper function.
    semantics("Everything looks good", Semantics::Success);
    semantics("Build information", Semantics::Info);
    semantics("Configuration is outdated", Semantics::Warning);
    semantics("Build failed", Semantics::Error);

    // ---------------------------------------------------------
    // Titles
    // ---------------------------------------------------------

    // Boxed title with centered text.
    title("Build Project", Title::Boxed, TitleAlign::Center, 40);

    // Heavy divider with left-aligned text.
    title("Compiling", Title::Heavy, TitleAlign::Left, 40);

    // Simple divider with centered text.
    title("Results", Title::Simple, TitleAlign::Center, 40);

    // Bracket-style title.
    title("Results", Title::Bracket, TitleAlign::Center, 40);

    // Marker-style title.
    title("Results", Title::Marker, TitleAlign::Left, 40);
}
