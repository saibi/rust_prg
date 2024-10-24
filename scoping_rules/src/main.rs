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
    test1_main();
    test2_main();
    test3_main();
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

// A type `Borrowed` which houses a reference to an
// `i32`. The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn test1_main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

#[derive(Debug)]
struct Borrowed2<'a> {
    x: &'a i32,
}

impl<'a> Default for Borrowed2<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

fn test2_main() {
    let b: Borrowed2 = Default::default();
    println!("b is {:?}", b);
}

use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T>(t: T)
where
    T: Debug,
{
    println!("'print': t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("'print_ref': t is {:?}", t);
}

fn test3_main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
