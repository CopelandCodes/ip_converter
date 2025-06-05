// src/ip_converter/mod.rs

pub mod decimal_to_binary;
pub mod binary_to_decimal;
pub mod hex_to_decimal;

pub struct IpConverter;

impl IpConverter {
    pub fn new() -> Self {
        IpConverter
    }

    pub fn decimal_to_bin_hex(&self, ip: &str) -> (String, String) {
        decimal_to_binary::decimal_to_binary(ip)
    }

    pub fn binary_to_decimal(&self, ip: &str) -> String {
        binary_to_decimal::binary_to_decimal(ip)
    }

    pub fn hex_to_decimal(&self, ip: &str) -> String {
        hex_to_decimal::hex_to_decimal(ip)
    }
}
