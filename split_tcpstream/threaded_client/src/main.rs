mod cmdargs;
mod logger;
mod stdinthread;
mod tcpthread;

use clap::Parser;
use cmdargs::Args;
use log::info;
use stdinthread::StdinThread;

fn main() {
    let args = Args::parse();
    let ver = env!("CARGO_PKG_VERSION");
    logger::start(
        args.log_level.as_str(),
        args.log_file.to_str().unwrap(),
        args.log_stderr,
    );

    info!("client test {}\nargs: {:?}", ver, args);

    let stdin = StdinThread::new();

    loop {
        if let Some(input) = stdin.read_line() {
            println!("input: {}", input);
            if input.trim() == "/q" {
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
