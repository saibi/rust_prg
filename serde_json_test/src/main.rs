use std::collections::HashMap;

use serde_json::Value;

struct LevelVal {
    dur: u32,
    col: u32,
    bri: u32,
}

impl LevelVal {}

struct BlinkPreset {
    cnt: u32,
    on: LevelVal,
    off: LevelVal,
}

struct Conf {
    color_alias: HashMap<String, u32>,
    define: HashMap<String, u32>,
    parameter: HashMap<String, u32>,
    blink_preset: Vec<BlinkPreset>,
}

impl Conf {
    fn new() -> serde_json::Result<Conf> {
        let data = include_str!("../data/default.json");
        let v: Value = serde_json::from_str(&data)?;
        let json_color_alias = v["color_alias"].as_object().unwrap();
        let mut color_alias = HashMap::new();

        for (color, hex_str) in json_color_alias {
            let hex_str = hex_str.as_str().unwrap();
            let hex_val = u32::from_str_radix(hex_str, 16).unwrap();
            println!("{}: 0x{:06x}", color, hex_val);
            color_alias.insert(color.to_string(), hex_val);
        }

        let json_define = v["define"].as_object().unwrap();
        let mut define = HashMap::new();
        for (name, value) in json_define {
            let u32_val = value.as_u64().unwrap() as u32;
            println!("{}: {}", name, u32_val);
            define.insert(name.to_string(), u32_val);
        }
        let json_parameter = v["parameter"].as_object().unwrap();
        let mut parameter = HashMap::new();
        for (name, value) in json_parameter {
            match value {
                Value::Number(n) => {
                    let u32_val = n.as_u64().unwrap() as u32;
                    println!("{}: {}", name, u32_val);
                    parameter.insert(name.to_string(), u32_val);
                }
                Value::String(s) => {
                    if let Ok(u32_val) = Self::parse_value_str(&s) {
                        println!("{}: {}", name, u32_val);
                        parameter.insert(name.to_string(), u32_val);
                    } else if let Some(define_val) = define.get(s) {
                        println!("{}: {}", name, define_val);
                        parameter.insert(name.to_string(), *define_val);
                    }
                }
                _ => {}
            }
        }

        let json_preset = v["preset"].as_object().unwrap();
        let json_blink_preset = json_preset["blink"].as_array().unwrap();
        for preset in json_blink_preset {
            let cnt = preset["cnt"].as_u64().unwrap() as u32;
            let on = LevelVal {
                dur: preset["on"]["dur"].as_u64().unwrap() as u32,
                col: preset["on"]["col"].as_u64().unwrap() as u32,
                bri: preset["on"]["bri"].as_u64().unwrap() as u32,
            };
            let off = LevelVal {
                dur: preset["off"]["dur"].as_u64().unwrap() as u32,
                col: preset["off"]["col"].as_u64().unwrap() as u32,
                bri: preset["off"]["bri"].as_u64().unwrap() as u32,
            };
            println!("cnt: {}", cnt);
            println!("on: dur: {}, col: {}, bri: {}", on.dur, on.col, on.bri);
            println!("off: dur: {}, col: {}, bri: {}", off.dur, off.col, off.bri);
        }

        Ok(Conf {
            color_alias,
            define,
            parameter,
            blink_preset: Vec::new(),
        })
    }

    fn parse_value_str(value: &str) -> Result<u32, std::num::ParseIntError> {
        if value.starts_with("0x") {
            u32::from_str_radix(&value[2..], 16)
        } else if value.len() != 8 {
            u32::from_str_radix(&value, 10).or_else(|_| u32::from_str_radix(&value, 16))
        } else {
            u32::from_str_radix(&value, 16)
        }
    }

    fn get_value_or_from_define(&self, value: &Value) -> u32 {
        match value {
            Value::Number(n) => n.as_u64().unwrap() as u32,
            Value::String(s) => {
                if let Ok(u32_val) = Self::parse_value_str(&s) {
                    u32_val
                } else if let Some(define_val) = self.define.get(s) {
                    *define_val
                } else {
                    0
                }
            }
            _ => 0,
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
}
