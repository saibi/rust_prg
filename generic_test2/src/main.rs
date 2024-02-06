struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: 'c', y: "Hello" };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<i32> {
//     fn y_ref(&self) -> &i32 {
//         &self.y
//     }
// }

// impl Point<f64> {
//     fn distance_from_origin(&self) -> f64 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let interger = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.7 };

//     println!("interger.x = {}, float.x = {}", interger.x(), float.x());
//     println!("interger.y = {} ", interger.y_ref(),);

//     println!("Distance from origin: {}", float.distance_from_origin());
// }
