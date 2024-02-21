use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel(); // multiple producer, single consumer

    thread::spawn(move || {
        let val = String::from("hi");
        println!("Sending: {}", val);
        tx.send(val).unwrap();
        //println!("Sent: {}", val); // borrow of moved value: `val`
    });

    // block the main thread until the value is received
    // let received = rx.recv().unwrap();
    // println!("Received: {}", received);

    // non-blocking
    for i in 1..100 {
        let received = rx.try_recv();
        if let Ok(val) = received {
            println!("Received: {}", val);
            break;
        } else {
            println!("Waiting for value... {}", i);
            thread::sleep(std::time::Duration::from_nanos(1));
        }
    }
}
