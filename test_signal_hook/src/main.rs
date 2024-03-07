use signal_hook::{consts::*, iterator::Signals};
use std::{error::Error, thread};
fn main() -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new(&[SIGINT, SIGTERM])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });

    println!("Waiting for signals...");
    thread::sleep(std::time::Duration::from_secs(60));
    println!("Done waiting.");
    Ok(())
}
