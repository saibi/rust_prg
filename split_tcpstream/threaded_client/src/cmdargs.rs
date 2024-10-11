use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Server Address
    #[arg(short = 'a', long, default_value = "10.82.79.2:8274")]
    pub address: String,

    /// log file path
    #[arg(short = 'd', long, default_value = "/tmp/client.log")]
    pub log_file: std::path::PathBuf,

    /// log level
    #[arg(short = 'l', long, default_value = "debug")]
    pub log_level: String,

    /// log to stderr as well
    #[arg(short = 'e', long, default_value = "true")]
    pub log_stderr: bool,

    /// fs test file path
    #[arg(short = 'f', long, default_value = "/tmp/DMB_BD.bin")]
    pub test_file: String,
}
