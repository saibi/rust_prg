use std::net::Ipv4Addr;

pub mod hmac_msg;
pub mod input;
pub mod udpm;

/// 멀티캐스트 네트워크 설정을 위한 상수들
pub const MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 0, 1);
pub const PORT: u16 = 12344;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multicast_constants() {
        // 멀티캐스트 주소가 유효한지 확인
        assert!(MULTICAST_ADDR.is_multicast());
        // assert!(PORT > 0);
        // assert!(BUFFER_SIZE > 0);
    }
}
