use std::env;
use std::fs;
use std::process::{Command, Stdio};
use toml::Value;

fn main() {
    // Load or create config once, don't repeat for performance
    let config = load_or_create_default_config();

    // Get the editor from config, default to nvim
    let editor = config
        .get("editor")
        .and_then(|v| v.as_str())
        .unwrap_or("nvim");

    // Fetch file argument (if any) and additional arguments
    let args: Vec<String> = env::args().skip(1).collect();

    // Check if we're already inside tmux or zellij
    if env::var("TMUX").is_ok() {
        // If inside tmux, just run the editor directly
        run_editor(&editor, &args);
    } else if env::var("ZELLIJ").is_ok() {
        // If inside zellij, run the editor directly
        run_editor(&editor, &args);
    } else {
        // Create a session in Zellij with the custom layout
        create_zellij_session(&editor, &args);
    }
}

fn run_editor(editor: &str, args: &[String]) {
    let mut command = Command::new(editor);
    command.args(args); // Pass file args directly to the editor

    // Execute the editor command with inherited I/O
    let _ = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
}

fn create_zellij_session(editor: &str, args: &[String]) {
    // Inject arguments into the existing layout file and run Zellij
    let layout_file_path = modify_existing_layout(editor, args);

    // Start Zellij with the modified layout
    let _ = Command::new("zellij")
        .arg("--layout")
        .arg(&layout_file_path)
        .status()
        .expect("Failed to start Zellij with the custom layout");
}



fn modify_existing_layout(editor: &str, args: &[String]) -> String {
    // Path to the existing layout file
    let layout_file_path = format!("{}/.config/zellij/layout.kdl", env::var("HOME").unwrap());

    // Read the layout file content
    let mut layout_content = fs::read_to_string(&layout_file_path)
        .expect("Unable to read the layout file");

    // Build the correct KDL structure with command and args
    let pane_block = if !args.is_empty() {
        // If there are arguments, pass them using the `args` block with "--"
        format!(
            "pane {{\n    command \"{}\"\n    args \"--\" \"{}\"\n    focus true\n}}",
            editor,
            args.join(" ")  // This becomes `args "--" ".cargo-lock"`
        )
    } else {
        // No arguments, just run the editor
        format!(
            "pane {{\n    command \"{}\"\n    focus true\n}}",
            editor
        )
    };

    // Replace the existing `nvim` pane in the layout file
    layout_content = layout_content.replace("pane command=\"nvim\"", &pane_block);

    // Write the modified layout back to a temporary layout file
    let temp_layout_path = format!("{}/.config/zellij/temp_layout.kdl", env::var("HOME").unwrap());
    fs::write(&temp_layout_path, layout_content).expect("Unable to write temporary layout file");

    // Return the path of the temporary layout file
    temp_layout_path
}







fn load_or_create_default_config() -> toml::value::Table {
    let config_path = dirs::home_dir()
        .map(|dir| dir.join(".config/txr/config.toml"))
        .expect("Could not determine home directory");

    // Check if config already exists and return it
    if let Ok(config_content) = fs::read_to_string(&config_path) {
        if let Ok(parsed) = config_content.parse::<Value>() {
            return parsed.as_table().unwrap().clone();
        }
    }

    // Create config if it doesn't exist
    let default_config = r#"
    editor = "nvim"
    multiplexer = "tmux"
    "#;
    let _ = fs::write(&config_path, default_config);
    default_config
        .parse::<Value>()
        .unwrap()
        .as_table()
        .unwrap()
        .clone()
}

