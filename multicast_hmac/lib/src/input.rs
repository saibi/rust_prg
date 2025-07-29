use std::{
    io::{self, Write},
    net::{IpAddr, SocketAddr, UdpSocket},
};

use crate::{
    MULTICAST_ADDR, PORT,
    udpm::{send_multicast_message_with_hmac, send_udp_msg},
};

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
                        if let Err(e) = send_udp_msg(socket_clone, multicast_addr, "hello") {
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
