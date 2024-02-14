use common;

fn main() {
    println!("Hello, world!");
    println!("{}", common::add(rand::random::<usize>() % 10, 2));
}
