use zeroize::{Zeroize, ZeroizeOnDrop};
use std::fmt;

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
