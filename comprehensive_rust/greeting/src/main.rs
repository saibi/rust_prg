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
    a[5] = 0;
    println!("a: {a:#?}");
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            transposed[i][j] = matrix[j][i];
        }
    }
    transposed
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

// 해당 좌표의 제곱을 더하고
// 제곱근을 사용하여 벡터의 크기를 계산합니다. `v.sqrt()`와 같은 `sqrt()` 메서드를 사용하여 제곱근을
// 계산합니다.

fn magnitude(point: &[f64; 3]) -> f64 {
    let mut sum = 0.0;
    for i in 0..3 {
        sum += point[i] * point[i];
    }
    sum.sqrt()
}

// 벡터의 크기를 계산하고 모든 좌표를 해당 크기로 나눠서
// 벡터를 정규화합니다.

fn normalize(v: &mut [f64; 3]) {
    let m = magnitude(v);
    for i in 0..3 {
        v[i] /= m;
    }
}

// 다음 `main`을 사용하여 작업을 테스트합니다.

fn test_normalize() {
    println!("단위 벡터의 크기: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("{v:?} 크기: {}", magnitude(&v));
    normalize(&mut v);
    println!("정규화 후 {v:?}의 크기: {}", magnitude(&v));
}

#[derive(Debug)]
/// 컨트롤러가 반응해야 하는 엘리베이터 시스템의 이벤트입니다.
enum Event {
    // TODO: 필요한 변형들을 추가하세요.
    Arrived(i32),
    Opened,
    Closed,
    Lobby(i32, Direction),
    Pressed(i32),
}

/// 이동 방향입니다.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// 엘리베이터가 지정된 층에 도착했습니다.
fn car_arrived(floor: i32) -> Event {
    // todo!()
    Event::Arrived(floor)
}

/// 엘리베이터 문이 열렸습니다.
fn car_door_opened() -> Event {
    // todo!()
    Event::Opened
}

/// 엘리베이터 문이 닫혔습니다.
fn car_door_closed() -> Event {
    // todo!()
    Event::Closed
}

/// 지정된 층의 엘리베이터 로비에서 방향 버튼을 눌렀습니다.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    // todo!()
    Event::Lobby(floor, dir)
}

/// 엘리베이터에서 층 버튼을 눌렀습니다.
fn car_floor_button_pressed(floor: i32) -> Event {
    // todo!()
    Event::Pressed(floor)
}

fn test_elevator() {
    println!(
        "1층 승객이 위쪽 버튼을 눌렀습니다. {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("엘리베이터가 1층에 도착했습니다: {:?}", car_arrived(0));
    println!("엘리베이터 문이 열렸습니다. {:?}", car_door_opened());
    println!(
        "승객이 3층 버튼을 눌렀습니다. {:?}",
        car_floor_button_pressed(3)
    );
    println!("엘리베이터 문이 닫혔습니다: {:?}", car_door_closed());
    println!("엘리베이터가 3층에 도착했습니다. {:?}", car_arrived(3));
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

    // test_array();

    // test_normalize();
    test_elevator();
}
