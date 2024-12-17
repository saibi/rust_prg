use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let _logger = lib::logger::start("debug", "", true);
    log::debug!("server!");

    let listener = TcpListener::bind("127.0.0.1:18181").unwrap();

    for stream in listener.incoming() {
        log::debug!("accept client");
        let mut stream = stream.unwrap();

        loop {
            let mut buf = [0; 1024];
            match stream.read(&mut buf) {
                Ok(n) => {
                    if n == 0 {
                        // connection was closed
                        break;
                    }
                    log::debug!("recv from client: {}", String::from_utf8_lossy(&buf));
                    stream.write(&buf[0..n]).unwrap();
                }
                Err(err) => {
                    panic!("{}", err);
                }
            }
        }
    }
}
