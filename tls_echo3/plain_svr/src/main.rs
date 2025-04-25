use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const ADDR: &str = "0.0.0.0:8279";

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Connection closed by client");
                break;
            }
            Ok(n) => {
                println!("recv & echo: {}", String::from_utf8_lossy(&buffer[..n]));
                if let Err(e) = stream.write_all(&buffer[..n]) {
                    println!("Write error: {}", e);
                    break;
                }
                if let Err(e) = stream.flush() {
                    println!("Flush error: {}", e);
                    break;
                }
            }
            Err(e) => {
                println!("Read error: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind(ADDR).unwrap();
    println!("server listening on {ADDR}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => println!("error: {}", e),
        }
    }
}
