use std::process::Command;
use log::debug;
use std::io;

pub fn get_manpage(command: &str) -> Result<String, io::Error> {
    debug!("Fetching manpage for command: {}", command);
    let output = Command::new("man")
        .arg(command)
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, error.to_string()))
    }
}
