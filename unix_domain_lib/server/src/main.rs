use log;
use std::thread;
use unix_domain_lib::{create_unix_domain_server, logger};

fn main() -> std::io::Result<()> {
    let _logger = logger::start("debug", "/tmp/server", true);

    let sock_path = "/tmp/.rdecho.sock";

    // 기존 소켓 파일이 있다면 제거
    if std::path::Path::new(sock_path).exists() {
        std::fs::remove_file(sock_path)?;
    }

    log::debug!("main loop start");

    let server = create_unix_domain_server(sock_path)?;

    loop {
        if let Some(recv_msg) = server.recv() {
            println!("main: received message: {}", recv_msg);
            server.send(recv_msg);
        }
        // if let Some(msg) = server.get_received_message() {
        //     println!("main: received message: {}", msg);
        // }
        thread::sleep(std::time::Duration::from_millis(100));
    }
    //log::debug!("main loop end.");
}
