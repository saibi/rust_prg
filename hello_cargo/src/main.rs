fn main() {
    let str = String::from("Hello, world!");

    println!("call again {}", str);
    hello(str);
}

fn hello(str: String) {
    println!("Hello {}", str);
}
