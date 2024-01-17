use clap::Command;
use std::process;

fn main() {
    let command = Command::new("Tasks")
        .version("0.1.0")
        .author("Nevo Golan")
        .about("Manage tasks CLI Tool.")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("ls").about("List of all the tasks"));

    let matches = command.get_matches();

    let command_result = match matches.subcommand_name() {
        Some("ls") => tasks::ls(),
        _ => Ok(()),
    };

    if let Err(e) = command_result {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
