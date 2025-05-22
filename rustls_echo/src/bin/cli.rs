use anyhow::Result;
use clap::Parser;
use rustls::{ClientConfig, RootCertStore};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

const SERVER_HOSTNAME: &str = "localhost";
const ROOT_CA_PATH: &str = "./certs/ca.cert.pem";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 서버 주소 (예: 127.0.0.1:8443)
    #[arg(short, long, default_value = "127.0.0.1:8443")]
    server_address: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Root CA 인증서 로드
    let mut root_store = RootCertStore::empty();
    let certs = rustls_pemfile::certs(&mut BufReader::new(std::fs::File::open(ROOT_CA_PATH)?))?;
    for cert in certs {
        root_store.add(&rustls::Certificate(cert))?;
    }

    // TLS 설정
    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let mut connector =
        rustls::ClientConnection::new(Arc::new(config), SERVER_HOSTNAME.try_into()?)?;

    // 서버에 연결
    let mut stream = TcpStream::connect(&cli.server_address)?;
    let mut tls_stream = rustls::Stream::new(&mut connector, &mut stream);

    println!("Connected. Enter message(type 'quit' to exit):");

    loop {
        // 사용자 입력 받기
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim() == "quit" {
            break;
        }

        // 서버에 데이터 전송
        if let Err(e) = tls_stream.write_all(input.as_bytes()) {
            println!("write error: {}", e);
            break;
        }

        // 서버로부터 응답 수신
        let mut buf = [0; 1024];
        match tls_stream.read(&mut buf) {
            Ok(size) => {
                let response = String::from_utf8_lossy(&buf[..size]);
                println!("recv: {}", response);
            }
            Err(e) => {
                println!("read error: {}", e);
                break;
            }
        }

        // 100ms 대기
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
