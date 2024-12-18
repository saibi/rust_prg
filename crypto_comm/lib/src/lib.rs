pub mod logger;
pub mod stdinthread;

// use std::{
//     io::{Read, Write},
//     net::TcpStream,
// };

// struct Connector {
//     stream: TcpStream,
// }

// use bytes::{BufMut, BytesMut};
use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

// header format
// | magic value (1 byte) = 0x42 | version number (1 byte, unsigned) | reserved (2 bytes) | data size (4 bytes) |

const SIMPLE_PACKET_HEADER_SIZE: usize = 8;
const SIMPLE_PACKET_MAGIC_NUMBER: u8 = 0x42;

struct SimplePacketHeader {
    magic: u8,
    version: u8,
    reserved: [u8; 2],
    data_size: u32,
}

impl SimplePacketHeader {
    fn new(data_size: u32) -> Self {
        SimplePacketHeader {
            magic: SIMPLE_PACKET_MAGIC_NUMBER,
            version: 1,
            reserved: [0, 0],
            data_size,
        }
    }

    fn to_bytes(&self) -> [u8; SIMPLE_PACKET_HEADER_SIZE] {
        let mut bytes = [0; SIMPLE_PACKET_HEADER_SIZE];
        bytes[0] = self.magic;
        bytes[1] = self.version;
        bytes[2..4].copy_from_slice(&self.reserved);
        bytes[4..8].copy_from_slice(&self.data_size.to_be_bytes());
        bytes
    }

    fn from_bytes(bytes: [u8; SIMPLE_PACKET_HEADER_SIZE]) -> Option<Self> {
        if bytes[0] != SIMPLE_PACKET_MAGIC_NUMBER {
            return None;
        }
        Some(SimplePacketHeader {
            magic: bytes[0],
            version: bytes[1],
            reserved: [bytes[2], bytes[3]],
            data_size: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
        })
    }
}

pub struct SimplePacketStream {
    inner: TcpStream,
}

impl SimplePacketStream {
    pub fn new(stream: TcpStream) -> Self {
        SimplePacketStream { inner: stream }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut header_buf = [0; SIMPLE_PACKET_HEADER_SIZE];
        let result = self.inner.read_exact(&mut header_buf);
        match result {
            Ok(_) => {}
            Err(err) => {
                if err.kind() == io::ErrorKind::UnexpectedEof {
                    return Ok(0);
                }
                return Err(err);
            }
        }

        let header = SimplePacketHeader::from_bytes(header_buf)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid header format"))?;

        let mut data_buf = vec![0; header.data_size as usize];
        self.inner.read_exact(&mut data_buf)?;

        buf[..header.data_size as usize].copy_from_slice(&data_buf);
        Ok(header.data_size as usize)
    }

    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let header = SimplePacketHeader::new(buf.len() as u32);
        let header_bytes = header.to_bytes();

        self.inner.write_all(&header_bytes)?;
        self.inner.write_all(buf)?;
        Ok(buf.len())
    }
}
