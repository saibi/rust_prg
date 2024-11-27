use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true) // 자체 서명된 인증서를 위해 필요
        .build()
        .unwrap();

    let stream = TcpStream::connect("127.0.0.1:8443")?;
    let mut stream = connector.connect("localhost", stream).unwrap();

    println!("서버에 연결되었습니다!");

    // 테스트 메시지 전송
    let msg = "안녕하세요!";
    stream.write_all(msg.as_bytes())?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    println!(
        "서버로부터 받은 응답: {}",
        String::from_utf8_lossy(&buffer[..n])
    );

    Ok(())
}
