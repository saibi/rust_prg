use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// address
    #[arg(short = 'a', long, default_value = "127.0.0.1:12345")]
    pub addr: String,
}
