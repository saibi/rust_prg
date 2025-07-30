use std::fmt;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// 컴파일 타임에 XOR 암호화를 수행하는 매크로
macro_rules! obfuscate {
    ($data:expr, $key:expr) => {{
        const DATA: &[u8] = $data.as_bytes();
        const LEN: usize = DATA.len();
        const KEY_PARTS: [u8; 4] = [
            $key.as_bytes()[0],
            $key.as_bytes()[1 % $key.len()],
            $key.as_bytes()[2 % $key.len()],
            $key.as_bytes()[3 % $key.len()],
        ];

        const fn xor_encrypt() -> [u8; LEN] {
            let mut result = [0u8; LEN];
            let mut i = 0;
            while i < LEN {
                result[i] = DATA[i] ^ KEY_PARTS[i % KEY_PARTS.len()];
                i += 1;
            }
            result
        }

        ObfuscatedData {
            data: xor_encrypt(),
            key_parts: [
                $key.as_bytes()[0],
                $key.as_bytes()[1 % $key.len()],
                $key.as_bytes()[2 % $key.len()],
                $key.as_bytes()[3 % $key.len()],
            ],
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
            decrypted[i] = self.data[i] ^ self.key_parts[i % self.key_parts.len()];
        });

        SecretString::new(String::from_utf8(decrypted).unwrap_or_default())
    }
}

fn print_xor_constant(secret: &str, key: &str) {
    print!("const XOR_CONSTANT: [u8; {}] = [", secret.len());
    (0..secret.len()).for_each(|i| {
        print!("{}, ", secret.as_bytes()[i] ^ key.as_bytes()[i % key.len()]);
    });
    println!("];");
}

fn decrypt_xor_constant(xor_constant: &[u8], key: &str) -> String {
    let mut decrypted = vec![0u8; xor_constant.len()];
    (0..xor_constant.len()).for_each(|i| {
        decrypted[i] = xor_constant[i] ^ key.as_bytes()[i % key.len()];
    });
    String::from_utf8(decrypted).unwrap_or_default()
}

fn main() {
    print_xor_constant("hello world", "yourkey");

    const XOR_CONSTANT: [u8; 11] = [17, 10, 25, 30, 4, 69, 14, 22, 29, 25, 22];
    let decrypted = decrypt_xor_constant(&XOR_CONSTANT, "yourkey");
    println!("decrypted: [{decrypted}]");

    // 1. 컴파일 타임에 난독화된 API 키들
    const OBFUSCATED_API_KEY: ObfuscatedData<35> =
        obfuscate!("sk-1234567890abcdef1234567890abcdef", "mykey");

    let api_key = OBFUSCATED_API_KEY.reveal();
    println!("api_key len: {}", api_key.len());
    api_key.use_secret(|s| {
        println!("secret {}", &s);
    });
}
