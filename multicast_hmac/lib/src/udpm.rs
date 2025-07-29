use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::thread;
use std::{io, str};

use crate::hmac_msg::{create_hmac_msg, verify_hmac_message};

pub const BUFFER_SIZE: usize = 1024;

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
pub fn init_multicast_socket(
    multicast_addr: &Ipv4Addr,
    port: u16,
) -> io::Result<(UdpSocket, UdpSocket)> {
    // 소켓 생성 및 바인딩 (멀티캐스트 수신을 위해 특정 포트에 바인딩)
    let socket = UdpSocket::bind(format!("0.0.0.0:{port}"))?;

    // 멀티캐스트 그룹 가입
    socket.join_multicast_v4(multicast_addr, &Ipv4Addr::new(0, 0, 0, 0))?;

    // 소켓을 복제하여 송신용과 수신용으로 사용
    let socket_clone = socket.try_clone()?;

    Ok((socket, socket_clone))
}

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
pub fn send_udp_msg(socket: &UdpSocket, addr: SocketAddr, message: &str) -> io::Result<()> {
    match socket.send_to(message.as_bytes(), addr) {
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

/// HMAC을 사용하여 멀티캐스트 그룹에 메시지를 전송하는 범용 함수
///
/// 이 함수는 메시지에 HMAC 서명을 추가하고, 기존 send_multicast_message를 활용해 전송합니다.
///
/// # Arguments
/// * `socket` - 전송에 사용할 UDP 소켓 참조
/// * `message` - 전송할 메시지 문자열
/// * `multicast_addr` - 멀티캐스트 그룹의 소켓 주소
/// * `secret_key` - HMAC 서명에 사용할 비밀키
///
/// # Returns
/// * `io::Result<()>` - 전송 성공 시 Ok(()), 실패 시 Err(io::Error)
pub fn send_multicast_message_with_hmac(
    socket: &UdpSocket,
    message: &str,
    multicast_addr: SocketAddr,
    secret_key: &[u8],
) -> io::Result<()> {
    let json_message = create_hmac_msg(message, secret_key);
    send_udp_msg(socket, multicast_addr, &json_message)
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
                        println!("Received message from myself: {src}");
                        // continue;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_ip_address() {
        let ip = get_local_ip_address();
        // IP 주소가 유효한지 확인 (127.0.0.1 또는 실제 IP)
        assert!(ip.is_ipv4() || ip.is_ipv6());
    }
}
