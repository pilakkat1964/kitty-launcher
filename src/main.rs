//! # Kitty Terminal Launcher
//!
//! A robust Rust wrapper for the kitty terminal emulator that allows launching
//! terminal sessions with flexible configuration presets.
//!
//! ## Features
//! - Validates input parameters to ensure they're safe and valid
//! - Searches for session configuration files in multiple standard locations
//! - Provides helpful error messages when something goes wrong
//! - Launches kitty terminal with the specified session preset
//! - Creates new session configuration files from templates
//! - Comprehensive help and man page documentation
//!
//! ## Configuration Search Path
//! The program searches for session configuration files in this order:
//! 1. `./etc/kitty/sessions` (current directory)
//! 2. `~/.local/etc/kitty/sessions` (user home directory)
//! 3. `/opt/etc/kitty/sessions` (optional system-wide)
//! 4. `~/.config/kitty/sessions` (kitty standard location)
//!
//! ## Usage
//! ```
//! kitty-launcher <session-name>              # Launch a session
//! kitty-launcher --create <name>             # Create a new session file
//! kitty-launcher --help                      # Show help
//! ```

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

const VERSION: &str = "0.4.0";

/// Represents the configuration for the kitty launcher application.
///
/// This struct contains the session name that the user wants to launch
/// and the resolved path to the configuration file.
struct LauncherConfig {
    /// The name of the session to launch (e.g., "dev", "work", "default")
    session_name: String,
    /// The full path to the configuration file that will be used
    config_path: PathBuf,
}

/// Prints comprehensive help message
fn print_help() {
    println!(
        "kitty-launcher v{} - Kitty Terminal Session Launcher",
        VERSION
    );
    println!();
    println!("SYNOPSIS:");
    println!("    A session manager for the kitty terminal emulator that allows you to:");
    println!("    • Launch kitty with predefined session configurations");
    println!("    • Generate session templates at organized search paths");
    println!("    • Create .desktop launcher files for convenient GUI access");
    println!();
    println!("USAGE:");
    println!("    kitty-launcher [OPTIONS] [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("    <SESSION_NAME>                              Launch a kitty session");
    println!("    -c, --create <NAME>                         Create a session template file");
    println!("    -l, --create-launcher <NAME> [SESSION]      Create a .desktop launcher file");
    println!(
        "    --generate-completions <SHELL>              Generate shell completions (bash|zsh)"
    );
    println!("    -h, --help                                  Show this help message");
    println!("    -V, --version                               Show version information");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help                                  Display help");
    println!("    -V, --version                               Display version");
    println!();
    println!("QUICK START:");
    println!("    # Create a session template");
    println!("    kitty-launcher -c my-session");
    println!("    $EDITOR ~/.local/etc/kitty/sessions/my-session.session");
    println!();
    println!("    # Launch the session");
    println!("    kitty-launcher my-session");
    println!();
    println!("    # Create a launcher file for GUI access");
    println!("    kitty-launcher -l 'My Session' my-session");
    println!();
    println!("SESSION SEARCH PATHS (in order of priority):");
    println!("    1. ./etc/kitty/sessions/");
    println!("    2. ~/.local/etc/kitty/sessions/");
    println!("    3. /opt/etc/kitty/sessions/");
    println!("    4. ~/.config/kitty/sessions/");
    println!();
    println!("SESSION FILE DISCOVERY:");
    println!("    • Looks for exact name first, then tries <NAME>.session");
    println!("    • Valid characters: alphanumeric, hyphens, underscores, dots");
    println!();
    println!("CREATING SESSION TEMPLATES:");
    println!("    kitty-launcher -c <NAME> generates a session file at:");
    println!("    ~/.local/etc/kitty/sessions/<NAME>.session");
    println!();
    println!("    Uses z-tools.session as template if available, otherwise creates a basic one.");
    println!("    Edit the generated file to customize your session.");
    println!();
    println!("CREATING LAUNCHER FILES:");
    println!("    kitty-launcher -l <LAUNCHER_NAME> [SESSION] generates a .desktop file.");
    println!("    If SESSION is omitted, <LAUNCHER_NAME> is used as the session name.");
    println!("    Files are created in ~/.local/share/applications/ for desktop access.");
    println!();
    println!("SHELL COMPLETIONS:");
    println!("    bash:  kitty-launcher --generate-completions bash >> ~/.bashrc");
    println!("    zsh:   kitty-launcher --generate-completions zsh >> ~/.zshrc");
    println!();
    println!("For more information: https://github.com/pilakkat1964/kitty-launcher");
}

/// Prints version information
fn print_version() {
    println!("kitty-launcher v{}", VERSION);
    println!("A robust Rust wrapper for the kitty terminal emulator");
    println!("License: MIT");
}

/// Validates the session name to ensure it's safe for use.
///
/// This function checks that the session name:
/// - Is not empty
/// - Contains only alphanumeric characters, hyphens, underscores, and dots
/// - Is not a special value like "." or ".."
///
/// # Arguments
/// * `name` - The session name to validate
///
/// # Returns
/// * `Ok(())` if the name is valid
/// * `Err(String)` if the name is invalid, with a descriptive error message
fn validate_session_name(name: &str) -> Result<(), String> {
    // Check if the name is empty
    if name.is_empty() {
        return Err("Session name cannot be empty".to_string());
    }

    // Check if the name is trying to traverse directories (path traversal attack)
    if name.contains('/') || name.contains('\\') || name == "." || name == ".." {
        return Err(format!(
            "Invalid session name: '{}'. Session names cannot contain path separators or special directory names.",
            name
        ));
    }

    // Check if the name contains only valid characters
    // Valid characters: alphanumeric, hyphens, underscores, and dots
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        return Err(format!(
            "Invalid session name: '{}'. Only alphanumeric characters, hyphens, underscores, and dots are allowed.",
            name
        ));
    }

    Ok(())
}

/// Gets the home directory path for the current user.
///
/// This function attempts to get the user's home directory from the `HOME`
/// environment variable. If it's not set, returns None.
///
/// # Returns
/// * `Some(PathBuf)` - The home directory path if available
/// * `None` - If the home directory cannot be determined
fn get_home_dir() -> Option<PathBuf> {
    env::var("HOME").ok().map(PathBuf::from)
}

/// Finds the configuration file for a given session name.
///
/// This function searches for the session configuration file in the standard
/// locations in the following order:
/// 1. `./etc/kitty/sessions/` (current directory)
/// 2. `~/.local/etc/kitty/sessions/` (user home directory)
/// 3. `/opt/etc/kitty/sessions/` (optional system-wide)
/// 4. `~/.config/kitty/sessions/` (kitty standard location)
///
/// If the session name doesn't already end with `.session`, the function will
/// first try to find the file as-is, then retry with `.session` extension appended.
///
/// The function returns the path to the first configuration file found.
/// If no file is found, it returns an error with suggestions and lists all tried paths.
///
/// # Arguments
/// * `session_name` - The name of the session (with or without .session extension)
///
/// # Returns
/// * `Ok(PathBuf)` - The path to the found configuration file
/// * `Err(String)` - An error message listing all attempted paths
fn find_config_file(session_name: &str) -> Result<PathBuf, String> {
    // Define the search paths in order of preference
    // Sessions are stored in a dedicated ./sessions subfolder to avoid
    // conflicts with kitty's own configuration files
    let mut search_paths: Vec<PathBuf> = vec![
        // Current directory (highest priority)
        PathBuf::from("./etc/kitty/sessions"),
    ];

    // Add user's local configuration directory if home dir is available
    if let Some(home) = get_home_dir() {
        search_paths.push(home.join(".local/etc/kitty/sessions"));
    }

    // Add optional system-wide directory
    search_paths.push(PathBuf::from("/opt/etc/kitty/sessions"));

    // Add kitty's standard configuration directory if home dir is available
    if let Some(home) = get_home_dir() {
        search_paths.push(home.join(".config/kitty/sessions"));
    }

    // Build list of session names to try: first the original, then with .session extension
    // (unless it already ends with .session)
    let session_names_to_try = if session_name.ends_with(".session") {
        vec![session_name.to_string()]
    } else {
        vec![
            session_name.to_string(),
            format!("{}.session", session_name),
        ]
    };

    // Track all attempted paths for error reporting
    let mut attempted_paths: Vec<PathBuf> = Vec::new();

    // Try each search path with each session name variant
    for search_path in search_paths.iter() {
        for session_variant in session_names_to_try.iter() {
            let config_file = search_path.join(session_variant);
            attempted_paths.push(config_file.clone());

            // Check if the file exists
            if config_file.exists() {
                // Verify it's actually a file (not a directory)
                if config_file.is_file() {
                    return Ok(config_file);
                }
            }
        }
    }

    // If we get here, no configuration file was found
    // Build a detailed error message with all attempted paths
    let mut error_msg = format!(
        "Configuration file for session '{}' not found.\n\n\
         Attempted paths:\n",
        session_name
    );

    for (i, path) in attempted_paths.iter().enumerate() {
        let display_path =
            if let Ok(abs_path) = std::fs::canonicalize(path.parent().unwrap_or(Path::new("."))) {
                abs_path.join(path.file_name().unwrap_or_default())
            } else {
                path.clone()
            };
        error_msg.push_str(&format!("  {}. {}\n", i + 1, display_path.display()));
    }

    error_msg.push_str(&format!(
        "\nPlease create a configuration file named '{}' (or '{}.session') in one of these directories.",
        session_name, session_name
    ));

    Err(error_msg)
}

/// Creates a new session configuration file from a template.
///
/// This function:
/// 1. Validates the session name
/// 2. Finds or creates the ~/.local/etc/kitty/sessions directory
/// 3. Reads the template file (z-tools.session)
/// 4. Creates a new session file with the provided name
///
/// # Arguments
/// * `name` - The name of the new session (without .session extension)
///
/// # Returns
/// * `Ok(PathBuf)` - The path to the created session file
/// * `Err(String)` - An error message if creation failed
fn create_session_file(name: &str) -> Result<PathBuf, String> {
    // Validate the session name
    validate_session_name(name)?;

    // Get home directory
    let home = get_home_dir().ok_or_else(|| "Could not determine home directory".to_string())?;

    // Define the session directory (with sessions subfolder for isolation)
    let session_dir = home.join(".local/etc/kitty/sessions");

    // Create the directory if it doesn't exist
    fs::create_dir_all(&session_dir).map_err(|e| {
        format!(
            "Failed to create directory {}: {}",
            session_dir.display(),
            e
        )
    })?;

    // Define template and new file paths
    let template_path = session_dir.join("z-tools.session");
    let new_file_path = session_dir.join(format!("{}.session", name));

    // Check if the new file already exists
    if new_file_path.exists() {
        return Err(format!(
            "Session file already exists: {}",
            new_file_path.display()
        ));
    }

    // Read the template file
    let template_content = if template_path.exists() {
        fs::read_to_string(&template_path).map_err(|e| {
            format!(
                "Failed to read template file {}: {}",
                template_path.display(),
                e
            )
        })?
    } else {
        // If no template exists, create a basic one
        create_default_template()
    };

    // Write the new session file
    fs::write(&new_file_path, template_content).map_err(|e| {
        format!(
            "Failed to create session file {}: {}",
            new_file_path.display(),
            e
        )
    })?;

    Ok(new_file_path)
}

/// Creates a default template if z-tools.session doesn't exist
fn create_default_template() -> String {
    r#"# Kitty Session Configuration
# Edit this file to customize your terminal session
# For more information, see: https://sw.kovidgoyal.net/kitty/conf/

# Define the first tab
new_tab Main
  launch

# Define the second tab
new_tab Development
  launch
"#
    .to_string()
}

/// Creates a .desktop file for launching a kitty session from application menus.
///
/// This function:
/// 1. Validates the launcher name (same rules as session names)
/// 2. Creates ~/.local/share/applications directory if needed
/// 3. Generates a standard .desktop file for the session
/// 4. Saves the .desktop file in the applications directory
///
/// The .desktop file can be used by desktop environments to add the launcher
/// to application menus and allow quick access to the session.
///
/// # Arguments
/// * `name` - The name for the launcher (e.g., "dev", "work-session")
/// * `session_name` - The session configuration to launch
///
/// # Returns
/// * `Ok(PathBuf)` - Path to the created .desktop file
/// * `Err(String)` - Error description if creation fails
fn create_launcher_file(name: &str, session_name: &str) -> Result<PathBuf, String> {
    // Validate both launcher name and session name
    validate_session_name(name)?;
    validate_session_name(session_name)?;

    // Get home directory
    let home = get_home_dir().ok_or_else(|| "Could not determine home directory".to_string())?;

    // Define the applications directory
    let apps_dir = home.join(".local/share/applications");

    // Create the directory if it doesn't exist
    fs::create_dir_all(&apps_dir)
        .map_err(|e| format!("Failed to create directory {}: {}", apps_dir.display(), e))?;

    // Define the .desktop file path
    let desktop_file_path = apps_dir.join(format!("kitty-launcher-{}.desktop", name));

    // Check if the .desktop file already exists
    if desktop_file_path.exists() {
        return Err(format!(
            "Launcher file already exists: {}",
            desktop_file_path.display()
        ));
    }

    // Generate the .desktop file content
    let desktop_content = format!(
        r#"[Desktop Entry]
Type=Application
Version=1.0
Name=Kitty: {}
Comment=Launch kitty terminal with {} session
Exec=kitty-launcher {}
Icon=kitty
Terminal=false
Categories=System;TerminalEmulator;
StartupNotify=true
MimeType=application/x-shellscript;text/x-shellscript;application/x-sh;text/x-sh;
"#,
        name, session_name, session_name
    );

    // Write the .desktop file
    fs::write(&desktop_file_path, desktop_content).map_err(|e| {
        format!(
            "Failed to create launcher file {}: {}",
            desktop_file_path.display(),
            e
        )
    })?;

    Ok(desktop_file_path)
}

/// Loads and validates the launcher configuration from command line arguments.
///
/// This function:
/// 1. Checks that exactly one argument (session name) is provided
/// 2. Validates the session name is safe to use
/// 3. Finds the configuration file for the session
///
/// # Returns
/// * `Ok(LauncherConfig)` - If everything is valid and the config file is found
/// * `Err(String)` - If there's a validation error or the config file is not found
fn load_config(session_name: &str) -> Result<LauncherConfig, String> {
    // Validate the session name
    validate_session_name(session_name)?;

    // Find the configuration file
    let config_path = find_config_file(session_name)?;

    Ok(LauncherConfig {
        session_name: session_name.to_string(),
        config_path,
    })
}

/// Launches the kitty terminal with the specified configuration.
///
/// This function spawns a new kitty process using the configuration file
/// found by the launcher. It sets the KITTY_CONF_DIR environment variable
/// to point to the directory containing the configuration file.
///
/// Prints the resolved configuration file path and session directory to stdout.
///
/// # Arguments
/// * `config` - The launcher configuration containing the session name and config file path
///
/// # Returns
/// * `Ok(())` - If kitty was launched successfully
/// * `Err(String)` - If there was an error launching kitty
fn launch_kitty(config: &LauncherConfig) -> Result<(), String> {
    // Extract the directory containing the configuration file
    let config_dir = config
        .config_path
        .parent()
        .ok_or_else(|| "Could not determine configuration directory".to_string())?;

    // Get the canonical (absolute) path to the config file for display
    let resolved_path = match std::fs::canonicalize(&config.config_path) {
        Ok(path) => path,
        Err(_) => config.config_path.clone(),
    };

    // Create the kitty command
    let mut command = Command::new("kitty");

    // Set the KITTY_CONF_DIR environment variable
    command.env("KITTY_CONF_DIR", config_dir);

    // Add the session argument
    command.arg("--session");
    command.arg(&config.config_path);

    // Attempt to execute kitty
    match command.spawn() {
        Ok(_) => {
            println!("Session: {}", config.session_name);
            println!("Config file: {}", resolved_path.display());
            println!("Config directory: {}", config_dir.display());
            println!("Status: Launched kitty terminal");
            Ok(())
        }
        Err(e) => Err(format!(
            "Failed to launch kitty: {}\n\n\
                 Please ensure kitty is installed and available in your PATH.",
            e
        )),
    }
}

/// Generates bash shell completion script for kitty-launcher
fn generate_bash_completion() {
    let completion_script = r#"# bash completion for kitty-launcher
# Generated by kitty-launcher --generate-completions bash

_kitty_launcher_complete() {
    local cur prev words cword
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    
     # Get list of available sessions
    local sessions=""
    if [[ -d "$HOME/.local/etc/kitty/sessions" ]]; then
        sessions=$(ls -1 "$HOME/.local/etc/kitty/sessions" 2>/dev/null | sed 's/\.session$//' | sort -u)
    fi
    
    # Handle options and commands
    if [[ $COMP_CWORD -eq 1 ]]; then
        # First argument: commands and sessions
        local commands="--help --version --create --create-launcher --generate-completions -h -V -c -l"
        COMPREPLY=( $(compgen -W "${commands} ${sessions}" -- "$cur") )
    elif [[ "$prev" == "--create" ]] || [[ "$prev" == "-c" ]]; then
        # After --create or -c: no completion (user enters new session name)
        COMPREPLY=()
    elif [[ "$prev" == "--create-launcher" ]] || [[ "$prev" == "-l" ]]; then
        # After --create-launcher: can complete with sessions or user enters launcher name
        COMPREPLY=( $(compgen -W "${sessions}" -- "$cur") )
    elif [[ "$prev" == "--generate-completions" ]]; then
        # After --generate-completions: offer bash and zsh
        COMPREPLY=( $(compgen -W "bash zsh" -- "$cur") )
    fi
}

complete -o bashdefault -o default -o nospace -F _kitty_launcher_complete kitty-launcher
"#;
    println!("{}", completion_script);
}

/// Generates zsh shell completion script for kitty-launcher
fn generate_zsh_completion() {
    let completion_script = r#"# zsh completion for kitty-launcher
# Generated by kitty-launcher --generate-completions zsh

_kitty_launcher() {
    local -a commands
    local -a sessions
     
    # Get list of available sessions
    if [[ -d "$HOME/.local/etc/kitty/sessions" ]]; then
        sessions=(${(f)"$(ls -1 "$HOME/.local/etc/kitty/sessions" 2>/dev/null | sed 's/\.session$//')"})
    fi
    
    commands=(
        '--help:Show help message'
        '--version:Show version information'
        '--create:Create a new session configuration file'
        '--create-launcher:Create a .desktop launcher file'
        '--generate-completions:Generate shell completion scripts'
        '-h:Show help message (short form)'
        '-V:Show version information (short form)'
        '-c:Create session (short form)'
        '-l:Create launcher (short form)'
    )
    
    _arguments \
        '1: :->cmd_or_session' \
        '2: :->second_arg' \
        '*:session names:(${sessions})'
    
    case $state in
        cmd_or_session)
            _describe -t commands 'kitty-launcher commands' commands
            _describe -t sessions 'available sessions' sessions
            ;;
        second_arg)
            case ${words[2]} in
                --create|-c)
                    # No completion for new session names
                    ;;
                --create-launcher|-l)
                    # Complete with available sessions or launcher name
                    _describe -t sessions 'session name' sessions
                    ;;
                --generate-completions)
                    _values 'shell' 'bash' 'zsh'
                    ;;
            esac
            ;;
    esac
}

compdef _kitty_launcher kitty-launcher
"#;
    println!("{}", completion_script);
}

/// The main entry point for the kitty launcher application.
///
/// This function:
/// 1. Parses command line arguments
/// 2. Handles help, version, and create options
/// 3. Launches kitty with the validated configuration
/// 4. Exits with appropriate error codes
fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check for help request or no arguments
    if args.len() == 1 {
        print_help();
        exit(0);
    }

    let first_arg = &args[1];

    // Handle version flag
    if first_arg == "--version" || first_arg == "-V" {
        print_version();
        exit(0);
    }

    // Handle help flag
    if first_arg == "--help" || first_arg == "-h" {
        print_help();
        exit(0);
    }

    // Handle create command (both --create and -c)
    if first_arg == "--create" || first_arg == "-c" {
        if args.len() != 3 {
            eprintln!(
                "Error: {} requires exactly one argument (session name)",
                first_arg
            );
            eprintln!("Usage: {} {} <SESSION_NAME>", args[0], first_arg);
            exit(2);
        }

        let session_name = &args[2];
        match create_session_file(session_name) {
            Ok(path) => {
                println!("Session file created successfully!");
                println!("Path: {}", path.display());
                println!("Edit this file to customize your session configuration.");
                exit(0);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(2);
            }
        }
    }

    // Handle create-launcher command (both --create-launcher and -l)
    // Now accepts optional session name: if omitted, uses launcher name as session
    if first_arg == "--create-launcher" || first_arg == "-l" {
        if args.len() < 3 || args.len() > 4 {
            eprintln!("Error: {} requires one or two arguments", first_arg);
            eprintln!(
                "Usage: {} {} <LAUNCHER_NAME> [SESSION_NAME]",
                args[0], first_arg
            );
            eprintln!();
            eprintln!("If SESSION_NAME is omitted, LAUNCHER_NAME is used as the session.");
            exit(2);
        }

        let launcher_name = &args[2];
        // If session name is provided, use it; otherwise use launcher name
        let session_name = if args.len() == 4 {
            &args[3]
        } else {
            launcher_name
        };

        match create_launcher_file(launcher_name, session_name) {
            Ok(path) => {
                println!("Launcher file created successfully!");
                println!("Path: {}", path.display());
                println!("Launcher name: {}", launcher_name);
                println!("Session: {}", session_name);
                println!("The launcher has been registered in your application menu.");
                println!(
                    "You may need to refresh your desktop environment for changes to take effect."
                );
                exit(0);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(2);
            }
        }
    }

    // Handle generate-completions command
    if first_arg == "--generate-completions" {
        if args.len() != 3 {
            eprintln!("Error: --generate-completions requires exactly one argument (shell type)");
            eprintln!("Usage: {} --generate-completions <SHELL>", args[0]);
            eprintln!();
            eprintln!("Supported shells:");
            eprintln!("  bash  - Generate bash completion script");
            eprintln!("  zsh   - Generate zsh completion script");
            eprintln!();
            eprintln!("Installation instructions:");
            eprintln!(
                "  Bash: {} --generate-completions bash >> ~/.bashrc",
                args[0]
            );
            eprintln!("  Zsh:  {} --generate-completions zsh >> ~/.zshrc", args[0]);
            exit(2);
        }

        let shell = &args[2];
        match shell.as_str() {
            "bash" => {
                generate_bash_completion();
                exit(0);
            }
            "zsh" => {
                generate_zsh_completion();
                exit(0);
            }
            _ => {
                eprintln!("Error: Unknown shell '{}'", shell);
                eprintln!("Supported shells: bash, zsh");
                exit(2);
            }
        }
    }

    // If we get here, treat as session launch
    if args.len() != 2 {
        eprintln!("Error: Expected exactly one session name argument");
        eprintln!("Use '{}' --help for usage information", args[0]);
        exit(2);
    }

    let session_name = &args[1];

    // Load configuration and validate inputs
    match load_config(session_name) {
        Ok(config) => {
            // Try to launch kitty
            match launch_kitty(&config) {
                Ok(()) => {
                    // Kitty launched successfully
                    exit(0);
                }
                Err(e) => {
                    // Error launching kitty - print error and exit with code 1
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        Err(e) => {
            // Configuration error - print error and exit with code 2
            eprintln!("Error: {}", e);
            exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that valid session names are accepted
    #[test]
    fn test_validate_session_name_valid() {
        assert!(validate_session_name("dev").is_ok());
        assert!(validate_session_name("work-session").is_ok());
        assert!(validate_session_name("session_2").is_ok());
        assert!(validate_session_name("dev.backup").is_ok());
        assert!(validate_session_name("z-tools.session").is_ok()); // with .session extension
        assert!(validate_session_name("default.session").is_ok()); // with .session extension
    }

    /// Test that invalid session names are rejected
    #[test]
    fn test_validate_session_name_invalid() {
        assert!(validate_session_name("").is_err()); // empty
        assert!(validate_session_name("../etc").is_err()); // path traversal
        assert!(validate_session_name("./config").is_err()); // path traversal
        assert!(validate_session_name("dev/session").is_err()); // contains slash
        assert!(validate_session_name("dev\\session").is_err()); // contains backslash
        assert!(validate_session_name(".").is_err()); // special directory
        assert!(validate_session_name("..").is_err()); // special directory
        assert!(validate_session_name("dev@home").is_err()); // invalid character
    }

    /// Test that session names with various extensions are accepted
    #[test]
    fn test_validate_session_name_with_extensions() {
        assert!(validate_session_name("dev.session").is_ok());
        assert!(validate_session_name("work.session").is_ok());
        assert!(validate_session_name("test.backup.session").is_ok());
        assert!(validate_session_name("session.config").is_ok());
    }

    /// Test the logic for determining session names to try
    #[test]
    fn test_session_name_variants() {
        // Test that session names without .session get tried with extension
        let name_without_ext = "dev";
        let should_retry = !name_without_ext.ends_with(".session");
        assert!(should_retry); // Should retry with .session

        // Test that session names with .session don't get tried again
        let name_with_ext = "dev.session";
        let should_not_retry = !name_with_ext.ends_with(".session");
        assert!(!should_not_retry); // Should not retry again
    }

    /// Test create session file validation
    #[test]
    fn test_create_session_validation() {
        // These should be valid names for creating sessions
        assert!(validate_session_name("my-session").is_ok());
        assert!(validate_session_name("project_v1").is_ok());
        assert!(validate_session_name("work.dev").is_ok());

        // These should not be valid
        assert!(validate_session_name("../evil").is_err());
        assert!(validate_session_name("/root").is_err());
    }

    /// Test launcher file name validation
    #[test]
    fn test_create_launcher_validation() {
        // These should be valid names for creating launchers
        assert!(validate_session_name("dev-launcher").is_ok());
        assert!(validate_session_name("work_env").is_ok());
        assert!(validate_session_name("project.v2").is_ok());

        // These should not be valid
        assert!(validate_session_name("../hack").is_err());
        assert!(validate_session_name("./local").is_err());
        assert!(validate_session_name("app@example").is_err());
    }

    /// Test that desktop content is properly formatted
    #[test]
    fn test_desktop_file_content() {
        let desktop_content = format!(
            r#"[Desktop Entry]
Type=Application
Version=1.0
Name=Kitty: {}
Comment=Launch kitty terminal with {} session
Exec=kitty-launcher {}
Icon=kitty
Terminal=false
Categories=System;TerminalEmulator;
StartupNotify=true
MimeType=application/x-shellscript;text/x-shellscript;application/x-sh;text/x-sh;
"#,
            "test", "test", "test"
        );

        // Verify the content contains required desktop entry fields
        assert!(desktop_content.contains("[Desktop Entry]"));
        assert!(desktop_content.contains("Type=Application"));
        assert!(desktop_content.contains("Version=1.0"));
        assert!(desktop_content.contains("Name=Kitty: test"));
        assert!(desktop_content.contains("Exec=kitty-launcher test"));
        assert!(desktop_content.contains("Icon=kitty"));
        assert!(desktop_content.contains("Terminal=false"));
    }
}
