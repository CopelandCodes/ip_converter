mod ip_converter;
mod get_input;

use ip_converter::IpConverter;
use get_input::get_input;

fn main() {
    println!("Welcome to the IP Address Converter!");
    let converter = IpConverter::new();

    loop {
        println!("\nPlease choose an option:");
        println!("1. Convert Decimal IP to Binary and Hexadecimal");
        println!("2. Convert Binary IP to Decimal");
        println!("3. Convert Hexadecimal IP to Decimal");
        println!("4. Exit");

        let choice = get_input("Enter choice: ");

        match choice.as_str() {
            "1" => {
                let ip = get_input("Enter Decimal IP Address (e.g., 192.168.1.1): ");
                let (binary, hex) = converter.decimal_to_bin_hex(&ip);
                println!("Binary: {}", binary);
                println!("Hexadecimal: {}", hex);
            }
            "2" => {
                let ip = get_input("Enter Binary IP Address (e.g., 11000000.10101000.00000001.00000001): ");
                let decimal = converter.binary_to_decimal(&ip);
                println!("Decimal: {}", decimal);
            }
            "3" => {
                let ip = get_input("Enter Hexadecimal IP Address (e.g., C0.A8.01.01): ");
                let decimal = converter.hex_to_decimal(&ip);
                println!("Decimal: {}", decimal);
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
