#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();

    loop {
        // Print the prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read the input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command = input.trim();
        handle_command(command);        
    }
}

fn handle_command(command: &str) {
    match command {
        "exit 0" => std::process::exit(0),
        _ => println!("{}: command not found", command),
    }
}
