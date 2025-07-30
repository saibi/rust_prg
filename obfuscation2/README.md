# Rust 공유 시크릿 난독화 예제

이 프로젝트는 Rust를 사용하여 공유 시크릿 값을 다양한 난독화 기법으로 보호하고 안전하게 하드코딩하는 방법을 보여주는 예제입니다.

## 🛡️ 보안 기능

### 지원하는 난독화 방법

1. **XOR 난독화**: 랜덤 키를 사용한 XOR 연산으로 시크릿을 난독화
2. **Base64 + XOR 이중 난독화**: XOR 난독화 후 Base64 인코딩으로 이중 보호
3. **시저 암호**: 문자 시프트를 통한 간단한 암호화
4. **SHA256 해시 검증**: 해시를 통한 시크릿 무결성 검증
5. **문자열 분할 난독화**: 시크릿을 여러 부분으로 분할하여 저장
6. **비트 반전 난독화**: 각 바이트의 비트를 반전하여 난독화

### 🔒 보안 강화 기능

- **소스코드 평문 제거**: 모든 시크릿이 난독화된 형태로 저장
- **완전히 난독화된 시크릿 매니저**: 소스코드에서 평문이 전혀 보이지 않음
- **환경 변수 기반 관리**: 런타임에 환경 변수에서 시크릿 로드
- **해시 기반 검증**: 시크릿 비교 시 해시를 사용하여 더 안전한 검증

## 🚀 사용법

### 기본 실행

```bash
cargo run
```

### 의존성

```toml
[dependencies]
sha2 = "0.10"      # SHA256 해시 함수
hex = "0.4"         # 16진수 인코딩/디코딩
base64 = "0.21"     # Base64 인코딩/디코딩
rand = "0.8"        # 랜덤 수 생성
```

## 📝 코드 예제

### 1. 기본 난독화 사용

```rust
let xor_secret = XorObfuscatedSecret::new("my_secret_key");
let secret = xor_secret.get_secret();
```

### 2. 완전히 안전한 시크릿 매니저 사용

```rust
use secure_secrets::SecureSecretManager;

let secure_manager = SecureSecretManager::new();

// API 키 가져오기 (소스코드에서 평문이 보이지 않음)
let api_key = secure_manager.get_api_key();

// 해시 기반 검증
let is_valid = secure_manager.verify_secret_hash("api_key", &api_key);
```

### 3. 환경 변수 기반 시크릿 관리

```rust
use secure_secrets::EnvironmentSecretManager;

let env_manager = EnvironmentSecretManager::new();

if env_manager.is_configured() {
    if let Some(api_key) = env_manager.get_api_key() {
        println!("API 키: {}", api_key);
    }
}
```

### 4. 환경 변수 설정

```bash
export API_KEY=your_api_key
export DB_PASSWORD=your_db_password
export JWT_SECRET=your_jwt_secret
export ENCRYPTION_KEY=your_encryption_key
```

## 🔒 보안 고려사항

### 장점

- **소스코드에서 직접적인 시크릿 노출 방지**: 시크릿이 난독화되어 저장됨
- **완전한 평문 제거**: 소스코드에서 평문이 전혀 보이지 않음
- **다양한 보호 계층**: 여러 난독화 방법을 조합하여 사용 가능
- **런타임 복원**: 프로그램 실행 시에만 시크릿이 복원됨
- **해시 기반 검증**: 시크릿 비교 시 해시를 사용하여 더 안전함
- **환경 변수 지원**: 런타임에 환경 변수에서 시크릿 로드

### 주의사항

- **완전한 보안은 아님**: 난독화는 암호화가 아니므로 전문가가 분석하면 복원 가능
- **메모리 보호 필요**: 런타임에 복원된 시크릿은 메모리에 평문으로 저장됨
- **키 관리**: 난독화 키도 안전하게 관리해야 함
- **환경 변수 보안**: 환경 변수도 적절히 보호해야 함

## 🏗️ 프로젝트 구조

```
obfuscation2/
├── Cargo.toml              # 프로젝트 설정 및 의존성
├── src/
│   ├── main.rs             # 메인 예제 코드
│   ├── secret_manager.rs   # 기본 시크릿 매니저
│   └── secure_secrets.rs   # 보안 강화된 시크릿 매니저
└── README.md               # 프로젝트 문서
```

## 🎯 사용 시나리오

### 1. API 키 보호 (완전히 안전한 방법)
```rust
// 소스코드에서 평문이 전혀 보이지 않음
let secure_manager = SecureSecretManager::new();
let api_key = secure_manager.get_api_key();
```

### 2. 환경 변수 기반 보호
```rust
// 런타임에 환경 변수에서 로드
let env_manager = EnvironmentSecretManager::new();
if let Some(api_key) = env_manager.get_api_key() {
    // API 키 사용
}
```

### 3. 해시 기반 검증
```rust
// 시크릿을 직접 비교하지 않고 해시로 검증
let is_valid = secure_manager.verify_secret_hash("api_key", &input_key);
```

## 🔧 커스터마이징

### 새로운 난독화 방법 추가

```rust
struct CustomObfuscatedSecret {
    obfuscated_data: Vec<u8>,
    // 추가 필드들...
}

impl CustomObfuscatedSecret {
    fn new(secret: &str) -> Self {
        // 난독화 로직 구현
    }
    
    fn get_secret(&self) -> String {
        // 복원 로직 구현
    }
}
```

### SecureSecretManager에 새로운 시크릿 추가

```rust
pub struct SecureSecretManager {
    // 기존 필드들...
    custom_secret_obfuscated: Vec<u8>,
}

impl SecureSecretManager {
    pub fn get_custom_secret(&self) -> String {
        // 복원 로직 구현
    }
}
```

## 📊 성능 비교

각 난독화 방법의 특징:

| 방법 | 보안 수준 | 성능 | 구현 복잡도 | 소스코드 안전성 |
|------|-----------|------|-------------|-----------------|
| XOR 난독화 | 중간 | 빠름 | 낮음 | 중간 |
| Base64 + XOR | 높음 | 중간 | 중간 | 높음 |
| 시저 암호 | 낮음 | 빠름 | 낮음 | 중간 |
| SHA256 검증 | 높음 | 중간 | 중간 | 높음 |
| 문자열 분할 | 중간 | 빠름 | 중간 | 높음 |
| 비트 반전 | 낮음 | 빠름 | 낮음 | 높음 |
| **SecureSecretManager** | **매우 높음** | **빠름** | **중간** | **매우 높음** |
| **EnvironmentSecretManager** | **매우 높음** | **빠름** | **낮음** | **완전 안전** |

## 🚨 보안 권장사항

### 1. 프로덕션 환경에서 권장하는 방법

1. **환경 변수 사용**: `EnvironmentSecretManager` 사용
2. **시크릿 관리 서비스**: AWS Secrets Manager, HashiCorp Vault 등 사용
3. **암호화된 설정 파일**: GPG나 다른 암호화 도구로 설정 파일 보호
4. **컨테이너 시크릿**: Docker secrets, Kubernetes secrets 사용

### 2. 개발 환경에서 권장하는 방법

1. **SecureSecretManager 사용**: 소스코드에서 평문 제거
2. **환경별 설정**: 개발/테스트/프로덕션 환경별 다른 시크릿 사용
3. **시크릿 로테이션**: 정기적으로 시크릿 변경

## 🤝 기여하기

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다.

## ⚠️ 면책 조항

이 코드는 교육 목적으로만 제공됩니다. 실제 프로덕션 환경에서 사용하기 전에 충분한 보안 검토를 진행하시기 바랍니다. 난독화는 완전한 암호화가 아니므로 전문가가 분석하면 복원 가능할 수 있습니다. 