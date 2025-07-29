use sha2::{Digest, Sha256};
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::str;
use std::thread;

/// 멀티캐스트 네트워크 설정을 위한 상수들
pub const MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 0, 1);
pub const PORT: u16 = 12344;
pub const BUFFER_SIZE: usize = 1024;

/// 멀티캐스트 그룹에 메시지를 전송하는 함수
///
/// 이 함수는 UDP 소켓을 통해 멀티캐스트 그룹의 모든 멤버에게 메시지를 전송합니다.
/// 전송 성공 시 콘솔에 성공 메시지를 출력하고, 실패 시 오류 메시지를 출력합니다.
///
/// # Arguments
/// * `socket` - 전송에 사용할 UDP 소켓 참조
/// * `message` - 전송할 메시지 문자열
/// * `multicast_addr` - 멀티캐스트 그룹의 소켓 주소
///
/// # Returns
/// * `io::Result<()>` - 전송 성공 시 Ok(()), 실패 시 Err(io::Error)
///
/// # Examples
/// ```
/// use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
/// use lib::{send_multicast_message, MULTICAST_ADDR, PORT};
///
/// let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
/// let multicast_addr = SocketAddr::new(IpAddr::V4(MULTICAST_ADDR), PORT);
/// let result = send_multicast_message(&socket, "hello", multicast_addr);
/// ```
pub fn send_multicast_message(
    socket: &UdpSocket,
    message: &str,
    multicast_addr: SocketAddr,
) -> io::Result<()> {
    match socket.send_to(message.as_bytes(), multicast_addr) {
        Ok(_) => {
            println!("Multicast message '{message}' sent successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to send multicast message: {e}");
            Err(e)
        }
    }
}

/// 로컬 IP 주소를 가져오는 함수
///
/// 시스템의 실제 로컬 IP 주소를 가져옵니다. 실패 시 기본값으로 127.0.0.1을 반환합니다.
/// 이 함수는 멀티캐스트 메시지 필터링에 사용됩니다.
///
/// # Returns
/// * `IpAddr` - 로컬 IP 주소 또는 기본값 127.0.0.1
///
/// # Examples
/// ```
/// use lib::get_local_ip_address;
/// let local_ip = get_local_ip_address();
/// println!("로컬 IP: {}", local_ip);
/// ```
pub fn get_local_ip_address() -> IpAddr {
    match local_ip_address::local_ip() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Failed to get local IP address: {e}");
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)) // 기본값으로 로컬호스트 사용
        }
    }
}

/// 멀티캐스트 수신 스레드를 시작하는 함수
///
/// 별도 스레드에서 멀티캐스트 메시지를 수신하고 처리합니다.
/// 수신된 메시지에 따라 적절한 응답을 처리합니다.
///
/// # Arguments
/// * `socket` - 수신에 사용할 UDP 소켓 (소유권 이동)
/// * `local_addr` - 로컬 주소 (자신이 보낸 메시지 필터링용)
///
/// # Returns
/// * `thread::JoinHandle<()>` - 수신 스레드의 핸들
///
/// # Examples
/// ```
/// use std::net::UdpSocket;
/// use lib::start_multicast_receiver;
///
/// let socket = UdpSocket::bind("0.0.0.0:12344").unwrap();
/// let local_addr = socket.local_addr().unwrap();
/// let receiver_handle = start_multicast_receiver(socket, local_addr);
/// ```
pub fn start_multicast_receiver(
    socket: UdpSocket,
    local_addr: SocketAddr,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut buffer = [0u8; BUFFER_SIZE];
        loop {
            match socket.recv_from(&mut buffer) {
                Ok((size, src)) => {
                    // 자신이 보낸 메시지는 무시 (IP 주소로 필터링)
                    if src.ip() == local_addr.ip() {
                        continue;
                    }

                    let message = str::from_utf8(&buffer[..size]).unwrap_or("Invalid UTF-8");
                    println!("Received: {message} (from: {src})");

                    // "hello" 메시지를 받으면 "ok" 응답
                    if message == "hello" {
                        println!("Received 'hello' message, sending 'ok' response");
                        let response = "ok";
                        // 멀티캐스트 그룹에 응답
                        if let Err(e) = socket.send_to(response.as_bytes(), src) {
                            eprintln!("Failed to send response: {e}");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Receive error: {e}");
                }
            }
        }
    })
}

/// 사용자 입력을 처리하는 함수
///
/// 표준 입력에서 명령어를 읽고 처리합니다.
/// 지원하는 명령어: /hello, /quit, /exit
///
/// # Arguments
/// * `socket_clone` - 메시지 전송에 사용할 UDP 소켓 참조
///
/// # Returns
/// * `io::Result<()>` - 처리 성공 시 Ok(()), 실패 시 Err
///
/// # Examples
/// ```
/// use std::net::UdpSocket;
/// use lib::handle_user_input;
///
/// let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
/// let socket_clone = socket.try_clone().unwrap();
/// let result = handle_user_input(&socket_clone);
/// ```
pub fn handle_user_input(socket_clone: &UdpSocket) -> io::Result<()> {
    println!("Enter commands (press Ctrl+C to exit):");
    println!("  /hello - Send 'hello' message to multicast group");
    println!("  /quit  - Exit program");

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
                        let multicast_addr = SocketAddr::new(IpAddr::V4(MULTICAST_ADDR), PORT);
                        if let Err(e) =
                            send_multicast_message(socket_clone, "hello", multicast_addr)
                        {
                            eprintln!("Failed to send message: {e}");
                        }
                    }
                    "/quit" | "/exit" => {
                        println!("Exiting program.");
                        break;
                    }
                    "" => continue, // 빈 입력 무시
                    _ => {
                        println!("Unknown command: {command}");
                        println!("Available commands:");
                        println!("  /hello - Send 'hello' message to multicast group");
                        println!("  /quit  - Exit program");
                    }
                }
            }
            Err(e) => {
                eprintln!("Input error: {e}");
                break;
            }
        }
    }

    Ok(())
}

/// 멀티캐스트 소켓을 초기화하는 함수
///
/// UDP 소켓을 생성하고 멀티캐스트 그룹에 가입시킵니다.
/// 소켓을 복제하여 송신용과 수신용으로 사용할 수 있도록 합니다.
///
/// # Returns
/// * `io::Result<(UdpSocket, UdpSocket)>` - (원본 소켓, 복제된 소켓) 또는 오류
///
/// # Examples
/// ```
/// use lib::initialize_multicast_socket;
///
/// match initialize_multicast_socket() {
///     Ok((socket, socket_clone)) => {
///         // 소켓 사용
///     }
///     Err(e) => eprintln!("소켓 초기화 실패: {}", e),
/// }
/// ```
pub fn initialize_multicast_socket() -> io::Result<(UdpSocket, UdpSocket)> {
    // 소켓 생성 및 바인딩 (멀티캐스트 수신을 위해 특정 포트에 바인딩)
    let socket = UdpSocket::bind(format!("0.0.0.0:{PORT}"))?;

    // 멀티캐스트 그룹 가입
    socket.join_multicast_v4(&MULTICAST_ADDR, &Ipv4Addr::new(0, 0, 0, 0))?;

    // 소켓을 복제하여 송신용과 수신용으로 사용
    let socket_clone = socket.try_clone()?;

    Ok((socket, socket_clone))
}

/// HMAC을 사용하여 멀티캐스트 그룹에 메시지를 전송하는 함수
///
/// 이 함수는 메시지에 HMAC 서명을 추가하여 전송합니다.
/// 메시지와 HMAC 서명을 JSON 형태로 직렬화하여 전송합니다.
///
/// # Arguments
/// * `socket` - 전송에 사용할 UDP 소켓 참조
/// * `message` - 전송할 메시지 문자열
/// * `multicast_addr` - 멀티캐스트 그룹의 소켓 주소
/// * `secret_key` - HMAC 서명에 사용할 비밀키
///
/// # Returns
/// * `io::Result<()>` - 전송 성공 시 Ok(()), 실패 시 Err(io::Error)
///
/// # Examples
/// ```
/// use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
/// use lib::{send_multicast_message_with_hmac, MULTICAST_ADDR, PORT};
///
/// let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
/// let multicast_addr = SocketAddr::new(IpAddr::V4(MULTICAST_ADDR), PORT);
/// let secret_key = b"my_secret_key";
/// let result = send_multicast_message_with_hmac(&socket, "hello", multicast_addr, secret_key);
/// ```
pub fn send_multicast_message_with_hmac(
    socket: &UdpSocket,
    message: &str,
    multicast_addr: SocketAddr,
    secret_key: &[u8],
) -> io::Result<()> {
    // HMAC 서명 생성 (키 + 메시지의 SHA256 해시)
    let mut hasher = Sha256::new();
    hasher.update(secret_key);
    hasher.update(message.as_bytes());
    let signature = hasher.finalize();

    // JSON 형태로 메시지와 서명을 직렬화
    let signature_hex = signature
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();

    let json_message = format!(r#"{{"message":"{message}","signature":"{signature_hex}"}}"#);

    match socket.send_to(json_message.as_bytes(), multicast_addr) {
        Ok(_) => {
            println!("HMAC signed multicast message '{message}' sent successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to send HMAC signed multicast message: {e}");
            Err(e)
        }
    }
}

/// HMAC 서명을 검증하는 함수
///
/// 수신된 JSON 메시지에서 HMAC 서명을 검증합니다.
/// 서명이 유효하면 메시지를 반환하고, 그렇지 않으면 None을 반환합니다.
///
/// # Arguments
/// * `json_data` - JSON 형태의 메시지 데이터
/// * `secret_key` - HMAC 검증에 사용할 비밀키
///
/// # Returns
/// * `Option<String>` - 검증 성공 시 메시지, 실패 시 None
///
/// # Examples
/// ```
/// use lib::verify_hmac_message;
///
/// let json_data = r#"{"message":"hello","signature":"abc123"}"#;
/// let secret_key = b"my_secret_key";
/// let message = verify_hmac_message(json_data, secret_key);
/// ```
pub fn verify_hmac_message(json_data: &str, secret_key: &[u8]) -> Option<String> {
    // JSON 파싱 (간단한 구현)
    if !json_data.contains(r#""message":"#) || !json_data.contains(r#""signature":"#) {
        return None;
    }

    // 메시지 추출
    let message_start = json_data.find(r#""message":"#).unwrap() + 11;
    let message_end = json_data[message_start..].find('"').unwrap() + message_start;
    let message = &json_data[message_start..message_end];

    // 서명 추출 (16진수 문자열 형식)
    let signature_start = json_data.find(r#""signature":"#).unwrap() + 13;
    let signature_end = json_data[signature_start..].find('"').unwrap() + signature_start;
    let signature_hex = &json_data[signature_start..signature_end];

    // 16진수 문자열을 바이트로 변환
    let signature_bytes: Vec<u8> = signature_hex
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| {
            let hex_str = chunk.iter().collect::<String>();
            u8::from_str_radix(&hex_str, 16).unwrap_or(0)
        })
        .collect();

    // HMAC 검증 (키 + 메시지의 SHA256 해시)
    let mut hasher = Sha256::new();
    hasher.update(secret_key);
    hasher.update(message.as_bytes());
    let expected_signature = hasher.finalize();

    // 서명 비교
    if signature_bytes != expected_signature.as_slice() {
        return None;
    }

    Some(message.to_string())
}

/// HMAC을 사용하는 멀티캐스트 수신 스레드를 시작하는 함수
///
/// 별도 스레드에서 HMAC 서명이 포함된 멀티캐스트 메시지를 수신하고 처리합니다.
/// 서명이 유효한 메시지만 처리하고, "hello" 메시지에 대해 "ok" 응답을 보냅니다.
///
/// # Arguments
/// * `socket` - 수신에 사용할 UDP 소켓 (소유권 이동)
/// * `local_addr` - 로컬 주소 (자신이 보낸 메시지 필터링용)
/// * `secret_key` - HMAC 검증에 사용할 비밀키
///
/// # Returns
/// * `thread::JoinHandle<()>` - 수신 스레드의 핸들
///
/// # Examples
/// ```
/// use std::net::UdpSocket;
/// use lib::start_multicast_receiver_with_hmac;
///
/// let socket = UdpSocket::bind("0.0.0.0:12344").unwrap();
/// let local_addr = socket.local_addr().unwrap();
/// let secret_key = b"my_secret_key";
/// let receiver_handle = start_multicast_receiver_with_hmac(socket, local_addr, secret_key);
/// ```
pub fn start_multicast_receiver_with_hmac(
    socket: UdpSocket,
    local_addr: SocketAddr,
    secret_key: &'static [u8],
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut buffer = [0u8; BUFFER_SIZE];
        loop {
            match socket.recv_from(&mut buffer) {
                Ok((size, src)) => {
                    // 자신이 보낸 메시지는 무시 (IP 주소로 필터링)
                    if src.ip() == local_addr.ip() {
                        continue;
                    }

                    let json_data = str::from_utf8(&buffer[..size]).unwrap_or("Invalid UTF-8");
                    println!("Received HMAC message: {json_data} (from: {src})");

                    // HMAC 검증
                    if let Some(message) = verify_hmac_message(json_data, secret_key) {
                        println!("HMAC verified message: {message} (from: {src})");

                        // "hello" 메시지를 받으면 "ok" 응답
                        if message == "hello" {
                            println!("Received verified 'hello' message, sending 'ok' response");
                            let response = "ok";

                            // 멀티캐스트 그룹에 응답
                            if let Err(e) = socket.send_to(response.as_bytes(), src) {
                                eprintln!("Failed to send response: {e}");
                            }
                        }
                    } else {
                        println!("HMAC verification failed for message from {src}");
                    }
                }
                Err(e) => {
                    eprintln!("Receive error: {e}");
                }
            }
        }
    })
}

/// HMAC을 사용하는 사용자 입력 처리 함수
///
/// 표준 입력에서 명령어를 읽고 HMAC 서명과 함께 처리합니다.
/// 지원하는 명령어: /hello, /quit, /exit
///
/// # Arguments
/// * `socket_clone` - 메시지 전송에 사용할 UDP 소켓 참조
/// * `secret_key` - HMAC 서명에 사용할 비밀키
///
/// # Returns
/// * `io::Result<()>` - 처리 성공 시 Ok(()), 실패 시 Err
///
/// # Examples
/// ```
/// use std::net::UdpSocket;
/// use lib::handle_user_input_with_hmac;
///
/// let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
/// let socket_clone = socket.try_clone().unwrap();
/// let secret_key = b"my_secret_key";
/// let result = handle_user_input_with_hmac(&socket_clone, secret_key);
/// ```
pub fn handle_user_input_with_hmac(socket_clone: &UdpSocket, secret_key: &[u8]) -> io::Result<()> {
    println!("Enter commands (press Ctrl+C to exit):");
    println!("  /hello - Send 'hello' message with HMAC to multicast group");
    println!("  /quit  - Exit program");

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
                        let multicast_addr = SocketAddr::new(IpAddr::V4(MULTICAST_ADDR), PORT);
                        if let Err(e) = send_multicast_message_with_hmac(
                            socket_clone,
                            "hello",
                            multicast_addr,
                            secret_key,
                        ) {
                            eprintln!("Failed to send HMAC message: {e}");
                        }
                    }
                    "/quit" | "/exit" => {
                        println!("Exiting program.");
                        break;
                    }
                    "" => continue, // 빈 입력 무시
                    _ => {
                        println!("Unknown command: {command}");
                        println!("Available commands:");
                        println!("  /hello - Send 'hello' message with HMAC to multicast group");
                        println!("  /quit  - Exit program");
                    }
                }
            }
            Err(e) => {
                eprintln!("Input error: {e}");
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_ip_address() {
        let ip = get_local_ip_address();
        // IP 주소가 유효한지 확인 (127.0.0.1 또는 실제 IP)
        assert!(ip.is_ipv4() || ip.is_ipv6());
    }

    #[test]
    fn test_multicast_constants() {
        // 멀티캐스트 주소가 유효한지 확인
        assert!(MULTICAST_ADDR.is_multicast());
        assert!(PORT > 0);
        assert!(BUFFER_SIZE > 0);
    }

    #[test]
    fn test_hmac_message_verification() {
        let secret_key = b"test_secret_key";
        let message = "hello";

        // HMAC 서명 생성 (키 + 메시지의 SHA256 해시)
        let mut hasher = Sha256::new();
        hasher.update(secret_key);
        hasher.update(message.as_bytes());
        let signature = hasher.finalize();

        // JSON 형태로 직렬화 (실제 함수와 동일한 형식 사용)
        let json_data = format!(
            r#"{{"message":"{message}","signature":"{}"}}"#,
            signature
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>()
        );

        // 검증 테스트
        let verified_message = verify_hmac_message(&json_data, secret_key);
        assert_eq!(verified_message, Some(message.to_string()));

        // 잘못된 키로 검증 시도
        let wrong_key = b"wrong_key";
        let verified_with_wrong_key = verify_hmac_message(&json_data, wrong_key);
        assert_eq!(verified_with_wrong_key, None);
    }
}
