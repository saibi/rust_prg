use rand::Rng;

#[derive(Clone)]
struct Drink(String);

fn get_drink() -> Option<Drink> {
    let drinks = vec![
        Some(Drink("Coke".to_string())),
        Some(Drink("Pepsi".to_string())),
        Some(Drink("Water".to_string())),
        Some(Drink("Juice".to_string())),
        Some(Drink("Tea".to_string())),
        None,
    ];

    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..drinks.len());
    drinks[idx].clone()
}

fn has_sugar(drink: Drink) -> Option<Drink> {
    if drink.0 == "Coke" || drink.0 == "Pepsi" {
        Some(drink)
    } else {
        None
    }
}

fn processing_drink(drink: Option<Drink>) -> Option<String> {
    drink.map(|drink| format!("I got a drink: {}", drink.0))
}

fn print_drink(drink: Option<String>) -> Option<()> {
    drink.map(|drink| println!("{}", drink))
}

fn main() {
    println!("Option combinator .map example");

    for i in 0..10 {
        println!("#{}", i);
        let drink = get_drink();
        print_drink(processing_drink(drink));

        println!("---");
        let sugar_drink = get_drink().and_then(has_sugar);
        print_drink(processing_drink(sugar_drink));
    }
}
