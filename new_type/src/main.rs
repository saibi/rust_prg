fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
    f();
}

fn return_long_type() -> Box<dyn Fn() + Send + 'static> {
    // --snip--
    Box::new(|| println!("hello"))
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type2(f: Thunk) {
    // --snip--
    f();
}

fn returns_long_type2() -> Thunk {
    // --snip--
    Box::new(|| println!("hello2"))
}

fn main() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    takes_long_type(f);

    let another_f = return_long_type();
    another_f();

    let f2: Thunk = Box::new(|| println!("hi2"));
    takes_long_type2(f2);

    let another_f2 = returns_long_type2();
    another_f2();
}
