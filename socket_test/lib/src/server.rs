use std::{net::TcpListener, os::unix::net::UnixListener};

use crate::streamthread::StreamThread;

pub struct SingleUnixServer {
    listener: UnixListener,
    stream_thread: Option<StreamThread>,
}

impl SingleUnixServer {
    pub fn new(sock_path: &str) -> Self {
        let listener = UnixListener::bind(sock_path).unwrap();
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

pub struct SingleTcpServer {
    listener: std::net::TcpListener,
    stream_thread: Option<StreamThread>,
}

impl SingleTcpServer {
    pub fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).unwrap();
        listener.set_nonblocking(true).unwrap();
        Self {
            listener,
            stream_thread: None,
        }
    }

    fn check_incoming(&mut self) {
        if self.stream_thread.is_none() {
            if let Ok((stream, _)) = self.listener.accept() {
                log::info!("accept new tcp stream");
                self.stream_thread = Some(StreamThread::new(stream));
            }
        }
    }

    pub fn recv(&mut self) -> Option<String> {
        match self.stream_thread {
            Some(ref worker) => {
                if worker.is_finished() {
                    self.stream_thread = None;
                    log::info!("tcp stream thread is finished");
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
                    log::info!("tcp stream thread is finished");
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
