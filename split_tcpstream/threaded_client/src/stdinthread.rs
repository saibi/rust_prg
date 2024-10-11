use log::debug;
use std::io;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub struct StdinThread {
    rx: Receiver<String>,
}

impl StdinThread {
    pub fn new() -> StdinThread {
        let (tx, rx) = std::sync::mpsc::channel();

        Self::start_reader_thread(tx);
        StdinThread { rx }
    }

    fn start_reader_thread(tx: Sender<String>) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            debug!("Start reader thread");
            Self::reader_loop(tx);
            debug!("Exit reader thread");
        })
    }

    fn reader_loop(tx: Sender<String>) {
        let mut input = String::new();
        loop {
            if io::stdin().read_line(&mut input).is_ok() {
                tx.send(input.clone()).unwrap();
                input.clear();
            }
        }
    }

    pub fn read_line(&self) -> Option<String> {
        self.rx.try_recv().ok()
    }
}
