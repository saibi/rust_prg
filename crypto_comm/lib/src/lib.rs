pub mod logger;

use std::{
    io::{Read, Write},
    net::TcpStream,
};

struct Connector {
    stream: TcpStream,
}
