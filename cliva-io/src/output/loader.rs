use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

#[derive(Debug, Clone, Copy)]
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
    let frames: &[&str] = match style {
        Loader::Dots => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
        Loader::Bar => &["|", "/", "-", "\\"],
    };

    let mut stdout = io::stdout();
    let mut i = 0;

    while !handle.is_finished() {
        let _ = write!(stdout, "\r{} {}  ", frames[i % frames.len()], text);
        let _ = stdout.flush();

        thread::sleep(Duration::from_millis(80));
        i += 1;
    }

    let _ = write!(stdout, "\r\x1b[2K");
    let _ = stdout.flush();

    match handle.join() {
        Ok(result) => result,
        Err(error) => std::panic::resume_unwind(error),
    }
}
