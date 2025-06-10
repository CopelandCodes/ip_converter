// src/handlers/handle_decimal_to_binary.rs

use crate::ip_converter::IpConverter;
use crate::utils::get_input;

/// Handles user input and calls the decimal_to_binary conversion function.
///
/// Prompts the user for a decimal IP address, converts it,
/// and prints the binary representation.
pub fn handle(converter: &IpConverter) {
    let ip = get_input("Enter Decimal IP Address (192.168.1.1): ");
    let result = converter.decimal_to_binary(&ip);
    println!("Binary: {}", result);
}
