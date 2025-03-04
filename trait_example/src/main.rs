fn main() {
    partial_eq_test();
    using_partial_eq();
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, Default)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author
    }
}

impl PartialEq<Person> for Book {
    fn eq(&self, other: &Person) -> bool {
        self.author.contains(&other.name)
    }
}

impl PartialEq<Book> for Person {
    fn eq(&self, other: &Book) -> bool {
        other.author.contains(&self.name)
    }
}

fn partial_eq_test() {
    let second = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };
    let steve = Person {
        name: "Steve Klabnik".to_string(),
        age: 30,
    };
    if second == steve {
        println!("Yes, this book is writtend by {:?}", steve);
    }
    if steve == second {
        println!("Yes, {:?} wrote the book {:?}", steve, second);
    }
}

use std::io;

#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    Quit,
    Execute(String),
    Run,
}

fn user_input() -> Result<Command, String> {
    println!("input h/q/r/e: ");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.as_str().strip_suffix("\n") {
            Some("h") => Ok(Command::Help),
            Some("q") => Ok(Command::Quit),
            Some("r") => Ok(Command::Run),
            Some("e") => Ok(Command::Execute("NOT IMPLEMENTED".to_string())),
            _ => Err(format!("Invalid command: {input}")),
        },
        Err(e) => Err(format!("Wrong input: {e}")),
    }
}

fn using_partial_eq() {
    let cmd = user_input().expect("Failed to read user input");
    let p = String::new();

    assert_ne!(cmd, Command::Execute(p));
    match cmd {
        Command::Help => println!("show help message"),
        Command::Quit => return,
        Command::Run => println!("do something"),
        Command::Execute(p) => println!("Execute: {p}"),
    }
    println!("end of using_partial_eq");
}
