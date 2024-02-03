use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:?}", scores);

    // let mut scores = HashMap::new();

    // let blue = String::from("Blue");
    // let yellow = String::from("Yellow");
    // let blue_score = 10;
    // let yellow_score = 50;

    // scores.insert(blue, blue_score);
    // scores.insert(yellow, yellow_score);

    // println!("Scores: {:?}", scores);
    // println!("{}", blue_score); // borrow of moved value
    //                             // println!("{}: {}", blue, blue_score); // borrow of moved value

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let score = scores.get("Blue");
    // println!("Score: {:?}", score);

    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }
}
