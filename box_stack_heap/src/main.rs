struct Point {
    x: i32,
    y: i32,
}

fn gen_point() -> Box<Point> {
    Box::new(Point { x: 1, y: 2 })
}

fn main() {
    println!("Hello, world!");

    let heap_point = gen_point();

    println!("heap_point.x = {}", heap_point.x);
    println!("heap_point.y = {}", heap_point.y);

    let b = heap_point;

    println!("heap_point.x = {}", heap_point.x);
}
