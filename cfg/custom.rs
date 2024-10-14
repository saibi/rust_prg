#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

#[cfg(my_cfg = "test")]
fn my_test() {
    println!("my_test");
}


fn main() {
    if cfg!(target_os = "linux") {
        println!("linux");
    } else {
        println!("not linux");
    }

    conditional_function();
    my_test();
}

