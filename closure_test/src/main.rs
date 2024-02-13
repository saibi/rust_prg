use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];
    println!("before defining closure: {:?}", list);

    // let only_borrows = || println!("From closure: {:?}", list);
    // println!("before calling closure: {:?}", list);
    // only_borrows();
    // println!("after calling closure: {:?}", list);

    // let mut borrows_mutably = || list.push(7);
    // // println!("before calling closure: {:?}", list); // compile error
    // borrows_mutably();
    // println!("after calling closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    // println!("after calling closure: {:?}", list); // compile error
}
