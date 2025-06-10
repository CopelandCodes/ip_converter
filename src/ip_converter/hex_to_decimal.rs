// src/ip_converter/hex_to_decimal.rs

// This function converts an IP address from hexadecimal notation (e.g., "C0.A8.01.01")
// to its decimal representation (e.g., "192.168.1.1").

/// * ip is a string slice containing the hexadecimal IP address, with octets separated by dots.
/// * Returns a String containing the decimal representation of the IP address,
/// 
pub fn hex_to_decimal(ip: &str) -> String {
    // Split the IP address string by the '.' character into octets.
    let parts: Vec<&str> = ip.split('.').collect();
    
    // Create a vector to hold the decimal octets as strings.
    let mut dec_parts = Vec::new();

    // Iterate over each hexadecimal octet string.
    for part in parts {
        // Attempt to parse the hexadecimal string to a u8 number.
        if let Ok(num) = u8::from_str_radix(part, 16) {
            // If successful, convert the number back to a decimal string and push it to the vector.
            dec_parts.push(num.to_string());
        } else {
            // If parsing fails, return an error message.
            return "Invalid hex IP".to_string();
        }
    }

    // Join the decimal octets back together with a period separating them.
    dec_parts.join(".")
}
