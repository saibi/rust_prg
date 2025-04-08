fn string_main() {
    let mut s1 = String::new();
    s1.push_str("안녕하세요");
    println!("s1: len = {}, 용량 = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, 용량 = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    println!("s3: len = {}, 문자 수 = {}", s3.len(), s3.chars().count());
}

fn vec_main() {
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, 용량 = {}", v1.len(), v1.capacity());

    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, 용량 = {}", v2.len(), v2.capacity());

    // 요소가 있는 벡터를 초기화하는 표준 매크로입니다.
    let mut v3 = vec![0, 0, 1, 2, 3, 4];

    // 짝수 요소만 유지합니다.
    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");

    // 연속 중복 삭제
    v3.dedup();
    println!("{v3:?}");
}

use std::collections::HashMap;

/// Counter는 각 T 타입 값이 표시된 횟수를 계산합니다.
struct Counter<T: Eq + std::hash::Hash> {
    values: HashMap<T, u64>,
}

impl<T: Eq + std::hash::Hash> Counter<T> {
    /// 새 Counter를 만듭니다.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// 지정된 값의 발생 횟수를 셉니다.
    fn count(&mut self, value: T) {
        let entry = self.values.entry(value).or_insert(0);
        *entry += 1;
    }

    /// 지정된 값이 표시된 횟수를 반환합니다.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

fn counter_main() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("{} 개의 {} 값을 발견했습니다.", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("사과");
    strctr.count("오렌지");
    strctr.count("사과");
    println!("사과 {}개 받음", strctr.times_seen("사과"));
}

fn main() {
    counter_main();
}
