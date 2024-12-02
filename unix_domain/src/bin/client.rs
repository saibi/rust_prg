use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/.rdecho.sock";
    let mut stream = UnixStream::connect(socket_path)?;
    println!("서버에 연결되었습니다");

    println!("/q 를 입력하면 종료합니다.");

    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin().read_line(&mut input)?;

        let trimmed = input.trim();
        if trimmed == "/q" {
            println!("프로그램을 종료합니다.");
            break;
        }

        stream.write_all(trimmed.as_bytes())?;
        println!("메시지 전송: {}", trimmed);

        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(n) => {
                let response = String::from_utf8_lossy(&buffer[..n]);
                println!("서버로부터 받은 응답: {}", response);
            }
            Err(e) => {
                eprintln!("응답을 읽는 중 오류 발생: {}", e);
                break;
            }
        }
    }
    Ok(())
}
