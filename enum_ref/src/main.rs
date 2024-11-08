use std::sync::{Arc, Mutex};

enum BinReceiverStatus {
    Idle,
    Received(String),
}
impl BinReceiverStatus {
    fn get(&self) -> Option<&str> {
        match self {
            BinReceiverStatus::Received(buf) => Some(buf),
            _ => None,
        }
    }
}
pub struct BinReceiver {
    status: Arc<Mutex<BinReceiverStatus>>,
    buf: String,
}

impl BinReceiver {
    pub fn new() -> Self {
        BinReceiver {
            status: Arc::new(Mutex::new(BinReceiverStatus::Idle)),
            buf: String::new(),
        }
    }

    pub fn set(&mut self) {
        let mut status = self.status.lock().unwrap();
        let msg = "Hello".to_string();
        self.buf = "hello".to_string();
        *status = BinReceiverStatus::Received(msg);
    }

    pub fn get(&self) -> Option<&str> {
        let status = self.status.lock().unwrap();
        match &*status {
            BinReceiverStatus::Received(_) => Some(self.buf.as_str()),
            _ => None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
