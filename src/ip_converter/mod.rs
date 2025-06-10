// This module serves as the main entry point for the ip_converter library.
// It imports and re-exports all the submodules.

pub mod decimal_to_binary;
pub mod decimal_to_hex;
pub mod binary_to_decimal;
pub mod hex_to_decimal;

/// Struct that acts as a namespace for IP address conversions.
pub struct IpConverter;

impl IpConverter {
    /// Creates a new IpConverter instance.
    pub fn new() -> Self {
        IpConverter
    }

    /// Converts a decimal IP address to its binary representation.
    pub fn decimal_to_binary(&self, ip: &str) -> String {
        decimal_to_binary::decimal_to_binary(ip)
    }

    /// Converts a decimal IP address to its hexadecimal representation.
    pub fn decimal_to_hex(&self, ip: &str) -> String {
        decimal_to_hex::decimal_to_hex(ip)
    }

    /// Converts a binary IP address to its decimal representation.
    pub fn binary_to_decimal(&self, ip: &str) -> String {
        binary_to_decimal::binary_to_decimal(ip)
    }

    /// Converts a hexadecimal IP address to its decimal representation.
    pub fn hex_to_decimal(&self, ip: &str) -> String {
        hex_to_decimal::hex_to_decimal(ip)
    }
}
