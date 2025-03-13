//! # 제목입니다.
//!
//! 이 파일 전체에 대한 소개를 담고 있습니다.
//! * 이런 Markdown 문법들을 사용할 수 있습니다.

/// '/'를 3개를 써서 주석을 만들고, 첫줄에는 함수에 대한 짧은 소개를 씁니다.
///
/// 공백 한줄을 만든 후, 여기에는 함수에 대한 자세한 설명을 씁니다.
/// 설명이 끝나면 아래에 공백 한줄을 만든 후 ```와 ```로 코드의 시작과 끝을 표시합니다.
/// 함수를 호출할때는 이름만 써주는게 아니라 크레이트의 이름도 같이 써주어야합니다.
/// 현재 이 크레이트는 lib_example입니다.
/// ```
/// assert_eq!(18, lib_example::add(3, 15));
/// ```
/// ```
/// assert_eq!(0, lib_example::add(0, 0));
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        println!("try 2 + 2");
        assert_eq!(result, 4);
    }
}

/// 구조체의 문서화에 대한 예제입니다.
///
/// 구조체에 대한 설명을 적습니다.
/// 다른 항목에 대한 링크는 [`add`]와 같은 형태로 적으면 자동으로 add함수에 대한 링크를 생성해줍니다.
///
/// # Examples
///
/// 이렇게 이 구조체를 사용하는 예제를 주석에 넣을 수도 있습니다. 이 예제도 cargo test에서 실행됩니다.
///
/// ```
/// let ex = lib_example::StructExample::new();
/// ```
pub struct StructExample {
    /// foo 필드에 대한 설명을 적습니다.
    pub foo: usize,
    /// pub가 없는 bar 필드에 대한 설명을 적습니다.
    /// 이 필드는 pub가 없기 때문에 외부에서 접근할 수 없습니다.
    /// pub가 없으면 doc 으로 문서화되지 않음
    bar: Option<String>,
}

impl StructExample {
    pub fn new() -> Self {
        StructExample { foo: 0, bar: None }
    }
}
