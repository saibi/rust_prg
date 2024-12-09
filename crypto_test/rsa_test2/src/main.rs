use base64::{engine::general_purpose, Engine as _};
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePublicKey},
    sha2::{Digest, Sha256},
    Pkcs1v15Encrypt, Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey,
};
use std::error::Error;
use std::fs;

fn read_public_key(path: &str) -> Result<RsaPublicKey, Box<dyn Error>> {
    let public_key_pem = fs::read_to_string(path)?;
    let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem)?;
    Ok(public_key)
}

fn read_private_key(path: &str) -> Result<RsaPrivateKey, Box<dyn Error>> {
    let private_key_pem = fs::read_to_string(path)?;
    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem)?;
    Ok(private_key)
}

// 공개키로 암호화
fn encrypt_with_public_key(
    public_key: &RsaPublicKey,
    data: &[u8],
) -> Result<String, Box<dyn Error>> {
    let encrypted = public_key.encrypt(&mut rand::thread_rng(), Pkcs1v15Encrypt, data)?;
    let encoded = general_purpose::STANDARD.encode(&encrypted);
    Ok(encoded)
}

// 공개키로 암호화한 데이터를 비밀키로 복호화
fn decrypt_with_private_key(
    private_key: &RsaPrivateKey,
    encrypted_data: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let decoded = general_purpose::STANDARD.decode(encrypted_data)?;
    let decrypted = private_key.decrypt(Pkcs1v15Encrypt, &decoded)?;
    Ok(decrypted)
}

fn main() -> Result<(), Box<dyn Error>> {
    // 키 파일 경로 (OpenSSL로 생성한 키 파일)
    let public_key_path = "public.pem";
    let private_key_path = "private.pem";

    // 공개키로 암호화
    let public_key = read_public_key(public_key_path)?;
    let message = b"Hello, Rust RSA public key encryption!";
    let encrypted_message = encrypt_with_public_key(&public_key, message)?;
    println!("Encrypted with Public Key (Base64): {}", encrypted_message);

    // 비밀키로 복호화
    let private_key = read_private_key(private_key_path)?;
    let decrypted_message = decrypt_with_private_key(&private_key, &encrypted_message)?;
    println!(
        "Decrypted with Private Key: {}",
        String::from_utf8_lossy(&decrypted_message)
    );

    // 서명할 메시지
    let message = b"Hello, World!";

    // 비밀키로 서명 생성
    let padding = Pkcs1v15Sign::new::<Sha256>();
    let hashed_message = Sha256::digest(message);
    let signature = private_key
        .sign(padding, &hashed_message)
        .expect("failed to sign message");

    // 공개키로 서명 검증
    let padding = Pkcs1v15Sign::new::<Sha256>();
    public_key
        .verify(padding, &hashed_message, &signature)
        .expect("failed to verify signature");

    println!("Signature verified!");

    Ok(())
}
