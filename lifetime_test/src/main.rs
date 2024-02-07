// missing lifetime specifier
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// generic / trait bound / lifetime
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // string1 == string2
    // let string1 = String::from("abcd");
    // {
    //     let string2 = "xyz";

    //     let result = longest(string1.as_str(), string2);
    //     println!("The longest string is {}", result);
    // }

    let string1 = String::from("abcd");
    let result;
    {
        // OK
        let string2 = "xyzzzz";
        result = longest(string1.as_str(), string2);

        // does not live long enough
        // let string2 = "xyzzzz".to_string();
        // result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    let result;
    {
        let string2 = "xyzzzz".to_string();
        result = get_first(string1.as_str(), string2.as_str());
    }
    println!("The first string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i.part: {}", i.part);

    // generic / trait bound / lifetime
    let longest_str = longest_with_an_announcement("x", "yy", "x vs yy");
    println!("The longest string is {}", longest_str);
}
