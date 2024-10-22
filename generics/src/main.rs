// struct Val {
//     val: f64,
// }

// impl Val {
//     fn value(&self) -> &f64 {
//         &self.val
//     }
// }

// struct GenVal<T> {
//     gen_val: T,
// }

// impl<T> GenVal<T> {
//     fn value(&self) -> &T {
//         &self.gen_val
//     }
// }

// fn main() {
//     let x = Val { val: 3.0 };
//     let y = GenVal { gen_val: 3.0f64 };

//     println!("{}, {}", x.value(), y.value());
//     println!("{}, {:.2}", x.value(), y.value());
// }

use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

fn main() {
    let rect = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    let triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rect);
    println!("Area: {}", area(&rect));

    println!("{}", is_red(&rect));

    // print_debug(&triangle);
    // println!("Area: {}", area(&triangle));

    let string = "words";
    compare_prints(&string);
}

trait Red {}

impl Red for Rectangle {}

fn is_red<T>(_: &T) -> &'static str
where
    T: Red,
{
    "red"
}

use std::fmt::Display;

fn compare_prints<T>(t: &T)
where
    T: Display + Debug,
{
    println!("Debug: '{:?}'", t);
    println!("Display: '{}'", t);
}
