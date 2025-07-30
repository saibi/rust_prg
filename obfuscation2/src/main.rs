use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use sha2::{Digest, Sha256};

mod secret_manager;
mod secure_secrets;
use secret_manager::SecretManager;
use secure_secrets::{EnvironmentSecretManager, SecureSecretManager};

// 방법 1: XOR 난독화를 사용한 시크릿 보호
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

// 방법 2: Base64 + XOR 이중 난독화
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

// 방법 3: 간단한 시저 암호를 사용한 시크릿 보호
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

                    ((c as u8 - base + shift) % 26 + base) as char
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

                    ((c as u8 - base + 26 - self.shift) % 26 + base) as char
                } else {
                    c
                }
            })
            .collect()
    }
}

// 방법 4: SHA256 해시를 사용한 시크릿 검증
struct HashVerifiedSecret {
    secret_hash: String,
    secret: String,
}

impl HashVerifiedSecret {
    fn new(secret: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        let secret_hash = hex::encode(hasher.finalize());

        Self {
            secret_hash,
            secret: secret.to_string(),
        }
    }

    fn verify_and_get_secret(&self, input_secret: &str) -> Option<String> {
        let mut hasher = Sha256::new();
        hasher.update(input_secret.as_bytes());
        let input_hash = hex::encode(hasher.finalize());

        if input_hash == self.secret_hash {
            Some(self.secret.clone())
        } else {
            None
        }
    }
}

// 방법 5: 문자열 분할 및 재배열을 통한 난독화
struct SplitObfuscatedSecret {
    parts: Vec<String>,
    indices: Vec<usize>,
}

impl SplitObfuscatedSecret {
    fn new(secret: &str) -> Self {
        let secret_bytes = secret.as_bytes();
        let mut rng = rand::thread_rng();

        // 문자열을 랜덤하게 분할
        let mut parts = Vec::new();
        let mut indices = Vec::new();
        let mut current_pos = 0;

        while current_pos < secret_bytes.len() {
            let chunk_size = rng.gen_range(1..=std::cmp::min(4, secret_bytes.len() - current_pos));
            let chunk = &secret_bytes[current_pos..current_pos + chunk_size];
            parts.push(hex::encode(chunk));
            indices.push(current_pos);
            current_pos += chunk_size;
        }

        Self { parts, indices }
    }

    fn get_secret(&self) -> String {
        let mut secret_bytes = Vec::new();

        for (part, &_index) in self.parts.iter().zip(self.indices.iter()) {
            let bytes = hex::decode(part).unwrap();
            secret_bytes.extend_from_slice(&bytes);
        }

        String::from_utf8(secret_bytes).unwrap()
    }
}

// 방법 6: 비트 반전을 통한 난독화
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

fn main() {
    println!("=== 공유 시크릿 난독화 예제 ===\n");

    // 원본 시크릿을 난독화된 형태로 저장 (소스코드에서 평문 노출 방지)
    let obfuscated_secret_data = [
        146, 134, 160, 140, 138, 143, 154, 141, 160, 140, 154, 156, 141, 154, 139, 160, 148, 154,
        134, 160, 206, 205, 204, 203, 202,
    ];
    let original_secret =
        String::from_utf8(obfuscated_secret_data.iter().map(|&b| !b).collect()).unwrap();
    println!("원본 시크릿: {original_secret}");

    // 방법 1: XOR 난독화
    println!("\n--- 방법 1: XOR 난독화 ---");
    let xor_secret = XorObfuscatedSecret::new(&original_secret);
    println!("난독화된 데이터: {:?}", xor_secret.obfuscated_data);
    println!("XOR 키: {:?}", xor_secret.xor_key);
    println!("복원된 시크릿: {}", xor_secret.get_secret());

    // 방법 2: Base64 + XOR 이중 난독화
    println!("\n--- 방법 2: Base64 + XOR 이중 난독화 ---");
    let double_secret = DoubleObfuscatedSecret::new(&original_secret);
    println!("난독화된 데이터: {}", double_secret.obfuscated_data);
    println!("XOR 키: {:?}", double_secret.xor_key);
    println!("복원된 시크릿: {}", double_secret.get_secret());

    // 방법 3: 시저 암호
    println!("\n--- 방법 3: 시저 암호 ---");
    let caesar_secret = CaesarObfuscatedSecret::new(&original_secret);
    println!("난독화된 데이터: {}", caesar_secret.obfuscated_data);
    println!("시프트 값: {}", caesar_secret.shift);
    println!("복원된 시크릿: {}", caesar_secret.get_secret());

    // 방법 4: SHA256 해시 검증
    println!("\n--- 방법 4: SHA256 해시 검증 ---");
    let hash_secret = HashVerifiedSecret::new(&original_secret);
    println!("시크릿 해시: {}", hash_secret.secret_hash);

    match hash_secret.verify_and_get_secret(&original_secret) {
        Some(secret) => println!("검증 성공! 시크릿: {secret}"),
        None => println!("검증 실패!"),
    }

    match hash_secret.verify_and_get_secret("wrong_secret") {
        Some(secret) => println!("검증 성공! 시크릿: {secret}"),
        None => println!("잘못된 시크릿 검증 실패!"),
    }

    // 방법 5: 문자열 분할 난독화
    println!("\n--- 방법 5: 문자열 분할 난독화 ---");
    let split_secret = SplitObfuscatedSecret::new(&original_secret);
    println!("분할된 부분들: {:?}", split_secret.parts);
    println!("인덱스: {:?}", split_secret.indices);
    println!("복원된 시크릿: {}", split_secret.get_secret());

    // 방법 6: 비트 반전 난독화
    println!("\n--- 방법 6: 비트 반전 난독화 ---");
    let bitflip_secret = BitFlipObfuscatedSecret::new(&original_secret);
    println!("난독화된 데이터: {:?}", bitflip_secret.obfuscated_data);
    println!("복원된 시크릿: {}", bitflip_secret.get_secret());

    println!("\n=== 모든 방법이 성공적으로 시크릿을 보호하고 복원했습니다! ===");

    // 실제 사용 예제
    println!("\n\n=== 실제 애플리케이션 사용 예제 ===");
    let secret_manager = SecretManager::new();

    println!("\n--- API 키 사용 ---");
    let api_key = secret_manager.get_api_key();
    println!("API 키: {api_key}");
    println!(
        "API 키 검증: {}",
        secret_manager.verify_secret("api_key", &api_key)
    );

    println!("\n--- 데이터베이스 비밀번호 사용 ---");
    let db_password = secret_manager.get_database_password();
    println!("DB 비밀번호: {db_password}");
    println!(
        "DB 비밀번호 검증: {}",
        secret_manager.verify_secret("database_password", &db_password)
    );

    println!("\n--- JWT 시크릿 사용 ---");
    let jwt_secret = secret_manager.get_jwt_secret();
    println!("JWT 시크릿: {jwt_secret}");
    println!(
        "JWT 시크릿 검증: {}",
        secret_manager.verify_secret("jwt_secret", &jwt_secret)
    );

    println!("\n--- 암호화 키 사용 ---");
    let encryption_key = secret_manager.get_encryption_key();
    println!("암호화 키: {encryption_key}");
    println!(
        "암호화 키 검증: {}",
        secret_manager.verify_secret("encryption_key", &encryption_key)
    );

    println!("\n=== 실제 애플리케이션에서 안전하게 시크릿을 사용할 수 있습니다! ===");

    // 더 안전한 시크릿 관리 방법 예제
    println!("\n\n=== 더 안전한 시크릿 관리 방법 ===");

    println!("\n--- 방법 1: 완전히 난독화된 시크릿 매니저 ---");
    let secure_manager = SecureSecretManager::new();

    println!("API 키: {}", secure_manager.get_api_key());
    println!("DB 비밀번호: {}", secure_manager.get_database_password());
    println!("JWT 시크릿: {}", secure_manager.get_jwt_secret());
    println!("암호화 키: {}", secure_manager.get_encryption_key());

    // 해시 기반 검증
    println!("\n--- 해시 기반 검증 ---");
    let test_api_key = secure_manager.get_api_key();
    println!(
        "API 키 해시 검증: {}",
        secure_manager.verify_secret_hash("api_key", &test_api_key)
    );
    println!(
        "잘못된 API 키 검증: {}",
        secure_manager.verify_secret_hash("api_key", "wrong_key")
    );

    println!("\n--- 방법 2: 환경 변수 기반 시크릿 관리 ---");
    let env_manager = EnvironmentSecretManager::new();

    if env_manager.is_configured() {
        println!("환경 변수가 모두 설정되어 있습니다.");
        if let Some(api_key) = env_manager.get_api_key() {
            println!("환경 변수 API 키: {}", api_key);
        }
    } else {
        println!("환경 변수가 설정되지 않았습니다. 다음 환경 변수를 설정하세요:");
        println!("  API_KEY=your_api_key");
        println!("  DB_PASSWORD=your_db_password");
        println!("  JWT_SECRET=your_jwt_secret");
        println!("  ENCRYPTION_KEY=your_encryption_key");
    }

    println!("\n=== 보안 강화된 시크릿 관리 방법을 사용하세요! ===");
}
