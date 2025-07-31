use std::env;

const XOR_KEY: u8 = 0xAA;

#[inline(never)]
fn print_simple_xor_constant(secret: &str) {
    print!("const XOR_CONSTANT: [u8; {}] = [", secret.len());
    (0..secret.len()).for_each(|i| {
        print!("{}, ", secret.as_bytes()[i] ^ XOR_KEY);
    });
    println!("];");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("사용법: {} <문자열>", args[0]);
        std::process::exit(1);
    }

    let input_string = &args[1];
    print_simple_xor_constant(input_string);
}
