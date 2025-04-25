use std::io::{self, Read, Write};
use std::net::TcpStream;

const ADDR: &str = "127.0.0.1:8279";

fn main() {
    let mut stream = TcpStream::connect(ADDR).unwrap();
    println!("Connected to server at {}", ADDR);
    println!("Type 'quit' to exit");

    let mut buffer = [0; 1024];
    let mut input = String::new();

    loop {
        input.clear();
        println!("Enter message: ");
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            println!("Exiting...");
            break;
        }

        if let Err(e) = stream.write_all(input.as_bytes()) {
            println!("Write error: {}", e);
            break;
        }
        stream.flush().unwrap();
        println!("send: {}", input);

        match stream.read(&mut buffer) {
            Ok(n) => println!("recv: {}", String::from_utf8_lossy(&buffer[..n])),
            Err(e) => {
                println!("Read error: {}", e);
                break;
            }
        }
    }
}
