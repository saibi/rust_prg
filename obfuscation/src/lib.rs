//! # Shared Secret 난독화 라이브러리
//!
//! 이 라이브러리는 컴파일 타임에 비밀값을 안전하게 난독화하고,
//! 런타임에 복호화하여 사용할 수 있는 기능을 제공합니다.

pub mod crypto;
pub mod secret;

#[cfg(test)]
mod tests;

pub use crypto::ObfuscatedData;
pub use secret::SecretString;

/// 고급 난독화 기능을 제공하는 매크로
#[macro_export]
macro_rules! secure_obfuscate {
    ($data:expr, $key1:expr, $key2:expr) => {{
        const DATA: &[u8] = $data.as_bytes();
        const KEY1: &[u8] = $key1.as_bytes();
        const KEY2: &[u8] = $key2.as_bytes();
        const LEN: usize = DATA.len();

        const fn multi_layer_encrypt() -> [u8; LEN] {
            let mut result = [0u8; LEN];
            let mut i = 0;
            while i < LEN {
                // 첫 번째 레이어: XOR with key1
                let temp = DATA[i] ^ KEY1[i % KEY1.len()];
                // 두 번째 레이어: XOR with key2 + position
                result[i] = temp ^ KEY2[i % KEY2.len()] ^ (i as u8);
                i += 1;
            }
            result
        }

        $crate::SecureObfuscatedData {
            data: multi_layer_encrypt(),
            key1_hash: const_hash(KEY1),
            key2_hash: const_hash(KEY2),
            len: LEN,
        }
    }};
}

/// 간단한 해시 함수 (컴파일 타임용)
const fn const_hash(input: &[u8]) -> u32 {
    let mut hash = 0x811c9dc5u32; // FNV offset basis
    let mut i = 0;
    while i < input.len() {
        hash ^= input[i] as u32;
        hash = hash.wrapping_mul(0x01000193); // FNV prime
        i += 1;
    }
    hash
}

/// 고급 보안 기능을 가진 난독화된 데이터
#[derive(Clone)]
pub struct SecureObfuscatedData<const N: usize> {
    pub data: [u8; N],
    pub key1_hash: u32,
    pub key2_hash: u32,
    pub len: usize,
}

impl<const N: usize> SecureObfuscatedData<N> {
    /// 이중 키를 사용하여 복호화
    pub fn reveal(&self, key1: &str, key2: &str) -> Option<SecretString> {
        // 키 검증
        if const_hash(key1.as_bytes()) != self.key1_hash
            || const_hash(key2.as_bytes()) != self.key2_hash
        {
            return None; // 잘못된 키
        }

        let key1_bytes = key1.as_bytes();
        let key2_bytes = key2.as_bytes();

        let mut decrypted = vec![0u8; self.len];
        for i in 0..self.len {
            // 역순으로 복호화
            let temp = self.data[i] ^ key2_bytes[i % key2_bytes.len()] ^ (i as u8);
            decrypted[i] = temp ^ key1_bytes[i % key1_bytes.len()];
        }

        match String::from_utf8(decrypted) {
            Ok(s) => Some(SecretString::new(s)),
            Err(_) => None,
        }
    }
}
