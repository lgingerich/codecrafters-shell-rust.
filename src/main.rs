use anyhow::Result;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;

pub enum Builtin {
    Cd,
    Echo,
    Exit,
    Pwd,
    Type,
}

pub struct Shell {
    stdin: io::Stdin,
    stdout: io::Stdout,
    path: Vec<String>,
}

#[derive(Debug)]
pub struct Command {
    name: String,
    args: Vec<String>,
}

fn is_builtin(name: &str) -> Option<Builtin> {
    match name {
        "cd"   => Some(Builtin::Cd),
        "echo" => Some(Builtin::Echo),
        "exit" => Some(Builtin::Exit),
        "pwd"  => Some(Builtin::Pwd),
        "type" => Some(Builtin::Type),
        _ => None,
    }
}

impl Command {
    fn new(input: String) -> Self {
        let mut parts = input.split_whitespace();
        Self {
            name: parts.next().unwrap_or("").to_string(),
            args: parts.map(String::from).collect(),
        }
    }
}

impl Shell {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            stdout: io::stdout(),
            path: Self::load_path(),
        }
    }

    fn load_path() -> Vec<String> {
        let path = std::env::var("PATH");
        match path {
            Ok(path) => path
                .split(':')
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            Err(_) => Vec::default(),
        }
    }

    fn in_path(&self, name: &str) -> Option<PathBuf> {
        self.path
            .iter()
            .map(|entry| PathBuf::from(entry).join(name))
            .find(|path| path.exists())
    }

    fn run(&mut self) -> Result<()> {
        loop {
            print!("$ ");
            self.stdout.flush()?;

            let mut input = String::new();
            self.stdin.read_line(&mut input)?;

            let command = Command::new(input);
            self.exec(command)?;
        }
    }

    fn exec(&mut self, command: Command) -> Result<()> {
        if let Some(builtin) = is_builtin(&command.name) {
            self.exec_builtin(builtin, &command)?;
        } else {
            self.exec_program(&command)?;
        };

        self.stdout.flush()?;

        Ok(())
    }

    fn exec_builtin(&mut self, builtin: Builtin, command: &Command) -> Result<()> {
        match builtin {
            Builtin::Cd => {
                // let new_path = std::env::set_current_dir(command.args[0].as_str());
                match std::env::set_current_dir(command.args[0].as_str()) {
                    Ok(_) => Ok(()),
                    Err(_) => {
                        println!("cd: {}: No such file or directory", command.args[0]);
                        Ok(())
                    }
                }
            }
            Builtin::Echo => {
                println!("{}", command.args.join(" "));
                Ok(())
            }
            Builtin::Exit => std::process::exit(0),
            Builtin::Pwd => {
                let current_path = std::env::current_dir()?;
                println!("{}", current_path.display());
                Ok(())
            }
            Builtin::Type => {
                let arg = &command.args[0];
                let message = match is_builtin(arg) {
                    Some(_) => format!("{} is a shell builtin", arg),
                    None => match self.in_path(arg) {
                        Some(entry) => format!("{} is {}", arg, entry.display()),
                        None => format!("{}: not found", arg),
                    },
                };
                println!("{}", message);
                Ok(())
            }
        }
    }

    fn exec_program(&mut self, command: &Command) -> Result<()> {
        match self.in_path(&command.name) {
            Some(entry) => {
                let proc = std::process::Command::new(entry)
                    .args(&command.args)
                    .output()?;
                self.stdout.write_all(&proc.stdout)?;
            }
            None => println!("{}: command not found", command.name),
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut shell = Shell::new();
    shell.run()
}
