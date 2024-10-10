use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
    let mut reader = io::BufReader::new(stream.try_clone().unwrap());
    let writer = stream;

    // 클라이언트로부터 받은 메시지를 에코하는 스레드
    let read_thread = thread::spawn(move || {
        let mut buffer = String::new();
        while reader.read_line(&mut buffer).is_ok() {
            if buffer.trim().is_empty() {
                break;
            }
            println!("from client: {}", buffer.trim());
            buffer.clear();
        }
    });

    // 서버에서 클라이언트로 보내는 스레드 (서버 stdin 입력)
    let write_thread = thread::spawn(move || {
        let stdin = io::stdin();
        let mut input = String::new();
        let mut writer = writer;
        while stdin.read_line(&mut input).is_ok() {
            writer.write_all(input.as_bytes()).expect("Failed to write");
            writer.flush().unwrap();
            input.clear();
        }
    });

    read_thread.join().unwrap();
    write_thread.join().unwrap();
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream?;
        println!("New connection");

        thread::spawn(move || {
            handle_client(stream);
        });
    }

    Ok(())
}
