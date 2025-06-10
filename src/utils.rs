use std::io::{self, Write};

///  * Prompts the user for input and returns the trimmed string.
///  * Returns a `String` containing the user's input with leading/trailing whitespace removed.

pub fn get_input(prompt: &str) -> String {
    // Print the prompt message without a newline so the user can type on the same line.
    print!("{}", prompt);

    // Flush the output buffer to ensure the prompt is displayed before waiting for input.
    io::stdout().flush().unwrap();

    // Create a mutable String variable to hold the user input.
    let mut input = String::new();

    // Read a line of input from the standard input (stdin).
    // `read_line` appends the input (including the newline) to the `input` string.
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Remove any leading/trailing whitespace (including the newline character).
    // Then return the cleaned-up string.
    input.trim().to_string()
}
