use std::{
    io::{Read, Write},
    os::unix::net::UnixListener,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};

pub trait NonblockingStream: Read + Write + Send {
    fn set_nonblocking(&self, nonblocking: bool) -> std::io::Result<()>;
}

impl NonblockingStream for std::os::unix::net::UnixStream {
    fn set_nonblocking(&self, nonblocking: bool) -> std::io::Result<()> {
        self.set_nonblocking(nonblocking)
    }
}

impl NonblockingStream for std::net::TcpStream {
    fn set_nonblocking(&self, nonblocking: bool) -> std::io::Result<()> {
        self.set_nonblocking(nonblocking)
    }
}

pub struct StreamThread {
    tx: Sender<String>,
    rx: Receiver<String>,
    exit_flag: Arc<Mutex<bool>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl StreamThread {
    pub fn new(stream: impl NonblockingStream + 'static) -> Self {
        let (tx, send_rx) = std::sync::mpsc::channel();
        let (recv_tx, rx) = std::sync::mpsc::channel();

        let exit_flag = Arc::new(Mutex::new(false));

        let exit_flag_clone = exit_flag.clone();
        let handle = std::thread::spawn(move || {
            log::debug!("stream_loop start");
            Self::stream_loop(stream, &send_rx, &recv_tx, &exit_flag_clone);
            log::debug!("stream_loop end");
        });
        Self {
            tx,
            rx,
            exit_flag,
            handle: Some(handle),
        }
    }

    pub fn create_unix_domain_server(sock_path: &str) -> Result<StreamThread, std::io::Error> {
        let sock_path = sock_path.to_string();

        let (tx, send_rx) = std::sync::mpsc::channel();
        let (recv_tx, rx) = std::sync::mpsc::channel();

        let exit_flag = Arc::new(Mutex::new(false));
        let exit_flag_clone = exit_flag.clone();

        let handle = std::thread::spawn(move || {
            log::debug!("stream thread start : {}", sock_path);
            let listener = UnixListener::bind(sock_path.as_str()).unwrap();
            // listener.set_nonblocking(true).unwrap();

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let exit_flag =
                            StreamThread::stream_loop(stream, &send_rx, &recv_tx, &exit_flag_clone);
                        if exit_flag {
                            break;
                        }
                    }
                    Err(err) => {
                        log::error!("연결 수락 오류: {:?}", err);
                    }
                }
            }
            log::debug!("stream thread end: {}", sock_path);
        });

        Ok(StreamThread {
            tx,
            rx,
            exit_flag,
            handle: Some(handle),
        })
    }

    fn stream_loop<T>(
        mut stream: T,
        rx: &Receiver<String>,
        tx: &Sender<String>,
        exit_flag: &Arc<Mutex<bool>>,
    ) -> bool
    where
        T: NonblockingStream + 'static,
    {
        let mut buf = [0; 2048];
        let mut incomplete_msg = String::new();

        stream.set_nonblocking(true).unwrap();
        loop {
            if *exit_flag.lock().unwrap() {
                log::debug!("exit flag is true");
                return true;
            }
            if let Ok(mut msg) = rx.try_recv() {
                if !msg.ends_with('\n') {
                    msg.push('\n');
                }
                stream.write_all(msg.as_bytes()).unwrap();
            }
            match stream.read(&mut buf) {
                Ok(0) => {
                    log::info!("Connection closed");
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
                    log::error!("Failed to receive data: {}", e);
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        false
    }

    pub fn send(&self, msg: String) {
        log::debug!("send: {}", msg);
        self.tx.send(msg).unwrap();
    }

    pub fn recv(&self) -> Option<String> {
        self.rx.try_recv().ok()
    }

    pub fn stop(&mut self) {
        log::debug!("stop");
        if let Some(handle) = self.handle.take() {
            *self.exit_flag.lock().unwrap() = true;
            log::debug!("wait for the thread to finish");

            if let Err(e) = handle.join() {
                log::error!("Failed to join thread: {:?}", e);
            }
            log::debug!("thread finished");
            self.handle = None;
        }
    }
}

impl Drop for StreamThread {
    fn drop(&mut self) {
        self.stop();
    }
}
