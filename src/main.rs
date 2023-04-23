#![allow(non_snake_case)]
use clap::{Parser, Subcommand};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

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
            let file = OpenOptions::new()
                .read(true)
                .create(true)
                .append(true)
                .open(&targets_file_path)
                .expect("Failed to open file");

            let reader = BufReader::new(&file);
            let mut lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
            let mut entry_already_exists = false;

            for (index, line) in lines.iter().enumerate() {
                let entry_name = line.split('=').next().unwrap_or("");
                if entry_name == name {
                    entry_already_exists = true;

                    println!("The entry already exists. Do you want to update it? (y/n)");
                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");

                    if input.trim().to_lowercase() == "y" {
                        lines[index] = format!("{}={}", name, address);
                        let mut file = OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .open(&targets_file_path)
                            .expect("Failed to open file for writing");

                        for line in lines {
                            writeln!(file, "{}", line).expect("Failed to write line");
                        }
                        println!("Target updated successfully.");
                    } else {
                        println!("Target not updated.");
                    }
                    break;
                }
            }

            if !entry_already_exists {
                let write_result = writeln!(&file, "{}={}", name, address);
                let success_message = "Target added successfully.";
                let error_message = "Failed to write to file.";
                write_result
                    .map(|_| println!("{}", success_message))
                    .unwrap_or_else(|_| eprintln!("{}", error_message));
            }
        }

        Commands::List => {
            let file = File::open(&targets_file_path).expect("Failed to open file");
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line.expect("Failed to read line");
                println!("{}", line);
            }
        }
        Commands::Purge => {
            {
                let _file = File::create(&targets_file_path).expect("Failed to purge file");
            }
            println!("Target list purged successfully.");
        }
        Commands::Remove { name } => {
            let file = File::open(&targets_file_path).expect("Failed to open file");
            let reader = BufReader::new(file);

            let mut lines: Vec<String> = Vec::new();
            let mut target_found = false;

            for line in reader.lines() {
                let line = line.expect("Failed to read line");
                let entry_name = line.split('=').next().unwrap_or("");
                if entry_name != name {
                    lines.push(line);
                } else {
                    target_found = true;
                }
            }
            if target_found {
                let mut file =
                    File::create(&targets_file_path).expect("Failed to open file for writing");
                for line in lines {
                    writeln!(file, "{}", line).expect("Failed to write line");
                }
                println!("Target removed successfully.");
            } else {
                println!("Target not found.");
            }
        }
        Commands::Install => {
            let shell_config_path = match env::var("SHELL") {
                Ok(shell_path) => {
                    let shell = shell_path.rsplit('/').next().unwrap_or("");
                    match shell {
                        "bash" => {
                            let mut home =
                                dirs::home_dir().expect("Could not find the home directory.");
                            home.push(".bashrc");
                            home
                        }
                        "zsh" => {
                            let mut home =
                                dirs::home_dir().expect("Could not find the home directory.");
                            home.push(".zshrc");
                            home
                        }
                        _ => panic!("Unsupported shell: {}", shell),
                    }
                }
                Err(_) => panic!("Could not detect the shell."),
            };

            ensure_shell_config_updated(&shell_config_path, &targets_file_path);
            println!("\nShell configuration updated successfully.");
            println!(
                "\nPlease either restart the shell, or run 'source {}' to load changes.",
                shell_config_path.to_string_lossy()
            );

            // Add the shell function to the config file
            let mut shell_config_file = OpenOptions::new()
                .append(true)
                .open(&shell_config_path)
                .expect("Failed to open shell config file for appending");

            writeln!(
                shell_config_file,
                "\n# shell function to allow sourcing \nfunction tip() {{\n  {} \"$@\"\n  source {}\n}}",
                exe_path.display(),
                shell_config_path.display()
            )
            .expect("Failed to write shell function to the config file");
        }
    }

    fn ensure_shell_config_updated(shell_config_path: &PathBuf, targets_file_path: &PathBuf) {
        let targets_file_path_str = targets_file_path
            .to_str()
            .expect("Failed to convert targets.txt path to string");

        let config_update = format!(
            "\n# Adds tip install folder to PATH if not already added\nif [[ \":$PATH:\" != *\":{}:\"* ]]; then\n    export PATH=\"$PATH:{}\"\nfi\n\n# Begin tip configuration\nfunction load_targets() {{\n    while IFS='=' read -r name address; do\n        export \"$name=$address\"\n    done < \"{}\"\n}}\n\n# Call the load_targets function during shell initialization\nload_targets",
            std::env::current_dir().unwrap().to_str().unwrap(),
            std::env::current_dir().unwrap().to_str().unwrap(),
            targets_file_path_str
        );

        let file = File::open(shell_config_path).expect("Failed to open shell config file");
        let reader = BufReader::new(file);
        let already_updated = reader.lines().any(|line| {
            line.as_ref()
                .map_or(false, |l| l.contains("# Begin tip configuration"))
        });

        if !already_updated {
            let mut file = OpenOptions::new()
                .append(true)
                .open(shell_config_path)
                .expect("Failed to open shell config file for appending");

            writeln!(file, "{}", config_update).expect("Failed to write tip configuration");
        }
    }
}
