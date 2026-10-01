use cliva_io::output::{Loader, loader};
use std::{thread, time::Duration};

fn main() {
    println!("Running loader examples...\n");

    loader(Loader::Dots, "Compiling project", || {
        thread::sleep(Duration::from_secs(5));
    });
    println!("✓ Compilation complete!\n");

    loader(Loader::Bar, "Downloading dependencies", || {
        thread::sleep(Duration::from_secs(5));
    });
    println!("✓ Download complete!\n");
}
