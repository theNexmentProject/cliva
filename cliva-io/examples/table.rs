use cliva_io::output::Table;

fn main() {
    // Example 1: Simple two-column table.
    Table::new()
        .headers(["Key", "Value"])
        .rows([
            ["Name", "Cliva"],
            ["Language", "Rust"],
            ["Version", "0.1.0"],
            ["Status", "Active"],
            ["License", "MIT"],
        ])
        .print();

    println!();

    // Example 2: Table with multiple columns.
    Table::new()
        .headers(["ID", "Name", "Role", "Language", "Status"])
        .rows([
            ["01", "Alice", "Developer", "Rust", "Active"],
            ["02", "Bob", "Designer", "CSS", "Active"],
            ["03", "Charlie", "Tester", "Python", "Pending"],
            ["04", "David", "Maintainer", "Rust", "Active"],
            ["05", "Eve", "Contributor", "Go", "Inactive"],
        ])
        .print();

    println!();

    // Example 3: Build a table one row at a time.
    let table = Table::new()
        .headers(["Package", "Version", "Downloads", "Description"])
        .row(["cliva", "0.1.0", "12,450", "CLI framework"])
        .row(["cliva-io", "0.1.0", "8,320", "Terminal output"])
        .row(["serde", "1.0.219", "2.1M", "Serialization"])
        .row(["tokio", "1.45.0", "3.8M", "Async runtime"])
        .row(["clap", "4.5.40", "5.2M", "Argument parser"]);

    table.print();
}
