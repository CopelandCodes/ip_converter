// src/handlers/handle_hex_to_decimal.rs

use crate::utils::get_input;
use crate::ip_converter::IpConverter;

/// Handles user input and calls the hex_to_decimal conversion function.
///
/// Prompts the user for a hexadecimal IP address, converts it,
/// and prints the decimal representation.
pub fn handle(converter: &IpConverter) {
    let ip = get_input("Enter Hexadecimal IP Address (e.g., C0.A8.01.01): ");
    let result = converter.hex_to_decimal(&ip);
    println!("Decimal: {}", result);
}
