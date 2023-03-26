
use std::collections::HashSet;

use crate::dsh::command::Command;

pub struct HelpCommand {
    name: &'static str,
    description: &'static str,
    program_name: &'static str,
    program_version: &'static str,
    aliases: HashSet<&'static str>
}

impl HelpCommand {
    pub fn new(program_name: &'static str, program_version: &'static str) -> HelpCommand {
        HelpCommand {
            name: "help",
            description: "Displays this message",
            program_name,
            program_version,
            aliases: HashSet::from_iter(vec![ "help", "??" ].iter().cloned())
        }
    }
}

impl Command for HelpCommand {

    fn get_aliases(&self) -> HashSet<&str> {
        self.aliases.clone()
    }

    fn execute(&self) {
        println!("{} v{} - a simple shell written in Rust", self.program_name, self.program_version);
        println!("Type 'help' or '??' to display this message");
        println!("Type 'exit' or 'qq' to exit the shell");
    }
}

pub struct ExitCommand {
    name: &'static str,
    description: &'static str,
    aliases: HashSet<&'static str>
}

impl ExitCommand {
    pub fn new() -> ExitCommand {
        ExitCommand {
            name: "exit",
            description: "Exits the shell",
            aliases: HashSet::from_iter(vec![ "quit", "qq", "exit" ].iter().cloned())
        }
    }
}

impl Command for ExitCommand {
    fn get_aliases(&self) -> HashSet<&str> {
        self.aliases.clone()
    }

    fn execute(&self) {
        std::process::exit(0);
    }
}