pub mod dsh {
    pub mod builtin;
    pub mod command;
    pub mod shell;
}

const PROGRAM_NAME: &'static str = "dsh";
const PROGRAM_VERSION: &'static str = "0.1";

fn main() -> std::io::Result<()> {
    let mut shell = dsh::shell::dsh::new();

    shell.add_command(Box::new(dsh::builtin::ExitCommand::new()));
    shell.add_command(Box::new(dsh::builtin::HelpCommand::new(
        PROGRAM_NAME,
        PROGRAM_VERSION,
    )));
    
    shell.dsh_loop();

    Ok(())
}
