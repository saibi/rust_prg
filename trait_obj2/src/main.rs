struct Dog {
    name: String,
    age: u8,
}

struct Cat {
    lives: u8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("my name {}, age {}", self.name, self.age)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        format!("I have {} lives", self.lives)
    }
}

fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat { lives: 9 }),
        Box::new(Dog {
            name: String::from("Rex"),
            age: 5,
        }),
    ];

    for pet in pets {
        println!("talk: {}", pet.talk());
    }

    println!(
        "{} {}",
        std::mem::size_of::<Dog>(),
        std::mem::size_of::<Cat>()
    );
    println!(
        "{} {}",
        std::mem::size_of::<&Dog>(),
        std::mem::size_of::<&Cat>()
    );
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());
}
