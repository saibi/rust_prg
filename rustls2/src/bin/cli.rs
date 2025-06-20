use anyhow::Result;
use clap::Parser;
use rustls::{Certificate, ClientConfig, PrivateKey, RootCertStore};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// root ca cert path
    #[arg(short = 'r', long, default_value = "ca.cert.pem")]
    pub rootca_cert: String,

    /// cert path
    #[arg(short = 'c', long, default_value = "cert.pem")]
    pub cert: String,

    /// key path
    #[arg(short = 'k', long, default_value = "key.pem")]
    pub key: String,

    /// 서버 주소 (예: 127.0.0.1:8443)
    #[arg(short = 'a', long, default_value = "127.0.0.1:8443")]
    server_address: String,

    /// server hostname (예: localhost)
    #[arg(short = 's', long, default_value = "localhost")]
    server_hostname: String,
}

fn load_client_cert_and_key(
    cert_path: &str,
    key_path: &str,
) -> Result<(Vec<Certificate>, PrivateKey)> {
    let certs = rustls_pemfile::certs(&mut BufReader::new(std::fs::File::open(cert_path)?))?;
    let certs = certs.into_iter().map(Certificate).collect();

    let mut keys =
        rustls_pemfile::pkcs8_private_keys(&mut BufReader::new(std::fs::File::open(key_path)?))?;
    let key = PrivateKey(keys.remove(0));

    Ok((certs, key))
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !std::path::Path::new(&args.rootca_cert).exists() {
        eprintln!(
            "Error: root CA cert file '{}' does not exist.",
            args.rootca_cert
        );
        std::process::exit(1);
    }
    if !std::path::Path::new(&args.cert).exists() {
        eprintln!("Error: cert file '{}' does not exist.", args.cert);
        std::process::exit(1);
    }
    if !std::path::Path::new(&args.key).exists() {
        eprintln!("Error: key file '{}' does not exist.", args.key);
        std::process::exit(1);
    }

    // Root CA 인증서 로드
    let mut root_store = RootCertStore::empty();
    let certs = rustls_pemfile::certs(&mut BufReader::new(std::fs::File::open(args.rootca_cert)?))?;
    for cert in certs {
        root_store.add(&rustls::Certificate(cert))?;
    }

    // 클라이언트 인증서와 키 로드
    let (client_certs, client_key) = load_client_cert_and_key(&args.cert, &args.key)?;

    // TLS 설정
    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        // .with_no_client_auth();
        .with_client_auth_cert(client_certs, client_key)?;

    let mut connector =
        rustls::ClientConnection::new(Arc::new(config), args.server_hostname.as_str().try_into()?)?;

    // 서버에 연결
    let mut stream = TcpStream::connect(&args.server_address)?;
    let mut tls_stream = rustls::Stream::new(&mut connector, &mut stream);

    // TLS 핸드셰이크를 완료하여 프로토콜 버전을 협상
    tls_stream.flush()?; // 핸드셰이크를 강제로 완료
    let protocol_version = tls_stream.conn.protocol_version();
    if let Some(version) = protocol_version {
        println!("Connected. TLS protocol version: {:?}", version);
    } else {
        println!("TLS protocol version: not negotiated yet");
    }
    println!("Enter message(type 'quit' to exit):");

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
