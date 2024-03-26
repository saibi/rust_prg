fn if_else_test() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let bin_n = if n < 10 && n > -10 {
        println!(", and it is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and it is a big number, reduce by two");
        n / 2
    };

    println!("{} -> {}", n, bin_n);
}

fn loop_test() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            break 'outer;
        }

        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn fizz_buzz() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn for_iter() {
    // iter - iterate over elements without changing them
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // into_iter - consume the collection and return owned values
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names);

    // iter_mut - iterate over mutable references
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}

fn match_test() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        14..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}

fn destructuring_test() {
    let triple = (10, -2, 4);

    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("The last element is `2`, and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, and the last is `4`, and the middle doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    let array = [3, 9, 9];

    match array {
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        // single value can be ignored with _
        [1, _, third] => println!("array[0] = 1, array[2] = {}", third),
        // you can also bind some and ignore the rest
        [-1, second, ..] => println!("array[0] = -1, array[1] = {}", second),
        // or store them in another array/slice
        [3, tail @ ..] => println!("array[0] = 3, tail = {:?}", tail),
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, last = {}",
            first, middle, last
        ),
    }
}

#[allow(dead_code)]
fn destructuring_enum() {
    enum Color {
        Red,
        Blue,
        Green,

        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::CMYK(122, 17, 40, 55);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, Saturation: {}, Value: {}", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, Saturation: {}, Lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, Magenta: {}, Yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, Magenta: {}, Yellow: {}, Key (Black): {}",
            c, m, y, k
        ),
    }
}

fn destructuring_ref() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;

    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

fn destructuring_struct() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };
    let Foo { x: x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar {
        foo: Foo {
            x: nested_x,
            y: nested_y,
        },
    } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}

fn guards_test() {
    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
    }

    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius", t),
        _ => println!("fahrenheit"),
    }

    // note that compiler won't take guard conditions into account when checking if all patterns are covered by the match expression
    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Should never reach here"),
    }
}
fn age() -> u32 {
    30
}

fn some_number() -> Option<u8> {
    Some(44)
}
fn binding_test() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }

    match some_number() {
        Some(n @ 10) => println!("n == 10: {}", n),
        Some(n) => println!("n: {}", n),
        _ => (),
    }
}

fn if_let_test() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("the condition is false, let's go with an emoticon :)")
    }

    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }

    enum Foo2 {
        Bar2,
    }

    let a = Foo2::Bar2;

    if let Foo2::Bar2 = a {
        println!("a is foobar");
    }
}

use std::str::FromStr;
fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("can't segment count item pair: '{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("can't parse count: '{count_str}'");
    };
    (count, item)
}

fn get_count_item2(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (count_str, item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("can't segment count item pair: '{s}'"),
    };
    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("can't parse count: '{count_str}'");
    };
    (count, item)
}

fn while_let_test() {
    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            }
            _ => {
                break;
            }
        }
    }

    optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
fn main() {
    if_else_test();
    loop_test();
    fizz_buzz();
    for_iter();
    match_test();
    destructuring_test();
    destructuring_enum();
    destructuring_ref();
    destructuring_struct();
    guards_test();
    binding_test();
    if_let_test();

    println!("{:?}", get_count_item("10 apples"));
    println!("{:?}", get_count_item2("10 apples"));

    while_let_test();
}
