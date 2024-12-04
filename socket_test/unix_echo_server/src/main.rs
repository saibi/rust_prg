use std::os::unix::net::UnixListener;

use lib::logger;
use lib::stdinthread::StdinThread;
use lib::streamthread::StreamThread;

fn main() {
    let _logger = logger::start("debug", "", true);

    let sock_path = "/tmp/echo.sock";

    if let Ok(_) = std::fs::remove_file(sock_path) {
        log::info!("remove old sock file");
    }

    let stdin = StdinThread::new();

    let listener = UnixListener::bind(sock_path).unwrap();
    listener.set_nonblocking(true).unwrap();

    let mut stream_thread: Option<StreamThread> = None;
    // let stream_thread = StreamThread::create_unix_domain_server("/tmp/echo.sock").unwrap();

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

        if let Some(worker) = &mut stream_thread {
            if let Some(msg) = worker.recv() {
                println!("echo : {}", msg);
                worker.send(msg);
            }
            if worker.is_finished() {
                log::info!("stream thread is finished");
                stream_thread = None;
            }
        } else {
            if let Ok((stream, _)) = listener.accept() {
                log::info!("accept new stream");
                stream_thread = Some(StreamThread::new(stream));
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn help() {
    log::info!("help");
    log::info!("/q : quit");
}
