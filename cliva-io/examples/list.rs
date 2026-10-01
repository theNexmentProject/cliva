use cliva_io::output::{List, list, nested_list};

fn main() {
    // Print an ordered list.
    list(
        List::Ordered,
        vec!["First item", "Second item", "Third item"],
    );

    println!();

    // Print an unordered list.
    list(List::Unordered, vec!["Apple", "Banana", "Orange"]);

    println!();

    // Print a list with a custom character.
    list(List::Custom('*'), vec!["Rust", "Python", "JavaScript"]);

    println!();

    // Print a nested list.
    nested_list(
        List::Unordered,
        vec![
            (
                "Programming languages",
                Some((List::Ordered, vec!["Rust", "Python", "JavaScript"])),
            ),
            (
                "Operating systems",
                Some((List::Custom('*'), vec!["Linux", "Windows", "macOS"])),
            ),
            ("Other tools", None),
        ],
    );
}
