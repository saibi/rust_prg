fn check_command(cmd: &str) -> Result<usize, String> {
    match cmd {
        "good" => Ok(0),
        "unsupported" => Err("Unsupported".to_owned()),
        "bad" => Err("Bad command".to_owned()),
        _ => Err("Wierd command".to_owned()),
    }
}

fn handle_command(cmd: &str) -> Result<usize, String> {
    let passed = check_command(cmd).unwrap();
    println!("first check passed: status={}", passed);

    let failed = check_command(cmd);
    if failed.is_ok() {
        println!("Second check passed: status={}", failed.unwrap());
    } else if failed.is_err() {
        println!("Second check failed: status={}", failed.unwrap());
    }

    match check_command(cmd) {
        Ok(s) => println!("Third check passed:{}", s),
        Err(s) => println!("Third check failed:{}", s),
    }
    Ok(0)
}

fn main() {
    // let status = handle_command("good");
    // if status.is_ok() {
    //     println!("Everything is fine");
    // } else {
    //     println!("I don't feel good");
    // }

    // let status = handle_command("bad");
    // if status.is_ok() {
    //     println!("Everything is fine");
    // } else {
    //     println!("I don't feel good");
    // }
    let status = handle_command2("bad");
    if status.is_ok() {
        println!("Everything is fine");
    } else {
        println!("I don't feel good");
    }
}

pub enum MyError {
    UnsupportedCommand,
    WrongInput(String),
    UnknownValue {
        name: String,
        expected: String,
        found: String,
    },
}

pub type Result<T, E = MyError> = std::result::Result<T, E>;

fn check_command2(cmd: &str) -> Result<usize> {
    match cmd {
        "good" => Ok(0),
        "unsupported" => Err(MyError::UnsupportedCommand),
        "bad" => Err(MyError::WrongInput(format!(
            "Cannot handle command {}",
            cmd
        ))),
        _ => Err(MyError::UnknownValue {
            name: "Wierd Command Error".to_owned(),
            expected: "good".to_string(),
            found: cmd.to_owned(),
        }),
    }
}

fn handle_command2(cmd: &str) -> Result<usize> {
    let passed = check_command2(cmd)?;
    println!("first check passed: status={}", passed);

    let _ = check_command(cmd)
        .map_err(|e| println!("Command failed: error={:?}", e))
        .map(|s| println!("Command passed: status={}", s));

    if let Ok(status) = check_command(cmd) {
        println!("Second check passed: status={}", status);
    } else {
        println!("Second check failed: command={}", cmd);
    }
    Ok(0)
}
