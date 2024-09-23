use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7879").unwrap();
    println!("Connected to server");

    loop {
        let mut buffer = String::new();
        println!("Enter message:");
        io::stdin().read_line(&mut buffer).unwrap();

        stream.write_all(buffer.as_bytes()).unwrap();

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received: {}", message);
    }
}
