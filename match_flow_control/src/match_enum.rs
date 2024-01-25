#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Lucky Dime!");
            10
        }
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Dime;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Quarter(UsState::Alaska);
    let coin4 = Coin::Quarter(UsState::Alabama);

    println!("{}", value_in_cents(&coin));
    println!("{:?}", coin);

    println!("{}", value_in_cents(&coin3));
    println!("{}", value_in_cents(&coin4));
}
