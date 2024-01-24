#[derive(Debug)]

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// write enum Message equivalant code with struct
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("I'm call method, {:?}", self);
    }
}

fn main() {
    let q = Message::Quit;
    let w = Message::Write(String::from("Hello"));

    q.call();
    w.call();
}
