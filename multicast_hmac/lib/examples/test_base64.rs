use lib::hmac_msg::create_hmac_msg;

fn main() {
    let secret_key = b"test_secret_key";
    let message = "hello world";

    // Base64 인코딩된 HMAC 서명이 포함된 JSON 생성
    let json_msg = create_hmac_msg(message, secret_key);

    println!("Original message: {}", message);
    println!("Generated JSON (with base64 signature): {}", json_msg);

    // 16진수 버전과 비교하기 위해 수동 계산
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(secret_key);
    hasher.update(message.as_bytes());
    let signature = hasher.finalize();

    let hex_signature: String = signature.iter().map(|b| format!("{:02x}", b)).collect();

    println!("Same signature in hex: {}", hex_signature);
    println!("Hex length: {} chars", hex_signature.len());

    // base64 길이 확인
    use base64::{Engine, engine::general_purpose};
    let base64_signature = general_purpose::STANDARD.encode(signature);
    println!("Same signature in base64: {}", base64_signature);
    println!("Base64 length: {} chars", base64_signature.len());

    println!("\nSize comparison:");
    println!("- Hex encoding: {} bytes", hex_signature.len());
    println!("- Base64 encoding: {} bytes", base64_signature.len());
    println!(
        "- Space saved: {} bytes ({:.1}%)",
        hex_signature.len() - base64_signature.len(),
        ((hex_signature.len() - base64_signature.len()) as f32 / hex_signature.len() as f32)
            * 100.0
    );
}
