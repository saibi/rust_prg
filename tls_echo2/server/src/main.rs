use native_tls::{Identity, TlsAcceptor};
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let mut cert_file = File::open("cert.pem")?;
    let mut key_file = File::open("key.pem")?;
    let mut cert = Vec::new();
    let mut key = Vec::new();
    cert_file.read_to_end(&mut cert)?;
    key_file.read_to_end(&mut key)?;

    let identity = Identity::from_pkcs8(&cert, &key).unwrap();
    let acceptor = TlsAcceptor::new(identity).unwrap();
    let listener = TcpListener::bind("0.0.0.0:8443")?;

    println!("서버가 8443 포트에서 실행 중입니다...");

    for stream in listener.incoming() {
        let acceptor = acceptor.clone();
        let mut stream = acceptor.accept(stream?).unwrap();

        std::thread::spawn(move || {
            let mut buffer = [0; 1024];
            loop {
                let bytes_read = stream.read(&mut buffer).unwrap();
                if bytes_read == 0 {
                    return;
                }
                println!("echo: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                stream.write_all(&buffer[..bytes_read]).unwrap();
            }
        });
    }
    Ok(())
}

// openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes -subj "/CN=localhost"
