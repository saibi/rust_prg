use clap::Parser;

/// Led control
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// listen port, e.g. 58991
    #[arg(short, long, default_value_t = 58991)]
    port: u16,

    /// listen on any address
    #[arg(short, long, default_value_t = false)]
    any: bool,
}

fn get_bind_addr(args: &Args) -> String {
    const BIND_ADDR_ANY: &str = "0.0.0.0";
    const BIND_ADDR_LOCAL: &str = "127.0.0.1";

    if args.any {
        format!("{}:{}", BIND_ADDR_ANY, args.port)
    } else {
        format!("{}:{}", BIND_ADDR_LOCAL, args.port)
    }
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let listener = TcpListener::bind(&get_bind_addr(&args)).unwrap();

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
