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
        println!("Target removed successfully.");
    } else {
        println!("Target not found.");
    }
}

pub fn list_targets(targets_file_path: &PathBuf) {
    let file = File::open(&targets_file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        println!("{}", line);
    }
}

pub fn purge_list(targets_file_path: &PathBuf) {
    {
        let _file = File::create(&targets_file_path).expect("Failed to purge file");
    }
    println!("Target list purged successfully.");
}
