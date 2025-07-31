// 빌드 스크립트에서 생성된 랜덤 키 포함
include!(concat!(env!("OUT_DIR"), "/random_key.rs"));

use std::fmt;
use zeroize::{Zeroize, ZeroizeOnDrop};

const XOR_KEY: u8 = 0xAA;

/// 컴파일 타임에 XOR 암호화를 수행하는 매크로 (자동 랜덤 키 사용)
macro_rules! obfuscate {
    ($data:expr) => {{
        const DATA: &[u8] = $data;
        const LEN: usize = DATA.len();
        const KEY_PARTS: [u8; 4] = RANDOM_KEY;

        const fn xor_encrypt() -> [u8; LEN] {
            let mut result = [0u8; LEN];
            let mut i = 0;
            while i < LEN {
                // result[i] = DATA[i] ^ KEY_PARTS[i % KEY_PARTS.len()];
                result[i] = DATA[i];
                i += 1;
            }
            result
        }

        ObfuscatedData {
            data: xor_encrypt(),
            key_parts: KEY_PARTS,
            len: LEN,
        }
    }};
}

/// 메모리에서 자동으로 지워지는 비밀 문자열
#[derive(Clone)]
pub struct SecretString {
    inner: String,
}

impl SecretString {
    pub fn new(s: String) -> Self {
        Self { inner: s }
    }

    /// 비밀 문자열을 안전하게 사용하기 위한 함수
    pub fn use_secret<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        f(&self.inner)
    }

    /// 비밀 문자열의 길이 반환
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 비밀 문자열이 비어있는지 확인
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// 바이트로 변환 (주의: 사용 후 즉시 지워짐)
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretString([REDACTED])")
    }
}

impl fmt::Display for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl Drop for SecretString {
    fn drop(&mut self) {
        // 메모리에서 비밀 문자열 지우기
        unsafe {
            let bytes = self.inner.as_mut_vec();
            bytes.zeroize();
        }
    }
}

impl ZeroizeOnDrop for SecretString {}
/// 기본 XOR 난독화를 위한 구조체
#[derive(Clone)]
pub struct ObfuscatedData<const N: usize> {
    pub data: [u8; N],
    pub key_parts: [u8; 4],
    pub len: usize,
}

impl<const N: usize> ObfuscatedData<N> {
    /// 난독화된 데이터를 복호화하여 반환
    pub fn reveal(&self) -> SecretString {
        let mut decrypted = vec![0u8; self.len];
        (0..self.len).for_each(|i| {
            decrypted[i] = self.data[i] ^ XOR_KEY;
            // decrypted[i] = self.data[i] ^ self.key_parts[i % self.key_parts.len()] ^ XOR_KEY;
        });

        SecretString::new(String::from_utf8(decrypted).unwrap_or_default())
    }
}

fn main() {
    const XOR_CONSTANT: [u8; 14] = [
        155, 152, 153, 158, 159, 156, 157, 146, 147, 203, 200, 201, 206, 207,
    ];

    let obfuscated_api_key = obfuscate!(&XOR_CONSTANT);

    let api_key = obfuscated_api_key.reveal();
    println!("len: {}", api_key.len());
    // api_key.use_secret(|s| {
    //     println!("value: [{}]", &s);
    // });
}
