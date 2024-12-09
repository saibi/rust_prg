use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM 256-bit 키
use rand::Rng;

fn main() {
    // 평문 데이터
    let plaintext = b"Hello, world!";

    // 256-bit 키 생성 (32 바이트)
    let binding = rand::thread_rng().gen::<[u8; 32]>();
    let key = Key::<Aes256Gcm>::from_slice(&binding);
    let cipher = Aes256Gcm::new(key);

    // 96-bit nonce 생성 (12 바이트)
    let binding = rand::thread_rng().gen::<[u8; 12]>();
    let nonce = Nonce::from_slice(&binding);

    println!("Plaintext: {}", String::from_utf8_lossy(plaintext));
    println!("Nonce: {:?}", nonce);
    println!("key: {:?}", key);

    // 암호화
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_ref())
        .expect("encryption failure!");
    println!("Ciphertext: {:?}", ciphertext);

    // 복호화
    let decrypted = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");
    println!("Decrypted: {:?}", String::from_utf8(decrypted).unwrap());
}
