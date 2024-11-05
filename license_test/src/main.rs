use cargo_license::{get_licenses, LicenseInfo, Licensed};
use clap::Command;
use std::error::Error;

fn main() {
    let matches = Command::new("my-program")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Description of your program")
        .subcommand(
            Command::new("licenses").about("Display license information for all dependencies"),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("licenses") {
        if let Err(e) = display_licenses() {
            eprintln!("Error displaying licenses: {}", e);
            std::process::exit(1);
        }
    }
}

fn display_licenses() -> Result<(), Box<dyn Error>> {
    // cargo-license 0.6 버전에서는 직접 get_licenses 함수를 호출합니다
    let licenses = get_licenses()?;

    println!("\nDependency Licenses:");
    println!("-------------------");

    for package in licenses {
        println!(
            "{} (v{}) - {}",
            package.name,
            package.version,
            package.license.unwrap_or_else(|| "Unknown".to_string())
        );
    }

    // 자체 프로그램의 라이선스 정보도 표시
    println!("\nThis program is licensed under:");
    println!("MIT License");
    println!("Copyright (c) {} {}", 2024, "Your Name");

    Ok(())
}

// Cargo.toml에 다음 종속성을 추가해야 합니다:
//
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }
// cargo-license = "0.6"
// log = "0.4"
