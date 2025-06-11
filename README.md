# Overview

I wrote this IP Address Converter in Rust to demonstrate key language features and data types in a math-based application. The program offers an interactive menu that converts IP addresses between decimal, binary, and hexadecimal formats. It uses String and &str for user input, Vec<&str> to split octets, and u8 for numeric conversions. Rust features like match statements, Result and Option, and ownership and borrowing were essential in managing parsing, error handling, and modular design. This project reinforced my understanding of Rust syntax, type safety, and module organization while building a practical, user-friendly tool.


# Development Environment

Editor: Visual Studio Code

Compiler: Rust (installed via rustup)

Language: Rust (Edition 2024)

Tooling:
- cargo for building and running the project
- cargo test for running unit tests
- cargo fmt for automatic code formatting

# Useful Websites

{Make a list of websites that you found helpful in this project}

- [The Rust Standard Library](https://doc.rust-lang.org/std/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustfmt Guide](https://github.com/rust-lang/rustfmt)
- [The Cargo Book](https://doc.rust-lang.org/cargo/) 

# Future Work

- Improve input validation to handle incomplete or malformed IP addresses more gracefully.
- Implement support for IPv6 address conversion.
- Add command-line argument support so users can run conversions directly without using interactive prompts.
- Extend unit tests to cover more edge cases and invalid input scenarios.
