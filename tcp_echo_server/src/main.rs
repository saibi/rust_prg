use clap::Parser;

/// Led control
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// listen port, e.g. 58991
    #[arg(short, long, default_value_t = 58991)]
    port: u16,

    #[arg(short, long, default_value_t = true)]
    local: bool,
}

fn main() {
    const BIND_ADDR_ANY: &str = "0.0.0.0";
    const BIND_ADDR_LOCAL: &str = "127.0.0.1";

    let args = Args::parse();

    let bind_addr = if args.local {
        format!("{}:{}", BIND_ADDR_LOCAL, args.port)
    } else {
        format!("{}:{}", BIND_ADDR_ANY, args.port)
    };

    println!("{:?}", args);
    let listener = TcpListener::bind(&bind_addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}

use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // read 20 bytes at a time from stream echoing back to stream
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}
