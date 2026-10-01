#![allow(unused_mut)]

use std::io::{self, Write};

#[derive(Clone, Copy, Debug)]
pub enum Progress {
    Bar,
    Hashtags,
    Blocks,
}

pub struct ProgressBar {
    text: String,
    style: Progress,
    current: u64,
    total: u64,
}

pub fn progress(text: impl Into<String>, style: Progress, current: u64, total: u64) -> ProgressBar {
    let mut bar = ProgressBar {
        text: text.into(),
        style,
        current: current.min(total),
        total,
    };

    bar.draw().expect("failed to render progress");
    bar
}

impl ProgressBar {
    pub fn inc(&mut self) {
        self.inc_by(1);
    }

    pub fn inc_by(&mut self, amount: u64) {
        self.current = self.current.saturating_add(amount).min(self.total);
        self.draw().expect("failed to render progress");
    }

    pub fn set_current(&mut self, current: u64) {
        self.current = current.min(self.total);
        self.draw().expect("failed to render progress");
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.draw().expect("failed to render progress");
    }

    pub fn current(&self) -> u64 {
        self.current
    }

    pub fn total(&self) -> u64 {
        self.total
    }

    pub fn is_finished(&self) -> bool {
        self.total > 0 && self.current >= self.total
    }

    pub fn finish(&mut self) {
        self.current = self.total;
        self.draw().expect("failed to render progress");
        println!();
    }

    fn draw(&self) -> io::Result<()> {
        let mut out = io::stdout();

        let percent = if self.total == 0 {
            0
        } else {
            (self.current as u128 * 100 / self.total as u128) as usize
        };

        let width = 40;
        let filled = width * percent / 100;
        let empty = width - filled;

        let bar = match self.style {
            Progress::Bar => {
                format!("[{}{}]", "=".repeat(filled), " ".repeat(empty))
            }
            Progress::Hashtags => {
                format!("[{}{}]", "#".repeat(filled), " ".repeat(empty))
            }
            Progress::Blocks => {
                format!("|{}{}|", "█".repeat(filled), "░".repeat(empty))
            }
        };

        write!(out, "\r{} {}    {}%        ", self.text, bar, percent)?;
        out.flush()
    }
}
