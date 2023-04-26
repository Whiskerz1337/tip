use crate::utility_functions;
use colored::Colorize;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn get_shell_config_path() -> PathBuf {
    let shell_config_path = match std::env::var("SHELL") {
        Ok(shell_path) => {
            let shell = shell_path.rsplit('/').next().unwrap_or("");
            match shell {
                "bash" => {
                    let mut home = dirs::home_dir().expect("Could not find the home directory.");
                    home.push(".bashrc");
                    home
                }
                "zsh" => {
                    let mut home = dirs::home_dir().expect("Could not find the home directory.");
                    home.push(".zshrc");
                    home
                }
                _ => panic!("Unsupported shell: {}", shell),
            }
        }
        Err(_) => panic!("Could not detect the shell."),
    };
    shell_config_path
}

pub fn tip_config_is_sourced(
    shell_config_path: &PathBuf,
    full_tip_config_path: &Path,
) -> std::io::Result<(bool, bool)> {
    let comment = "# Source tip configuration";
    let source_line = format!("source {}", full_tip_config_path.to_string_lossy());

    let file = std::fs::File::open(shell_config_path)?;
    let reader = std::io::BufReader::new(file);

    let (comment_present, source_line_present) =
        reader.lines().fold((false, false), |acc, line| {
            let line = line.unwrap_or_default();
            let comment_present = acc.0 || line.contains(comment);
            let source_line_present = acc.1 || line.contains(&source_line);
            (comment_present, source_line_present)
        });

    Ok((!comment_present, !source_line_present))
}

pub fn source_tip_config(
    shell_config_path: &PathBuf,
    full_tip_config_path: &Path,
) -> std::io::Result<()> {
    let comment = "# Source tip configuration";
    let source_line = format!("source {}", full_tip_config_path.to_string_lossy());

    let mut file = OpenOptions::new()
        .append(true)
        .open(shell_config_path)
        .expect("Failed to open shell config file for appending");

    writeln!(file, "\n{}", comment)?;
    writeln!(file, "{}", source_line)?;

    Ok(())
}

pub fn get_full_path(directory: &Path, relative_path: &PathBuf) -> PathBuf {
    directory.join(relative_path)
}

pub fn shell_config_validation(exe_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let tip_config_path = get_tip_config_path();
    let shell_config_path = get_shell_config_path();
    let parent_directory = exe_path
        .parent()
        .ok_or("Failed to get parent directory of executable")?;

    let full_tip_config_path = get_full_path(parent_directory, &tip_config_path);

    let (comment_not_present, source_line_not_present) =
        tip_config_is_sourced(&shell_config_path, &full_tip_config_path)?;

    if comment_not_present || source_line_not_present {
        source_tip_config(&shell_config_path, &full_tip_config_path)?;
        println!("tip configuration file sourced {}", "successfully".green());
    } else {
        println!("{}", "The tip configuration is already sourced.".yellow());
    }
    Ok(())
}

pub fn target_list_exists(targets_file_path: &Path) -> bool {
    targets_file_path.exists() && targets_file_path.is_file()
}

pub fn tip_config_exists(tip_config_path: &Path) -> bool {
    tip_config_path.exists() && tip_config_path.is_file()
}

pub fn create_empty_target_list(targets_file_path: &PathBuf) -> std::io::Result<()> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(targets_file_path)?;

    Ok(())
}

pub fn target_list_validation(
    targets_file_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    if !target_list_exists(targets_file_path) {
        create_empty_target_list(targets_file_path)?;
        println!("targets.txt created {}", "successfully".green());
    } else if utility_functions::user_confirmation(
        "Target list found. Would you like to wipe the list? y/n".yellow(),
    ) {
        create_empty_target_list(targets_file_path)?;
        println!("Target list wiped {}", "successfully".green());
    }
    Ok(())
}

pub fn create_tip_config(
    targets_file_path: &Path,
    exe_path: &Path,
    tip_config_path: &Path,
    full_tip_config_path: &Path,
) -> Result<PathBuf, Box<dyn Error>> {
    let binding = std::env::current_dir()?;
    let current_dir = binding.to_str().unwrap();
    let targets_file_path_str = targets_file_path.to_string_lossy();
    let config_dir = PathBuf::from("config");
    std::fs::create_dir_all(config_dir)?;
    let config_update = format!(
        "# Adds tip install folder to PATH if not already added\nif [[ \":$PATH:\" != *\":{}:\"* ]]; then\n    export PATH=\"$PATH:{}\"\nfi\n\n# Begin tip configuration\nfunction load_targets() {{\n    while IFS='=' read -r name address; do\n        export \"$name=$address\"\n    done < \"{}\"\n}}\n\n# Call the load_targets function during shell initialization\nload_targets\n\n# Shell function to allow sourcing\nfunction tip() {{\n  {} \"$@\"\n  source {}\n}}",
        current_dir, current_dir, targets_file_path_str, exe_path.display(), full_tip_config_path.display()
    );

    std::fs::write(tip_config_path, config_update)?;
    Ok(tip_config_path.to_path_buf())
}

pub fn get_tip_config_path() -> PathBuf {
    PathBuf::from("config/tip-config.sh")
}

pub fn tip_config_validation(
    targets_file_path: &Path,
    exe_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = PathBuf::from("config");
    let tip_config_path = config_dir.join("tip-config.sh");
    let parent_directory = exe_path
        .parent()
        .ok_or("Failed to get parent directory of executable")?;
    let full_tip_config_path = get_full_path(parent_directory, &tip_config_path);

    if !tip_config_exists(&full_tip_config_path) {
        create_tip_config(
            targets_file_path,
            exe_path,
            &tip_config_path,
            &full_tip_config_path,
        )?;
        println!("Configuration file created {}", "successfully".green());
    } else if utility_functions::user_confirmation(
        "Found configuration file. Would you like to reset it? y/n".yellow(),
    ) {
        create_tip_config(
            targets_file_path,
            exe_path,
            &tip_config_path,
            &full_tip_config_path,
        )?;
        println!("Configuration file reset {}", "successfully.".green());
    }
    Ok(())
}

pub fn remove_shell_source_line(exe_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let tip_config_path = get_tip_config_path();
    let shell_config_path = get_shell_config_path();
    let parent_directory = exe_path
        .parent()
        .ok_or("Failed to get parent directory of executable")?;

    let full_tip_config_path = get_full_path(parent_directory, &tip_config_path);

    let (comment_not_present, source_line_not_present) =
        tip_config_is_sourced(&shell_config_path, &full_tip_config_path)?;

    if !comment_not_present || !source_line_not_present {
        delete_config_lines(&shell_config_path, &full_tip_config_path)?;
        println!(
            "\ntip configuration sourcing removed {}",
            "sucessfully".green()
        );
    } else {
        println!("{}", "The tip configuration is not sourced.".yellow());
    }
    Ok(())
}

pub fn delete_config_lines(
    shell_config_path: &PathBuf,
    full_tip_config_path: &Path,
) -> std::io::Result<()> {
    let comment = "# Source tip configuration";
    let source_line = format!("source {}", full_tip_config_path.to_string_lossy());

    let content = std::fs::read_to_string(shell_config_path)?;
    let lines: Vec<&str> = content.lines().collect();

    let filtered_lines: Vec<&str> = lines
        .iter()
        .filter(|&line| line.trim() != comment && line.trim() != source_line)
        .copied()
        .collect();

    let mut file = std::fs::File::create(shell_config_path)?;
    for line in filtered_lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
