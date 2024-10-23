use std::boxed;

fn main() {
    println!("Hello, world!");
    let x = [1, 2, 3]; // "hello".to_string()
    let y = x;

    println!("x = {:?}", x);
    println!("y = {:?}", y);

    mutability();
    partial_moves();
    borrowing();
}

fn mutability() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    //*immutable_box = 1;

    let mut mutable_box = immutable_box;

    //println!("print immutable_box contains {}", immutable_box);

    *mutable_box = 3;
    println!("mutable_box contains {}", mutable_box);
}

fn partial_moves() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person { name, ref age } = person;

    println!("age is {}", age);
    println!("name is {}", name);

    //println!("persion is {:?}", person);

    println!("person.age is {}", person.age);
}

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn borrowing() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32 = &boxed_i32;
        // eat_box_i32(boxed_i32);
        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);
}
