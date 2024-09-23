use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

fn handle_client(mut stream: TcpStream, tx: Sender<String>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => break, // Connection closed
            Ok(_) => {
                print!("Received: {}", buffer);
                tx.send(buffer).unwrap();
            }
            Err(_) => break,
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7879")?;
    println!("Server listening on port 8080");

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

    // Accept a single client connection
    let (mut stream, _) = listener.accept()?;
    println!("Client connected");

    let stream_clone = stream.try_clone()?;
    thread::spawn(move || {
        handle_client(stream_clone, tx);
    });

    // Main loop to handle messages
    for received in rx {
        stream.write_all(received.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}
