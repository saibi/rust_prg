pub mod saibi_util;
use saibi_util::*;

use std::collections::HashMap;

use serde_json::{Result, Value};

#[derive(Debug)]
struct LevelVal {
    dur: u32,
    col: u32,
    bri: u32,
}

#[derive(Debug)]
struct BlinkPreset {
    cnt: u32,
    on: LevelVal,
    off: LevelVal,
}

#[derive(Debug)]
struct FadePreset {
    fadein: LevelVal,
    fadeout: LevelVal,
}

#[derive(Debug)]
struct GradPreset {
    dur: u32,
    bri: [u32; 2],
    colors: Vec<u32>,
}

struct Conf {
    color_alias: HashMap<String, u32>,
    define: HashMap<String, u32>,
    parameter: HashMap<String, u32>,
    blink_preset: Vec<BlinkPreset>,
    fade_preset: Vec<FadePreset>,
    grad_preset: Vec<GradPreset>,
}

impl Conf {
    fn new() -> Result<Conf> {
        let data = include_str!("../data/default.json");
        let v: Value = serde_json::from_str(&data)?;

        let color_alias = Self::value_to_color_alias(&v["color_alias"]);

        let define = Self::value_to_define(&v["define"]);

        let parameter = Self::value_to_parameter(&v["parameter"], &color_alias, &define);

        let blink_preset =
            Self::value_to_blink_preset(&v["preset"]["blink"], &color_alias, &define);
        let fade_preset = Self::value_to_fade_preset(&v["preset"]["fade"], &color_alias, &define);

        let grad_preset = Self::value_to_grad_preset(&v["preset"]["grad"], &color_alias, &define);

        Ok(Conf {
            color_alias,
            define,
            parameter,
            blink_preset,
            fade_preset,
            grad_preset,
        })
    }

    fn value_to_color_alias(value: &Value) -> HashMap<String, u32> {
        let mut color_alias = HashMap::new();
        let json_color_alias = value.as_object().unwrap();
        for (color, hex_str) in json_color_alias {
            let hex_str = hex_str.as_str().unwrap();
            let hex_val = color_rgb_str_to_u32(hex_str).unwrap();
            println!("{}: 0x{:06x}", color, hex_val);
            color_alias.insert(color.to_string(), hex_val);
        }
        color_alias
    }

    fn value_to_define(value: &Value) -> HashMap<String, u32> {
        let mut define = HashMap::new();
        let json_define = value.as_object().unwrap();
        for (name, value) in json_define {
            let u32_val = value.as_u64().unwrap() as u32;
            println!("{}: {}", name, u32_val);
            define.insert(name.to_string(), u32_val);
        }
        define
    }

    fn value_to_parameter(
        value: &Value,
        color_alias: &HashMap<String, u32>,
        define: &HashMap<String, u32>,
    ) -> HashMap<String, u32> {
        let mut parameter = HashMap::new();
        let json_parameter = value.as_object().unwrap();
        for (name, value) in json_parameter {
            if name == "col" {
                let u32_val = Self::value_to_color_u32(value, color_alias);
                println!("{}: 0x{:06x}", name, u32_val);
                parameter.insert(name.to_string(), u32_val);
                continue;
            }
            match value {
                Value::Number(n) => {
                    let u32_val = n.as_u64().unwrap() as u32;
                    println!("{}: {}", name, u32_val);
                    parameter.insert(name.to_string(), u32_val);
                }
                Value::String(s) => {
                    if let Some(define_val) = define.get(s) {
                        println!("{}: {}", name, define_val);
                        parameter.insert(name.to_string(), *define_val);
                    } else if let Ok(u32_val) = str_to_u32(&s) {
                        println!("{}: {}", name, u32_val);
                        parameter.insert(name.to_string(), u32_val);
                    }
                }
                _ => {}
            }
        }
        parameter
    }

    fn value_to_blink_preset(
        value: &Value,
        color_alias: &HashMap<String, u32>,
        define: &HashMap<String, u32>,
    ) -> Vec<BlinkPreset> {
        let mut list = Vec::new();
        let json_preset = value.as_array().unwrap();
        for entry in json_preset {
            let cnt = entry["cnt"].as_u64().unwrap() as u32;
            let on = Self::value_to_levelval(&entry["on"], &color_alias, &define);
            let off = Self::value_to_levelval(&entry["off"], &color_alias, &define);
            println!("cnt: {}", cnt);
            println!("on: dur: {}, col: {}, bri: {}", on.dur, on.col, on.bri);
            println!("off: dur: {}, col: {}, bri: {}", off.dur, off.col, off.bri);

            list.push(BlinkPreset { cnt, on, off });
        }
        list
    }

    fn value_to_fade_preset(
        value: &Value,
        color_alias: &HashMap<String, u32>,
        define: &HashMap<String, u32>,
    ) -> Vec<FadePreset> {
        let mut list = Vec::new();
        let json_preset = value.as_array().unwrap();
        for entry in json_preset {
            let fadein = Self::value_to_levelval(&entry["in"], &color_alias, &define);
            let fadeout = Self::value_to_levelval(&entry["out"], &color_alias, &define);
            println!(
                "fadein: dur: {}, col: {}, bri: {}",
                fadein.dur, fadein.col, fadein.bri
            );
            println!(
                "fadeout: dur: {}, col: {}, bri: {}",
                fadeout.dur, fadeout.col, fadeout.bri
            );
            list.push(FadePreset { fadein, fadeout });
        }
        list
    }

    fn value_to_grad_preset(
        value: &Value,
        color_alias: &HashMap<String, u32>,
        define: &HashMap<String, u32>,
    ) -> Vec<GradPreset> {
        let mut list = Vec::new();
        let json_preset = value.as_array().unwrap();
        for entry in json_preset {
            let dur = Self::value_to_u32(&entry["dur"], define);
            let start_bri = Self::value_to_u32(&entry["bri"][0], define);
            let end_bri = Self::value_to_u32(&entry["bri"][0], define);
            let colors = Self::value_to_colors(&entry["colors"], color_alias);

            list.push(GradPreset {
                dur,
                bri: [start_bri, end_bri],
                colors,
            });
        }

        list
    }

    fn value_to_colors(value: &Value, color_alias: &HashMap<String, u32>) -> Vec<u32> {
        let mut list = Vec::new();
        let json_colors = value.as_array().unwrap();
        for entry in json_colors {
            let color = Self::value_to_color_u32(entry, color_alias);
            list.push(color);
        }
        list
    }

    fn value_to_color_u32(value: &Value, color_alias: &HashMap<String, u32>) -> u32 {
        match value {
            Value::Number(n) => n.as_u64().unwrap() as u32,
            Value::String(s) => {
                if let Some(hex_val) = color_alias.get(s) {
                    *hex_val
                } else if let Ok(u32_val) = color_rgb_str_to_u32(&s) {
                    u32_val
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn value_to_u32(value: &Value, define: &HashMap<String, u32>) -> u32 {
        match value {
            Value::Number(n) => n.as_u64().unwrap() as u32,
            Value::String(s) => {
                if let Some(hex_val) = define.get(s) {
                    *hex_val
                } else if let Ok(u32_val) = str_to_u32(s) {
                    u32_val
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn value_to_levelval(
        value: &Value,
        color_alias: &HashMap<String, u32>,
        define: &HashMap<String, u32>,
    ) -> LevelVal {
        LevelVal {
            dur: Self::value_to_u32(&value["dur"], define),
            col: Self::value_to_color_u32(&value["col"], color_alias),
            bri: Self::value_to_u32(&value["bri"], define),
        }
    }
}

fn main() {
    println!("read json!");
    let conf = Conf::new().unwrap();
    println!("read json done!");

    println!("color_alias: {:?}", conf.color_alias);
    println!("define: {:?}", conf.define);
    println!("parameter: {:?}", conf.parameter);
    println!("blink_preset: {:?}", conf.blink_preset);
    println!("fade_preset: {:?}", conf.fade_preset);
    println!("grad_preset: {:?}", conf.grad_preset);
}
