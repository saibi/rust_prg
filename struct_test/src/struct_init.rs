struct Person {
    name: String,
    age: u8,
    mothers_name: String,
}

fn create_person(name: String, age: u8) -> Person {
    Person { name, age }
}

fn main() {
    let person = Person {
        name: "John".to_string(),
        age: 30,
        mothers_name: "Jane".to_string(),
    };

    println!("Person name is {} and age is {}", person.name, person.age);

    let person2 = Person {
        name: "Doe".to_string(),
        age: person.age,
        mothers_name: person.mothers_name,
    };

    // or
    let person3 = Person {
        name: "Foo".to_string(),
        ..person
    };
}
