use std::sync::mpsc::{Receiver, Sender};
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use log::{debug, error, info};

pub fn connect_to_server(addr: &str) -> Result<TcpThread, std::io::Error> {
    let stream = TcpStream::connect(addr)?;

    let (tx, send_rx) = std::sync::mpsc::channel();
    let (recv_tx, rx) = std::sync::mpsc::channel();

    let exit_flag = Arc::new(Mutex::new(false));

    let exit_flag_clone = exit_flag.clone();
    let handle = std::thread::spawn(move || {
        debug!("stream_loop start");
        TcpThread::stream_loop(stream, send_rx, recv_tx, exit_flag_clone);
        debug!("stream_loop end");
    });
    Ok(TcpThread {
        tx,
        rx,
        exit_flag,
        handle: Some(handle),
    })
}

fn connect_n_send(addr: &str, msg: Vec<u8>) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(&msg)?;
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;
    Ok(())
}

pub fn send_bin_to_server(
    addr: &str,
    msg: Vec<u8>,
) -> std::thread::JoinHandle<Result<(), std::io::Error>> {
    let addr = addr.to_string();
    std::thread::spawn(move || {
        debug!("send_bin_to_server thread start {} ", addr);
        let ret = connect_n_send(addr.as_str(), msg);
        debug!("send_bin_to_serve thread end");
        ret
    })
}

pub struct TcpThread {
    tx: Sender<String>,
    rx: Receiver<String>,
    exit_flag: Arc<Mutex<bool>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl TcpThread {
    fn stream_loop(
        mut stream: TcpStream,
        rx: Receiver<String>,
        tx: Sender<String>,
        exit_flag: Arc<Mutex<bool>>,
    ) {
        let mut buf = [0; 2048];
        let mut incomplete_msg = String::new();
        loop {
            if *exit_flag.lock().unwrap() {
                debug!("exit flag is true");
                break;
            }
            if let Ok(mut msg) = rx.try_recv() {
                if !msg.ends_with('\n') {
                    msg.push('\n');
                }
                stream.write_all(msg.as_bytes()).unwrap();
            }
            match stream
                .set_nonblocking(true)
                .and_then(|_| stream.read(&mut buf))
            {
                Ok(0) => {
                    info!("Connection closed");
                    break;
                }
                Ok(n) => {
                    let data = String::from_utf8_lossy(&buf[..n]);
                    incomplete_msg.push_str(&data);

                    while let Some(newline_idx) = incomplete_msg.find('\n') {
                        let msg = incomplete_msg[..newline_idx].to_string();
                        tx.send(msg).unwrap();
                        incomplete_msg = incomplete_msg[newline_idx + 1..].to_string();
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No data available right now, just continue
                }
                Err(e) => {
                    error!("Failed to receive data: {}", e);
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    pub fn send(&self, msg: String) {
        debug!("send: {}", msg);
        self.tx.send(msg).unwrap();
    }

    pub fn recv(&self) -> Option<String> {
        self.rx.try_recv().ok()
    }

    pub fn stop(&mut self) {
        debug!("stop");
        if let Some(handle) = self.handle.take() {
            *self.exit_flag.lock().unwrap() = true;
            debug!("wait for the thread to finish");

            if let Err(e) = handle.join() {
                error!("Failed to join thread: {:?}", e);
            }
            debug!("thread finished");
            self.handle = None;
        }
    }
}

impl Drop for TcpThread {
    fn drop(&mut self) {
        self.stop();
    }
}
