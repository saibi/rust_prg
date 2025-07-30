# Rust Shared Secret 난독화 라이브러리

컴파일 타임에 비밀값을 안전하게 난독화하고, 런타임에 복호화하여 사용할 수 있는 Rust 라이브러리입니다.

## 🔒 주요 기능

- **컴파일 타임 난독화**: 소스 코드에서 비밀값이 평문으로 노출되지 않음
- **자동 메모리 클리어**: 사용 후 메모리에서 자동으로 비밀값 제거
- **디버그 보호**: Debug/Display 출력 시 비밀값이 노출되지 않음
- **이중 키 보안**: 고급 보안을 위한 다중 레이어 암호화
- **타입 안전성**: Rust의 타입 시스템을 활용한 안전한 API

## 🚀 사용 방법

### 1. 기본 난독화

```rust
use obfuscation::{SecretString, ObfuscatedData};

// 컴파일 타임에 XOR 암호화를 수행하는 매크로
macro_rules! obfuscate {
    ($data:expr, $key:expr) => {{
        // 실제 구현은 라이브러리 참조
    }};
}

fn main() {
    // 컴파일 타임에 난독화된 API 키
    const API_KEY: ObfuscatedData<35> = obfuscate!("sk-1234567890abcdef1234567890abcdef", "secretkey");
    
    // 런타임에 복호화하여 사용
    let api_key = API_KEY.reveal();
    
    // 안전하게 사용
    api_key.use_secret(|key| {
        println!("API 키 길이: {}", key.len());
        // 실제 API 호출
    });
    
    // 자동으로 메모리에서 지워짐
}
```

### 2. 고급 보안 (이중 키)

```rust
use obfuscation::{SecureObfuscatedData, secure_obfuscate};

fn main() {
    // 이중 키를 사용한 고급 보안
    const SECURE_TOKEN: SecureObfuscatedData<22> = secure_obfuscate!(
        "super-secret-api-token", 
        "primary_key", 
        "secondary_key"
    );
    
    // 두 키 모두 일치해야 복호화 가능
    if let Some(token) = SECURE_TOKEN.reveal("primary_key", "secondary_key") {
        token.use_secret(|t| {
            // 안전하게 사용
        });
    }
    
    // 잘못된 키로는 복호화 불가
    assert!(SECURE_TOKEN.reveal("wrong", "keys").is_none());
}
```

### 3. 실제 애플리케이션 예제

```rust
struct DatabaseConfig {
    url: SecretString,
    password: SecretString,
}

impl DatabaseConfig {
    fn new() -> Self {
        Self {
            url: obfuscate!("postgresql://user:pass@localhost/db", "dbkey").reveal(),
            password: obfuscate!("super_secret_password", "passkey").reveal(),
        }
    }
    
    fn connect(&self) -> Result<Connection, Error> {
        self.url.use_secret(|url| {
            self.password.use_secret(|pass| {
                // 실제 데이터베이스 연결
                Database::connect(url, pass)
            })
        })
    }
}
```

## 🛡️ 보안 기능

### 1. 메모리 자동 클리어
```rust
{
    let secret = obfuscate!("sensitive_data", "key").reveal();
    // 스코프 종료 시 자동으로 메모리에서 지워짐
} // <- 여기서 메모리 클리어
```

### 2. 디버그 출력 보호
```rust
let secret = obfuscate!("api_key", "key").reveal();
println!("{:?}", secret); // "SecretString([REDACTED])" 출력
println!("{}", secret);   // "[REDACTED]" 출력
```

### 3. 타입 안전성
```rust
// 컴파일 타임에 크기 검증
const WRONG_SIZE: ObfuscatedData<10> = obfuscate!("toolongstring", "key"); // 컴파일 오류
```

## 📦 의존성

```toml
[dependencies]
zeroize = "1.8"    # 메모리 클리어
sha2 = "0.10"      # 해시 함수
```

## ⚠️ 보안 고려사항

1. **키 관리**: 암호화 키는 소스 코드에 하드코딩되므로, 소스 코드 접근 시 복호화 가능
2. **컴파일 타임 보안**: 바이너리 분석을 통해 패턴을 찾을 수 있음
3. **메모리 덤프**: 런타임 중 메모리 덤프 시 평문 노출 가능
4. **디버거**: 디버거 사용 시 평문 확인 가능

## 🎯 사용 권장사항

### ✅ 적합한 사용 사례:
- 개발/테스트 환경의 임시 비밀값
- 내부 서비스 간 통신용 토큰
- 설정 파일 대신 바이너리에 포함할 비밀값
- 소스 코드 스캔에서 비밀값 숨기기

### ❌ 부적합한 사용 사례:
- 프로덕션 환경의 중요한 비밀값
- 법적/규정 준수가 필요한 민감 데이터
- 장기간 사용되는 암호화 키
- 외부 공격자로부터 완전히 보호해야 하는 데이터

## 🔧 고급 사용법

### 커스텀 암호화 함수
```rust
use obfuscation::crypto::advanced;

let key = advanced::derive_key(b"password", b"salt", 10000);
let encrypted = advanced::simple_encrypt(b"data", &key);
let decrypted = advanced::simple_decrypt(&encrypted, &key);
```

### 설정 관리자 패턴
```rust
struct ConfigManager {
    configs: HashMap<String, SecretString>,
}

impl ConfigManager {
    fn load_from_obfuscated() -> Self {
        let mut configs = HashMap::new();
        
        configs.insert("db_url".to_string(), 
            obfuscate!("postgresql://...", "dbkey").reveal());
        configs.insert("api_key".to_string(), 
            obfuscate!("sk-...", "apikey").reveal());
            
        Self { configs }
    }
    
    fn get(&self, key: &str) -> Option<&SecretString> {
        self.configs.get(key)
    }
}
```

## 📚 추가 자료

- [Rust Zeroize 문서](https://docs.rs/zeroize/)
- [메모리 보안 Best Practices](https://owasp.org/www-community/vulnerabilities/Memory_leak)
- [Rust 보안 프로그래밍 가이드](https://anssi-fr.github.io/rust-guide/)

## 🤝 기여하기

1. Fork the repository
2. Create your feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## 📄 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다.
