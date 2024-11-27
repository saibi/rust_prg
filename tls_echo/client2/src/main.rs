use native_tls::{Certificate, TlsConnector};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;

fn load_certificate(cert_path: &str) -> Certificate {
    let mut cert_file = File::open(cert_path).expect("Cannot open certificate file");
    let mut cert_reader = BufReader::new(cert_file);
    let mut cert_contents = String::new();
    cert_reader
        .read_to_string(&mut cert_contents)
        .expect("Failed to read certificate");

    // PEM 형식의 인증서에서 Base64 디코딩
    let cert_lines: Vec<&str> = cert_contents
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect();
    let cert_base64 = cert_lines.join("");

    let cert_der = base64::decode(&cert_base64).expect("Failed to decode base64");

    Certificate::from_der(&cert_der).expect("Failed to create certificate")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 서버 인증서 로드
    let cert = load_certificate("cert.crt");

    // TLS 커넥터 생성
    let mut connector = TlsConnector::builder();
    connector.add_root_certificate(cert);
    let connector = connector.build()?;

    // TCP 스트림 생성 및 TLS 연결
    let stream = TcpStream::connect("127.0.0.1:8443")?;
    let mut tls_stream = connector.connect("localhost", stream)?;

    // 메시지 전송 및 에코 수신
    let messages = ["Hello", "TLS", "Encryption"];

    for msg in &messages {
        tls_stream.write_all(msg.as_bytes())?;

        let mut buffer = [0; 1024];
        let n = tls_stream.read(&mut buffer)?;

        println!("Received echo: {}", String::from_utf8_lossy(&buffer[0..n]));
    }

    Ok(())
}
