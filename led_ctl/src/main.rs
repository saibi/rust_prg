use anyhow::Context;
use clap::Parser;
use std::io::Write;
use std::path::PathBuf;

/// Led control
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// led clk device path, e.g. /sys/class/leds/ph22_led_clk/brightness
    #[arg(
        short = 'k',
        long = "clk",
        default_value = "/sys/class/leds/ph22_led_clk/brightness"
    )]
    clk_path: PathBuf,

    /// led dat device path, e.g. /sys/class/leds/ph23_led_dat/brightness
    #[arg(
        short = 'd',
        long = "dat",
        default_value = "/sys/class/leds/ph23_led_dat/brightness"
    )]
    dat_path: PathBuf,

    /// led color value, e.g. 0x05ff00ff, 05ff00ff, 12345
    #[arg(name = "color_value")]
    color_value: String,
}

fn check_device_files_exist(args: &Args) -> anyhow::Result<()> {
    if !args.clk_path.exists() {
        return Err(anyhow::anyhow!(
            "Device file '{}' does not exist",
            args.clk_path.display()
        ));
    }
    if !args.dat_path.exists() {
        return Err(anyhow::anyhow!(
            "Device file '{}' does not exist",
            args.dat_path.display()
        ));
    }
    Ok(())
}

use std::fs::File;

#[derive(Debug)]
struct LedDevice {
    clk_file: File,
    dat_file: File,
}

impl LedDevice {
    const MAX_LED_COUNT: u32 = 10;

    fn open(clk_path: &str, dat_path: &str) -> anyhow::Result<Self> {
        let clk_file = File::options()
            .read(true)
            .write(true)
            .open(clk_path)
            .with_context(|| format!("failed to open file '{}'", clk_path))?;
        let dat_file = File::options()
            .read(true)
            .write(true)
            .open(dat_path)
            .with_context(|| format!("failed to open file '{}'", dat_path))?;

        Ok(Self { clk_file, dat_file })
    }

    fn _write_bit(&mut self, bit: bool) -> anyhow::Result<()> {
        self.clk_file
            .write_all(&[b'0'])
            .with_context(|| "failed to write to clk file")?;
        self.dat_file
            .write_all(if bit { &[b'1'] } else { &[b'0'] })
            .with_context(|| "failed to write to dat file")?;
        self.clk_file.write_all(&[b'1'])?;
        Ok(())
    }

    fn _write_u32(&mut self, value: u32) -> anyhow::Result<()> {
        for i in 0..32 {
            self._write_bit((value >> (31 - i)) & 1 == 1)?;
        }
        Ok(())
    }

    fn set_color(&mut self, color: u32) -> anyhow::Result<()> {
        self._write_u32(0x00000000)?;

        let value = color | 0xE0000000;

        for _ in 0..LedDevice::MAX_LED_COUNT {
            self._write_u32(value)?;
        }

        self._write_u32(0xFFFFFFFF)?;

        Ok(())
    }

    // fn set_rbg_color(&mut self, brightness: u8, r: u8, g: u8, b: u8) -> anyhow::Result<()> {
    //     self.set_color(
    //         ((brightness as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32),
    //     )
    // }
}

impl Drop for LedDevice {
    fn drop(&mut self) {
        self.dat_file.write_all(&[b'1']).unwrap();
        self.clk_file.write_all(&[b'1']).unwrap();

        std::thread::sleep(std::time::Duration::from_micros(10));
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("led_ctl: {:?}", args);

    check_device_files_exist(&args)?;

    let color_value: u32;

    if args.color_value.starts_with("0x") {
        color_value = u32::from_str_radix(&args.color_value[2..], 16)
            .with_context(|| format!("failed to parse color value '{}'", args.color_value))?;
    } else {
        color_value = u32::from_str_radix(&args.color_value, 10)
            .or_else(|_| u32::from_str_radix(&args.color_value, 16))
            .with_context(|| format!("failed to parse color value '{}'", args.color_value))?;
    }

    println!("color_value: 0x{:08x}", color_value);

    let mut led = LedDevice::open(
        &args.clk_path.to_string_lossy(),
        &args.dat_path.to_string_lossy(),
    )?;

    led.set_color(color_value)?;

    Ok(())
}
