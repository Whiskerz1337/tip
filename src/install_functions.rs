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
    tip_config_path: &PathBuf,
) -> std::io::Result<(bool, bool)> {
    let comment = "# Source tip configuration";
    let source_line = format!("source {}", tip_config_path.to_string_lossy());

    let file = std::fs::File::open(shell_config_path)?;
    let reader = std::io::BufReader::new(file);

    let (comment_present, source_line_present) =
        reader.lines().fold((false, false), |acc, line| {
            let line = line.unwrap_or_default();
            let comment_present = acc.0 || line.contains(comment);
            let source_line_present = acc.1 || line.contains(&source_line);
            (comment_present, source_line_present)
        });

    Ok((comment_present, source_line_present))
}

pub fn source_tip_config(
    shell_config_path: &PathBuf,
    tip_config_path: &PathBuf,
) -> std::io::Result<()> {
    let comment = "# Source tip configuration";
    let source_line = format!("source {}", tip_config_path.to_string_lossy());

    let mut file = OpenOptions::new()
        .append(true)
        .open(shell_config_path)
        .expect("Failed to open shell config file for appending");

    writeln!(file, "\n{}", comment)?;
    writeln!(file, "{}", source_line)?;

    Ok(())
}

pub fn target_list_exists(targets_file_path: &PathBuf) -> bool {
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

pub fn target_list_validation(targets_file_path: &PathBuf) {
    println!("Beginning tip installation...");
    if !target_list_exists(&targets_file_path) {
        if let Err(e) = create_empty_target_list(&targets_file_path) {
            eprintln!("Failed to create targets.txt: {}", e);
        } else {
            println!("targets.txt created sucessfully.");
        }
    } else {
        println!("Found targets.txt.")
    }
}

pub fn create_tip_config(
    targets_file_path: &PathBuf,
    exe_path: &PathBuf,
) -> Result<PathBuf, Box<dyn Error>> {
    let binding = std::env::current_dir()?;
    let current_dir = binding.to_str().unwrap();
    let targets_file_path_str = targets_file_path.to_string_lossy();

    // Create the 'config' directory in the current folder
    let config_dir = PathBuf::from("config");
    std::fs::create_dir_all(&config_dir)?;

    // Set the tip_config_path to be inside the 'config' directory
    let tip_config_path = config_dir.join("tip-config.sh");

    let config_update = format!(
        "# Adds tip install folder to PATH if not already added\nif [[ \":$PATH:\" != *\":{}:\"* ]]; then\n    export PATH=\"$PATH:{}\"\nfi\n\n# Begin tip configuration\nfunction load_targets() {{\n    while IFS='=' read -r name address; do\n        export \"$name=$address\"\n    done < \"{}\"\n}}\n\n# Call the load_targets function during shell initialization\nload_targets\n\n# Shell function to allow sourcing\nfunction tip() {{\n  {} \"$@\"\n  source {}\n}}",
        current_dir, current_dir, targets_file_path_str, exe_path.display(), tip_config_path.display()
    );

    std::fs::write(&tip_config_path, config_update)?;
    Ok(tip_config_path)
}
