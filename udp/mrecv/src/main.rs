use std::io;
use std::net::{Ipv4Addr, UdpSocket};
use std::str;

const MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 0, 1);
const PORT: u16 = 12344;
const BUFFER_SIZE: usize = 1024;

fn main() -> io::Result<()> {
    // 소켓 생성 및 바인딩 (멀티캐스트 수신을 위해 특정 포트에 바인딩)
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", PORT))?;

    // 멀티캐스트 그룹 가입
    socket.join_multicast_v4(&MULTICAST_ADDR, &Ipv4Addr::new(0, 0, 0, 0))?;

    println!("멀티캐스트 수신 준비 완료");
    println!("멀티캐스트 주소: {}, 포트: {}", MULTICAST_ADDR, PORT);

    // 수신 loop 시작
    let mut buffer = [0u8; BUFFER_SIZE];
    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, src)) => {
                let message = str::from_utf8(&buffer[..size]).unwrap_or("Invalid UTF-8");
                println!("수신: {} (발신지: {})", message, src);

                // "hello" 메시지를 받으면 "ok" 응답
                if message == "hello" {
                    println!("'hello' 메시지 수신, 'ok' 응답 전송");
                    let response = "ok";
                    if let Err(e) = socket.send_to(response.as_bytes(), src) {
                        eprintln!("응답 전송 실패: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("수신 오류: {}", e);
            }
        }
    }
}
