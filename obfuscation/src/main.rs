use obfuscation::{ObfuscatedData, SecretString, SecureObfuscatedData, secure_obfuscate};

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

/// 컴파일 타임에 XOR 암호화를 수행하는 매크로
macro_rules! obfuscate {
    ($data:expr, $key:expr) => {{
        const DATA: &[u8] = $data.as_bytes();
        const KEY: &[u8] = $key.as_bytes();
        const LEN: usize = DATA.len();

        const fn xor_encrypt() -> [u8; LEN] {
            let mut result = [0u8; LEN];
            let mut i = 0;
            while i < LEN {
                result[i] = DATA[i] ^ KEY[i % KEY.len()];
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

/// API 키나 토큰을 사용하는 예제 구조체
pub struct ApiClient {
    api_key: SecretString,
    base_url: String,
}

impl ApiClient {
    pub fn new(api_key: SecretString, base_url: String) -> Self {
        Self { api_key, base_url }
    }

    /// API 호출 시뮬레이션
    pub fn make_request(&self, endpoint: &str) -> Result<String, &'static str> {
        // 실제로는 HTTP 클라이언트를 사용하겠지만, 여기서는 시뮬레이션
        self.api_key.use_secret(|key| {
            println!("Making request to: {}{}", self.base_url, endpoint);
            println!("Using API key: {}...", &key[..4.min(key.len())]);
            Ok(format!("Response from {endpoint}"))
        })
    }
}

fn main() {
    println!("=== Shared Secret 난독화 예제 ===\n");

    // 1. 컴파일 타임에 난독화된 API 키들
    const OBFUSCATED_API_KEY: ObfuscatedData<35> =
        obfuscate!("sk-1234567890abcdef1234567890abcdef", "mysecretkey");
    const OBFUSCATED_DB_PASSWORD: ObfuscatedData<17> = obfuscate!("super_secret_pass", "dbkey123");
    const OBFUSCATED_JWT_SECRET: ObfuscatedData<25> =
        obfuscate!("jwt_signing_secret_key_42", "jwtkey");

    // 2. 런타임에 복호화하여 사용
    let api_key = OBFUSCATED_API_KEY.reveal();
    let db_password = OBFUSCATED_DB_PASSWORD.reveal();
    let jwt_secret = OBFUSCATED_JWT_SECRET.reveal();

    println!("✅ 비밀값들이 성공적으로 복호화되었습니다.");
    println!("API Key 길이: {}", api_key.len());
    println!("DB Password 길이: {}", db_password.len());
    println!("JWT Secret 길이: {}", jwt_secret.len());
    println!();

    // 3. API 클라이언트 생성 및 사용
    let client = ApiClient::new(api_key, "https://api.example.com".to_string());

    match client.make_request("/users") {
        Ok(response) => println!("✅ API 응답: {response}"),
        Err(e) => println!("❌ API 오류: {e}"),
    }
    println!();

    // 4. 데이터베이스 연결 시뮬레이션
    db_password.use_secret(|password| {
        println!(
            "✅ 데이터베이스 연결 성공 (패스워드 길이: {})",
            password.len()
        );
        // 실제로는 여기서 데이터베이스 연결을 설정
    });

    // 5. JWT 토큰 서명 시뮬레이션
    jwt_secret.use_secret(|secret| {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(b"sample_payload");
        hasher.update(secret.as_bytes());
        let signature = hasher.finalize();
        println!("✅ JWT 토큰 서명 생성됨: {signature:x}");
    });

    println!("\n=== 고급 보안 난독화 예제 ===");

    // 6. 이중 키 보안 (더 강력한 난독화)
    const SECURE_API_TOKEN: SecureObfuscatedData<22> =
        secure_obfuscate!("super-secret-api-token", "key1", "key2");

    // 올바른 키로 복호화
    if let Some(token) = SECURE_API_TOKEN.reveal("key1", "key2") {
        println!("✅ 고급 보안 토큰 복호화 성공");
        token.use_secret(|t| println!("토큰 길이: {}", t.len()));
    } else {
        println!("❌ 고급 보안 토큰 복호화 실패");
    }

    // 잘못된 키로 복호화 시도
    if let Some(_) = SECURE_API_TOKEN.reveal("wrong", "keys") {
        println!("❌ 보안 오류: 잘못된 키로 복호화됨!");
    } else {
        println!("✅ 보안 검증: 잘못된 키는 거부됨");
    }

    println!("\n=== 실제 사용 사례 예제 ===");

    // 7. 설정 관리자 예제
    struct ConfigManager {
        encrypted_configs: Vec<(&'static str, SecretString)>,
    }

    impl ConfigManager {
        fn new() -> Self {
            let db_url = obfuscate!("postgresql://user:pass@localhost/db", "dbkey").reveal();
            let redis_url = obfuscate!("redis://localhost:6379", "rediskey").reveal();
            let s3_key = obfuscate!("AKIAIOSFODNN7EXAMPLE", "s3key").reveal();

            Self {
                encrypted_configs: vec![
                    ("database_url", db_url),
                    ("redis_url", redis_url),
                    ("s3_access_key", s3_key),
                ],
            }
        }

        fn get_config(&self, key: &str) -> Option<&SecretString> {
            self.encrypted_configs
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| v)
        }
    }

    let config = ConfigManager::new();

    if let Some(db_url) = config.get_config("database_url") {
        db_url.use_secret(|url| {
            println!("✅ 데이터베이스 연결: {}...", &url[..12]);
        });
    }

    if let Some(s3_key) = config.get_config("s3_access_key") {
        s3_key.use_secret(|key| {
            println!("✅ S3 액세스 키: {}...", &key[..4]);
        });
    }

    println!("\n=== 보안 기능 데모 ===");

    // 8. 디버그 출력 확인 (비밀값이 노출되지 않음) - Clone으로 해결
    let api_key_clone = OBFUSCATED_API_KEY.reveal();
    println!("Debug - API Key: {api_key_clone:?}");
    println!("Display - DB Password: {db_password}");

    // 9. 스코프 종료시 자동으로 메모리에서 지워짐
    println!("✅ 프로그램 종료 시 모든 비밀값이 메모리에서 자동으로 지워집니다.");
}
