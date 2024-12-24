#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();

    // Define valid commands
    let valid_commands = vec!["echo", "exit", "ls", "type"];

    loop {
        // Print the prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read the input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // let path = std::env::var("PATH").unwrap()
        //     .split(':')
        //     .for_each(|dir| println!("{dir}"));

        // println!("{:?}", path);

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
            if args.len() != 1 {
                println!("{}: command not found", command);
                return;
            }

            let cmd = args[0];

            // Check for valid builtins
            if valid_commands.contains(&cmd) {
                println!("{} is a shell builtin", cmd);
                return;
            }

            // Search PATH for executable
            if let Ok(path) = std::env::var("PATH") {
                for dir in path.split(':') {
                    let cmd_path = format!("{}/{}", dir, cmd);
                    if std::path::Path::new(&cmd_path).exists() {
                        println!("{} is {}", cmd, cmd_path);
                        return;
                    }
                }
            }
            
            println!("{}: not found", cmd);
        }
        _ => println!("{}: command not found", command),
    }
            // match args.len() {
            //     2.. => println!("{}: command not found", command),
            //     1 => match valid_commands.contains(&args[0]) {
            //         true => match args[0] {
            //             "ls" => println!("{} is {}", args[0], std::env::var("PATH").unwrap_or_default()),
            //             _ => println!("{} is a shell builtin", args[0])
            //         },
            //         false => println!("{}: not found", args[0]),
            //     },
            //     _ => (),
            // }
        // },
        // _ => println!("{}: command not found", command),
    // }
}
