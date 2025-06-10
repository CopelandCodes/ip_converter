// src/handlers/handle_binary_to_decimal.rs

use crate::utils::get_input;
use crate::ip_converter::IpConverter;

/// Handles user input and calls the binary_to_decimal conversion function.
///
/// Prompts the user for a binary IP address, converts it,
/// and prints the decimal representation.
pub fn handle(converter: &IpConverter) {
    let ip = get_input("Enter Binary IP Address (e.g., 11000000.10101000.00000001.00000001): ");
    let result = converter.binary_to_decimal(&ip);
    println!("Decimal: {}", result);
}
