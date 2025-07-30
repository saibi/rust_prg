use base64::{Engine as _, engine::general_purpose};
use std::str;

// 방법 1: XOR 암호화 (간단하지만 약함)
fn xor_decrypt(encrypted: &[u8], key: &[u8]) -> String {
    let mut decrypted = Vec::new();
    for (i, &byte) in encrypted.iter().enumerate() {
        decrypted.push(byte ^ key[i % key.len()]);
    }
    String::from_utf8(decrypted).unwrap_or_default()
}

// XOR 암호화 함수 (비밀번호를 암호화할 때 사용)
fn xor_encrypt(plaintext: &str, key: &[u8]) -> Vec<u8> {
    let mut encrypted = Vec::new();
    for (i, byte) in plaintext.bytes().enumerate() {
        encrypted.push(byte ^ key[i % key.len()]);
    }
    encrypted
}

// 방법 2: Base64 인코딩 (가장 간단)
fn base64_decode(encoded: &str) -> String {
    let decoded = general_purpose::STANDARD
        .decode(encoded)
        .unwrap_or_default();
    String::from_utf8(decoded).unwrap_or_default()
}

// 방법 3: 문자열 분할 및 재조합
fn reconstruct_password() -> String {
    let parts = ["my_se", "cret_", "pass", "word"];
    parts.join("")
}

// 방법 4: ASCII 값 변환
fn ascii_to_string(ascii_values: &[u8]) -> String {
    ascii_values.iter().map(|&b| b as char).collect()
}

fn main() {
    println!("=== 비밀번호 하드코딩 방법들 ===\n");

    // 방법 1: XOR 암호화
    println!("1. XOR 암호화 방법:");
    // "my_secret_password"를 "secret_key_123"으로 XOR 암호화한 결과
    let encrypted_password = vec![
        45, 18, 10, 62, 10, 42, 9, 10, 42, 10, 62, 10, 42, 9, 10, 42, 9, 10,
    ];
    let key = b"secret_key_123";
    let password1 = xor_decrypt(&encrypted_password, key);
    println!("   복호화된 비밀번호: {password1}\n");

    // 방법 2: Base64 인코딩
    println!("2. Base64 인코딩 방법:");
    let encoded = "bXlfc2VjcmV0X3Bhc3N3b3Jk"; // "my_secret_password"를 base64로 인코딩
    let password2 = base64_decode(encoded);
    println!("   복호화된 비밀번호: {password2}\n");

    // 방법 3: 문자열 분할
    println!("3. 문자열 분할 방법:");
    let password3 = reconstruct_password();
    println!("   재조합된 비밀번호: {password3}\n");

    // 방법 4: ASCII 값 변환
    println!("4. ASCII 값 변환 방법:");
    let ascii_values = [
        109, 121, 95, 115, 101, 99, 114, 101, 116, 95, 112, 97, 115, 115, 119, 111, 114, 100,
    ]; // "my_secret_password"
    let password4 = ascii_to_string(&ascii_values);
    println!("   변환된 비밀번호: {password4}\n");

    // 실제 사용 예시
    let expected_password = "my_secret_password";
    if password1 == expected_password {
        println!("✅ XOR 방법으로 비밀번호 확인 성공!");
    }
    if password2 == expected_password {
        println!("✅ Base64 방법으로 비밀번호 확인 성공!");
    }
    if password3 == expected_password {
        println!("✅ 문자열 분할 방법으로 비밀번호 확인 성공!");
    }
    if password4 == expected_password {
        println!("✅ ASCII 변환 방법으로 비밀번호 확인 성공!");
    }

    // XOR 암호화 테스트
    println!("\n=== XOR 암호화 테스트 ===");
    let test_password = "my_secret_password";
    let test_key = b"secret_key_123";
    let encrypted = xor_encrypt(test_password, test_key);
    let decrypted = xor_decrypt(&encrypted, test_key);
    println!("원본: {test_password}");
    println!("암호화: {encrypted:?}");
    println!("복호화: {decrypted}");
    println!("일치: {}", test_password == decrypted);
}
