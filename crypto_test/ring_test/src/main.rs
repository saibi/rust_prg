use ring::aead::{
    Aad, BoundKey, Nonce as NonceType, NonceSequence, OpeningKey, SealingKey, UnboundKey,
    AES_128_GCM, NONCE_LEN,
};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};

// Nonce를 위한 구조체 정의
struct NonceGen {
    nonce: [u8; NONCE_LEN],
}

impl NonceGen {
    fn new(nonce: [u8; NONCE_LEN]) -> Self {
        Self { nonce }
    }
}

impl NonceSequence for NonceGen {
    fn advance(&mut self) -> Result<NonceType, Unspecified> {
        Ok(NonceType::assume_unique_for_key(self.nonce))
    }
}

fn main() -> Result<(), Unspecified> {
    // 랜덤 키 생성 (16 바이트)
    let rng = SystemRandom::new();
    let mut key_bytes = [0u8; 16];
    rng.fill(&mut key_bytes)?;

    // 랜덤 nonce 생성 (12 바이트)
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rng.fill(&mut nonce_bytes)?;
    let nonce_gen = NonceGen::new(nonce_bytes);

    // 암호화할 데이터
    let plaintext = b"Hello, ring encryption!";
    println!("원본 데이터: {}", String::from_utf8_lossy(plaintext));

    // 암호화 키 설정
    let unbound_key = UnboundKey::new(&AES_128_GCM, &key_bytes)?;
    let mut sealing_key = SealingKey::new(unbound_key, nonce_gen);

    // 암호화
    let mut ciphertext = plaintext.to_vec();
    sealing_key.seal_in_place_append_tag(Aad::empty(), &mut ciphertext)?;
    println!("암호화된 데이터 (with tag): {:?}", ciphertext);

    // 복호화 키 설정
    let unbound_key = UnboundKey::new(&AES_128_GCM, &key_bytes)?;
    let mut opening_key = OpeningKey::new(unbound_key, NonceGen::new(nonce_bytes));

    // 복호화
    let decrypted = opening_key.open_in_place(Aad::empty(), &mut ciphertext)?;
    println!("복호화된 데이터: {}", String::from_utf8_lossy(decrypted));

    Ok(())
}
