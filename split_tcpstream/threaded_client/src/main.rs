mod cmdargs;
mod logger;
mod stdinthread;
mod tcpthread;

use std::{fs::File, io::Read};

use clap::Parser;
use cmdargs::Args;
use log::{debug, info};
use stdinthread::StdinThread;

fn main() {
    let args = Args::parse();
    let ver = env!("CARGO_PKG_VERSION");
    logger::start(
        args.log_level.as_str(),
        args.log_file.to_str().unwrap(),
        args.log_stderr,
    );

    info!("client test {}", ver);
    info!("args: {:?}", args);

    let stdin = StdinThread::new();
    let tcp = tcpthread::connect_to_server(&args.addr).unwrap();
    let test_bin_path = args.test_file.as_str();

    info!("/q to exit");

    debug!("start main loop");
    loop {
        if let Some(input) = stdin.read_line() {
            debug!("input: [{}]", input);
            if input == "/q" {
                debug!("exit");
                break;
            } else if input == "/fs" {
                let cmd = format!(
                    "nt f;nt fs {} {}\n",
                    get_file_name(test_bin_path),
                    get_file_size(test_bin_path)
                );
                debug!("file send test: send {}", cmd);
                tcp.send(cmd);
            } else {
                tcp.send(input);
            }
        }
        if let Some(msg) = tcp.recv() {
            debug!("recv: [{}]", msg);
            if msg.contains("nt fs") {
                let buf = load_file(test_bin_path).unwrap();
                let checksum = calculate_checksum(&buf);
                debug!("file send test: checksum {}", checksum);

                tcpthread::send_bin_to_server("10.82.79.2:8273", buf);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    debug!("end main loop");
}

fn load_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn calculate_checksum(data: &Vec<u8>) -> u8 {
    data.iter().fold(0, |acc, &byte| acc ^ byte)
}

fn get_file_size(path: &str) -> u64 {
    if let Ok(metadata) = std::fs::metadata(path) {
        metadata.len()
    } else {
        return 0;
    }
}

fn get_file_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
