// #[derive(Debug)]
fn main() {
    let config_max: Option<u8> = None;
    // let config_max = Some(10u8);

    match config_max {
        Some(max) => println!("Max: {}", max),
        _ => (),
    }

    using_if_let();
}

fn using_if_let() {
    let config_max = Some(10u8);
    if let Some(max) = config_max {
        println!("Max: {}", max);
    }
}
