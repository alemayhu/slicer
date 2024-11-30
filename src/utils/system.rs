use std::process::Command;

pub fn get_current_username() -> String {
    Command::new("whoami")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string())
        .trim()
        .to_string()
} 