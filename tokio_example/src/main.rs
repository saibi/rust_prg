use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    println!("Server started on 127.0.0.1:5342");
    let listener = TcpListener::bind("127.0.0.1:5342".to_string())
        .await
        .unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            connection_handler(socket).await;
        });
    }
}

async fn connection_handler(mut socket: TcpStream) {
    println!("processing started");
    let mut buf = vec![0; 1024];
    socket.read(&mut buf).await.unwrap();
    println!("received: {}", String::from_utf8_lossy(&buf));
    sleep(Duration::from_millis(3000)).await;
    let contents = "finished processing";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    socket.write(response.as_bytes()).await.unwrap();
    socket.flush().await.unwrap();
    println!("processing finished");
}
