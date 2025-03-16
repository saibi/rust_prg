use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    const PORT: u16 = 12333;
    // 서버 소켓 생성 및 바인딩 (localhost:8888)
    let socket = UdpSocket::bind(format!("127.0.0.1:{}", PORT))?;
    println!("UDP 에코 서버가 127.0.0.1:{}에서 실행 중입니다...", PORT);

    // 버퍼 크기 설정
    let mut buf = [0; 1024];

    // 무한 루프로 클라이언트 요청 처리
    loop {
        // 클라이언트로부터 데이터 수신
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                println!(
                    "클라이언트 {}로부터 메시지 수신: {}",
                    src,
                    String::from_utf8_lossy(&buf[..size])
                );

                // 수신한 데이터를 그대로 클라이언트에게 다시 전송
                socket.send_to(&buf[..size], src)?;
            }
            Err(e) => {
                eprintln!("데이터 수신 중 오류 발생: {}", e);
            }
        }
    }
}
