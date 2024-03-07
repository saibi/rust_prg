#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

use grrs::find_matches;

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("Hello, {} {}", args.pattern, args.path.display());

    // use expect method to handle error : panic with message
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // manual error handling
    // let result = std::fs::read_to_string(&args.path);
    // let content = match result {
    //     Ok(content) => content,
    //     Err(err) => {
    //         return Err(err.into());
    //     }
    // };

    // use ? operator
    // let content = std::fs::read_to_string(&args.path)
    //     .map_err(|err| CustomError(format!("Error reading '{}': {}", args.path.display(), err)))?;

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file '{}'", args.path.display()))?;

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    find_matches(&content, &args.pattern, std::io::stdout());

    Ok(())
}
