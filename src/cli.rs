use std::process::Command;
use log::debug;

pub fn get_cli_commands() -> Vec<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg("compgen -c")
        .output()
        .expect("Failed to execute command");
    let commands = String::from_utf8_lossy(&output.stdout);
    let result: Vec<String> = commands
        .lines()
        .filter(|cmd| {
            cmd.len() > 1 && // Filtert Befehle mit nur einem Zeichen
            cmd.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') && // Erlaubt nur alphanumerische Zeichen, Unterstriche und Bindestriche
            !cmd.starts_with('-') // Filtert Optionen (beginnend mit '-') aus
        })
        .map(|s| s.to_string())
        .collect();
    debug!("Fetched {} filtered CLI commands", result.len());
    result
}
