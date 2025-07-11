use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;

use rustls::{
    Certificate, PrivateKey, RootCertStore, ServerConfig, ServerConnection, Stream,
    server::AllowAnyAuthenticatedClient,
};

const BIND_ADDRESS: &str = "0.0.0.0:8443";

use clap::Parser;

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("cwd: {}", std::env::current_dir()?.display());

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

    // 루트 CA 인증서 로드
    let mut root_store = RootCertStore::empty();
    let root_ca_data = fs::read(args.rootca_cert)?;
    for cert in rustls_pemfile::certs(&mut root_ca_data.as_slice())? {
        root_store.add(&Certificate(cert))?;
    }

    // 클라이언트 인증 검증기 설정
    let client_auth = AllowAnyAuthenticatedClient::new(root_store);

    // 서버 설정 구성
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_client_cert_verifier(Arc::new(client_auth))
        // .with_no_client_auth()
        .with_single_cert(
            rustls_pemfile::certs(&mut fs::read(args.cert)?.as_slice())?
                .into_iter()
                .map(Certificate)
                .collect(),
            PrivateKey(
                rustls_pemfile::pkcs8_private_keys(&mut fs::read(args.key)?.as_slice())?.remove(0),
            ),
        )?;

    let config = Arc::new(config);

    // TCP 리스너 생성
    let listener = TcpListener::bind(BIND_ADDRESS)?;
    println!("서버가 {}에서 실행 중입니다...", BIND_ADDRESS);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let server_config = Arc::clone(&config);

                match ServerConnection::new(server_config) {
                    Ok(mut conn) => {
                        let mut tls = Stream::new(&mut conn, &mut stream);

                        // Perform handshake before accessing protocol_version
                        match tls.conn.complete_io(&mut tls.sock) {
                            Ok(_) => {
                                if let Some(protocol) = tls.conn.protocol_version() {
                                    println!("TLS version: {:?}", protocol);
                                } else {
                                    println!("Could not retrieve TLS version information.");
                                }
                            }
                            Err(e) => {
                                println!("TLS handshake failed: {}", e);
                                continue;
                            }
                        }

                        // 에코 서비스
                        let mut buf = [0; 1024];
                        loop {
                            match tls.read(&mut buf) {
                                Ok(0) => {
                                    println!("클라이언트 연결 종료");
                                    break;
                                }
                                Ok(n) => {
                                    tls.write_all(&buf[..n])?;
                                    println!("에코 완료: {} 바이트", n);
                                }
                                Err(e) => {
                                    println!("읽기 오류: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => println!("TLS 연결 생성 실패: {}", e),
                }
            }
            Err(e) => println!("연결 오류: {}", e),
        }
    }

    Ok(())
}
