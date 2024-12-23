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

        // let mut response = input.splitn(2, ' ');
        // let command = response.next().unwrap_or("cmd");
        // let payload = response.next().unwrap_or("pld");

        handle_input(&input);
    }
}

fn handle_input(input: &str) {
    let command  = input.trim().split_whitespace().next().unwrap();
    let args = input.trim().split_whitespace().skip(1).collect::<Vec<_>>();

    match command {
        "exit" => std::process::exit(0),
        "echo" => println!("{}", args.join(" ")),
        _ => println!("{}: command not found", command),
    }
}
