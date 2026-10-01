use std::{thread, time::Duration};

use cliva_io::output::{Progress, progress};

fn main() {
    // Run each progress style one by one.
    for (name, style) in [
        ("Bar", Progress::Bar),
        ("Hashtags", Progress::Hashtags),
        ("Blocks", Progress::Blocks),
    ] {
        // Start progress at zero out of 100.
        let mut bar = progress(name, style, 0, 100);

        // Increase progress until it reaches 100.
        for _ in 0..100 {
            bar.inc();
            thread::sleep(Duration::from_millis(20));
        }

        // Finish the progress bar and move to a new line.
        bar.finish();
    }

    // Show that all progress bars are complete.
    println!("All tasks completed!");
}
