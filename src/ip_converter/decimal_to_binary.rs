// src/ip_converter/decimal_to_binary.rs

pub fn decimal_to_binary(ip: &str) -> String {
    let parts: Vec<&str> = ip.split('.').collect();
    let mut bin_parts = Vec::new();

    for part in parts {
        if let Ok(num) = part.parse::<u8>() {
            bin_parts.push(format!("{:08b}", num));
        } else {
            return "Invalid IP".to_string();
        }
    }

    bin_parts.join(".")
}