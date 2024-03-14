use anyhow::Context;
use clap::Parser;

/// Led control
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// led clk device path, e.g. /sys/class/leds/ph23_led_clk/brightness
    #[arg(
        short = 'k',
        long = "clk",
        default_value = "/sys/class/leds/ph23_led_clk/brightness"
    )]
    clk_path: String,

    /// led dat device path, e.g. /sys/class/leds/ph22_led_dat/brightness
    #[arg(
        short = 'd',
        long = "dat",
        default_value = "/sys/class/leds/ph22_led_dat/brightness"
    )]
    dat_path: String,

    /// led color hex value, e.g. 0x05ff00ff
    #[arg(short = 'c', long = "col")]
    color_value: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("led_ctl: {:?}", args);

    let content = std::fs::read_to_string("/tmp/not_found")
        .with_context(|| format!("failed to read file '{}'", "/tmp/not_found"))?;

    println!("content: {:?}", content);

    Ok(())
}
