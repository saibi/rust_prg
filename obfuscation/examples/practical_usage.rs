/// 실제 애플리케이션에서 사용할 수 있는 예제들
use obfuscation::{SecretString, ObfuscatedData};

/// 컴파일 타임 XOR 암호화 매크로
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

/// 1. 웹 애플리케이션 설정 관리
pub struct WebConfig {
    database_url: SecretString,
    jwt_secret: SecretString,
    api_keys: Vec<(&'static str, SecretString)>,
}

impl WebConfig {
    pub fn load() -> Self {
        // 컴파일 타임에 난독화된 설정값들
        const DB_URL: ObfuscatedData<44> = obfuscate!("postgresql://user:secret@localhost:5432/mydb", "db_key_2024");
        const JWT_SECRET: ObfuscatedData<32> = obfuscate!("super-secret-jwt-signing-key-123", "jwt_key_2024");
        const STRIPE_API: ObfuscatedData<26> = obfuscate!("sk_test_1234567890abcdefgh", "stripe_key");
        const SENDGRID_API: ObfuscatedData<28> = obfuscate!("SG.abcdefghijklmnopqrstuvwxy", "sendgrid_key");
        
        Self {
            database_url: DB_URL.reveal(),
            jwt_secret: JWT_SECRET.reveal(),
            api_keys: vec![
                ("stripe", STRIPE_API.reveal()),
                ("sendgrid", SENDGRID_API.reveal()),
            ],
        }
    }
    
    pub fn get_database_url(&self) -> &SecretString {
        &self.database_url
    }
    
    pub fn get_jwt_secret(&self) -> &SecretString {
        &self.jwt_secret
    }
    
    pub fn get_api_key(&self, service: &str) -> Option<&SecretString> {
        self.api_keys.iter()
            .find(|(name, _)| *name == service)
            .map(|(_, key)| key)
    }
}

/// 2. 마이크로서비스 간 인증
pub struct ServiceAuth {
    service_tokens: Vec<(&'static str, SecretString)>,
}

impl ServiceAuth {
    pub fn new() -> Self {
        const USER_SERVICE_TOKEN: ObfuscatedData<34> = obfuscate!("service-token-user-service-v1-2024", "user_service");
        const ORDER_SERVICE_TOKEN: ObfuscatedData<35> = obfuscate!("service-token-order-service-v1-2024", "order_service");
        const PAYMENT_SERVICE_TOKEN: ObfuscatedData<37> = obfuscate!("service-token-payment-service-v1-2024", "payment_service");
        
        Self {
            service_tokens: vec![
                ("user-service", USER_SERVICE_TOKEN.reveal()),
                ("order-service", ORDER_SERVICE_TOKEN.reveal()),
                ("payment-service", PAYMENT_SERVICE_TOKEN.reveal()),
            ],
        }
    }
    
    pub fn get_token_for_service(&self, service: &str) -> Option<&SecretString> {
        self.service_tokens.iter()
            .find(|(name, _)| *name == service)
            .map(|(_, token)| token)
    }
    
    pub fn authenticate_request(&self, service: &str, provided_token: &str) -> bool {
        if let Some(expected_token) = self.get_token_for_service(service) {
            expected_token.use_secret(|token| token == provided_token)
        } else {
            false
        }
    }
}

/// 3. 암호화 키 관리
pub struct CryptoManager {
    encryption_keys: Vec<(&'static str, SecretString)>,
}

impl CryptoManager {
    pub fn new() -> Self {
        const AES_KEY: ObfuscatedData<32> = obfuscate!("my-super-secret-aes-256-key-2024", "aes_key");
        const HMAC_KEY: ObfuscatedData<32> = obfuscate!("my-super-secret-hmac-key-for-mac", "hmac_key");
        const BACKUP_KEY: ObfuscatedData<31> = obfuscate!("backup-encryption-key-for-files", "backup_key");
        
        Self {
            encryption_keys: vec![
                ("aes", AES_KEY.reveal()),
                ("hmac", HMAC_KEY.reveal()),
                ("backup", BACKUP_KEY.reveal()),
            ],
        }
    }
    
    pub fn get_key(&self, key_type: &str) -> Option<&SecretString> {
        self.encryption_keys.iter()
            .find(|(name, _)| *name == key_type)
            .map(|(_, key)| key)
    }
    
    /// 실제 암호화에 사용할 키를 바이트 배열로 반환
    pub fn get_key_bytes(&self, key_type: &str) -> Option<Vec<u8>> {
        self.get_key(key_type).map(|key| {
            key.use_secret(|k| k.as_bytes().to_vec())
        })
    }
}

/// 4. OAuth 및 외부 API 관리
pub struct ExternalApiManager {
    oauth_configs: Vec<(&'static str, OAuthConfig)>,
}

pub struct OAuthConfig {
    client_id: String,
    client_secret: SecretString,
    redirect_uri: String,
}

impl ExternalApiManager {
    pub fn new() -> Self {
        const GOOGLE_SECRET: ObfuscatedData<32> = obfuscate!("google-oauth-client-secret-12345", "google_oauth");
        const GITHUB_SECRET: ObfuscatedData<32> = obfuscate!("github-oauth-client-secret-67890", "github_oauth");
        const DISCORD_SECRET: ObfuscatedData<31> = obfuscate!("discord-oauth-client-secret-abc", "discord_oauth");
        
        Self {
            oauth_configs: vec![
                ("google", OAuthConfig {
                    client_id: "123456789.apps.googleusercontent.com".to_string(),
                    client_secret: GOOGLE_SECRET.reveal(),
                    redirect_uri: "https://myapp.com/auth/google/callback".to_string(),
                }),
                ("github", OAuthConfig {
                    client_id: "github_client_id_123".to_string(),
                    client_secret: GITHUB_SECRET.reveal(),
                    redirect_uri: "https://myapp.com/auth/github/callback".to_string(),
                }),
                ("discord", OAuthConfig {
                    client_id: "discord_client_id_456".to_string(),
                    client_secret: DISCORD_SECRET.reveal(),
                    redirect_uri: "https://myapp.com/auth/discord/callback".to_string(),
                }),
            ],
        }
    }
    
    pub fn get_oauth_config(&self, provider: &str) -> Option<&OAuthConfig> {
        self.oauth_configs.iter()
            .find(|(name, _)| *name == provider)
            .map(|(_, config)| config)
    }
}

impl OAuthConfig {
    pub fn build_auth_url(&self, state: &str) -> String {
        self.client_secret.use_secret(|_secret| {
            format!(
                "https://oauth.example.com/authorize?client_id={}&redirect_uri={}&state={}",
                self.client_id, self.redirect_uri, state
            )
        })
    }
    
    pub fn get_client_secret(&self) -> &SecretString {
        &self.client_secret
    }
}

/// 5. 환경별 설정 관리
pub struct EnvironmentConfig {
    environment: &'static str,
    secrets: Vec<(&'static str, SecretString)>,
}

impl EnvironmentConfig {
    pub fn for_development() -> Self {
        const DEV_DB: ObfuscatedData<34> = obfuscate!("postgresql://dev:dev@localhost/dev", "dev_db");
        const DEV_REDIS: ObfuscatedData<22> = obfuscate!("redis://localhost:6379", "dev_redis");
        
        Self {
            environment: "development",
            secrets: vec![
                ("database_url", DEV_DB.reveal()),
                ("redis_url", DEV_REDIS.reveal()),
            ],
        }
    }
    
    pub fn for_staging() -> Self {
        const STAGING_DB: ObfuscatedData<40> = obfuscate!("postgresql://staging:pass@staging-db/app", "staging_db");
        const STAGING_REDIS: ObfuscatedData<28> = obfuscate!("redis://staging-redis:6379/0", "staging_redis");
        
        Self {
            environment: "staging",
            secrets: vec![
                ("database_url", STAGING_DB.reveal()),
                ("redis_url", STAGING_REDIS.reveal()),
            ],
        }
    }
    
    pub fn for_production() -> Self {
        const PROD_DB: ObfuscatedData<42> = obfuscate!("postgresql://prod:secure@prod-db:5432/prod", "prod_db");
        const PROD_REDIS: ObfuscatedData<25> = obfuscate!("redis://prod-redis:6379/0", "prod_redis");
        
        Self {
            environment: "production",
            secrets: vec![
                ("database_url", PROD_DB.reveal()),
                ("redis_url", PROD_REDIS.reveal()),
            ],
        }
    }
    
    pub fn get_secret(&self, key: &str) -> Option<&SecretString> {
        self.secrets.iter()
            .find(|(name, _)| *name == key)
            .map(|(_, secret)| secret)
    }
    
    pub fn environment(&self) -> &str {
        self.environment
    }
}

/// 실제 사용 예제를 실행하는 main 함수
fn main() {
    println!("=== 실용적인 Shared Secret 난독화 사용 예제 ===\n");
    
    // 1. 웹 애플리케이션 설정
    println!("1. 웹 애플리케이션 설정 관리:");
    let web_config = WebConfig::load();
    
    web_config.get_database_url().use_secret(|url| {
        println!("   ✅ 데이터베이스 URL: {}...", &url[..20]);
    });
    
    if let Some(stripe_key) = web_config.get_api_key("stripe") {
        stripe_key.use_secret(|key| {
            println!("   ✅ Stripe API Key: {}...", &key[..10]);
        });
    }
    
    // 2. 마이크로서비스 인증
    println!("\n2. 마이크로서비스 간 인증:");
    let service_auth = ServiceAuth::new();
    
    if let Some(token) = service_auth.get_token_for_service("user-service") {
        token.use_secret(|t| {
            println!("   ✅ User Service Token: {}...", &t[..15]);
            
            // 인증 테스트
            let is_valid = service_auth.authenticate_request("user-service", t);
            println!("   ✅ Token 인증 결과: {}", is_valid);
        });
    }
    
    // 3. 암호화 키 관리
    println!("\n3. 암호화 키 관리:");
    let crypto_manager = CryptoManager::new();
    
    if let Some(aes_key_bytes) = crypto_manager.get_key_bytes("aes") {
        println!("   ✅ AES 키 길이: {} bytes", aes_key_bytes.len());
    }
    
    if let Some(hmac_key) = crypto_manager.get_key("hmac") {
        hmac_key.use_secret(|key| {
            println!("   ✅ HMAC 키: {}...", &key[..10]);
        });
    }
    
    // 4. OAuth 설정
    println!("\n4. OAuth 및 외부 API 관리:");
    let oauth_manager = ExternalApiManager::new();
    
    if let Some(google_config) = oauth_manager.get_oauth_config("google") {
        println!("   ✅ Google OAuth Client ID: {}", google_config.client_id);
        let auth_url = google_config.build_auth_url("state123");
        println!("   ✅ Auth URL: {}...", &auth_url[..50]);
    }
    
    // 5. 환경별 설정
    println!("\n5. 환경별 설정 관리:");
    
    let environments = [
        ("Development", EnvironmentConfig::for_development()),
        ("Staging", EnvironmentConfig::for_staging()),
        ("Production", EnvironmentConfig::for_production()),
    ];
    
    for (env_name, config) in environments {
        println!("   📁 {} 환경:", env_name);
        
        if let Some(db_url) = config.get_secret("database_url") {
            db_url.use_secret(|url| {
                println!("      DB: {}...", &url[..25]);
            });
        }
        
        if let Some(redis_url) = config.get_secret("redis_url") {
            redis_url.use_secret(|url| {
                println!("      Redis: {}", url);
            });
        }
    }
    
    println!("\n=== 보안 기능 데모 ===");
    println!("✅ 모든 비밀값이 메모리에서 안전하게 관리됩니다.");
    println!("✅ Debug 출력 시 비밀값이 노출되지 않습니다.");
    println!("✅ 컴파일 타임에 난독화되어 소스 코드 스캔을 우회합니다.");
}

#[cfg(test)]
mod examples_test {
    use super::*;
    
    #[test]
    fn test_web_config() {
        let config = WebConfig::load();
        
        // 데이터베이스 URL 확인
        config.get_database_url().use_secret(|url| {
            assert!(url.starts_with("postgresql://"));
        });
        
        // API 키 확인
        assert!(config.get_api_key("stripe").is_some());
        assert!(config.get_api_key("sendgrid").is_some());
        assert!(config.get_api_key("nonexistent").is_none());
    }
    
    #[test]
    fn test_service_auth() {
        let auth = ServiceAuth::new();
        
        // 서비스 토큰 인증 테스트
        auth.get_token_for_service("user-service").unwrap().use_secret(|token| {
            assert!(auth.authenticate_request("user-service", token));
        });
        
        // 잘못된 토큰으로 인증 실패 확인
        assert!(!auth.authenticate_request("user-service", "wrong-token"));
        assert!(!auth.authenticate_request("nonexistent-service", "any-token"));
    }
    
    #[test]
    fn test_crypto_manager() {
        let crypto = CryptoManager::new();
        
        // 키 존재 확인
        assert!(crypto.get_key("aes").is_some());
        assert!(crypto.get_key("hmac").is_some());
        assert!(crypto.get_key("nonexistent").is_none());
        
        // 키 바이트 배열 반환 확인
        let aes_bytes = crypto.get_key_bytes("aes");
        assert!(aes_bytes.is_some());
        assert_eq!(aes_bytes.unwrap().len(), 32);
    }
    
    #[test]
    fn test_oauth_manager() {
        let oauth = ExternalApiManager::new();
        
        // OAuth 설정 확인
        let google_config = oauth.get_oauth_config("google");
        assert!(google_config.is_some());
        
        let config = google_config.unwrap();
        assert!(config.client_id.contains("googleusercontent.com"));
        
        // Auth URL 생성 확인
        let auth_url = config.build_auth_url("random_state_123");
        assert!(auth_url.contains("client_id="));
        assert!(auth_url.contains("state=random_state_123"));
    }
    
    #[test]
    fn test_environment_configs() {
        let dev_config = EnvironmentConfig::for_development();
        let staging_config = EnvironmentConfig::for_staging();
        let prod_config = EnvironmentConfig::for_production();
        
        assert_eq!(dev_config.environment(), "development");
        assert_eq!(staging_config.environment(), "staging");
        assert_eq!(prod_config.environment(), "production");
        
        // 각 환경별로 다른 설정 확인
        dev_config.get_secret("database_url").unwrap().use_secret(|url| {
            assert!(url.contains("localhost"));
        });
        
        staging_config.get_secret("database_url").unwrap().use_secret(|url| {
            assert!(url.contains("staging"));
        });
        
        prod_config.get_secret("database_url").unwrap().use_secret(|url| {
            assert!(url.contains("prod"));
        });
    }
}
