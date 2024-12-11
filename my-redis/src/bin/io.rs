use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut f = File::create("/tmp/foo2.txt").await?;

    io::copy(&mut reader, &mut f).await?;
    Ok(())
}

// async fn write_test() -> io::Result<()> {
//     let mut f = File::create("/tmp/foo2.txt").await?;

//     let n = f.write(b"some bytes some bytes").await?;

//     println!("Wrote the first {} bytes of 'some bytes'.", n);
//     Ok(())
// }
// async fn test_read_to_end() -> io::Result<()> {
//     let mut f = File::open("/tmp/foo.txt").await?;
//     let mut buffer = Vec::new();

//     let n = f.read_to_end(&mut buffer).await?;

//     println!("The bytes: {:?}", &buffer[..n]);
//     Ok(())
// }
// async fn test_read() -> io::Result<()> {
//     let mut f = File::open("/tmp/foo.txt").await?;
//     let mut buffer = [0; 10];

//     let n = f.read(&mut buffer).await?;

//     println!("The bytes: {:?}", &buffer[..n]);
//     Ok(())
// }
