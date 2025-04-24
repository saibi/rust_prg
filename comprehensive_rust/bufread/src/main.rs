use std::io::{BufRead, BufReader, Read, Result, Write};

fn count_lines<R: Read>(reader: R) -> usize {
    let buf_reader = BufReader::new(reader);
    buf_reader.lines().count()
}
fn main() -> Result<()> {
    let slice: &[u8] = b"foo\nbar\nbaz\n";
    println!("sline count: {}", count_lines(slice));

    let file = std::fs::File::open(std::env::current_exe()?)?;
    println!("current_exe: {:?}", std::env::current_exe()?);
    println!("line count: {}", count_lines(file));

    write_test()?;
    Ok(())
}

fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
    writer.write_all(msg.as_bytes())?;
    writer.write_all(b"\n")
}

fn write_test() -> Result<()> {
    let mut buffer = Vec::new();
    log(&mut buffer, "hello")?;
    log(&mut buffer, "world")?;
    println!("{}", String::from_utf8_lossy(&buffer));
    Ok(())
}
