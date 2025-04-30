use anyhow::Result;
use clap::Parser;
use rustls::{Certificate, ClientConfig, PrivateKey, RootCertStore};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

const SERVER_HOSTNAME: &str = "127.0.0.1";
const ROOT_CA_PATH: &str = "../certs/rootCA.pem";
const CLIENT_CERT_PATH: &str = "../certs/echo-client.pem";
const CLIENT_KEY_PATH: &str = "../certs/echo-client-key.pem";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 서버 주소 (예: 127.0.0.1:8443)
    #[arg(short, long, default_value = "127.0.0.1:8443")]
    server_address: String,
}

fn load_client_cert_and_key() -> Result<(Vec<Certificate>, PrivateKey)> {
    let certs = rustls_pemfile::certs(&mut BufReader::new(std::fs::File::open(CLIENT_CERT_PATH)?))?;
    let certs = certs.into_iter().map(Certificate).collect();

    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut BufReader::new(std::fs::File::open(
        CLIENT_KEY_PATH,
    )?))?;
    let key = PrivateKey(keys.remove(0));

    Ok((certs, key))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Root CA 인증서 로드
    let mut root_store = RootCertStore::empty();
    let certs = rustls_pemfile::certs(&mut BufReader::new(std::fs::File::open(ROOT_CA_PATH)?))?;
    for cert in certs {
        root_store.add(&rustls::Certificate(cert))?;
    }

    // 클라이언트 인증서와 키 로드
    let (client_certs, client_key) = load_client_cert_and_key()?;

    // TLS 설정
    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    // .with_client_auth_cert(client_certs, client_key)?;

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
