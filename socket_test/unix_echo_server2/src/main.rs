use lib::logger;
use lib::server::SingleUnixServer;
use lib::stdinthread::StdinThread;

fn main() {
    let _logger = logger::start("debug", "", true);

    let sock_path = "/tmp/echo.sock";

    if let Ok(_) = std::fs::remove_file(sock_path) {
        log::info!("remove old sock file");
    }

    let stdin = StdinThread::new();

    let mut server = SingleUnixServer::new(sock_path);

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

        if let Some(msg) = server.recv() {
            println!("echo : {}", msg);
            server.send(msg);
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn help() {
    log::info!("help");
    log::info!("/q : quit");
}
