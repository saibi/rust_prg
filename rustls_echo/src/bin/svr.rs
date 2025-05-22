use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;

use rustls::{Certificate, PrivateKey, RootCertStore, ServerConfig, ServerConnection, Stream};

const ROOT_CA_PATH: &str = "./certs/ca.cert.pem";
const SERVER_CERT_PATH: &str = "./certs/server.cert.pem";
const SERVER_KEY_PATH: &str = "./certs/server.key.pem";
const BIND_ADDRESS: &str = "0.0.0.0:8443";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cwd: {}", std::env::current_dir()?.display());

    // 루트 CA 인증서 로드
    let mut root_store = RootCertStore::empty();
    let root_ca_data = fs::read(ROOT_CA_PATH)?;
    for cert in rustls_pemfile::certs(&mut root_ca_data.as_slice())? {
        root_store.add(&Certificate(cert))?;
    }

    // 서버 설정 구성
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(
            rustls_pemfile::certs(&mut fs::read(SERVER_CERT_PATH)?.as_slice())?
                .into_iter()
                .map(Certificate)
                .collect(),
            PrivateKey(
                rustls_pemfile::pkcs8_private_keys(&mut fs::read(SERVER_KEY_PATH)?.as_slice())?
                    .remove(0),
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
