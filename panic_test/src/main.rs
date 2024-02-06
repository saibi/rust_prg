// use core::panic;
use std::fs::File;
// use std::io::ErrorKind;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => {
    //         println!("The file was opened successfully: {:?}", file);
    //         file
    //     }
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => {
    //                 println!("The file was created successfully: {:?}", file);
    //                 file
    //             }
    //             Err(error) => {
    //                 panic!("Problem creating the file: {:?}", error);
    //             }
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // use unwrap_or_else & closure
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");
    // println!("The greeting file is: {:?}", greeting_file);

    let username = read_username_from_file().expect("Error reading username from file");
    println!("The username is: {}", username);

    let username2 = read_username_from_file_short().expect("Error reading username2 from file");
    println!("The username2 is: {}", username2);

    let username3 = read_username_from_file_shorter().expect("Error reading username3 from file");
    println!("The username3 is: {}", username3);

    let username4 = read_username_from_file_shortest().expect("Error reading username4 from file");
    println!("The username4 is: {}", username4);

    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

use std::io::{self, Read};
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}
