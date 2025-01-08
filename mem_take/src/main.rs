use std::mem;

#[derive(Debug)]
enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D,
}

fn swizzle(e: &mut MultiVariateEnum) {
    use MultiVariateEnum::*;
    *e = match e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { name } => B {
            name: mem::take(name),
        },
        B { name } => A {
            name: mem::take(name),
        },
        C => D,
        D => C,
    }
}
fn main() {
    println!("Hello, world!");

    let mut a = MultiVariateEnum::A {
        name: "a".to_string(),
    };
    println!("{:?}", a);
    swizzle(&mut a);
    println!("{:?}", a);

    let mut b = MultiVariateEnum::B {
        name: "b".to_string(),
    };
    println!("{:?}", b);
    b = MultiVariateEnum::A {
        name: "a".to_string(),
    };
    println!("{:?}", b);
}
