use std::fmt::Write;

use crate::output::get_text;

#[derive(Clone, Debug, Default)]
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.headers = headers.into_iter().map(Into::into).collect();
        self
    }

    pub fn row<I, S>(mut self, row: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.rows.push(row.into_iter().map(Into::into).collect());
        self
    }

    pub fn rows<I, R, S>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for row in rows {
            self.rows.push(row.into_iter().map(Into::into).collect());
        }
        self
    }

    pub fn render(&self) -> String {
        let columns = self
            .headers
            .len()
            .max(self.rows.iter().map(Vec::len).max().unwrap_or(0));

        if columns == 0 {
            return String::new();
        }

        let mut widths = vec![0usize; columns];

        for (i, header) in self.headers.iter().enumerate() {
            widths[i] = widths[i].max(header.chars().count());
        }

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                widths[i] = widths[i].max(cell.chars().count());
            }
        }

        let border = |left: &str, middle: &str, right: &str| {
            let mut line = String::from(left);

            for (i, width) in widths.iter().enumerate() {
                if i > 0 {
                    line.push_str(middle);
                }

                line.push_str(&"─".repeat(width + 2));
            }

            line.push_str(right);
            line
        };

        let mut output = String::new();

        let _ = writeln!(output, "{}", border("┌", "┬", "┐"));

        if !self.headers.is_empty() {
            output.push('│');

            for i in 0..columns {
                let value = self.headers.get(i).map(String::as_str).unwrap_or("");
                let styled = get_text(value, crate::output::Style::Multiple(vec!["bold", "cyan"]));
                let padding = widths[i].saturating_sub(value.chars().count());

                let _ = write!(output, " {styled}{} │", " ".repeat(padding));
            }

            output.push('\n');
            let _ = writeln!(output, "{}", border("├", "┼", "┤"));
        }

        for (index, row) in self.rows.iter().enumerate() {
            output.push('│');

            for i in 0..columns {
                let value = row.get(i).map(String::as_str).unwrap_or("");
                let padding = widths[i].saturating_sub(value.chars().count());

                let _ = write!(output, " {value}{} │", " ".repeat(padding));
            }

            output.push('\n');

            if index + 1 < self.rows.len() {
                let _ = writeln!(output, "{}", border("├", "┼", "┤"));
            }
        }

        let _ = writeln!(output, "{}", border("└", "┴", "┘"));

        output
    }

    pub fn print(&self) {
        print!("{}", self.render());
    }
}
