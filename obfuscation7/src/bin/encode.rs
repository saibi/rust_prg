use std::env;

/// Secret을 난독화해서 ENCODED_SECRET 소스코드 형태로 출력
/// Secret 길이에 맞게 Shuffle 순서 생성
fn generate_shuffle_order(length: usize) -> Vec<usize> {
    let mut order: Vec<usize> = (0..length).collect();
    for i in 0..length {
        let swap_idx = (i * 7 + 3) % length;
        order.swap(i, swap_idx);
    }
    order
}

fn print_encoded_secret_source(plain_secret: &str) {
    let length = plain_secret.len();
    let shuffle_order = generate_shuffle_order(length);
    let mut encoded = vec![0u8; length];

    for (decoded_index, &encoded_index) in shuffle_order.iter().enumerate() {
        let mut val = plain_secret.as_bytes()[decoded_index];
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <plain_secret>", args[0]);
        eprintln!("Example: {} \"my secret text\"", args[0]);
        std::process::exit(1);
    }

    let plain_secret = &args[1];
    println!("Plain secret: {plain_secret}");
    println!("Length: {}", plain_secret.len());
    println!();

    print_encoded_secret_source(plain_secret);
}
