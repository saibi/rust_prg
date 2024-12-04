mod cmdargs;

use clap::Parser;
use std::net::TcpStream;

use cmdargs::Args;
use lib::{stdinthread::StdinThread, streamthread::StreamThread};

fn main() {
    let _logger = lib::logger::start("debug", "", true);
    let args = Args::parse();
    let stdin = StdinThread::new();
    println!("echo client start");

    let stream = TcpStream::connect(args.addr).unwrap();
    let mut stream_thread = StreamThread::new(stream);

    println!("/q : quit\n");
    loop {
        if let Some(cmd) = stdin.read_line() {
            println!("cmd from stdin: {}", cmd);
            match cmd.as_str() {
                "/q" => {
                    stream_thread.stop();
                    break;
                }
                _ => {
                    let msg = cmd + "\n";
                    stream_thread.send(msg);
                }
            }
        }
        if let Some(msg) = stream_thread.recv() {
            println!("recv msg from server : {}", msg);
        }
    }
}
