// src/generic_trait/main.rs
trait Printable<T> {
    fn print(&self);
    fn get_age(&self) -> T;
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
}

impl Printable<u32> for Person {
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.age);
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn print_info(item: &dyn Printable<u32>) {
    item.print();
}

fn main() {
    let person: Person = Person::new("Alice", 22);
    print_info(&person);
}
