use std::{
    io::{self, Write},
    thread::{self, JoinHandle},
    time::Duration,
};

pub enum Loader {
    Dots,
    Bar,
}

pub fn loader<F, T>(style: Loader, text: &str, task: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let handle = thread::spawn(task);

    match style {
        Loader::Dots => loader_dots(text, &handle),
        Loader::Bar => loader_bar(text, &handle),
    }

    clear_line();

    match handle.join() {
        Ok(result) => result,
        Err(error) => std::panic::resume_unwind(error),
    }
}

fn animate(text: &str, frames: &[&str], handle: &JoinHandle<impl Send>) {
    let mut i = 0;

    while !handle.is_finished() {
        print!("\r{} {}", frames[i % frames.len()], text);
        let _ = io::stdout().flush();

        thread::sleep(Duration::from_millis(80));
        i += 1;
    }
}

fn loader_dots(text: &str, handle: &JoinHandle<impl Send>) {
    animate(
        text,
        &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
        handle,
    );
}

fn loader_bar(text: &str, handle: &JoinHandle<impl Send>) {
    animate(text, &["|", "/", "-", "\\"], handle);
}

fn clear_line() {
    print!("\r\x1b[2K");
    let _ = io::stdout().flush();
}
