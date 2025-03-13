use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use std::io::{Write, stdin, stdout};

fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

fn main() {
    println!("Please input 4-digits Customer ID: ");
    let customerid = Some(get_user_input());

    println!("Please input 8-digits Product ID: ");
    let productid = Some(get_user_input());

    let plain_serial = format!("{}{}", customerid.unwrap(), productid.unwrap());
    println!("Plain serial: {}", plain_serial); // 암호화 전 시리얼 출력

    let mc = new_magic_crypt!("magickey", 256);
    let serial = mc.encrypt_str_to_base64(&plain_serial);
    println!("Encrypted serial: {}", serial); // 암호화된 시리얼 출력
    let dec = mc.decrypt_base64_to_string(serial).unwrap();
    println!("Decrypted serial: {}", dec); // 복호화된 시리얼 출력

    let verify_customerid = &dec[0..4];
    let verify_productid = &dec[4..12];
    println!("Verify Customer ID: {}", verify_customerid);
    println!("Verify Product ID: {}", verify_productid);
}

fn magic_crypt_example() {
    let mc = new_magic_crypt!("magickey", 256);
    let base64 = mc.encrypt_str_to_base64("http://magiclen.org");
    println!("Encrypted: {}", base64); // 암호화된 문자열 출력
    assert_eq!("DS/2U8royDnJDiNY2ps3f6ZoTbpZo8ZtUGYLGEjwLDQ=", base64);

    assert_eq!(
        "http://magiclen.org",
        mc.decrypt_base64_to_string(&base64).unwrap()
    );
}
