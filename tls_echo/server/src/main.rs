use native_tls::{Identity, TlsAcceptor};
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // PKCS12 형식으로 인증서 변환이 필요합니다
    let mut cert_file = File::open("cert.crt")?;
    let mut key_file = File::open("private.key")?;
    let mut cert = Vec::new();
    let mut key = Vec::new();
    cert_file.read_to_end(&mut cert)?;
    key_file.read_to_end(&mut key)?;

    let identity = Identity::from_pkcs8(&cert, &key).unwrap();
    let acceptor = TlsAcceptor::new(identity).unwrap();
    let listener = TcpListener::bind("127.0.0.1:8443")?;

    println!("서버가 8443 포트에서 실행 중입니다...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                std::thread::spawn(move || {
                    let mut stream = acceptor.accept(stream).unwrap();
                    let mut buffer = [0; 1024];

                    loop {
                        match stream.read(&mut buffer) {
                            Ok(0) => break, // 연결 종료
                            Ok(n) => {
                                println!("받은 데이터: {}", String::from_utf8_lossy(&buffer[..n]));
                                stream.write_all(&buffer[..n]).unwrap();
                            }
                            Err(_) => break,
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("연결 수락 오류: {}", e);
            }
        }
    }
    Ok(())
}

// # 개인키 생성
// openssl genpkey -algorithm RSA -out private.key -pkeyopt rsa_keygen_bits:2048

// # 인증서 서명 요청(CSR) 생성
// openssl req -new -key private.key -out cert.csr -subj "/CN=localhost"

// # 자체 서명된 인증서 생성
// openssl x509 -req -days 365 -in cert.csr -signkey private.key -out cert.crt
