pub use crate::install_functions;
pub use crate::utility_functions;
use colored::*;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

pub fn add_target(targets_file_path: &PathBuf, name: &String, address: &String) {
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

            if utility_functions::user_confirmation(
                "The entry already exists, would you like to update it? y/n".yellow(),
            ) {
                lines[index] = format!("{}={}", name, address);
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&targets_file_path)
                    .expect("Failed to open file for writing");

                for line in lines {
                    writeln!(file, "{}", line).expect("Failed to write line");
                }
                println!("{}", "Target updated".green());
            } else {
                println!("Target not updated");
            }
            break;
        }
    }
    if !entry_already_exists {
        let write_result = writeln!(&file, "{}={}", name, address);
        let error_message = "Failed to write to file";
        write_result
            .map(|_| println!("{}", "Target added".green()))
            .unwrap_or_else(|_| eprintln!("{}", error_message));
    }
}

pub fn remove_target(targets_file_path: &PathBuf, name: &String) {
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
        let mut file = File::create(&targets_file_path).expect("Failed to open file for writing");
        for line in lines {
            writeln!(file, "{}", line).expect("Failed to write line");
        }
        println!("{}", "Target removed".green());
    } else {
        println!("{}", "Target not found".yellow());
    }
}

pub fn list_targets(targets_file_path: &PathBuf) {
    let file = File::open(&targets_file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    println!("{}", "\nCurrent targets\n".magenta().bold());

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split('=').collect();
        println!("{}: {}", parts[0].bold().cyan(), parts[1]);
    }
}

pub fn purge_list(targets_file_path: &PathBuf) {
    if utility_functions::user_confirmation(
        "Warning: This action will erase all entries. Proceed? y/n".yellow(),
    ) {
        let _file = File::create(&targets_file_path).expect("Failed to purge file");
        println!("{}", "Target list purged".green());
    } else {
        println!("{}", "Aborting purge...".yellow());
    }
}

pub fn install_tip(targets_file_path: &PathBuf, exe_path: &PathBuf) {
    install_functions::target_list_validation(targets_file_path);
    install_functions::tip_config_validation(targets_file_path, exe_path);
    install_functions::shell_config_validation(exe_path);
}
