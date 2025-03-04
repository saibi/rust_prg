fn main() {
    partial_eq_test();
    using_partial_eq();
    eq_test();
    partial_ord_test();
    from_test();
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

use std::{collections::HashMap, io};

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

#[derive(PartialEq, Eq, Hash)]
struct MyKey {
    x: i32,
    y: i32,
}

struct MyVal {
    distance: f32,
}

fn eq_test() {
    let mut map = HashMap::new();
    map.insert(MyKey { x: 1, y: 2 }, MyVal { distance: 3.0 });
}

#[derive(Debug, PartialEq)]
struct Person2 {
    name: String,
    age: i16,
    height: i16,
}

impl PartialOrd for Person2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.height <= 0 || other.height <= 0 {
            return None;
        }

        // if self.height > other.height {
        //     Some(std::cmp::Ordering::Greater)
        // } else if self.height < other.height {
        //     Some(std::cmp::Ordering::Less)
        // } else {
        //     Some(std::cmp::Ordering::Equal)
        // }
        Some(self.height.cmp(&other.height))
    }
}

fn partial_ord_test() {
    let a = Person2 {
        name: "Alice".to_string(),
        age: 28,
        height: 193,
    };

    let b = Person2 {
        name: "Bob".to_string(),
        age: 25,
        height: 180,
    };

    if a > b {
        println!("{} is taller than {}", a.name, b.name);
    } else if a < b {
        println!("{} is shorter than {}", a.name, b.name);
    } else {
        println!("{} and {} are the same height", a.name, b.name);
    }

    if a.partial_cmp(&b).unwrap() == std::cmp::Ordering::Greater {
        println!("{} is taller than {}", a.name, b.name);
    }

    let mut class = vec![
        Person2 {
            name: "Alice".to_string(),
            age: 28,
            height: 173,
        },
        Person2 {
            name: "Bob".to_string(),
            age: 25,
            height: 180,
        },
        Person2 {
            name: "Charlie".to_string(),
            age: 30,
            height: 165,
        },
    ];

    class.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("sorted by height: {:?}", class);
}

#[derive(Debug)]
struct Book2 {
    title: String,
    author: String,
    published: u32,
    isbn: String,
}

// impl From<Book2> for u32 {
//     fn from(book: Book2) -> u32 {
//         book.isbn.parse().unwrap_or(0)
//     }
// }

impl TryFrom<Book2> for u32 {
    type Error = &'static str;

    fn try_from(book: Book2) -> Result<u32, Self::Error> {
        book.isbn.parse().map_err(|_| "Invalid ISBN")
    }
}

fn from_test() {
    let book = Book2 {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        published: 20230228,
        isbn: "18A-1593278281".to_string(),
    };

    let rust_in_action = Book2 {
        title: "Rust in Action".to_string(),
        author: "Tim McNamara".to_string(),
        published: 20230228,
        isbn: "1617294551".to_string(),
    };

    // let isbn: u32 = book.into();
    // let isbn2 = u32::from(rust_in_action);
    // println!("isbn: {}, isbn2: {}", isbn, isbn2);

    let isbn: Result<u32, &str> = book.try_into();
    let isbn2 = u32::try_from(rust_in_action);
    println!("isbn: {:?}, isbn2: {:?}", isbn, isbn2);
}
