#![deny(clippy::all)]

fn unit_return_function() -> () {}

fn say_hello_world() -> String {
    println!("say_hello_world: Hello, world!");
    String::from("Hello, world!")
}

fn say_hello(msg: &str) {
    println!("say_hello: {}", msg);
}

fn say_hello_to_person(to_person: &str) -> String {
    format!("Hello, {}!", to_person)
}

fn process_name(name: &str, callback: fn(&str) -> ()) {
    callback(name);
}

fn main() {
    let message = say_hello_world();
    println!("{}", message);

    say_hello("Hello, world!");

    // inline function
    let say_hello_to = |name: &str| format!("Hello, {} !", name);
    let full_name = |first_name: &str, last_name: &str| format!("{} {}", first_name, last_name);

    let multiply_by_2 = |x: i32| x * 2;

    let ptr = multiply_by_2;

    println!("{}", ptr(2));

    process_name("John", say_hello);
}
