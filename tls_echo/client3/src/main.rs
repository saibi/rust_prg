use openssl::ssl::{SslConnector, SslMethod};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut ctx = SslConnector::builder(SslMethod::tls()).unwrap();
    ctx.set_ca_file("../server/cert.crt").unwrap(); // 루트 CA 인증서 설정 (필요한 경우)
    let stream = TcpStream::connect("127.0.0.1:8443").unwrap();
    let mut ssl = ctx.build().connect("localhost", stream).unwrap();

    let mut buf = [0; 1024];
    loop {
        let stdin = std::io::stdin();
        let mut s = String::new();
        stdin.read_line(&mut s).expect("Failed to read from stdin");
        ssl.write_all(s.as_bytes()).unwrap();

        let len = ssl.read(&mut buf).unwrap();
        if len == 0 {
            break;
        }
        println!("{}", String::from_utf8_lossy(&buf[..len]));
    }
}
