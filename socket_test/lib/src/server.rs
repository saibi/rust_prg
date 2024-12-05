use std::{net::TcpListener, os::unix::net::UnixListener};

use crate::streamthread::{NonblockingStream, StreamThread};

use std::io;

pub type SingleUnixServer = SingleServer<UnixListener>;
pub type SingleTcpServer = SingleServer<TcpListener>;

pub trait StreamListener {
    type Stream: NonblockingStream;

    fn bind(addr: &str) -> io::Result<Self>
    where
        Self: Sized;
    fn accept(&self) -> io::Result<(Self::Stream, std::net::SocketAddr)>;
    fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()>;
}

impl StreamListener for UnixListener {
    type Stream = std::os::unix::net::UnixStream;

    fn bind(addr: &str) -> io::Result<Self> {
        UnixListener::bind(addr)
    }

    fn accept(&self) -> io::Result<(Self::Stream, std::net::SocketAddr)> {
        self.accept().map(|(stream, _)| {
            (
                stream,
                std::net::SocketAddr::new(
                    std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
                    0,
                ),
            )
        })
    }

    fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.set_nonblocking(nonblocking)
    }
}

impl StreamListener for TcpListener {
    type Stream = std::net::TcpStream;

    fn bind(addr: &str) -> io::Result<Self> {
        TcpListener::bind(addr)
    }

    fn accept(&self) -> io::Result<(Self::Stream, std::net::SocketAddr)> {
        self.accept()
    }

    fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.set_nonblocking(nonblocking)
    }
}

pub struct SingleServer<L: StreamListener + 'static> {
    listener: L,
    stream_thread: Option<StreamThread>,
}

impl<L: StreamListener> SingleServer<L> {
    pub fn new(addr: &str) -> Self {
        let listener = L::bind(addr).unwrap();
        listener.set_nonblocking(true).unwrap();
        Self {
            listener,
            stream_thread: None,
        }
    }

    fn check_incoming(&mut self) {
        if self.stream_thread.is_none() {
            if let Ok((stream, _)) = self.listener.accept() {
                log::info!("accept new stream");
                self.stream_thread = Some(StreamThread::new(stream));
            }
        }
    }

    pub fn recv(&mut self) -> Option<String> {
        match self.stream_thread {
            Some(ref worker) => {
                if worker.is_finished() {
                    self.stream_thread = None;
                    log::info!("stream thread is finished");
                    None
                } else {
                    worker.recv()
                }
            }
            None => {
                self.check_incoming();
                None
            }
        }
    }

    pub fn send(&mut self, msg: String) {
        match self.stream_thread {
            Some(ref worker) => {
                if worker.is_finished() {
                    self.stream_thread = None;
                    log::info!("stream thread is finished");
                } else {
                    worker.send(msg);
                }
            }
            None => {
                self.check_incoming();
            }
        }
    }
}
