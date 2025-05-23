use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::TcpListener;

// const ROOT_CA_PATH: &str = "../certs/rootCA.pem";
const SERVER_CERT_PATH: &str = "../certs/server.pem";
const SERVER_KEY_PATH: &str = "../certs/server-key.pem";

const BIND_ADDRESS: &str = "0.0.0.0:8443";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TLS acceptor 설정
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

    acceptor.set_private_key_file(SERVER_KEY_PATH, SslFiletype::PEM)?;

    // load private key from file
    // let key_data = std::fs::read("../localhost-key.pem")?;
    // let key = openssl::pkey::PKey::private_key_from_pem(&key_data)?;
    // acceptor.set_private_key(&key)?;

    acceptor.set_certificate_chain_file(SERVER_CERT_PATH)?;
    // load certificate from file
    // let cert_data = std::fs::read("../localhost.pem")?;
    // let cert = openssl::x509::X509::from_pem(&cert_data)?;
    // acceptor.set_certificate(&cert)?;

    // 클라이언트 인증서 검증 설정
    // acceptor.set_verify(SslVerifyMode::PEER | SslVerifyMode::FAIL_IF_NO_PEER_CERT);
    // acceptor.set_ca_file(ROOT_CA_PATH)?;

    acceptor.check_private_key()?;
    let acceptor = acceptor.build();

    // TCP 리스너 생성
    let listener = TcpListener::bind(BIND_ADDRESS)?;
    println!("Server is running on {}...", BIND_ADDRESS);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // TLS 스트림 생성
                let mut ssl_stream = match acceptor.accept(stream) {
                    Ok(stream) => stream,
                    Err(e) => {
                        println!("TLS handshake failed: {}", e);
                        continue;
                    }
                };

                // 클라이언트 인증서 검증
                if let Some(cert) = ssl_stream.ssl().peer_certificate() {
                    println!("Client certificate verification successful");
                    if let Some(subject) = cert.subject_name().entries().next() {
                        println!("Client subject: {}", subject.data().as_utf8()?);
                    }
                }

                // 클라이언트로부터 데이터 수신
                let mut buf = [0; 1024];
                loop {
                    match ssl_stream.read(&mut buf) {
                        Ok(0) => {
                            println!("Client connection closed");
                            break;
                        }
                        Ok(size) => {
                            ssl_stream.write_all(&buf[..size])?;
                            println!(
                                "recv&echo {} bytes : {}",
                                size,
                                String::from_utf8_lossy(&buf[..size])
                            );
                        }
                        Err(e) => {
                            println!("Read error: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => println!("Connection error: {}", e),
        }
    }

    Ok(())
}
