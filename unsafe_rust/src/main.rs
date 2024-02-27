static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("{}", HELLO_WORLD);

    // raw pointers
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        dangerous();
    }
    //dangerous() // calling unsafe function is not allowed

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

unsafe fn dangerous() {
    println!("This is unsafe function");
}

use std::slice;
fn split_at_mut(value: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = value.len();
    let ptr = value.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
