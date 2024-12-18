use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let _logger = lib::logger::start("debug", "", true);

    log::debug!("client");

    let stdin = lib::stdinthread::StdinThread::new();
    // test_bytes();

    let stream = TcpStream::connect("127.0.0.1:18181").unwrap();
    let mut stream = lib::SimplePacketStream::new(stream);

    stream.write(b"hello world\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    log::debug!("recv from server: [{}]", String::from_utf8_lossy(&buf));

    log::debug!("/q : quit\n");
    log::debug!("/t : bytes test\n");
    loop {
        if let Some(cmd) = stdin.read_line() {
            println!("cmd from stdin: {}", cmd);
            match cmd.as_str() {
                "/q" => {
                    break;
                }
                "/t" => {
                    test_bytes();
                }
                _ => {
                    stream.write(cmd.as_bytes()).unwrap();
                    let mut buf = [0; 1024];
                    stream.read(&mut buf).unwrap();
                    log::debug!("recv from server: [{}]", String::from_utf8_lossy(&buf));
                }
            }
        }
    }
}

use bytes::{BufMut, BytesMut};

fn test_bytes() {
    let mut buf = BytesMut::with_capacity(128);

    buf.put(&b"hello"[..]);
    log::debug!("{:?}", buf);

    let a = buf.split();
    log::debug!("a = {:?}", a);
    log::debug!("buf = {:?}", buf);

    let mut random_32bytes = rand::random::<[u8; 32]>();
    let random_12bytes = rand::random::<[u8; 12]>();

    random_32bytes[0..2].copy_from_slice([0x42, 0xb].as_ref());

    log::debug!(
        "random_32 {:x?}, {:x} {:x}",
        random_32bytes,
        random_32bytes[0],
        random_32bytes[1]
    );
    log::debug!("random_12 {:x?}", random_12bytes);

    buf.put(&random_32bytes[..]);
    buf.put(&random_12bytes[..]);
    log::debug!("buf len = {}", buf.len());

    buf[4..8].copy_from_slice(&0x18u32.to_be_bytes());
    log::debug!("buf = {:x?}", buf);

    let net_buf = buf.split();
    // network order 는 big endian 이므로 to_be_bytes() 를 사용하여 변환
    // 이미 buf[4..8] 에서 to_be_bytes() 를 사용했으므로 추가 변환 불필요
    log::debug!("network buffer = {:x?}", net_buf);

    let c = u32::from_be_bytes(net_buf[4..8].try_into().unwrap());
    log::debug!("c = {:x}", c);
}
