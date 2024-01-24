#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
    V4NUM(u8, u8, u8, u8),
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));
    let localhost = IpAddrKind::V4NUM(127, 0, 0, 1);

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
    println!("localhost: {:?}", localhost);
}

// import struct Ipv4Addr and Ipv6Addr from std::net
// use std::net::{Ipv4Addr, Ipv6Addr};

// enum MyIpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
