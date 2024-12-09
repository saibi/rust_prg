fn main() {
    let str = String::from("Hello, world!");

    println!("call again {}", str);
    hello(str);

    let mut number = 5i32;
    println!("number is {}", number);
    number = 6;
    println!("number is {}", number);

    let number = 4i32;
    println!("number is {}", number);
}

fn hello(str: String) {
    println!("Hello {}", str);
}
