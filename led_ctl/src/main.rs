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

    /// fade in/out sleep, in seconds, default 0
    #[arg(short, long, default_value_t = 0)]
    fade_in_sleep: u32,
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

    fn set_value(&mut self, val: u32) -> anyhow::Result<()> {
        self._write_u32(0x00000000)?;

        let value = val | 0xE0000000;

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

struct LedVal {
    /// brightness level, 0~31
    l: u8,

    /// red color value, 0~255
    r: u8,

    /// green color value, 0~255
    g: u8,

    /// blue color value, 0~255
    b: u8,
}

impl LedVal {
    // const MAX_BRIGHTNESS: u8 = 31;

    // fn new(l: u8, r: u8, g: u8, b: u8) -> Option<LedVal> {
    //     if l <= LedVal::MAX_BRIGHTNESS {
    //         Some(LedVal { l, r, g, b })
    //     } else {
    //         None
    //     }
    // }
    fn from_value(val: u32) -> LedVal {
        LedVal {
            l: ((val >> 24) & 0x1F) as u8,
            r: ((val >> 16) & 0xFF) as u8,
            g: ((val >> 8) & 0xFF) as u8,
            b: (val & 0xFF) as u8,
        }
    }

    fn to_u32(&self) -> u32 {
        ((self.l as u32) << 24) | ((self.b as u32) << 16) | ((self.g as u32) << 8) | (self.r as u32)
    }

    fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }

    fn inc_brightness(&mut self) -> bool {
        if self.l < 31 {
            self.l += 1;
            true
        } else {
            false
        }
    }

    fn dec_brightness(&mut self) -> bool {
        if self.l > 0 {
            self.l -= 1;
            true
        } else {
            false
        }
    }
}

struct LedControl {
    led: LedDevice,
    val: LedVal,
}

impl LedControl {
    fn new(led: LedDevice, val: u32) -> LedControl {
        LedControl {
            led,
            val: LedVal::from_value(val),
        }
    }

    fn set_value(&mut self, val: u32) {
        self.led
            .set_value(LedVal::from_value(val).to_u32())
            .unwrap();
    }

    fn fade_in(&mut self) {
        self.val.l = 0;
        loop {
            self.led.set_value(self.val.to_u32()).unwrap();

            if !self.val.inc_brightness() {
                break;
            }
        }
    }
    fn fade_out(&mut self) {
        loop {
            self.led.set_value(self.val.to_u32()).unwrap();

            if !self.val.dec_brightness() {
                break;
            }
        }
        self.val.set_rgb(0, 0, 0);
    }
}

fn parse_color_value(color_value: &str) -> Result<u32, std::num::ParseIntError> {
    if color_value.starts_with("0x") {
        u32::from_str_radix(&color_value[2..], 16)
    } else if color_value.len() != 8 {
        u32::from_str_radix(&color_value, 10).or_else(|_| u32::from_str_radix(&color_value, 16))
    } else {
        u32::from_str_radix(&color_value, 16).or_else(|_| u32::from_str_radix(&color_value, 10))
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("led_ctl: {:?}", args);

    check_device_files_exist(&args)?;

    let color_value: u32 = parse_color_value(&args.color_value)
        .with_context(|| format!("failed to parse color value '{}'", args.color_value))?;

    println!("color_value: 0x{:08x}", color_value);

    let led = LedDevice::open(
        &args.clk_path.to_string_lossy(),
        &args.dat_path.to_string_lossy(),
    )?;

    let mut led_ctl = LedControl::new(led, color_value);
    if args.fade_in_sleep == 0 {
        led_ctl.set_value(color_value);
    } else {
        println!("DBG fade_in");
        led_ctl.fade_in();
        println!("DBG sleep");
        std::thread::sleep(std::time::Duration::from_secs(args.fade_in_sleep as u64));
        println!("DBG fade_out");
        led_ctl.fade_out();
    }

    Ok(())
}
