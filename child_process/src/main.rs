use core::panic;
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};
fn main() {
    wait_test();

    pipe_test();

    run_command("rustc", "--version");
    run_command("rustc", "--invalidargs");
    run_command("invalid_rustc", "--version");
}

fn run_command(program: &str, args: &str) {
    let output = Command::new(program)
        .arg(args)
        .output()
        .unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e);
        });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("rustc failed and stderr was:\n{}", s);
    }
}

fn pipe_test() {
    static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

    let mut cmd = if cfg!(target_family = "windows") {
        let mut cmd = Command::new("powershell");
        cmd.arg("-Command")
            .arg("$input | Measure-Object -Line -Word -Character");
        cmd
    } else {
        Command::new("wc")
    };

    let process = match cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("sent pangram to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}

fn wait_test() {
    println!("spawn sleep 5");
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();

    for _ in 0..5 {
        println!("waiting...");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    let _result = child.wait().unwrap();

    println!("reached end of main");
}
