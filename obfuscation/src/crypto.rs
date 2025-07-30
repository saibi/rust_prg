use crate::secret::SecretString;

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
        let mut key = Vec::new();
        key.extend_from_slice(&self.key_parts);
        
        let mut decrypted = vec![0u8; self.len];
        for i in 0..self.len {
            decrypted[i] = self.data[i] ^ key[i % key.len()];
        }
        
        SecretString::new(String::from_utf8(decrypted).unwrap_or_default())
    }
}

/// 고급 암호화 함수들
pub mod advanced {
    use super::*;
    use sha2::{Sha256, Digest};
    
    /// PBKDF2 스타일의 키 유도 함수
    pub fn derive_key(password: &[u8], salt: &[u8], iterations: u32) -> [u8; 32] {
        let mut key = password.to_vec();
        for _ in 0..iterations {
            let mut hasher = Sha256::new();
            hasher.update(&key);
            hasher.update(salt);
            key = hasher.finalize().to_vec();
        }
        
        let mut result = [0u8; 32];
        result.copy_from_slice(&key[..32]);
        result
    }
    
    /// AES-like 라운드 함수 (간단한 버전)
    pub fn simple_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
        let mut result = data.to_vec();
        let key_len = key.len();
        
        // 여러 라운드의 변환
        for round in 0..4 {
            for (i, byte) in result.iter_mut().enumerate() {
                *byte ^= key[i % key_len];
                *byte = byte.wrapping_add(round as u8);
                *byte = (*byte << 3) | (*byte >> 5); // 비트 회전
            }
        }
        
        result
    }
    
    /// 복호화 함수
    pub fn simple_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
        let mut result = data.to_vec();
        let key_len = key.len();
        
        // 암호화의 역순으로 진행
        for round in (0..4).rev() {
            for (i, byte) in result.iter_mut().enumerate() {
                *byte = (*byte >> 3) | (*byte << 5); // 비트 회전 역순
                *byte = byte.wrapping_sub(round as u8);
                *byte ^= key[i % key_len];
            }
        }
        
        result
    }
}
