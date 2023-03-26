use std::collections::HashSet;

use crate::dsh::command::Command;

use super::shell::dsh;

pub struct HelpCommand {
    name: &'static str,
    description: &'static str,
    program_name: &'static str,
    program_version: &'static str,
    aliases: HashSet<&'static str>,
}

impl HelpCommand {
    pub fn new(program_name: &'static str, program_version: &'static str) -> HelpCommand {
        HelpCommand {
            name: "help",
            description: "Displays this message",
            program_name,
            program_version,
            aliases: HashSet::from_iter(vec!["help", "??"].iter().cloned()),
        }
    }
}

impl Command for HelpCommand {
    fn get_aliases(&self) -> HashSet<&str> {
        self.aliases.clone()
    }

    fn execute(&self, dsh: &dsh, args: Vec<&str>) {
        if args.len() == 0 {
            println!(
                "{} v{} - a simple shell written in Rust",
                self.program_name, self.program_version
            );
            println!("Type 'help' or '??' to display this message");
            println!("Type 'exit' or 'qq' to exit the shell");
        } else {
            // if its a builtin, display the builtin help
            // if its a shell command, display the shell command help
            match dsh.get_command(args[0]) {
                Some(cmd) => {
                    println!("{} - {}", cmd.get_name(), cmd.get_description());
                }
                None => {
                    if cfg!(target_os = "windows") {
                        let shell_cmd = std::process::Command::new("cmd")
                            .arg("/C")
                            .arg("help")
                            .arg(args[0])
                            .spawn();
                        match shell_cmd {
                            Ok(mut child) => {
                                child.wait().unwrap();
                            }
                            Err(_) => {
                                println!("dsh: help: command not found: {}", args[0]);
                            }
                        }
                    } else {
                        let shell_cmd = std::process::Command::new("sh")
                            .arg("-c")
                            .arg("help")
                            .arg(args[0])
                            .spawn();
                        match shell_cmd {
                            Ok(mut child) => {
                                child.wait().unwrap();
                            }
                            Err(_) => {
                                println!("dsh: help: command not found: {}", args[0]);
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_description(&self) -> &'static str {
        self.description
    }
}

pub struct ExitCommand {
    name: &'static str,
    description: &'static str,
    aliases: HashSet<&'static str>,
}

impl ExitCommand {
    pub fn new() -> ExitCommand {
        ExitCommand {
            name: "exit",
            description: "Exits the shell",
            aliases: HashSet::from_iter(vec!["quit", "qq", "exit"].iter().cloned()),
        }
    }
}

impl Command for ExitCommand {
    fn get_aliases(&self) -> HashSet<&str> {
        self.aliases.clone()
    }

    fn execute(&self, dsh: &dsh, _args: Vec<&str>) {
        std::process::exit(0);
    }

    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_description(&self) -> &'static str {
        self.description
    }
}
