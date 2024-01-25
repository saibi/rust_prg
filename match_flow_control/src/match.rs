#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Lucky Dime!");
            10
        }
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    let coin2 = Coin::Nickel;

    println!("{}", value_in_cents(&coin));
    println!("{:?}", coin)
}
