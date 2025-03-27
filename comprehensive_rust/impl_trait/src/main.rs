fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x, x)
}

fn main() {
    let many = add_42_millions(42_i8);
    println!("{many}");

    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");

    let debuggable = pair_of(27);
    // let debuggable: () = pair_of(27);
    println!("debuggable: {debuggable:?}");

    min_test();
}

use std::cmp::Ordering;

// TODO: `main`에 사용되는 `min` 함수를 구현합니다.

fn min<T: Ord>(a: T, b: T) -> T {
    // if a < b { a } else { b }
    if a.cmp(&b) == Ordering::Less { a } else { b }
}

fn min_test() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
