use crate::dsh::command::Command;
use std::{io::Write, str::SplitWhitespace};

pub struct dsh {
    commands: Vec<Box<dyn Command>>,
}

impl dsh {
    pub fn new() -> dsh {
        dsh {
            commands: Vec::new(),
        }
    }

    fn execute_shell_command(&self, command: &str, args: SplitWhitespace) -> bool {
        let shell_cmd = std::process::Command::new(command).args(args).spawn();
        match shell_cmd {
            Ok(mut child) => {
                child.wait().unwrap();
                true
            }
            Err(_) => {
                false
            }
        }
    }

    fn execute(&self, input: String) {
        if input.is_empty() {
            return;
        }

        let mut args = input.split_whitespace();
        let command = args.next().unwrap();

        let mut executed = false;

        for cmd in &self.commands {
            if cmd.get_aliases().contains(command) {
                cmd.execute();
                executed = true;
                break;
            }
        }

        // if the command was not found, run it as a shell command
        if !executed {
            if self.execute_shell_command(command, args) {
                executed = true;
            }
        }

        if !executed {
            println!("dsh: command not found: {}", command);
        }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    ///
    /// dsh.loop - main loop of the shell, reads input and executes commands
    ///
    pub fn dsh_loop(self) {
        let mut input = String::new();

        loop {
            // prompt the user for input
            print!("dsh > ");
            std::io::stdout().flush().unwrap();
            input = String::new();

            std::io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();

            self.execute(input);
        }
    }
}
