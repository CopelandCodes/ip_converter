use ip_converter::IpConverter;
use utils::get_input;

mod ip_converter;
mod utils;

fn main() {
    println!("\nIP Address Converter"); 
    let converter = IpConverter::new();

    loop {
        println!("\nPlease choose an option:");
        println!("1. Convert Decimal to Binary");
        println!("2. Convert Decimal to Hexadecimal");
        println!("3. Convert Binary to Decimal");
        println!("4. Convert Decimal to Hexadecimal");
        println!("5. Convert Hexadecimal to Decimal");
        println!("0. Exit");

        let choice = get_input("Enter choice: ");

        match choice.as_str() {
            "1" => {
                let ip = get_input("Enter Decimal IP Address (e.g., 192.168.1.1): ");
                let result = converter.decimal_to_binary(&ip);
                println!("Binary: {}", result);
            }
            "2" => {
                let ip = get_input("Enter Decimal IP Address (e.g., 192.168.1.1): ");
                let result = converter.decimal_to_hex(&ip);
                println!("Hexadecimal: {}", result);
            }
            "3" => {
                let ip = get_input("Enter Binary IP Address (e.g., 11000000.10101000.00000001.00000001): ");
                let result = converter.binary_to_decimal(&ip);
                println!("Decimal: {}", result);
            }
            "4" => {
                let ip = get_input("Enter Decimal IP Address (e.g., 192.168.1.1): ");
                let result = converter.decimal_to_hex(&ip);
                println!("Hexadecimal: {}", result);
            }
            "5" => {
                let ip = get_input("Enter Hexadecimal IP Address (e.g., C0.A8.01.01): ");
                let result = converter.hex_to_decimal(&ip);
                println!("Decimal: {}", result);
            }
            "0" => {
                println!("\nExiting...\n");
                break;
            }
            _ => {
                println!("\nInvalid choice, please try again.");
            }
        }
    }
}
