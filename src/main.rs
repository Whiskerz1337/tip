// use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

mod command_functions;
pub mod install_functions;
pub mod utility_functions;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// adds a new target entry
    Add { name: String, address: String },
    /// lists the current target list
    List,
    /// purges the target list
    Purge,
    /// removes a single target entry by name
    Remove { name: String },
    /// adds neccessary scripts to the shell configuration file
    Install,
    /// removes shell configuration entry
    Uninstall,
}

fn main() {
    let cli = Cli::parse();

    let exe_path = std::env::current_exe().expect("Failed to get executable path");
    let main_dir = exe_path
        .parent()
        .expect("Failed to get parent directory of executable");

    let targets_file_path = main_dir.join("targets.txt");

    match &cli.command {
        Commands::Add { name, address } => {
            if let Err(e) = command_functions::add_target(&targets_file_path, name, address) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::Remove { name } => {
            if let Err(e) = command_functions::remove_target(&targets_file_path, name) {
                eprintln!("Error: {}", e)
            }
        }
        Commands::List => {
            if let Err(e) = command_functions::list_targets(&targets_file_path) {
                eprintln!("Error: {}", e)
            }
        }
        Commands::Purge => {
            if let Err(e) = command_functions::purge_list(&targets_file_path) {
                eprint!("Error: {}", e)
            }
        }
        Commands::Install => command_functions::install_tip(&targets_file_path, &exe_path),
        Commands::Uninstall => command_functions::uninstall_tip(&exe_path),
    }
}
