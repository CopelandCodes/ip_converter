// src/ip_converter/decimal_to_binary.rs

// This module handles converting a decimal IP address (e.g., "192.168.1.1")
// to its binary representation (e.g., "11000000.10101000.00000001.00000001").

/// * ip is a string slice containing the decimal IP address.
/// * Returns a String containing the binary representation, or an error message.
/// 
pub fn decimal_to_binary(ip: &str) -> String {
    // Split the IP address string into octets.
    let parts: Vec<&str> = ip.split('.').collect();
    let mut bin_parts = Vec::new();

    // Convert each octet to an 8-bit binary string.
    for part in parts {
        if let Ok(num) = part.parse::<u8>() {
            bin_parts.push(format!("{:08b}", num));
        } else {
            // Return an error message if any octet is invalid.
            return "Invalid IP".to_string();
        }
    }

    // Join the binary octets back together with a period separating them.
    bin_parts.join(".")
}

// Unit test for decimal to binary converter
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_binary_valid() {
        let input = "192.168.1.1";
        let expected = "11000000.10101000.00000001.00000001";
        assert_eq!(decimal_to_binary(input), expected);
    }

    #[test]
    fn test_decimal_to_binary_invalid() {
        let input = "999.168.1.1"; // Invalid octet
        let expected = "Invalid IP";
        assert_eq!(decimal_to_binary(input), expected);
    }
}