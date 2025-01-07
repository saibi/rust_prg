use std::{path::PathBuf, time::Duration};

// note that we can simply auto-derive Default here.
#[derive(Default, Debug, PartialEq)]
struct MyConfiguration {
    // Option defaults to None
    output: Option<PathBuf>,
    // Vecs default to empty vector
    search_path: Vec<PathBuf>,
    // Duration defaults to zero time
    timeout: Duration,
    // bool defaults to false
    check: bool,
}

impl MyConfiguration {
    // add setters here
}

fn main() {
    // construct a new instance with default values
    let mut conf = MyConfiguration::default();
    // do something with conf here
    conf.check = true;
    println!("conf = {conf:#?}");

    // partial initialization with default values, creates the same instance
    let conf1 = MyConfiguration {
        check: true,
        ..Default::default()
    };
    // ..Default::default() 구문은 Rust의 구조체 업데이트 문법(struct update syntax)의 한 예입니다.
    // 이 문법은 다음과 같은 의미를 가집니다:
    // Default::default()는 MyConfiguration 구조체의 기본값을 생성합니다
    // ..는 "나머지 필드들"을 의미합니다
    // 따라서 check: true를 제외한 모든 필드는 기본값을 사용하게 됩니다
    // 이 문법은 구조체의 일부 필드만 커스터마이즈하고 나머지는 기본값을 사용하고 싶을 때 매우 유용합니다. 코드가 더 간결해지고 가독성이 좋아집니다.
    assert_eq!(conf, conf1);
}
