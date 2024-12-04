use lib::{stdinthread::StdinThread, streamthread::StreamThread};
fn main() {
    let stdin = StdinThread::new();
    let sock_path = "/tmp/echo.sock";
    println!("echo client start");
    println!("sock: {}", sock_path);

    let stream = std::os::unix::net::UnixStream::connect(sock_path).unwrap();
    let mut stream_thread = StreamThread::new(stream);

    println!("/q : quit\n");
    loop {
        if let Some(cmd) = stdin.read_line() {
            println!("cmd from stdin: {}", cmd);
            match cmd.as_str() {
                "/q" => {
                    stream_thread.stop();
                    break;
                }
                _ => {
                    let msg = cmd + "\n";
                    stream_thread.send(msg);
                }
            }
        }
        if let Some(msg) = stream_thread.recv() {
            println!("recv msg from server : {}", msg);
        }
    }
}
