use std::env;
use std::fs;
use std::process::{Command, Stdio};
use toml::Value;
use whoami;

fn main() {
    // Load or create config once, don't repeat for performance
    let config = load_or_create_default_config();

    // Get the editor from config, default to nvim
    let editor = config
        .get("editor")
        .and_then(|v| v.as_str())
        .unwrap_or("nvim");

    // Generate unique session name
    let session_name = generate_unique_session_name();

    // Fetch file argument (if any)
    let file_arg = env::args().nth(1).unwrap_or_default();

    // Build and execute tmux command efficiently
    let mut command = Command::new("tmux");
    command
        .arg("new-session")
        .arg("-s")
        .arg(&session_name)
        .arg(editor);

    if !file_arg.is_empty() {
        command.arg(&file_arg);
    }

    // Execute tmux command with inherited I/O to avoid bottleneck
    let _ = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
}

fn load_or_create_default_config() -> toml::value::Table {
    let config_path = dirs::home_dir()
        .map(|dir| dir.join(".config/n/config.toml"))
        .expect("Could not determine home directory");

    // Check if config already exists and return it
    if let Ok(config_content) = fs::read_to_string(&config_path) {
        if let Ok(parsed) = config_content.parse::<Value>() {
            return parsed.as_table().unwrap().clone();
        }
    }

    // Create config if it doesn't exist
    let default_config = r#"editor = "nvim""#;
    let _ = fs::write(&config_path, default_config);
    default_config
        .parse::<Value>()
        .unwrap()
        .as_table()
        .unwrap()
        .clone()
}

fn generate_unique_session_name() -> String {
    let base_session_name = format!(
        "{}@{}",
        whoami::username(),
        whoami::fallible::hostname().unwrap_or("unknown".to_string())
    );

    // Incrementally find an available session name
    (1..)
        .map(|suffix| format!("{}-{}", base_session_name, suffix))
        .find(|session_name| {
            Command::new("tmux")
                .arg("has-session")
                .arg("-t")
                .arg(session_name)
                .output()
                .map_or(false, |output| !output.status.success())
        })
        .unwrap()
}
