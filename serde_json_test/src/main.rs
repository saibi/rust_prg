use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize)]
struct BlinkRawData {
    dur: String,
    col: String,
    bri: u32,
}

fn test() -> Result<()> {
    let data = include_str!("../data/default.json");
    //let data = fs::read_to_string("data/default.json").expect("Unable to read file");
    let v: Value = serde_json::from_str(&data)?;
    let color_alias = v["color_alias"].as_object().unwrap();
    // let red = color_alias.get("red").unwrap();
    // println!("red : {}", red);
    // let red_str = red.as_str().unwrap();
    // let red_val = u32::from_str_radix(red_str, 16).unwrap();
    // println!("red value: 0x{:06x}", red_val);

    for (color, hex) in color_alias {
        let hex_str = hex.as_str().unwrap();
        let hex_val = u32::from_str_radix(hex_str, 16).unwrap();
        println!("{}: 0x{:06x}", color, hex_val);
    }

    let define = v["define"].as_object().unwrap();
    for (name, value) in define {
        let u32_val = value.as_u64().unwrap() as u32;
        println!("{}: {}", name, u32_val);
    }

    let blink_preset = v["blink_preset"].as_array().unwrap();

    for preset in blink_preset {
        let on_value = preset["on"].as_object().unwrap();
        let off_value = preset["off"].as_object().unwrap();

        println!("on: {} off: {}", on_value["dur"], off_value["dur"]);

        let blink_raw_data = BlinkRawData {
            dur: on_value["dur"].as_str().unwrap().to_string(),
            col: on_value["col"].as_str().unwrap().to_string(),
            bri: on_value["bri"].as_u64().unwrap() as u32,
        };
        println!(
            "blink_raw_data: {} {} {}",
            blink_raw_data.dur, blink_raw_data.col, blink_raw_data.bri
        );
    }

    Ok(())
}

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!(
        "Please call {} at the number {}, age {}",
        v["name"], v["phones"][0], v["age"]
    );

    Ok(())
}

fn main() {
    println!("read json!");
    // untyped_example().unwrap();
    test().unwrap();
}
