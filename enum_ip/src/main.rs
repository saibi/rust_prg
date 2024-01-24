fn main() {
    let number1: Option<i32> = None;
    let number2 = Some(1);

    println!("number1: {:?}", number1);
    println!("number2: {:?}", number2);

    if number1.is_none() {
        println!("is none");
    }
    if number2.is_some() {
        println!("is some i32");
        // println!("number2 add 2: {}", number2 + 2);
        let num = number2.unwrap();

        println!("add 2: {}", num + 2);
    }
}
