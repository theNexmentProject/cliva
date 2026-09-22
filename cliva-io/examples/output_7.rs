use cliva_io::output::table;

fn main() {
	let headers = ["Name", "Version", "Status"];

	let rows = vec![
		vec!["cliva_io", "0.1.0", "Stable"],
		vec!["serde", "1.0.219", "Stable"],
		vec!["tokio", "1.44.0", "Active"],
	];

	table(&headers, &rows);
}
