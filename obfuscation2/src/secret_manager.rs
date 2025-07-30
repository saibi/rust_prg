use base64::{engine::general_purpose, Engine as _};
use rand::Rng;

/// 실제 애플리케이션에서 사용할 수 있는 시크릿 매니저
pub struct SecretManager {
    api_key: XorObfuscatedSecret,
    database_password: DoubleObfuscatedSecret,
    jwt_secret: CaesarObfuscatedSecret,
    encryption_key: BitFlipObfuscatedSecret,
}

impl SecretManager {
    pub fn new() -> Self {
        // 실제 환경에서는 환경 변수나 설정 파일에서 읽어와야 함
        Self {
            api_key: XorObfuscatedSecret::new("sk-1234567890abcdef"),
            database_password: DoubleObfuscatedSecret::new("MySecurePassword123!"),
            jwt_secret: CaesarObfuscatedSecret::new("super-secret-jwt-key"),
            encryption_key: BitFlipObfuscatedSecret::new("encryption-key-256"),
        }
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.get_secret()
    }

    pub fn get_database_password(&self) -> String {
        self.database_password.get_secret()
    }

    pub fn get_jwt_secret(&self) -> String {
        self.jwt_secret.get_secret()
    }

    pub fn get_encryption_key(&self) -> String {
        self.encryption_key.get_secret()
    }

    /// 시크릿이 올바른지 검증
    pub fn verify_secret(&self, secret_type: &str, input: &str) -> bool {
        match secret_type {
            "api_key" => self.api_key.get_secret() == input,
            "database_password" => self.database_password.get_secret() == input,
            "jwt_secret" => self.jwt_secret.get_secret() == input,
            "encryption_key" => self.encryption_key.get_secret() == input,
            _ => false,
        }
    }
}

// XOR 난독화를 사용한 시크릿 보호
struct XorObfuscatedSecret {
    obfuscated_data: Vec<u8>,
    xor_key: Vec<u8>,
}

impl XorObfuscatedSecret {
    fn new(secret: &str) -> Self {
        let secret_bytes = secret.as_bytes();
        let mut rng = rand::thread_rng();

        // 랜덤 XOR 키 생성
        let xor_key: Vec<u8> = (0..secret_bytes.len()).map(|_| rng.gen::<u8>()).collect();

        // XOR로 난독화
        let obfuscated_data: Vec<u8> = secret_bytes
            .iter()
            .zip(xor_key.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        Self {
            obfuscated_data,
            xor_key,
        }
    }

    fn get_secret(&self) -> String {
        let secret_bytes: Vec<u8> = self
            .obfuscated_data
            .iter()
            .zip(self.xor_key.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        String::from_utf8(secret_bytes).unwrap()
    }
}

// Base64 + XOR 이중 난독화
struct DoubleObfuscatedSecret {
    obfuscated_data: String,
    xor_key: Vec<u8>,
}

impl DoubleObfuscatedSecret {
    fn new(secret: &str) -> Self {
        let secret_bytes = secret.as_bytes();
        let mut rng = rand::thread_rng();

        // 랜덤 XOR 키 생성
        let xor_key: Vec<u8> = (0..secret_bytes.len()).map(|_| rng.gen::<u8>()).collect();

        // XOR로 난독화 후 Base64 인코딩
        let obfuscated_bytes: Vec<u8> = secret_bytes
            .iter()
            .zip(xor_key.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        let obfuscated_data = general_purpose::STANDARD.encode(obfuscated_bytes);

        Self {
            obfuscated_data,
            xor_key,
        }
    }

    fn get_secret(&self) -> String {
        let obfuscated_bytes = general_purpose::STANDARD
            .decode(&self.obfuscated_data)
            .unwrap();

        let secret_bytes: Vec<u8> = obfuscated_bytes
            .iter()
            .zip(self.xor_key.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        String::from_utf8(secret_bytes).unwrap()
    }
}

// 시저 암호를 사용한 시크릿 보호
struct CaesarObfuscatedSecret {
    obfuscated_data: String,
    shift: u8,
}

impl CaesarObfuscatedSecret {
    fn new(secret: &str) -> Self {
        let mut rng = rand::thread_rng();
        let shift = rng.gen_range(1..=25);

        let obfuscated_data: String = secret
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let shifted = ((c as u8 - base + shift) % 26 + base) as char;
                    shifted
                } else {
                    c
                }
            })
            .collect();

        Self {
            obfuscated_data,
            shift,
        }
    }

    fn get_secret(&self) -> String {
        self.obfuscated_data
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let shifted = ((c as u8 - base + 26 - self.shift) % 26 + base) as char;
                    shifted
                } else {
                    c
                }
            })
            .collect()
    }
}

// 비트 반전을 통한 난독화
struct BitFlipObfuscatedSecret {
    obfuscated_data: Vec<u8>,
}

impl BitFlipObfuscatedSecret {
    fn new(secret: &str) -> Self {
        let secret_bytes = secret.as_bytes();

        // 각 바이트의 비트를 반전
        let obfuscated_data: Vec<u8> = secret_bytes.iter().map(|&b| !b).collect();

        Self { obfuscated_data }
    }

    fn get_secret(&self) -> String {
        let secret_bytes: Vec<u8> = self.obfuscated_data.iter().map(|&b| !b).collect();

        String::from_utf8(secret_bytes).unwrap()
    }
}
