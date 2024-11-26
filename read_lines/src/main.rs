use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("reading file");

    if let Ok(lines) = read_lines("/etc/hosts") {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

// fn read_lines(filename: &str) -> Vec<String> {
//     let mut result = Vec::new();

//     for line in read_to_string(filename).unwrap().lines() {
//         result.push(line.to_string());
//     }
//     result
// }

// fn read_lines(filename: &str) -> Vec<String> {
//     read_to_string(filename)
//         .unwrap()
//         .lines()
//         .map(String::from)
//         .collect()
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
