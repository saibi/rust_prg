use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

fn handle_client(mut stream: UnixStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // 연결 종료
            Ok(n) => {
                println!(
                    "echo from client: {}",
                    String::from_utf8_lossy(&buffer[..n])
                );
                if stream.write_all(&buffer[..n]).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/.rdecho.sock";

    // 기존 소켓 파일이 있다면 제거
    if std::path::Path::new(socket_path).exists() {
        std::fs::remove_file(socket_path)?;
    }

    let listener = UnixListener::bind(socket_path)?;
    println!("서버가 {} 에서 대기 중입니다", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(err) => {
                eprintln!("연결 수락 오류: {:?}", err);
            }
        }
    }
    Ok(())
}
