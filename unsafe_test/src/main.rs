fn main() {
    raw_pointer_test();

    unsafe_fn_test();
}

fn raw_pointer_test() {
    let raw_p: *const u32 = &18;

    unsafe {
        println!("raw pointer value: {}", *raw_p);
    }
}

fn unsafe_fn_test() {
    let some_vector = vec![1, 2, 3, 4, 5];
    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    println!("length: {}", length);
    unsafe {
        let my_slice: &[u32] = std::slice::from_raw_parts(pointer, length);
        assert_eq!(my_slice, some_vector.as_slice());
    }
}
