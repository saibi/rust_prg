use std::process::Command;

fn main() {
    // 쉘 명령 실행
    let output = Command::new("ls")
        .arg("-al")
        .output()
        .expect("failed to execute command");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
