use native_tls::{Certificate, TlsConnector};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    println!("cwd : {:?}", std::env::current_dir().unwrap());

    //let cert = include_bytes!("../cert.pem");
    let cert = include_bytes!("../../rootCA.pem");
    let cert = Certificate::from_pem(cert).unwrap();

    let mut builder = TlsConnector::builder();
    //builder.disable_built_in_roots(true);
    builder.add_root_certificate(cert);
    //builder.danger_accept_invalid_certs(true);
    //builder.danger_accept_invalid_hostnames(true);
    let connector = builder.build().unwrap();

    let stream = TcpStream::connect("localhost:8443")?;
    let mut stream = connector.connect("localhost", stream).unwrap();

    println!("서버에 연결되었습니다. 메시지를 입력하세요:");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        stream.write_all(input.as_bytes())?;

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        println!(
            "서버로부터 받은 응답: {}",
            String::from_utf8_lossy(&buffer[..bytes_read])
        );
    }
}
