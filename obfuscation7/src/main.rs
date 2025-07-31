use zeroize::Zeroize;

/// 실제 사용할 Secret (가변 길이 지원)
const PLAIN_SECRET: &[u8] = b"My-Very-Secret-Key!!!"; // <- 길이 마음대로 가능

/// Secret 길이에 맞게 Shuffle 순서 생성
fn generate_shuffle_order(length: usize) -> Vec<usize> {
    let mut order: Vec<usize> = (0..length).collect();
    for i in 0..length {
        let swap_idx = (i * 7 + 3) % length;
        order.swap(i, swap_idx);
    }
    order
}

/// Secret을 난독화해서 ENCODED_SECRET 소스코드 형태로 출력
fn print_encoded_secret_source() {
    let length = PLAIN_SECRET.len();
    let shuffle_order = generate_shuffle_order(length);
    let mut encoded = vec![0u8; length];

    for (decoded_index, &encoded_index) in shuffle_order.iter().enumerate() {
        let mut val = PLAIN_SECRET[decoded_index];
        val ^= (encoded_index as u8).wrapping_mul(3);
        let dynamic_key = 0xAA ^ (encoded_index as u8);
        val ^= dynamic_key;
        encoded[encoded_index] = val;
    }

    // ENCODED_SECRET 출력
    print!("const ENCODED_SECRET: [u8; {length}] = [\n    ");
    for (i, b) in encoded.iter().enumerate() {
        print!("0x{b:02X}");
        if i != length - 1 {
            print!(", ");
        }
        if (i + 1) % 8 == 0 {
            print!("\n    ");
        }
    }
    println!("];");

    // SHUFFLE_ORDER 출력
    print!("const SHUFFLE_ORDER: [usize; {length}] = [\n    ");
    for (i, &v) in shuffle_order.iter().enumerate() {
        print!("{v}");
        if i != length - 1 {
            print!(", ");
        }
        if (i + 1) % 16 == 0 {
            print!("\n    ");
        }
    }
    println!("];");
}

/// 복호화 함수 (배포용 코드에서는 ENCODED_SECRET과 SHUFFLE_ORDER를 const로 하드코딩)
fn get_secret(encoded_secret: &[u8], shuffle_order: &[usize]) -> Vec<u8> {
    let mut decoded = vec![0u8; encoded_secret.len()];

    for (decoded_index, &encoded_index) in shuffle_order.iter().enumerate() {
        let mut val = encoded_secret[encoded_index];
        let dynamic_key = 0xAA ^ (encoded_index as u8);
        val ^= dynamic_key;
        val ^= (encoded_index as u8).wrapping_mul(3);
        decoded[decoded_index] = val;
    }

    decoded
}

fn main() {
    // 개발 시 사용할 소스코드 생성기
    print_encoded_secret_source();

    // 예시: 하드코딩된 값으로 복호화 테스트
    // const ENCODED_SECRET: [u8; 20] = [
    //     0xE9, 0xCC, 0xA3, 0xDA, 0x84, 0xC7, 0xB0, 0xE4, 0xC1, 0x8C, 0xC2, 0xB3, 0xA0, 0x8C, 0xE9,
    //     0xCE, 0xB6, 0x8A, 0xE9, 0x9A,
    // ];
    // const SHUFFLE_ORDER: [usize; 20] = [
    //     3, 10, 7, 16, 1, 8, 13, 4, 19, 0, 5, 12, 17, 2, 9, 14, 6, 15, 18, 11,
    // ];

    const ENCODED_SECRET: [u8; 21] = [
        0xD3, 0xCD, 0xDC, 0xED, 0x8F, 0xF3, 0xDB, 0xCC, 0xC8, 0xDD, 0xC7, 0xAD, 0xC9, 0xE5, 0xAF,
        0xA9, 0xAB, 0xA5, 0xD8, 0xE3, 0xFB,
    ];
    const SHUFFLE_ORDER: [usize; 21] = [
        3, 10, 17, 18, 1, 2, 0, 4, 5, 6, 19, 8, 9, 7, 11, 12, 13, 20, 15, 16, 14,
    ];

    let mut secret = get_secret(&ENCODED_SECRET, &SHUFFLE_ORDER);
    println!("Decoded Secret: {:?}", String::from_utf8_lossy(&secret));

    secret.zeroize(); // 메모리 삭제
}
