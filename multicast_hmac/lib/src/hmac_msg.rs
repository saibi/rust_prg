use base64::{Engine, engine::general_purpose};
use sha2::{Digest, Sha256};

const HMAC_MSG_FIELD_MESSAGE: &str = "msg";
const HMAC_MSG_FIELD_SIGNATURE: &str = "sig";

/// HMAC 서명이 포함된 JSON 메시지를 생성하는 함수
///
/// # Arguments
/// * `message` - 전송할 메시지 문자열
/// * `secret_key` - HMAC 서명에 사용할 비밀키
///
/// # Returns
/// * `String` - HMAC 서명이 포함된 JSON 문자열
pub fn create_hmac_msg(message: &str, secret_key: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(secret_key);
    hasher.update(message.as_bytes());
    let signature = hasher.finalize();
    let signature_base64 = general_purpose::STANDARD.encode(signature);
    format!(
        r#"{{"{HMAC_MSG_FIELD_MESSAGE}":"{message}","{HMAC_MSG_FIELD_SIGNATURE}":"{signature_base64}"}}"#
    )
}

/// HMAC 서명을 검증하는 함수
///
/// 수신된 JSON 메시지에서 HMAC 서명을 검증합니다.
/// 서명이 유효하면 메시지를 반환하고, 그렇지 않으면 None을 반환합니다.
///
/// # Arguments
/// * `json_data` - JSON 형태의 메시지 데이터
/// * `secret_key` - HMAC 검증에 사용할 비밀키
///
/// # Returns
/// * `Option<String>` - 검증 성공 시 메시지, 실패 시 None
///
/// # Examples
/// ```
/// use lib::verify_hmac_message;
///
/// let json_data = r#"{"message":"hello","signature":"abc123"}"#;
/// let secret_key = b"my_secret_key";
/// let message = verify_hmac_message(json_data, secret_key);
/// ```
pub fn verify_hmac_message(json_data: &str, secret_key: &[u8]) -> Option<String> {
    // JSON 파싱 (간단한 구현)
    let msg_pattern = format!(r#""{HMAC_MSG_FIELD_MESSAGE}":""#);
    let sig_pattern = format!(r#""{HMAC_MSG_FIELD_SIGNATURE}":""#);

    if !json_data.contains(&msg_pattern) || !json_data.contains(&sig_pattern) {
        return None;
    }

    // 메시지 추출
    let message_start = json_data.find(&msg_pattern).unwrap() + msg_pattern.len();
    let message_end = json_data[message_start..].find('"').unwrap() + message_start;
    let message = &json_data[message_start..message_end];

    // 서명 추출 (base64 문자열 형식)
    let signature_start = json_data.find(&sig_pattern).unwrap() + sig_pattern.len();
    let signature_end = json_data[signature_start..].find('"').unwrap() + signature_start;
    let signature_base64 = &json_data[signature_start..signature_end];

    // base64 문자열을 바이트로 변환
    let signature_bytes = match general_purpose::STANDARD.decode(signature_base64) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    // HMAC 검증 (키 + 메시지의 SHA256 해시)
    let mut hasher = Sha256::new();
    hasher.update(secret_key);
    hasher.update(message.as_bytes());
    let expected_signature = hasher.finalize();

    // 서명 비교
    if signature_bytes != expected_signature.as_slice() {
        return None;
    }

    Some(message.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_message_verification() {
        let secret_key = b"test_secret_key";
        let message = "hello";

        // create_hmac_msg 함수를 사용해서 JSON 생성
        let json_data = create_hmac_msg(message, secret_key);

        // 검증 테스트
        let verified_message = verify_hmac_message(&json_data, secret_key);
        assert_eq!(verified_message, Some(message.to_string()));

        // 잘못된 키로 검증 시도
        let wrong_key = b"wrong_key";
        let verified_with_wrong_key = verify_hmac_message(&json_data, wrong_key);
        assert_eq!(verified_with_wrong_key, None);
    }
}
