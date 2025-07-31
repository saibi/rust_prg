use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // 랜덤 키 생성 (4바이트)
    let mut key = [0u8; 4];
    getrandom::getrandom(&mut key).expect("Failed to generate random key");

    // 출력 디렉토리 생성
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("random_key.rs");

    // 랜덤 키를 포함한 Rust 코드 생성
    let mut f = File::create(&dest_path).unwrap();
    writeln!(f, "// Auto-generated random key").unwrap();
    writeln!(f, "pub const RANDOM_KEY: [u8; 4] = {key:?};").unwrap();

    // 빌드 스크립트가 변경될 때마다 다시 실행되도록 설정
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=RANDOM_KEY={key:?}");
}
