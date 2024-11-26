use std::{error::Error, fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new(".");

    let path_display = path.display();
    println!("The path is {}", path_display);

    let mut new_path = path.join("..");

    println!("The new path is {}", new_path.display());

    new_path.push("path_test");
    new_path.push("src");
    new_path.push("main.rs");
    println!("The new path is {}", new_path.display());

    // new_path.set_file_name("main.rs");

    match new_path.to_str() {
        Some(s) => println!("The new path is {}", s),
        None => println!("The new path is not a valid UTF-8 sequence"),
    }

    let mut file = match File::open(&new_path) {
        Err(why) => panic!(
            "couldn't open {}: {}",
            new_path.display(),
            why.description()
        ),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!(
            "couldn't read {}: {}",
            new_path.display(),
            why.description()
        ),
        Ok(_) => print!("{} contains:\n{}", new_path.display(), s),
    }

    file_create_test();
}

use std::io::prelude::*;

fn file_create_test() {
    static LOREM_IPSUM: &str =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
