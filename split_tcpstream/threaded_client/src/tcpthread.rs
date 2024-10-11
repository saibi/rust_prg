use std::sync::mpsc::{Receiver, Sender};
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

pub fn connect_to_server(addr: &str) -> Result<TcpThread, std::io::Error> {
    let stream = TcpStream::connect(addr)?;

    let (tx, send_rx) = std::sync::mpsc::channel();
    let (recv_tx, rx) = std::sync::mpsc::channel();

    let exit_flag = Arc::new(Mutex::new(false));

    let exit_flag_clone = exit_flag.clone();
    let handle = std::thread::spawn(move || {
        TcpThread::stream_loop(stream, send_rx, recv_tx, exit_flag_clone);
    });
    Ok(TcpThread {
        tx,
        rx,
        exit_flag,
        handle,
    })
}

pub struct TcpThread {
    tx: Sender<String>,
    rx: Receiver<String>,
    exit_flag: Arc<Mutex<bool>>,
    handle: std::thread::JoinHandle<()>,
}

impl TcpThread {
    fn stream_loop(
        mut stream: TcpStream,
        rx: Receiver<String>,
        tx: Sender<String>,
        exit_flag: Arc<Mutex<bool>>,
    ) {
        // split stream into reader and writer
        loop {
            if *exit_flag.lock().unwrap() {
                break;
            }
            if let Ok(msg) = rx.try_recv() {
                stream.write_all(msg.as_bytes()).unwrap();
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
