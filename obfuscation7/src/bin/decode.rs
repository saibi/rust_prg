use zeroize::Zeroize;

/// 복호화 함수 (배포용 코드에서는 ENCODED_SECRET과 SHUFFLE_ORDER를 const로 하드코딩)
#[inline(never)]
fn get_secret(encoded_secret: &[u8], shuffle_order: &[usize]) -> Vec<u8> {
    // let mut decoded = vec![0u8; encoded_secret.len()];
    let mut decoded = Vec::with_capacity(encoded_secret.len());

    for &encoded_index in shuffle_order.iter() {
        let mut val = encoded_secret[encoded_index];
        let dynamic_key = 0xAA ^ (encoded_index as u8);
        val ^= dynamic_key;
        val ^= (encoded_index as u8).wrapping_mul(3);
        // println!("decoded_index: {decoded_index}, encoded_index: {encoded_index}, val: {val}");
        decoded.push(val);
    }

    decoded
}

fn main() {
    // copy encode output to here
    const ENCODED_SECRET: [u8; 11] = [
        0xC6, 0xC7, 0xCB, 0xCC, 0xCA, 0x80, 0xD2, 0xD7, 0xDE, 0xCA, 0xC9,
    ];
    const SHUFFLE_ORDER: [usize; 11] = [4, 2, 0, 6, 7, 5, 10, 1, 9, 3, 8];

    let mut secret = get_secret(&ENCODED_SECRET, &SHUFFLE_ORDER);
    println!("Decoded Secret: {:?}", String::from_utf8_lossy(&secret));

    secret.zeroize(); // 메모리 삭제
}
