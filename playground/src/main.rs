#![deny(clippy::all)]

fn main() {
    let name = String::from("Alice"); // name is a String

    take_ownership(name); // name is moved to the function

    // Error: name has been moved and is no longer valid
    println!("Hello, {}", name);
}

fn take_ownership(s: String) {
    // s is a String that takes ownership
    println!("Received ownership of {}", s);
}
