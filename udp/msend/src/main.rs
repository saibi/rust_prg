use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::str;
use std::thread;
use std::time::Duration;

const MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 0, 1);
const PORT: u16 = 12344;
const BUFFER_SIZE: usize = 1024;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // 소켓을 복제하여 송신용과 수신용으로 사용
    let socket_clone = socket.try_clone()?;

    // 수신 스레드 시작
    let _receiver_thread = thread::spawn(move || {
        let mut buffer = [0u8; BUFFER_SIZE];
        loop {
            match socket.recv_from(&mut buffer) {
                Ok((size, src)) => {
                    let message = str::from_utf8(&buffer[..size]).unwrap_or("Invalid UTF-8");
                    println!("수신: {} (발신지: {})", message, src);
                }
                Err(e) => {
                    eprintln!("수신 오류: {}", e);
                }
            }
        }
    });

    // 잠시 대기 후 "hello" 메시지 전송 (수신 준비 시간 확보)
    thread::sleep(Duration::from_millis(500));
    println!("멀티캐스트 그룹에 'hello' 메시지 전송");
    let multicast_addr = SocketAddr::new(IpAddr::V4(MULTICAST_ADDR), PORT);
    socket_clone.send_to("hello".as_bytes(), multicast_addr)?;

    // 메인 스레드가 종료되지 않도록 대기
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
