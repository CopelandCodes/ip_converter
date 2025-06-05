// src/ip_converter/decimal_to_hex.rs

pub fn decimal_to_hex(ip: &str) -> String {
    let parts: Vec<&str> = ip.split('.').collect();
    let mut hex_parts = Vec::new();

    for part in parts {
        if let Ok(num) = part.parse::<u8>() {
            hex_parts.push(format!("{:02X}", num));
        } else {
            return "Invalid IP".to_string();
        }
    }

    hex_parts.join(".")
}