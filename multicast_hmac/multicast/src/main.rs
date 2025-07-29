use std::io;
use std::net::{IpAddr, SocketAddr};
use std::thread;
use std::time::Duration;

use lib::{
    MULTICAST_ADDR, PORT, get_local_ip_address, handle_user_input, initialize_multicast_socket,
    send_multicast_message, start_multicast_receiver,
};

/// 멀티캐스트 클라이언트 메인 함수
///
/// 이 함수는 멀티캐스트 네트워크에 참여하여 메시지를 송수신하는 클라이언트를 실행합니다.
/// 프로그램 시작 시 "hello" 메시지를 전송하고, 사용자 입력을 처리합니다.
///
/// # Returns
/// * `io::Result<()>` - 프로그램 실행 성공 시 Ok(()), 실패 시 Err
fn main() -> io::Result<()> {
    // 멀티캐스트 소켓 초기화
    let (socket, socket_clone) = initialize_multicast_socket()?;

    // 자신의 실제 IP 주소 가져오기 (127.0.0.1이나 0.0.0.0이 아닌)
    let my_ip = get_local_ip_address();
    println!("My IP address: {my_ip}");

    // 자신의 IP 주소 가져오기 (메시지 필터링용)
    let local_addr = socket.local_addr()?;
    println!("Local address: {local_addr}");

    // 수신 스레드 시작
    let _receiver_thread = start_multicast_receiver(socket, local_addr);

    // 잠시 대기 후 "hello" 메시지 전송 (수신 준비 시간 확보)
    thread::sleep(Duration::from_millis(500));
    let multicast_addr = SocketAddr::new(IpAddr::V4(MULTICAST_ADDR), PORT);
    send_multicast_message(&socket_clone, "hello", multicast_addr)?;

    // 사용자 입력 처리
    handle_user_input(&socket_clone)?;

    Ok(())
}
