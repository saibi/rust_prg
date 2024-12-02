use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self, JoinHandle};

fn handle_client(mut stream: UnixStream, tx: &Sender<String>) {
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
                tx.send(String::from_utf8_lossy(&buffer[..n]).to_string())
                    .unwrap();
            }
            Err(_) => break,
        }
    }
}

struct UnixDomainServer {
    sock_path: String,
    handle: Option<JoinHandle<()>>,
    receiver: Option<Receiver<String>>,
}

impl UnixDomainServer {
    fn new(sock_path: &str) -> Self {
        Self {
            sock_path: sock_path.to_string(),
            handle: None,
            receiver: None,
        }
    }

    fn run(&mut self) {
        let sock_path = self.sock_path.clone();

        let (tx, rx) = std::sync::mpsc::channel();

        let handle = thread::spawn(move || {
            let listener = UnixListener::bind(sock_path.as_str()).unwrap();
            println!("서버가 {} 에서 대기 중입니다", sock_path);

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        handle_client(stream, &tx);
                    }
                    Err(err) => {
                        eprintln!("연결 수락 오류: {:?}", err);
                    }
                }
            }
        });

        self.handle = Some(handle);
        self.receiver = Some(rx);
    }

    fn get_received_message(&self) -> Option<String> {
        match &self.receiver {
            Some(rx) => match rx.try_recv() {
                Ok(msg) => Some(msg),
                Err(_) => None,
            },
            None => {
                eprintln!("receiver is None");
                None
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/.rdecho.sock";

    // 기존 소켓 파일이 있다면 제거
    if std::path::Path::new(socket_path).exists() {
        std::fs::remove_file(socket_path)?;
    }

    println!("main thread start");

    let mut server = UnixDomainServer::new(socket_path);
    server.run();

    println!("main thread end. loop start");
    loop {
        if let Some(msg) = server.get_received_message() {
            println!("main: received message: {}", msg);
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }
}
