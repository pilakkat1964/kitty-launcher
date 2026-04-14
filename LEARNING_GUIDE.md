# Kitty Launcher - Deep Dive Learning Guide

This guide provides a thorough walkthrough of the code for Rust learners.

## Table of Contents

1. [Module Overview](#module-overview)
2. [Code Walkthrough](#code-walkthrough)
3. [Rust Concepts Explained](#rust-concepts-explained)
4. [Why Each Decision Was Made](#why-each-decision-was-made)
5. [Exercises for Learning](#exercises-for-learning)

## Module Overview

The entire application is in `src/main.rs` and contains:

- **4 Data Structures**: `LauncherConfig` struct
- **6 Functions**: For validation, configuration, and launching
- **1 Main Entry Point**: The `main()` function
- **8 Unit Tests**: For validating critical functions

### Dependency Graph

```
main()
├── load_config()
│   ├── validate_session_name()
│   └── find_config_file()
│       └── get_home_dir()
├── launch_kitty()
│   └── get_home_dir()
└── (error handling & exit codes)
```

## Code Walkthrough

### Part 1: The Data Structure

```rust
struct LauncherConfig {
    session_name: String,
    config_path: PathBuf,
}
```

**What is a struct?**
A struct is like a template for organizing related data. Imagine a filing cabinet:
- The struct is the cabinet design
- Each field is a drawer
- When you create an instance, you have an actual cabinet with items in it

**Why `String` vs `&str`?**
- `String`: Owns its data, can be modified, takes up memory on the heap
- `&str`: Borrows data, cannot be modified, just a reference

Since we're storing this in a struct that we own, we use `String` so the struct can own the session name.

**Why `PathBuf` vs `&Path`?**
Same reasoning! `PathBuf` is to `&Path` as `String` is to `&str`.

### Part 2: Input Validation

```rust
fn validate_session_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Session name cannot be empty".to_string());
    }
    // ... more checks ...
    Ok(())
}
```

**The `Result<T, E>` Type**

In Rust, functions that might fail return `Result`:
- `Ok(value)` - Success, here's your value
- `Err(error)` - Failure, here's why

We write `Result<(), String>` meaning:
- On success: `Ok(())` - the `()` means "nothing" (void)
- On failure: `Err(String)` - we return an error message

**Why validate?**

This is a security practice called "input validation." We check:
1. Not empty - can't launch nothing
2. No path separators - prevents: `../../etc/passwd` attacks
3. No special directory names - prevents: `.` and `..` tricks
4. Only safe characters - prevents: shell injection

**Critical Security Check:**

```rust
if name.contains('/') || name.contains('\\') {
    return Err("Cannot contain path separators".to_string());
}
```

This prevents a path traversal attack. Without this, someone could pass `../etc/passwd` and potentially access files outside our intended directory.

### Part 3: Finding Configuration Files

```rust
fn find_config_file(session_name: &str) -> Result<PathBuf, String> {
    let mut search_paths: Vec<PathBuf> = vec![
        PathBuf::from("./etc/kitty"),
    ];
    
    if let Some(home) = get_home_dir() {
        search_paths.push(home.join(".local/etc/kitty"));
    }
    // ... more paths ...
    
    for search_path in search_paths.iter() {
        let config_file = search_path.join(session_name);
        if config_file.exists() && config_file.is_file() {
            return Ok(config_file);
        }
    }
    
    Err("File not found".to_string())
}
```

**The `if let` Pattern**

```rust
if let Some(home) = get_home_dir() {
    // Use 'home' here
}
```

This is a compact way to handle `Option`:
- If `get_home_dir()` returns `Some(path)`, bind it to `home` and run the block
- If it returns `None`, skip the block

It's equivalent to:
```rust
match get_home_dir() {
    Some(home) => {
        // Use 'home'
    }
    None => {}
}
```

**The Search Path Strategy**

We search in priority order:
1. **Current directory** (`./etc/kitty`) - Most specific to current project
2. **User home** (`~/.local/etc/kitty`) - User-level config
3. **System** (`/opt/etc/kitty`) - Shared system config  
4. **Kitty standard** (`~/.config/kitty`) - Where kitty looks by default

This gives flexibility while respecting Unix conventions.

### Part 4: Getting Home Directory

```rust
fn get_home_dir() -> Option<PathBuf> {
    env::var("HOME")
        .ok()
        .map(PathBuf::from)
}
```

**Method Chaining**

This is Rust's "fluent" style. Let's break it down:

```rust
env::var("HOME")           // Returns Result<String, VarError>
    .ok()                   // Converts to Option<String>
    .map(PathBuf::from)     // Transforms String to PathBuf
```

Equivalent verbose version:
```rust
match env::var("HOME") {
    Ok(home_str) => Some(PathBuf::from(home_str)),
    Err(_) => None,
}
```

**Why `Option` instead of panicking?**

On some systems (like containers or minimal environments), `HOME` might not be set. Returning `None` gracefully handles this—we just won't search in that location.

### Part 5: Launching Kitty

```rust
fn launch_kitty(config: &LauncherConfig) -> Result<(), String> {
    let config_dir = config
        .config_path
        .parent()
        .ok_or_else(|| "Could not determine config directory".to_string())?;
    
    let mut command = Command::new("kitty");
    command.env("KITTY_CONF_DIR", config_dir);
    command.arg("--session");
    command.arg(&config.config_path);
    
    match command.spawn() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to launch: {}", e)),
    }
}
```

**The `parent()` method**

`PathBuf::parent()` returns `Option<&Path>`:
- `Some(&path)` - the parent directory
- `None` - no parent (can't happen with normal paths, but might with weird ones)

We use `.ok_or_else()` to convert this:
```rust
.ok_or_else(|| "error message".to_string())?
```

The `?` operator:
- If `Ok`, extract the value and continue
- If `Err`, return early with the error

**Environment Variables**

```rust
command.env("KITTY_CONF_DIR", config_dir);
```

This sets an environment variable for kitty's process. Kitty uses this to know where to find session files.

**The Command API**

```rust
let mut command = Command::new("kitty");
command.arg("--session");
command.spawn();
```

This is the builder pattern:
1. Create a command object
2. Add arguments and configuration (methods return `&mut self`)
3. Finally call `spawn()` to execute

The `mut` keyword allows us to mutate the command by adding arguments.

### Part 6: Main Entry Point

```rust
fn main() {
    match load_config() {
        Ok(config) => {
            match launch_kitty(&config) {
                Ok(()) => exit(0),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(2);
        }
    }
}
```

**Nested Match Expressions**

We have two levels of `match`:
1. Did configuration load successfully?
2. Did kitty launch successfully?

Each branch handles success or failure.

**Exit Codes**

- `exit(0)` - Success
- `exit(1)` - Failed to launch kitty
- `exit(2)` - Configuration error

This lets scripts calling our program know what went wrong.

## Rust Concepts Explained

### Ownership

**The Rule**: Every value in Rust has exactly one owner. When the owner goes away, so does the value.

```rust
let config = LauncherConfig { /* ... */ };
// config is owned by this scope
// When this scope ends, config is dropped (memory freed)
```

### Borrowing

**The Rule**: You can borrow a value with `&`. The owner still owns it; you're just borrowing it.

```rust
fn validate_session_name(name: &str) {
    // name is borrowed, we can read it but not own it
    // When this function ends, we give it back
}

let my_name = String::from("dev");
validate_session_name(&my_name);  // Borrow my_name
// my_name is still ours!
```

### Result Type

**The Idea**: Functions that might fail return `Result<T, E>`.

```rust
// This function might fail
fn risky_operation() -> Result<String, String> {
    if bad_condition {
        Err("Something went wrong".to_string())
    } else {
        Ok("Success!".to_string())
    }
}

// You MUST handle both cases
match risky_operation() {
    Ok(value) => println!("Got: {}", value),
    Err(error) => println!("Error: {}", error),
}
```

### Option Type

**The Idea**: When something might not exist, use `Option<T>`.

```rust
// This value might exist
fn maybe_get_something() -> Option<String> {
    if has_thing {
        Some("Found it!".to_string())
    } else {
        None
    }
}

// Handle both cases
match maybe_get_something() {
    Some(value) => println!("Got: {}", value),
    None => println!("Nothing found"),
}
```

## Why Each Decision Was Made

### Why use Result?
- Forces error handling
- Prevents "forgot to check for errors" bugs
- Clear that function can fail

### Why validate input?
- Security (prevent injection attacks)
- Usability (catch mistakes early)
- Robustness (know what we're dealing with)

### Why search multiple paths?
- Flexibility (users choose where to put configs)
- Follows Unix conventions
- Works in different environments

### Why use `Command::new()` builder pattern?
- Type-safe (compiler checks arguments)
- Readable (clear what each line does)
- Flexible (easy to add more options later)

## Exercises for Learning

### Easy Exercises

1. **Add a `--version` flag**
   - Modify `load_config()` to check for `--version` argument
   - Print the version and exit
   - Hint: Check `args[1]` before assuming it's the session name

2. **Add logging**
   - Print which directory we're searching in `find_config_file()`
   - Show which config file was found

3. **Better error messages**
   - List the actual paths we searched in `find_config_file()`
   - Suggest how to fix the problem

### Medium Exercises

4. **Add `--list` command**
   - List all available sessions by scanning config directories
   - Hint: Use `std::fs::read_dir()`

5. **Configuration file**
   - Create a TOML config file that specifies search paths
   - Parse it in `main()`
   - Hint: Use the `toml` crate

6. **Better session name validation**
   - Allow session names with `.conf` extension
   - Support more characters

### Advanced Exercises

7. **Add logging with the `log` crate**
   - Use debug logs to trace execution
   - Use `env_logger` to control verbosity

8. **Make a library**
   - Extract the core logic into a library (`src/lib.rs`)
   - Make a binary that uses the library
   - Allows other programs to use your code

9. **Add session templates**
   - Copy a default session if one doesn't exist
   - Hint: Include template files with `include_str!()` macro

## Summary

The kitty launcher demonstrates:
- ✅ Safe error handling with `Result`
- ✅ Input validation for security
- ✅ File system operations
- ✅ External process spawning
- ✅ Rust's ownership system in practice
- ✅ Pattern matching
- ✅ Good code documentation

All within ~300 lines of well-commented code!

---

**Next Steps**: Try the exercises above, then explore the Rust standard library documentation to see what else you can add!
