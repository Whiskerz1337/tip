pub use crate::install_functions;
pub use crate::utility_functions;
use colored::Colorize;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

pub fn add_target(
    targets_file_path: &PathBuf,
    name: &String,
    address: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(targets_file_path)?;

    let reader = BufReader::new(&file);
    let mut lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut entry_already_exists = false;

    for (index, line) in lines.iter().enumerate() {
        let entry_name = line.split('=').next().unwrap_or("");
        if entry_name == name {
            entry_already_exists = true;

            if utility_functions::user_confirmation(
                "The entry already exists, would you like to update it? y/n".yellow(),
            ) {
                lines[index] = format!("{}={}", name, address);
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(targets_file_path)?;

                for line in lines {
                    writeln!(file, "{}", line)?;
                }
                println!("{}", "Target updated".green());
            } else {
                println!("Target not updated");
            }
            break;
        }
    }
    if !entry_already_exists {
        let mut file = OpenOptions::new().append(true).open(targets_file_path)?;
        writeln!(&mut file, "{}={}", name, address)?;
        println!("{}", "Target added".green());
    }

    Ok(())
}

pub fn remove_target(
    targets_file_path: &PathBuf,
    name: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(targets_file_path)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut target_found = false;

    for line in reader.lines() {
        let line = line?;
        let entry_name = line.split('=').next().unwrap_or("");
        if entry_name != name {
            lines.push(line);
        } else {
            target_found = true;
        }
    }

    if target_found {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(targets_file_path)?;

        for line in lines {
            writeln!(file, "{}", line)?;
        }
        println!("{}", "Target removed".green());
    } else {
        println!("{}", "Target not found".yellow());
    }

    Ok(())
}

pub fn list_targets(targets_file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(targets_file_path)?;
    let reader = BufReader::new(file);

    let mut has_entries = false;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            if !has_entries {
                println!("{}", "\nCurrent targets\n".magenta().bold().underline());
                has_entries = true;
            }
            println!("{}: {}", parts[0].bold().cyan(), parts[1]);
        } else {
            eprintln!("Invalid target entry: {}", line);
        }
    }

    if !has_entries {
        println!("{}", "No current targets".yellow());
    } else {
        println!("\n");
    }

    Ok(())
}

pub fn purge_list(targets_file_path: &PathBuf) {
    if utility_functions::user_confirmation(
        "Warning: This action will erase all entries. Proceed? y/n".yellow(),
    ) {
        let _file = File::create(targets_file_path).expect("Failed to purge file");
        println!("{}", "Target list purged".green());
    } else {
        println!("{}", "Aborting purge...".yellow());
    }
}

pub fn install_tip(targets_file_path: &PathBuf, exe_path: &Path) {
    install_functions::target_list_validation(targets_file_path);
    install_functions::tip_config_validation(targets_file_path, exe_path);
    install_functions::shell_config_validation(exe_path);
}
