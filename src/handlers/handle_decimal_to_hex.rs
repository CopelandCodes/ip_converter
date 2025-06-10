// src/handlers/handle_decimal_to_hex.rs

use crate::ip_converter::IpConverter;
use crate::utils::get_input;

/// Handles user input and calls the decimal_to_hex conversion function.
///
/// Prompts the user for a decimal IP address, converts it,
/// and prints the hexadecimal representation.
pub fn handle(converter: &IpConverter) {
    let ip = get_input("Enter Decimal IP Address (e.g., 192.168.1.1): ");
    let result = converter.decimal_to_hex(&ip);
    println!("Hexadecimal: {}", result);
}
