// src/lib.rs

// Declare the ip_converter module
pub mod ip_converter;

// Re-export the IpConverter struct at the top level of the crate
pub use ip_converter::IpConverter;
