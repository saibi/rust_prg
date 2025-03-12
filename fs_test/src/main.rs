use std::{env::current_dir, fs::File, io::Read, path::Path};

fn grep(filename: &Path, word: &str) -> std::io::Result<()> {
    let mut f = File::open(filename)?;
    let mut text_buffer = String::new();

    f.read_to_string(&mut text_buffer)?;
    for line in text_buffer.split('\n') {
        if line.contains(word) {
            println!("{line}");
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut filename = current_dir()?;
    filename.push("src/main.rs");
    grep(&filename, "main")?;
    Ok(())
}
