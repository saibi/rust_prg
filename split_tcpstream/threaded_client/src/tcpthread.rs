use std::sync::mpsc::{Receiver, Sender};
use std::{
    io::{Read, Write},
    mem,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use log::{debug, error};

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
        // split stream into reader and writer
        let mut buf: Vec<u8> = vec![0; 2048];
        loop {
            if *exit_flag.lock().unwrap() {
                break;
            }
            if let Ok(msg) = rx.try_recv() {
                stream.write_all(msg.as_bytes()).unwrap();
            }
            match stream
                .set_nonblocking(true)
                .and_then(|_| stream.read(&mut buf))
            {
                Ok(0) => {
                    error!("Connection closed");
                    break;
                }
                Ok(n) => {
                    buf.truncate(n);
                    tx.send(String::from_utf8_lossy(&buf).to_string()).unwrap();
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No data available right now, just continue
                }
                Err(e) => {
                    error!("Failed to receive data: {}", e);
                    break;
                }
            }
        }
    }

    pub fn send(&self, msg: String) {
        self.tx.send(msg).unwrap();
    }

    pub fn recv(&self) -> Option<String> {
        self.rx.recv().ok()
    }
}

impl Drop for TcpThread {
    fn drop(&mut self) {
        // Set the exit flag to true
        let mut exit = self.exit_flag.lock().unwrap();
        *exit = true;

        // Wait for the thread to finish
        if let Some(handle) = self.handle.take() {
            if let Err(e) = handle.join() {
                error!("Failed to join thread: {:?}", e);
            }
        }
    }
}
