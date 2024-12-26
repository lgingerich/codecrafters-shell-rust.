#![warn(unused_variables)]

use anyhow::Result;
use std::io::{self, Write};
use std::path::PathBuf;

enum Builtin {
    Cd,
    Echo,
    Exit,
    Pwd,
    Type,
}

struct Shell {
    stdin: io::Stdin,
    stdout: io::Stdout,
    path: Vec<String>,
    home: PathBuf,
    current_dir: PathBuf,
}

#[derive(Debug)]
struct Command {
    name: String,
    args: Vec<String>,
}

fn is_builtin(name: &str) -> Option<Builtin> {
    match name {
        "cd" => Some(Builtin::Cd),
        "echo" => Some(Builtin::Echo),
        "exit" => Some(Builtin::Exit),
        "pwd" => Some(Builtin::Pwd),
        "type" => Some(Builtin::Type),
        _ => None,
    }
}

impl Command {
    fn new(input: String) -> Self {
        let mut split = input.trim_end().splitn(2, ' ');
        let first = split.next().unwrap_or("").trim().to_string();
        let rest = split.next().map(|s| s.to_string());
        let args = rest.map_or(Vec::new(), Self::parse_arguments);
        Self { name: first, args }
    }

    fn parse_arguments(input: String) -> Vec<String> {
        let mut chars = input.chars().peekable();
        let mut current = String::new();
        let mut args = Vec::new();
        let mut in_single_quotes = false;
        let mut in_double_quotes = false;
        let mut escaped = false;

        while let Some(c) = chars.next() {
            match c {
                // Toggle single quote mode if not in double quotes and not escaped
                '\'' if !in_double_quotes && !escaped => {
                    in_single_quotes = !in_single_quotes;
                    continue; // Skip adding the quote character itself
                }
                // Inside single quotes, treat everything literally including backslashes
                c if in_single_quotes => current.push(c),
                // Backslash handling - treat it as literal in double quotes unless escaping special chars
                '\\' if !escaped && in_double_quotes => {
                    if let Some(&next) = chars.peek() {
                        if next == '\\' || next == '$' || next == '"' || next == '\n' {
                            escaped = true;
                        } else {
                            // If not escaping special char in double quotes, treat as literal
                            current.push('\\');
                        }
                    }
                }
                // Backslash outside quotes - always escape next character
                '\\' if !escaped => {
                    escaped = true;
                }
                // Handle escaped character
                c if escaped => {
                    current.push(c);
                    escaped = false;
                }
                // Toggle double quote mode if not in single quotes and not escaped
                '"' if !in_single_quotes && !escaped => in_double_quotes = !in_double_quotes,
                // Space is delimiter only when not in quotes and not escaped
                ' ' if !in_single_quotes && !in_double_quotes && !escaped => {
                    if !current.is_empty() {
                        args.push(current.clone());
                        current.clear();
                    }
                }
                // Add all other characters literally
                _ => current.push(c),
            }
        }

        // Push final argument if buffer not empty
        if !current.is_empty() {
            args.push(current);
        }

        args
    }
}

impl Shell {
    fn new() -> Result<Self> {
        Ok(Self {
            stdin: io::stdin(),
            stdout: io::stdout(),
            path: Self::get_path(),
            home: Self::get_home(),
            current_dir: Shell::get_current_dir()?,
        })
    }

    fn get_path() -> Vec<String> {
        let path = std::env::var("PATH");
        match path {
            Ok(path) => path
                .split(':')
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            Err(_) => Vec::default(),
        }
    }

    fn get_home() -> PathBuf {
        std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/"))
    }

    fn get_current_dir() -> Result<PathBuf> {
        let current_path = std::env::current_dir();
        match current_path {
            Ok(path) => Ok(path),
            Err(_) => Ok(PathBuf::default()),
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
            // println!("Command: {:?}", command);
            self.exec(command)?;
            self.stdout.flush()?;
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
                let cmd = &command.args[0];

                if cmd.starts_with('~') {
                    // Handle home directory navigation
                    if cmd == "~" {
                        let home_path = PathBuf::from(&self.home);
                        std::env::set_current_dir(&home_path)?;
                        self.current_dir = home_path;
                        Ok(())
                    } else {
                        Ok(()) // TODO: What needs to be implemented here?
                    }
                } else if cmd.starts_with('.') {
                    // Handle relative path navigation
                    for part in cmd.split('/') {
                        match part {
                            "." => continue, // Single dot (.) — stay in current directory
                            ".." => {
                                self.current_dir.pop();
                            } // Double dot (.) — move up one directory. Wrap in braces to return `()`.
                            "" => continue, // Handle consecutive slashes
                            dir => self.current_dir.push(dir),
                        }
                    }
                    std::env::set_current_dir(&self.current_dir)?;
                    Ok(())
                } else {
                    // Handle absolute path navigation
                    let new_dir = PathBuf::from(cmd);
                    if std::env::set_current_dir(&new_dir).is_ok() {
                        self.current_dir = new_dir;
                        Ok(())
                    } else {
                        println!("cd: {}: No such file or directory", cmd);
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
                println!("{}", self.current_dir.display());
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
    Shell::new()?.run()
}
