#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();

    // Define valid commands
    let valid_commands = vec!["echo", "exit", "type"];

    loop {
        // Print the prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read the input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // Parse and handle input
        handle_input(&input, &valid_commands);
    }
}

fn handle_input(input: &str, valid_commands: &[&str]) {
    let command  = input.split_whitespace().next().unwrap();
    let args = input.split_whitespace().skip(1).collect::<Vec<_>>();

    match command {
        "exit" => std::process::exit(0),
        "echo" => println!("{}", args.join(" ")),
        "type" => {
            match args.len() {
                2.. => println!("{}: command not found", command),
                1 => match valid_commands.contains(&args[0]) {
                    true => println!("{} is a shell builtin", args[0]),
                    false => println!("{}: not found", args[0]),
                },
                _ => (),
            }
        },
        _ => println!("{}: command not found", command),
    }
}
