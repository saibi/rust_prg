use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

fn handle_client(mut stream: TcpStream, tx: Sender<Vec<u8>>) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed
            Ok(n) => {
                print!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                io::stdout().flush().unwrap();
                tx.send(buffer[..n].to_vec()).unwrap();
            }
            Err(_) => break,
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7879")?;
    println!("Server listening on port 8080");

    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
    let tx_clone = tx.clone();

    // Spawn a thread to handle user input
    thread::spawn(move || {
        let mut buffer = String::new();
        loop {
            if io::stdin().read_line(&mut buffer).is_ok() {
                tx_clone.send(buffer.as_bytes().to_vec()).unwrap();
                buffer.clear();
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
        stream.write_all(&received)?;
        stream.flush()?;
    }

    Ok(())
}
