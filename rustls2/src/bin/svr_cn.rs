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

    /// allowed client CN (Common Name)
    #[arg(short = 'n', long, default_value = "client")]
    pub allowed_cn: String,
}

// CN 값을 추출하는 함수
fn extract_cn_from_cert(cert: &Certificate) -> Result<String, Box<dyn std::error::Error>> {
    use std::io::Write;
    use std::process::Command;
    use tempfile::NamedTempFile;

    // 임시 파일에 인증서 저장
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(&cert.0)?;
    temp_file.flush()?;

    // OpenSSL 명령어로 CN 추출
    let output = Command::new("openssl")
        .args(&[
            "x509",
            "-noout",
            "-subject",
            "-in",
            temp_file.path().to_str().unwrap(),
        ])
        .output()?;

    if output.status.success() {
        let subject = String::from_utf8(output.stdout)?;
        // "subject=CN = client" 형태에서 CN 값 추출
        if let Some(cn_start) = subject.find("CN=") {
            let cn_part = &subject[cn_start + 5..];
            if let Some(cn_end) = cn_part.find('\n') {
                return Ok(cn_part[..cn_end].trim().to_string());
            } else {
                return Ok(cn_part.trim().to_string());
            }
        }
    }

    Err("CN not found in certificate".into())
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
    println!("허용된 클라이언트 CN: {}", args.allowed_cn);

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

                                // 클라이언트 인증서에서 CN 추출 및 검증
                                if let Some(client_certs) = tls.conn.peer_certificates() {
                                    if let Some(client_cert) = client_certs.first() {
                                        match extract_cn_from_cert(client_cert) {
                                            Ok(cn) => {
                                                if cn == args.allowed_cn {
                                                    println!(
                                                        "클라이언트 인증서 CN 검증 성공: {}",
                                                        cn
                                                    );
                                                } else {
                                                    println!(
                                                        "클라이언트 인증서 CN 불일치: 예상={}, 실제={}",
                                                        args.allowed_cn, cn
                                                    );
                                                    println!("연결을 거부합니다.");
                                                    // TLS 연결 종료 후 다음 연결로 넘어감
                                                    let _ = tls.conn.send_close_notify();
                                                    let _ = tls.conn.complete_io(&mut tls.sock);
                                                    continue;
                                                }
                                            }
                                            Err(e) => {
                                                println!("클라이언트 인증서 CN 추출 실패: {}", e);
                                                println!("연결을 거부합니다.");
                                                // TLS 연결 종료 후 다음 연결로 넘어감
                                                let _ = tls.conn.send_close_notify();
                                                let _ = tls.conn.complete_io(&mut tls.sock);
                                                continue;
                                            }
                                        }
                                    } else {
                                        println!("클라이언트 인증서가 없습니다.");
                                        // TLS 연결 종료 후 다음 연결로 넘어감
                                        let _ = tls.conn.send_close_notify();
                                        let _ = tls.conn.complete_io(&mut tls.sock);
                                        continue;
                                    }
                                } else {
                                    println!("클라이언트 인증서 정보를 가져올 수 없습니다.");
                                    // TLS 연결 종료 후 다음 연결로 넘어감
                                    let _ = tls.conn.send_close_notify();
                                    let _ = tls.conn.complete_io(&mut tls.sock);
                                    continue;
                                }
                            }
                            Err(e) => {
                                println!("TLS handshake failed: {}", e);
                                continue; // 핸드셰이크 실패 시 바로 다음 연결로
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
