use anyhow::Result;
use std::io::{self, Write};
use std::path::PathBuf;

pub enum Builtin {
    CD,
    ECHO,
    EXIT,
    PWD,
    TYPE,
}

pub struct Shell {
    stdin: io::Stdin,
    stdout: io::Stdout,
    path: Vec<String>,
    home: PathBuf,
    current_dir: PathBuf,
}

#[derive(Debug)]
pub struct Command {
    name: String,
    args: Vec<String>,
}

fn is_builtin(name: &str) -> Option<Builtin> {
    match name {
        "cd"   => Some(Builtin::CD),
        "echo" => Some(Builtin::ECHO),
        "exit" => Some(Builtin::EXIT),
        "pwd"  => Some(Builtin::PWD),
        "type" => Some(Builtin::TYPE),
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
    fn new() -> Result<Self> {
        Ok(Self {
            stdin: io::stdin(),
            stdout: io::stdout(),
            path: Self::get_path(),
            home: Self::get_home(),
            current_dir: Shell::get_current_dir()?
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
            Err(_) => Ok(PathBuf::default())
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
            Builtin::CD => {
                let cmd = &command.args[0];

                if cmd.starts_with('~') {
                // Handle home directory navigation
                    if cmd == "~" {
                        let home_path = PathBuf::from(&self.home);
                        std::env::set_current_dir(&home_path)?;
                        self.current_dir = home_path;
                        Ok(())
                    } else {
                        println!("nulll");
                        Ok(())
                    }
                } else if cmd.starts_with('.') {
                // Handle relative path navigation
                    for part in cmd.split('/') {
                        match part {
                            "."  => continue, // Single dot (.) — stay in current directory
                            ".." => { self.current_dir.pop(); }, // Double dot (.) — move up one directory. Wrap in braces to return `()`.
                            ""   => continue, // Handle consecutive slashes
                            dir => self.current_dir.push(dir)
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
            Builtin::ECHO => {
                println!("{}", command.args.join(" "));
                Ok(())
            }
            Builtin::EXIT => std::process::exit(0),
            Builtin::PWD => {
                println!("{}", self.current_dir.display());
                Ok(())
            }
            Builtin::TYPE => {
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
