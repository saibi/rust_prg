use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();
    println!("Server listening on port 7879");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("New connection: {:?}", stream.peer_addr().unwrap());

        std::thread::spawn(move || {
            let mut buffer = [0; 1024];

            loop {
                let bytes_read = stream.read(&mut buffer).unwrap();
                if bytes_read == 0 {
                    return;
                }

                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received: {}", message);

                let response = format!("Echo: {}", message);
                stream.write_all(response.as_bytes()).unwrap();
            }
        });
    }
}
