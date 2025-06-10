// src/mod.rs

// * This file is the main entry point for the ip_converter library.
// * It organizes and re-exports all submodules and defines the IpConverter struct
// * as a namespace for IP address conversions.

pub mod binary_to_decimal;
pub mod decimal_to_binary;
pub mod decimal_to_hex;
pub mod hex_to_decimal;

/// Struct that acts as a namespace for IP address conversion methods.
pub struct IpConverter;

impl IpConverter {
    /// Creates a new instance of IpConverter.
    pub fn new() -> Self {
        IpConverter
    }

    /// Converts a decimal IP address to its binary representation.
    ///
    /// * ip is a string slice containing the decimal IP address (e.g., "192.168.1.1").
    /// * Returns a string containing the binary representation or an error message.
    ///
    pub fn decimal_to_binary(&self, ip: &str) -> String {
        decimal_to_binary::decimal_to_binary(ip)
    }

    /// Converts a decimal IP address to its hexadecimal representation.
    ///
    /// * ip is a string slice containing the decimal IP address (e.g., "192.168.1.1").
    /// * Returns a string containing the hexadecimal representation or an error message.
    ///
    pub fn decimal_to_hex(&self, ip: &str) -> String {
        decimal_to_hex::decimal_to_hex(ip)
    }

    /// Converts a binary IP address to its decimal representation.
    ///
    /// * ip is a string slice containing the binary IP address (e.g., "11000000.10101000.00000001.00000001").
    /// * Returns a string containing the decimal representation or an error message.
    ///
    pub fn binary_to_decimal(&self, ip: &str) -> String {
        binary_to_decimal::binary_to_decimal(ip)
    }

    /// Converts a hexadecimal IP address to its decimal representation.
    ///
    /// * ip is a string slice containing the hexadecimal IP address (e.g., "C0.A8.01.01").
    /// * Returns a string containing the decimal representation or an error message.
    ///
    pub fn hex_to_decimal(&self, ip: &str) -> String {
        hex_to_decimal::hex_to_decimal(ip)
    }
}
