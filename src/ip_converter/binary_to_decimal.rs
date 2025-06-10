// src/ip_converter/binary_to_decimal.rs

// This module handles converting a binary IP address
// (e.g., "11000000.10101000.00000001.00000001") to decimal
// (e.g., "192.168.1.1").

/// * ip is a string slice containing the binary IP address.
/// * Returns a String containing the decimal representation, or an error message.
///

pub fn binary_to_decimal(ip: &str) -> String {
    // Split the IP address string into octets.
    let parts: Vec<&str> = ip.split('.').collect();
    let mut dec_parts = Vec::new();

    // Convert each octet from binary to decimal.
    for part in parts {
        if let Ok(num) = u8::from_str_radix(part, 2) {
            dec_parts.push(num.to_string());
        } else {
            // Return an error message if any octet is invalid.
            return "Invalid binary IP".to_string();
        }
    }

    // Join the decimal octets back together with a period separating them.
    dec_parts.join(".")
}

// Unit test for binary to decimal converter
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_to_decimal_valid() {
        let input = "11000000.10101000.00000001.00000001";
        let expected = "192.168.1.1";
        assert_eq!(binary_to_decimal(input), expected);
    }

    #[test]
    fn test_binary_to_decimal_invalid() {
        let input = "11000000.10101000.00000001.2";
        let expected = "Invalid binary IP";
        assert_eq!(binary_to_decimal(input), expected);
    }
}