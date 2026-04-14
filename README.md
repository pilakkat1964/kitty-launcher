# Kitty Launcher - A Rust Learning Project

Welcome! This project is a well-documented example of a robust Rust application suitable for learning Rust fundamentals. The kitty launcher is a wrapper for the [kitty terminal emulator](https://sw.kovidgoyal.net/kitty/) that allows you to launch terminal sessions with flexible configuration presets.

## Project Overview

This project demonstrates several important Rust concepts in a practical, real-world context:

- **Error Handling**: Using `Result<T, E>` types for robust error management
- **Input Validation**: Security best practices to validate user input
- **File System Operations**: Reading and validating file paths
- **Process Management**: Spawning external processes
- **Documentation**: Writing clear, beginner-friendly code comments
- **Testing**: Unit tests for critical functions

## Getting Started

### Prerequisites

- **Rust & Cargo**: Install from [rustup.rs](https://rustup.rs/)
- **Kitty Terminal**: Install from [sw.kovidgoyal.net/kitty](https://sw.kovidgoyal.net/kitty/)

### Building the Project

```bash
cd kitty-launcher
cargo build --release
```

The compiled binary will be at `target/release/kitty-launcher`.

### Running the Project

```bash
# Create a test configuration
mkdir -p ~/.local/etc/kitty

# Copy an existing kitty session file or create one
# For example, if you have a session file:
cp /path/to/your/session ~/.local/etc/kitty/dev

# Run the launcher
./target/release/kitty-launcher dev
```

## Understanding the Code

### File Structure

```
kitty-launcher/
├── Cargo.toml          # Project metadata and dependencies
└── src/
    └── main.rs         # Main application code (heavily documented)
```

### Core Concepts Explained

#### 1. **Structs** (`LauncherConfig`)

```rust
struct LauncherConfig {
    session_name: String,
    config_path: PathBuf,
}
```

A `struct` is a way to group related data together. Think of it as a container that holds the information we need: which session the user wants and where its configuration file is located.

**Rust Concept**: Ownership. When you create a `LauncherConfig`, Rust takes ownership of the strings and paths inside it.

#### 2. **Functions with Result Types**

```rust
fn validate_session_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Session name cannot be empty".to_string());
    }
    Ok(())
}
```

A function that returns `Result<(), String>` means:
- **Success case** (`Ok(())`): Everything went fine, nothing to return (the `()` is "unit" type - think of it as void)
- **Error case** (`Err(String)`): Something failed, and we're returning an error message

This is Rust's way of saying "this function might fail, and you MUST handle both cases."

**Rust Concept**: The `?` operator can be used to propagate errors up the call stack.

#### 3. **Option Types**

```rust
fn get_home_dir() -> Option<PathBuf> {
    env::var("HOME")
        .ok()
        .map(PathBuf::from)
}
```

`Option<T>` means "either Some value or None". This is Rust's way of saying "this might not exist."

**Rust Concept**: Pattern matching. You handle both the `Some` and `None` cases explicitly using:
```rust
match some_option {
    Some(value) => { /* do something with value */ }
    None => { /* handle missing value */ }
}
```

#### 4. **Ownership and References**

```rust
fn validate_session_name(name: &str) -> Result<(), String> {
    // ^-- '&str' means we're borrowing a string slice, not taking ownership
}
```

When you use `&`, you're borrowing a value instead of taking ownership. This is safe and efficient.

**Rust Concept**: Rust's ownership system prevents memory errors at compile time.

### Key Functions Explained

#### `validate_session_name(name: &str) -> Result<(), String>`

**What it does**: Checks if the session name is safe to use.

**Why it's important**: Prevents path traversal attacks (e.g., someone passing `../../../etc/passwd`).

**Key validation**:
- Not empty
- No directory separators (`/` or `\`)
- Only alphanumeric characters, hyphens, underscores, and dots

#### `find_config_file(session_name: &str) -> Result<PathBuf, String>`

**What it does**: Searches for the configuration file in standard locations.

**Why it's important**: Provides flexibility so users can put config files in their preferred location.

**Search order**:
1. `./etc/kitty` (current directory - highest priority)
2. `~/.local/etc/kitty` (user's local config)
3. `/opt/etc/kitty` (system-wide optional)
4. `~/.config/kitty` (kitty standard location)

#### `launch_kitty(config: &LauncherConfig) -> Result<(), String>`

**What it does**: Spawns the kitty terminal process.

**Why it's important**: This is where we actually start the terminal with the right configuration.

**Key step**: We use `Command::new("kitty")` to create a process, configure it with environment variables and arguments, then `spawn()` to start it.

### Error Handling Pattern

The program uses a pattern called "early return" for error handling:

```rust
fn main() {
    match load_config() {
        Ok(config) => {
            match launch_kitty(&config) {
                Ok(()) => exit(0),           // Success
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);                 // Kitty launch error
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(2);                         // Configuration error
        }
    }
}
```

This ensures we handle every possible outcome and provide appropriate exit codes.

## Rust Learning Resources

### Concepts Used in This Project

- **Ownership & Borrowing**: [https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- **Error Handling**: [https://doc.rust-lang.org/book/ch09-00-error-handling.html](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- **Structs**: [https://doc.rust-lang.org/book/ch05-00-using-structs.html](https://doc.rust-lang.org/book/ch05-00-using-structs.html)
- **Pattern Matching**: [https://doc.rust-lang.org/book/ch06-00-enums.html](https://doc.rust-lang.org/book/ch06-00-enums.html)

### Official Resources

- **The Rust Book**: https://doc.rust-lang.org/book/ - Start here!
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustlings**: https://github.com/rust-lang/rustlings - Interactive exercises

## Running Tests

This project includes unit tests for validation functions:

```bash
cargo test
```

The tests verify that:
- Valid session names are accepted
- Invalid session names are rejected (path traversal, special characters, etc.)

## Development Commands

```bash
# Build in debug mode (faster compilation, slower runtime)
cargo build

# Build in release mode (slower compilation, faster runtime)
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check

# Format code according to Rust standards
cargo fmt

# Find common mistakes and improvements
cargo clippy
```

## Common Beginner Questions

**Q: Why use `Result` instead of returning the value or null?**
A: `Result` forces you to explicitly handle errors. This prevents bugs where you forget to check if something failed.

**Q: What's the difference between `String` and `&str`?**
A: `String` owns its data (heap allocated), `&str` is a borrowed slice of string data. Use `&str` for function parameters that don't need to take ownership.

**Q: Why does Rust require explicit error handling?**
A: It's a safety feature. Many bugs come from ignoring errors. Rust makes this impossible.

**Q: What does the `?` operator do?**
A: It's a shorthand for error propagation. In a function that returns `Result`, `value?` returns early if there's an error, otherwise continues with the value.

## Project Enhancement Ideas

Once you understand the basics, try adding:

1. **Configuration file format**: Parse TOML or YAML config files
2. **Logging**: Add the `log` crate for debug output
3. **Additional validation**: More sophisticated path checking
4. **List sessions**: Show available sessions with `--list`
5. **Help system**: Add `--help` and `--version` flags

## Contributing & Feedback

This is a learning project! If you have suggestions or find improvements, please contribute or file an issue.

## License

This project is provided as-is for educational purposes.

---

**Happy learning! Remember: Rust's strict compiler is your friend—it catches bugs at compile time instead of at runtime.** 🦀
