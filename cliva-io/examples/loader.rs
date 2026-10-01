use std::{thread, time::Duration};

use cliva_io::output::{Loader, loader};

fn main() {
    // Run a task with the dots loader.
    loader(Loader::Dots, "Loading data", || {
        thread::sleep(Duration::from_secs(2));
    });

    // Run a task with the bar loader.
    loader(Loader::Bar, "Saving data", || {
        thread::sleep(Duration::from_secs(2));
    });

    // Show that both tasks are complete.
    println!("All tasks completed!");
}
