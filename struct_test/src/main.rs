#[derive(Debug)]
struct Point(f64, f64, f64);

impl Point {
    fn make_twice(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }
    fn twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }
    fn describe(&self) {
        println!("x = {}, y = {}, z = {}", self.0, self.1, self.2);
    }
}

impl Point {
    fn zero() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}

fn main() {
    let p = Point(1.0, 2.0, 3.0);
    p.describe();

    println!("{:?}", p); // do not remove #[derive(Debug)] from Point

    let p2 = p.twice();
    println!("{:?}", p2);

    let mut p3 = Point(1.0, 1.0, 1.0);
    p3.make_twice();
    println!("{:?}", p3);

    // let p3 = Point(1.0, 1.0, 1.0);
    // p3.make_twice(); // cannot mutate

    let p4 = Point::zero();
    println!("{:?}", p4);
}
