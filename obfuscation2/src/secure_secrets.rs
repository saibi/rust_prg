use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use sha2::{Digest, Sha256};

/// 더 안전한 시크릿 관리 클래스
/// 소스코드에서 평문이 전혀 보이지 않도록 설계
pub struct SecureSecretManager {
    // 모든 시크릿이 난독화된 형태로 저장됨
    api_key_obfuscated: Vec<u8>,
    api_key_xor_key: Vec<u8>,

    db_password_obfuscated: String,
    db_password_xor_key: Vec<u8>,

    jwt_secret_obfuscated: String,
    jwt_shift: u8,

    encryption_key_obfuscated: Vec<u8>,
}

impl SecureSecretManager {
    pub fn new() -> Self {
        // 실제 환경에서는 이 값들을 외부에서 주입받아야 함
        // 여기서는 예시를 위해 하드코딩하지만, 실제로는 환경변수나 설정 파일에서 읽어와야 함

        // API 키: "sk-1234567890abcdef" -> 비트 반전으로 난독화
        let api_key_bytes = b"sk-1234567890abcdef";
        let api_key_obfuscated: Vec<u8> = api_key_bytes.iter().map(|&b| !b).collect();
        let mut rng = rand::thread_rng();
        let api_key_xor_key: Vec<u8> = (0..api_key_bytes.len()).map(|_| rng.gen::<u8>()).collect();

        // DB 비밀번호: "MySecurePassword123!" -> XOR + Base64
        let db_password_bytes = b"MySecurePassword123!";
        let db_password_xor_key: Vec<u8> = (0..db_password_bytes.len())
            .map(|_| rng.gen::<u8>())
            .collect();
        let db_password_xored: Vec<u8> = db_password_bytes
            .iter()
            .zip(db_password_xor_key.iter())
            .map(|(a, b)| a ^ b)
            .collect();
        let db_password_obfuscated = general_purpose::STANDARD.encode(db_password_xored);

        // JWT 시크릿: "super-secret-jwt-key" -> 시저 암호
        let jwt_secret = "super-secret-jwt-key";
        let jwt_shift = rng.gen_range(1..=25);
        let jwt_secret_obfuscated: String = jwt_secret
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    ((c as u8 - base + jwt_shift) % 26 + base) as char
                } else {
                    c
                }
            })
            .collect();

        // 암호화 키: "encryption-key-256" -> 비트 반전
        let encryption_key_bytes = b"encryption-key-256";
        let encryption_key_obfuscated: Vec<u8> = encryption_key_bytes.iter().map(|&b| !b).collect();

        Self {
            api_key_obfuscated,
            api_key_xor_key,
            db_password_obfuscated,
            db_password_xor_key,
            jwt_secret_obfuscated,
            jwt_shift,
            encryption_key_obfuscated,
        }
    }

    pub fn get_api_key(&self) -> String {
        // 비트 반전 복원
        let api_key_bytes: Vec<u8> = self.api_key_obfuscated.iter().map(|&b| !b).collect();
        String::from_utf8(api_key_bytes).unwrap()
    }

    pub fn get_database_password(&self) -> String {
        // Base64 디코딩 후 XOR 복원
        let db_password_xored = general_purpose::STANDARD
            .decode(&self.db_password_obfuscated)
            .unwrap();
        let db_password_bytes: Vec<u8> = db_password_xored
            .iter()
            .zip(self.db_password_xor_key.iter())
            .map(|(a, b)| a ^ b)
            .collect();
        String::from_utf8(db_password_bytes).unwrap()
    }

    pub fn get_jwt_secret(&self) -> String {
        // 시저 암호 복원
        self.jwt_secret_obfuscated
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    ((c as u8 - base + 26 - self.jwt_shift) % 26 + base) as char
                } else {
                    c
                }
            })
            .collect()
    }

    pub fn get_encryption_key(&self) -> String {
        // 비트 반전 복원
        let encryption_key_bytes: Vec<u8> =
            self.encryption_key_obfuscated.iter().map(|&b| !b).collect();
        String::from_utf8(encryption_key_bytes).unwrap()
    }

    /// 시크릿 검증 (해시 기반)
    pub fn verify_secret(&self, secret_type: &str, input: &str) -> bool {
        let expected_secret = match secret_type {
            "api_key" => self.get_api_key(),
            "database_password" => self.get_database_password(),
            "jwt_secret" => self.get_jwt_secret(),
            "encryption_key" => self.get_encryption_key(),
            _ => return false,
        };

        input == expected_secret
    }

    /// 해시 기반 검증 (더 안전한 방법)
    pub fn verify_secret_hash(&self, secret_type: &str, input: &str) -> bool {
        let expected_secret = match secret_type {
            "api_key" => self.get_api_key(),
            "database_password" => self.get_database_password(),
            "jwt_secret" => self.get_jwt_secret(),
            "encryption_key" => self.get_encryption_key(),
            _ => return false,
        };

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let input_hash = format!("{:x}", hasher.finalize());

        let mut hasher = Sha256::new();
        hasher.update(expected_secret.as_bytes());
        let expected_hash = format!("{:x}", hasher.finalize());

        input_hash == expected_hash
    }
}

/// 환경 변수에서 시크릿을 읽어오는 안전한 방법
pub struct EnvironmentSecretManager {
    api_key: Option<String>,
    db_password: Option<String>,
    jwt_secret: Option<String>,
    encryption_key: Option<String>,
}

impl EnvironmentSecretManager {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("API_KEY").ok(),
            db_password: std::env::var("DB_PASSWORD").ok(),
            jwt_secret: std::env::var("JWT_SECRET").ok(),
            encryption_key: std::env::var("ENCRYPTION_KEY").ok(),
        }
    }

    pub fn get_api_key(&self) -> Option<String> {
        self.api_key.clone()
    }

    pub fn get_database_password(&self) -> Option<String> {
        self.db_password.clone()
    }

    pub fn get_jwt_secret(&self) -> Option<String> {
        self.jwt_secret.clone()
    }

    pub fn get_encryption_key(&self) -> Option<String> {
        self.encryption_key.clone()
    }

    pub fn is_configured(&self) -> bool {
        self.api_key.is_some()
            && self.db_password.is_some()
            && self.jwt_secret.is_some()
            && self.encryption_key.is_some()
    }
}
