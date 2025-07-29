use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::str;
use std::thread;
use std::time::Duration;

const MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 0, 1);
const PORT: u16 = 12344;
const BUFFER_SIZE: usize = 1024;

fn main() -> io::Result<()> {
    // 소켓 생성 및 바인딩 (멀티캐스트 수신을 위해 특정 포트에 바인딩)
    // let socket = UdpSocket::bind(format!("0.0.0.0:{}", PORT))?;

    let socket = UdpSocket::bind(format!("0.0.0.0:{PORT}"))?;

    // 멀티캐스트 그룹 가입
    socket.join_multicast_v4(&MULTICAST_ADDR, &Ipv4Addr::new(0, 0, 0, 0))?;

    // 소켓을 복제하여 송신용과 수신용으로 사용
    let socket_clone = socket.try_clone()?;

    // 자신의 실제 IP 주소 가져오기 (127.0.0.1이나 0.0.0.0이 아닌)
    let my_ip = match local_ip_address::local_ip() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("로컬 IP 주소를 가져오는 데 실패했습니다: {e}");
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)) // 기본값으로 로컬호스트 사용
        }
    };
    println!("내 IP 주소: {my_ip}");

    // Cargo.toml에 다음 의존성 추가 필요:
    // local-ip-address = "0.2.0"

    // 자신의 IP 주소 가져오기 (메시지 필터링용)
    let local_addr = socket.local_addr()?;
    println!("로컬 주소: {local_addr}");

    // 수신 스레드 시작
    let _receiver_thread = thread::spawn(move || {
        let mut buffer = [0u8; BUFFER_SIZE];
        loop {
            match socket.recv_from(&mut buffer) {
                Ok((size, src)) => {
                    // 자신이 보낸 메시지는 무시 (IP 주소로 필터링)
                    if src.ip() == local_addr.ip() {
                        continue;
                    }

                    let message = str::from_utf8(&buffer[..size]).unwrap_or("Invalid UTF-8");
                    println!("수신: {message} (발신지: {src})");

                    // "hello" 메시지를 받으면 "ok" 응답
                    if message == "hello" {
                        println!("'hello' 메시지 수신, 'ok' 응답 전송");
                        let response = "ok";
                        // 멀티캐스트 그룹에 응답
                        // let multicast_addr = SocketAddr::new(IpAddr::V4(MULTICAST_ADDR), PORT);
                        if let Err(e) = socket.send_to(response.as_bytes(), src) {
                            eprintln!("응답 전송 실패: {e}");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("수신 오류: {e}");
                }
            }
        }
    });

    // 잠시 대기 후 "hello" 메시지 전송 (수신 준비 시간 확보)
    thread::sleep(Duration::from_millis(500));
    println!("멀티캐스트 그룹에 'hello' 메시지 전송");
    let multicast_addr = SocketAddr::new(IpAddr::V4(MULTICAST_ADDR), PORT);
    socket_clone.send_to("hello".as_bytes(), multicast_addr)?;

    // 사용자 입력 처리 루프
    println!("명령어를 입력하세요 (종료하려면 Ctrl+C):");
    println!("  /hello - 멀티캐스트 그룹에 'hello' 메시지 전송");

    let mut input = String::new();
    loop {
        input.clear();
        print!("> ");
        io::stdout().flush()?;

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let command = input.trim();

                match command {
                    "/hello" => {
                        println!("멀티캐스트 그룹에 'hello' 메시지 전송");
                        let multicast_addr = SocketAddr::new(IpAddr::V4(MULTICAST_ADDR), PORT);
                        match socket_clone.send_to("hello".as_bytes(), multicast_addr) {
                            Ok(_) => println!("메시지 전송 완료"),
                            Err(e) => eprintln!("메시지 전송 실패: {e}"),
                        }
                    }
                    "/quit" | "/exit" => {
                        println!("프로그램을 종료합니다.");
                        break;
                    }
                    "" => continue, // 빈 입력 무시
                    _ => {
                        println!("알 수 없는 명령어: {command}");
                        println!("사용 가능한 명령어:");
                        println!("  /hello - 멀티캐스트 그룹에 'hello' 메시지 전송");
                        println!("  /quit  - 프로그램 종료");
                    }
                }
            }
            Err(e) => {
                eprintln!("입력 오류: {e}");
                break;
            }
        }
    }

    Ok(())
}
