use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let _logger = lib::logger::start("debug", "", true);

    log::debug!("client");
    let mut stream = TcpStream::connect("127.0.0.1:18181").unwrap();

    stream.write_all(b"hello\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("recv from server: {}", String::from_utf8_lossy(&buf));
}
