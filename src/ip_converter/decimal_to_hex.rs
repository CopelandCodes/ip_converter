// src/ip_converter/decimal_to_hex.rs

// This module handles converting a decimal IP address (e.g., "192.168.1.1")
// to its hexadecimal representation (e.g., "C0.A8.01.01").

/// * ip is a string slice containing the decimal IP address.
/// * Returns a String containing the hexadecimal representation, or an error message.
///
pub fn decimal_to_hex(ip: &str) -> String {
    // Split the IP address string into octets.
    let parts: Vec<&str> = ip.split('.').collect();
    let mut hex_parts = Vec::new();

    // Convert each octet to a 2-digit uppercase hexadecimal string.
    for part in parts {
        if let Ok(num) = part.parse::<u8>() {
            hex_parts.push(format!("{:02X}", num));
        } else {
            // Return an error message if any octet is invalid.
            return "Invalid IP".to_string();
        }
    }

    // Join the hexadecimal octets back together with a period separating them.
    hex_parts.join(".")
}

// Unit test for decimal to hex converter
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_hex_valid() {
        let input = "192.168.1.1";
        let expected = "C0.A8.01.01";
        assert_eq!(decimal_to_hex(input), expected);
    }

    #[test]
    fn test_decimal_to_hex_invalid() {
        let input = "192.999.1.1";
        let expected = "Invalid IP";
        assert_eq!(decimal_to_hex(input), expected);
    }
}
