use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7879")?;
    println!("Connected to server");

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let tx_clone = tx.clone();

    // Spawn a thread to handle user input
    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(input) = line {
                tx_clone.send(input).unwrap();
            }
        }
    });

    // Spawn a thread to handle incoming messages
    let mut reader = BufReader::new(stream.try_clone()?);
    thread::spawn(move || {
        loop {
            let mut buffer = String::new();
            match reader.read_line(&mut buffer) {
                Ok(0) => break, // Connection closed
                Ok(_) => print!("Received: {}", buffer),
                Err(_) => break,
            }
        }
    });

    // Main loop to handle outgoing messages
    for message in rx {
        stream.write_all(message.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
    }

    Ok(())
}
