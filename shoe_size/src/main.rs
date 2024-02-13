use shoe_size::*;

fn main() {
    let my_shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sendal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(my_shoes, 10);

    println!("in_my_size {:?}", in_my_size);
}
