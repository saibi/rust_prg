use std::ops::Add;

fn pick<T: Add<Output = T>>(n: i32, even: T, odd: T) -> T {
    if n == 0 {
        even + odd
    } else if n % 2 == 0 {
        even
    } else {
        odd
    }
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

struct PointTU<T, U> {
    x: T,
    y: U,
}
impl<T, U> PointTU<T, U> {
    fn coords(&self) -> (&T, &U) {
        (&self.x, &self.y)
    }
}

fn main() {
    println!("pick(1, 2, 3) = {:?}", pick(1, 2, 3));
    // println!("pick(2, ..., ...) = {:?}", pick(2, ("dog", 18), ("cat", 3)));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer coords: {:?}", integer.coords());
    println!("float coords: {:?}", float.coords());

    let point = PointTU { x: 5, y: 10.0 };
    println!("point coords: {:?}", point.coords());
}
