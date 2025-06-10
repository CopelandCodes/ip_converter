use ip_converter::IpConverter;
use utils::get_input;

// Declare local modules
mod ip_converter;
mod utils;
mod handlers;

fn main() {
    // Print a welcome message
    println!("\nIP Address Converter");
    let converter = IpConverter::new();

    // Main application loop
    loop {
        // Display the main menu options
        println!("\nPlease choose an option:");
        println!("1. Convert Decimal to Binary");
        println!("2. Convert Decimal to Hexadecimal");
        println!("3. Convert Binary to Decimal");
        println!("4. Convert Hexadecimal to Decimal");
        println!("0. Exit");

        // Prompt the user to enter their choice
        let choice = get_input("Enter choice: ");

        // Handle the user choice using match statements
        match choice.as_str() {
            "1" => handlers::handle_decimal_to_binary::handle(&converter),
            "2" => handlers::handle_decimal_to_hex::handle(&converter),
            "3" => handlers::handle_binary_to_decimal::handle(&converter),
            "4" => handlers::handle_hex_to_decimal::handle(&converter),
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
