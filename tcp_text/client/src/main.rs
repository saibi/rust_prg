use std::io::{self, Read, Write};
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
        let mut buffer = String::new();
        loop {
            if io::stdin().read_line(&mut buffer).is_ok() {
                tx_clone.send(buffer.clone()).unwrap();
                buffer.clear();
            }
        }
    });

    // Spawn a thread to handle incoming messages
    let mut reader = stream.try_clone()?;
    thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break, // Connection closed
                Ok(n) => {
                    print!("{}", String::from_utf8_lossy(&buffer[..n]));
                    io::stdout().flush().unwrap();
                }
                Err(_) => break,
            }
        }
    });

    // Main loop to handle outgoing messages
    for message in rx {
        stream.write_all(message.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}
