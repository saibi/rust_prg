use std::{thread, time::Duration};

/// kill -2 <pid>
fn main() {
    println!("Hello, world!");

    ctrlc::set_handler(|| {
        println!("Received Ctrl-C!");
    })
    .expect("Error setting Ctrl-C handler");

    println!("waiting for Ctrl-C... (30 seconds)");
    thread::sleep(Duration::from_secs(30));
}
