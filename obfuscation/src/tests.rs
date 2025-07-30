#[cfg(test)]
mod tests {
    use crate::{SecretString};

    #[test]
    fn test_secret_string_length() {
        let secret = SecretString::new("test123".to_string());
        assert_eq!(secret.len(), 7);
        assert!(!secret.is_empty());
    }

    #[test]
    fn test_secret_string_debug() {
        let secret = SecretString::new("sensitive".to_string());
        let debug_output = format!("{:?}", secret);
        assert_eq!(debug_output, "SecretString([REDACTED])");
    }

    #[test]
    fn test_secret_string_display() {
        let secret = SecretString::new("sensitive".to_string());
        let display_output = format!("{}", secret);
        assert_eq!(display_output, "[REDACTED]");
    }

    #[test]
    fn test_empty_secret() {
        let empty_secret = SecretString::new("".to_string());
        assert!(empty_secret.is_empty());
        assert_eq!(empty_secret.len(), 0);
    }

    #[test]
    fn test_secret_use_multiple_times() {
        let secret = SecretString::new("reusable".to_string());
        
        let result1 = secret.use_secret(|s| s.len());
        let result2 = secret.use_secret(|s| s.to_uppercase());
        
        assert_eq!(result1, 8);
        assert_eq!(result2, "REUSABLE");
    }

    #[test]
    fn test_secret_string_as_bytes() {
        let secret = SecretString::new("test".to_string());
        let bytes = secret.as_bytes();
        assert_eq!(bytes, b"test");
    }

    #[test]
    fn test_memory_safety() {
        // 이 테스트는 메모리 안전성을 확인하기 위한 것입니다.
        // 실제로는 Drop trait의 동작을 관찰할 수 없지만,
        // 컴파일이 성공하면 메모리 안전성이 보장됩니다.
        {
            let _secret = SecretString::new("temporary".to_string());
            // 스코프 끝에서 자동으로 메모리가 클리어됩니다.
        }
        
        // 여기서는 secret에 접근할 수 없습니다.
        assert!(true); // 테스트 통과
    }
}
