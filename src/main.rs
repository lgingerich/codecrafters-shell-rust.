#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        // Print the prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read the input
        stdin.read_line(&mut input).unwrap();

        // Print the input
        println!("{}: command not found", input.trim());

        // Clear the input
        input = String::new();
    }
}
