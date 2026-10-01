use std::fmt::Write;

#[derive(Clone, Copy, Debug)]
pub enum List {
    Ordered,
    Unordered,
    Custom(char),
}

pub fn list(kind: List, items: Vec<&str>) {
    let output = format_list(kind, &items, 0);
    print!("{output}");
}

fn format_list(kind: List, items: &[&str], depth: usize) -> String {
    let mut output = String::new();

    let indent = "  ".repeat(depth);

    for (index, item) in items.iter().enumerate() {
        let marker = match kind {
            List::Ordered => format!("{}. ", index + 1),
            List::Unordered => "•".to_string(),
            List::Custom(character) => character.to_string(),
        };

        writeln!(output, "{indent}{marker} {item}").unwrap();
    }

    output
}

pub fn nested_list(kind: List, items: Vec<(&str, Option<(List, Vec<&str>)>)>) {
    let mut output = String::new();

    for (index, (item, children)) in items.iter().enumerate() {
        let marker = match kind {
            List::Ordered => format!("{}. ", index + 1),
            List::Unordered => "•".to_string(),
            List::Custom(character) => character.to_string(),
        };

        writeln!(output, "{marker} {item}").unwrap();

        if let Some((child_kind, child_items)) = children {
            output.push_str(&format_list(*child_kind, child_items, 1));
        }
    }

    print!("{output}");
}
