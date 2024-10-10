use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::thread;

fn handle_reading(mut reader: impl BufRead) {
    let mut buffer = String::new();
    while let Ok(_) = reader.read_line(&mut buffer) {
        if buffer.trim().is_empty() {
            break;
        }
        println!("Received from server: {}", buffer.trim());
        buffer.clear();
    }
}

fn handle_writing(mut writer: impl Write) {
    let stdin = io::stdin();
    let mut input = String::new();
    while stdin.read_line(&mut input).is_ok() {
        println!("Sending to server: {}", input.trim());
        writer
            .write_all(input.as_bytes())
            .expect("Failed to send message");
        writer.flush().unwrap();
        input.clear();
    }
}

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to server");
    let writer = stream.try_clone()?;
    let reader = io::BufReader::new(stream);

    // 읽기 스레드
    let read_thread = thread::spawn(move || {
        handle_reading(reader);
    });

    // 쓰기 스레드
    let write_thread = thread::spawn(move || {
        handle_writing(writer);
    });

    read_thread.join().unwrap();
    write_thread.join().unwrap();

    Ok(())
}
