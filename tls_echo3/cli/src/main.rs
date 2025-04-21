use openssl::ssl::{SslConnector, SslFiletype, SslMethod};
use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TLS connector 설정
    let mut connector = SslConnector::builder(SslMethod::tls())?;
    connector.set_ca_file("../rootCA.pem")?;

    // 클라이언트 인증서 설정
    connector.set_certificate_file("../echo-client.pem", SslFiletype::PEM)?;
    connector.set_private_key_file("../echo-client-key.pem", SslFiletype::PEM)?;

    let connector = connector.build();

    // 서버에 연결
    let stream = TcpStream::connect("127.0.0.1:8443")?;
    let mut ssl_stream = connector.connect("localhost", stream)?;

    println!("서버에 연결되었습니다. 메시지를 입력하세요 (종료하려면 'quit' 입력):");

    loop {
        // 사용자 입력 받기
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim() == "quit" {
            break;
        }

        // 서버에 데이터 전송
        ssl_stream.write_all(input.as_bytes())?;

        // 서버로부터 응답 수신
        let mut buf = [0; 1024];
        match ssl_stream.read(&mut buf) {
            Ok(size) => {
                let response = String::from_utf8_lossy(&buf[..size]);
                println!("서버 응답: {}", response);
            }
            Err(e) => println!("읽기 오류: {}", e),
        }
    }

    Ok(())
}
