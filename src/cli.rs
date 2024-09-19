use std::process::Command;
use log::debug;

pub fn get_cli_commands() -> Vec<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg("compgen -c")
        .output()
        .expect("Failed to execute command");
    let commands = String::from_utf8_lossy(&output.stdout);
    let result: Vec<String> = commands.lines().map(|s| s.to_string()).collect();
    debug!("Fetched {} CLI commands", result.len());
    result
}
