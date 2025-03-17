fn test_str() {
    let greeting: &str = "  인사말";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("마지막 문장: {}", sentence);
    println!("01234567890123456789");
    println!("{}", sentence);
    println!("{:?}", &sentence[0..5]);
    println!("{:?}", &sentence[12..13]);
    // println!("{:?}", &sentence[2..3]);
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}
fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn test_infer() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    // takes_u32(y);
}

fn fib(n: u32) -> u32 {
    if n <= 2 { 1 } else { fib(n - 1) + fib(n - 2) }
}

fn test_if() {
    let x = 10;
    let size = if x < 20 { "작은" } else { "대형" };
    println!("숫자 크기: {}", size);
}

fn test_block() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
}

fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 { gcd(b, a % b) } else { a }
}
fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fizzbuzz(n: u32) -> u32 {
    todo!()
}

/// `n`에서 시작하는 콜라츠 수열의 길이를 결정합니다.
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 0;
    loop {
        println!("{n}");
        len += 1;
        if n == 1 {
            break;
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    len
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(3), 8);
}

fn test_array() {
    let mut a: [i8; 10] = [42; 10];
    a[15] = 0;
    println!("a: {a:?}");
}

fn main() {
    // test_str();
    // test_infer();

    // let n = 20;
    // println!("fib({n}) = {}", fib(n));

    // fib(0);
    // println!("gcd: {}", gcd(143, 52));

    // let n = 4;
    // println!("{n}! = {}", factorial(n));

    // fizzbuzz(n); // panic

    // println!("콜라츠 수열 길이: {}", collatz_length(3));

    test_array();
}
