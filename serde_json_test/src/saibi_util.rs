/// convert string to u32: "0x1f010e03" or "1f010e03" or "123"
pub fn str_to_u32(value: &str) -> Result<u32, std::num::ParseIntError> {
    if value.starts_with("0x") {
        u32::from_str_radix(&value[2..], 16)
    } else if value.len() != 8 {
        // if length is not 8, try to parse as decimal first
        u32::from_str_radix(&value, 10).or_else(|_| u32::from_str_radix(&value, 16))
    } else if value.starts_with("0") || value.contains(|c: char| c < '0' || c > '9') {
        u32::from_str_radix(&value, 16)
    } else {
        u32::from_str_radix(&value, 10)
    }
}

/// convert color rgb string to u32: "0xRRGGBB" or "RRGGBB" or "RGB"
pub fn color_rgb_str_to_u32(value: &str) -> Result<u32, std::num::ParseIntError> {
    if value.starts_with("0x") {
        u32::from_str_radix(&value[2..], 16)
    } else if value.len() == 3 {
        // if length is 3, convert to 6 digits
        let mut value_6 = String::with_capacity(6);
        for c in value.chars() {
            value_6.push(c);
            value_6.push(c);
        }
        u32::from_str_radix(&value_6, 16)
    } else {
        u32::from_str_radix(&value, 16)
    }
}
