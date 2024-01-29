struct Point(f64, f64, f64);

fn main() {
    let point = Point(1.0, 2.0, 3.0);

    println!("x = {}, y = {}, z = {}", point.0, point.1, point.2);
}
