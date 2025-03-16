use std::io::{self, BufRead};
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    const SERVER_ADDR: &str = "127.0.0.1:12333";

    // UDP 소켓 생성 및 바인딩 (임의의 포트)
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // 서버 주소 설정
    let server_addr = SERVER_ADDR;
    println!("UDP 에코 클라이언트가 시작되었습니다.");
    println!("메시지를 입력하세요 ('/q'를 입력하면 종료):");

    // 표준 입력에서 한 줄씩 읽기
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 버퍼 크기 설정
    let mut buf = [0; 1024];

    // 사용자 입력 처리
    while let Some(Ok(line)) = lines.next() {
        // '/q' 입력 시 종료
        if line == "/q" {
            println!("프로그램을 종료합니다.");
            break;
        }

        // 서버로 메시지 전송
        socket.send_to(line.as_bytes(), server_addr)?;

        // 서버로부터 응답 수신
        match socket.recv_from(&mut buf) {
            Ok((size, _)) => {
                let response = String::from_utf8_lossy(&buf[..size]);
                println!("서버 응답: {}", response);
            }
            Err(e) => {
                eprintln!("서버 응답 수신 중 오류 발생: {}", e);
            }
        }
    }

    Ok(())
}
