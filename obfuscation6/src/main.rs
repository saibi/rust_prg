use zeroize::Zeroize;

#[inline(never)]
fn xor_again(xor_constant: &[u8]) -> Vec<u8> {
    let mut decoded = vec![0u8; xor_constant.len()];
    (0..xor_constant.len()).for_each(|i| {
        decoded[i] = xor_constant[i] ^ 0xAA; // strings 로 문자열 검출됨 
        // decoded.push(xor_constant[i] ^ 0xAA); // 벡터로 하면 문자열로 검출되지 않음
    });
    decoded
}

// Secret 복호화 함수 (inline 방지)
#[inline(never)]
fn get_secret(encoded_secret: &[u8]) -> Vec<u8> {
    let mut decoded = Vec::with_capacity(encoded_secret.len());
    for &b in encoded_secret.iter() {
        decoded.push(b ^ 0xAA); // XOR 복호화
    }
    decoded
}

fn main() {
    const XOR_CONSTANT: [u8; 32] = [
        206, 207, 218, 207, 196, 206, 245, 208, 207, 216, 197, 245, 194, 222, 222, 218, 144, 133,
        133, 216, 207, 217, 223, 198, 222, 245, 217, 222, 216, 195, 196, 205,
    ];

    // let mut secret = xor_again(&XOR_CONSTANT);

    let mut secret = get_secret(&XOR_CONSTANT);

    // Vec<u8>을 String으로 변환
    let secret_string = String::from_utf8_lossy(&secret);
    println!("Secret (string): {secret_string}");
    println!("Secret (hex): {secret:?}");

    secret.zeroize();
    println!("Secret after zeroize: {secret:?}");
}
