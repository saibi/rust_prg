use std::os::unix::net::UnixListener;

use lib::logger;
use lib::stdinthread::StdinThread;
use lib::streamthread::StreamThread;

fn main() {
    let _logger = logger::start("debug", "", true);

    let stream_thread = StreamThread::create_unix_domain_server("/tmp/echo.sock").unwrap();

    let stdin = StdinThread::new();

    log::info!("Start main loop");
    help();
    loop {
        if let Some(cmd) = stdin.read_line() {
            println!("cmd from stdin: {}", cmd);

            match cmd.as_str() {
                "/q" => {
                    break;
                }
                _ => {
                    log::info!("unknown command: {}", cmd);
                    help();
                }
            }
        }
        if let Some(msg) = stream_thread.recv() {
            println!("echo : {}", msg);
            stream_thread.send(msg);
        }
    }
}

fn help() {
    log::info!("help");
    log::info!("/q : quit");
}
