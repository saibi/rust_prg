use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TLS acceptor 설정
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    acceptor.set_private_key_file("localhost-key.pem", SslFiletype::PEM)?;
    acceptor.set_certificate_chain_file("localhost.pem")?;
    acceptor.check_private_key()?;
    let acceptor = acceptor.build();

    // TCP 리스너 생성
    let listener = TcpListener::bind("127.0.0.1:8443")?;
    println!("서버가 127.0.0.1:8443에서 실행 중입니다...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // TLS 스트림 생성
                let mut ssl_stream = acceptor.accept(stream)?;

                // 클라이언트로부터 데이터 수신
                let mut buf = [0; 1024];
                loop {
                    match ssl_stream.read(&mut buf) {
                        Ok(0) => {
                            // 연결이 종료됨
                            println!("클라이언트 연결 종료");
                            break;
                        }
                        Ok(size) => {
                            // 수신한 데이터를 그대로 에코
                            ssl_stream.write_all(&buf[..size])?;
                            println!("에코 완료: {} 바이트", size);
                        }
                        Err(e) => {
                            println!("읽기 오류: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => println!("연결 오류: {}", e),
        }
    }

    Ok(())
}
