use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SSH 키 파일 경로 (예시)
    // openssl genrsa -out private.pem 2048
    // openssl rsa -in private.pem -pubout -out public.pem
    let private_key_path = "./src/private.pem";
    let public_key_path = "./src/public.pem";

    // 키 파일 읽기
    let private_key_pem = fs::read_to_string(private_key_path)?;
    let public_key_pem = fs::read_to_string(public_key_path)?;

    // RSA 키 파싱
    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem)?;
    let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem)?;

    // 테스트 메시지
    let msg = b"hello world";

    // 공개키로 암호화
    let encrypted = public_key.encrypt(&mut rand::thread_rng(), Pkcs1v15Encrypt, msg)?;

    // 개인키로 복호화
    let decrypted = private_key.decrypt(Pkcs1v15Encrypt, &encrypted)?;

    // 결과 확인
    println!("원본 메시지: {}", String::from_utf8_lossy(msg));
    println!("암호화된 메시지: {:?}", encrypted);
    println!("복호화된 메시지: {}", String::from_utf8_lossy(&decrypted));

    sig_test();
    Ok(())
}

use rsa::pkcs1v15::{SigningKey, VerifyingKey};
use rsa::sha2::{Digest, Sha256};
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};

fn sig_test() {
    let mut rng = rand::thread_rng();

    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let verifying_key = signing_key.verifying_key();

    // Sign
    let data = b"hello world";
    let signature = signing_key.sign_with_rng(&mut rng, data);
    assert_ne!(signature.to_bytes().as_ref(), data.as_slice());
    println!("Signature: {:?}", signature);

    // Verify
    verifying_key
        .verify(data, &signature)
        .expect("failed to verify");
}
