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
//!
//! ## Configuration Search Path
//! The program searches for session configuration files in this order:
//! 1. `./etc/kitty` (current directory)
//! 2. `~/.local/etc/kitty` (user home directory)
//! 3. `/opt/etc/kitty` (optional system-wide)
//! 4. `~/.config/kitty` (kitty standard location)
//!
//! ## Usage
//! ```
//! kitty-launcher <session-name>
//! ```
//!
//! Example: `kitty-launcher dev` would launch a kitty session using the
//! configuration file found at one of the standard locations.

use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};

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

/// Validates the session name to ensure it's safe for use.
///
/// This function checks that the session name:
/// - Is not empty
/// - Contains only alphanumeric characters, hyphens, and underscores
/// - Is not a special value like "." or ".."
///
/// # Arguments
/// * `name` - The session name to validate
///
/// # Returns
/// * `Ok(())` if the name is valid
/// * `Err(String)` if the name is invalid, with a descriptive error message
///
/// # Example
/// ```
/// assert!(validate_session_name("dev").is_ok());
/// assert!(validate_session_name("../etc/passwd").is_err());
/// ```
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

/// Finds the configuration file for a given session name.
///
/// This function searches for the session configuration file in the standard
/// locations in the following order:
/// 1. `./etc/kitty/` (current directory)
/// 2. `~/.local/etc/kitty/` (user home directory)
/// 3. `/opt/etc/kitty/` (optional system-wide)
/// 4. `~/.config/kitty/` (kitty standard location)
///
/// The function returns the path to the first configuration file found.
/// If no file is found, it returns an error with suggestions.
///
/// # Arguments
/// * `session_name` - The name of the session (without file extension)
///
/// # Returns
/// * `Ok(PathBuf)` - The path to the found configuration file
/// * `Err(String)` - An error message if no configuration file is found
fn find_config_file(session_name: &str) -> Result<PathBuf, String> {
    // Define the search paths in order of preference
    let mut search_paths: Vec<PathBuf> = vec![
        // Current directory (highest priority)
        PathBuf::from("./etc/kitty"),
    ];

    // Add user's local configuration directory if home dir is available
    if let Some(home) = get_home_dir() {
        search_paths.push(home.join(".local/etc/kitty"));
    }

    // Add optional system-wide directory
    search_paths.push(PathBuf::from("/opt/etc/kitty"));

    // Add kitty's standard configuration directory if home dir is available
    if let Some(home) = get_home_dir() {
        search_paths.push(home.join(".config/kitty"));
    }

    // Try each search path
    for search_path in search_paths.iter() {
        let config_file = search_path.join(session_name);

        // Check if the file exists
        if config_file.exists() {
            // Verify it's actually a file (not a directory)
            if config_file.is_file() {
                return Ok(config_file);
            }
        }
    }

    // If we get here, no configuration file was found
    Err(format!(
        "Configuration file for session '{}' not found in any of the standard locations:\n  \
         - ./etc/kitty/\n  \
         - ~/.local/etc/kitty/\n  \
         - /opt/etc/kitty/\n  \
         - ~/.config/kitty/\n\n\
         Please create a configuration file named '{}' in one of these directories.",
        session_name, session_name
    ))
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
fn load_config() -> Result<LauncherConfig, String> {
    // Collect command line arguments, skipping the program name
    let args: Vec<String> = env::args().collect();

    // Check if exactly one argument (the session name) was provided
    if args.len() != 2 {
        return Err(format!(
            "Usage: {} <session-name>\n\n\
             Example: {} dev\n\n\
             Arguments:\n  \
             <session-name>    The name of the kitty session configuration to launch",
            args[0], args[0]
        ));
    }

    // Get the session name from the first argument
    let session_name = args[1].clone();

    // Validate the session name
    validate_session_name(&session_name)?;

    // Find the configuration file
    let config_path = find_config_file(&session_name)?;

    Ok(LauncherConfig {
        session_name,
        config_path,
    })
}

/// Launches the kitty terminal with the specified configuration.
///
/// This function spawns a new kitty process using the configuration file
/// found by the launcher. It sets the KITTY_CONF_DIR environment variable
/// to point to the directory containing the configuration file.
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
            println!("Launched kitty with session: {}", config.session_name);
            Ok(())
        }
        Err(e) => Err(format!(
            "Failed to launch kitty: {}\n\n\
                 Please ensure kitty is installed and available in your PATH.",
            e
        )),
    }
}

/// The main entry point for the kitty launcher application.
///
/// This function:
/// 1. Loads and validates the configuration
/// 2. Displays helpful error messages if validation fails
/// 3. Launches kitty with the validated configuration
/// 4. Exits with appropriate error codes
fn main() {
    // Load configuration and validate inputs
    match load_config() {
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
}
