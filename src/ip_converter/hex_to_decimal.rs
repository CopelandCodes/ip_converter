// src/ip_converter/hex_to_decimal.rs

pub fn hex_to_decimal(ip: &str) -> String {
    let parts: Vec<&str> = ip.split('.').collect();
    let mut dec_parts = Vec::new();

    for part in parts {
        if let Ok(num) = u8::from_str_radix(part, 16) {
            dec_parts.push(num.to_string());
        } else {
            return "Invalid hex IP".to_string();
        }
    }

    dec_parts.join(".")
}
