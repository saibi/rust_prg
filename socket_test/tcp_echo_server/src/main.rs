use lib::logger;
use lib::server::SingleTcpServer;
use lib::stdinthread::StdinThread;

fn main() {
    let _logger = logger::start("debug", "", true);
    let stdin = StdinThread::new();
    let mut server = SingleTcpServer::new("127.0.0.1:12345");

    log::info!("Start main loop - tcp echo server :12345");
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
