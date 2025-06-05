// src/ip_converter/binary_to_decimal.rs

pub fn binary_to_decimal(ip: &str) -> String {
    let parts: Vec<&str> = ip.split('.').collect();
    let mut dec_parts = Vec::new();

    for part in parts {
        if let Ok(num) = u8::from_str_radix(part, 2) {
            dec_parts.push(num.to_string());
        } else {
            return "Invalid binary IP".to_string();
        }
    }

    dec_parts.join(".")
}
