---
layout: default
title: Quick Reference
---

# Kitty Launcher - Quick Reference

## Getting Started (30 seconds)

```bash
# Build
cd kitty-launcher
cargo build --release

# Create a session
mkdir -p ~/.local/etc/kitty
touch ~/.local/etc/kitty/mysession

# Run
./target/release/kitty-launcher mysession
```

## Directory Structure

```
z-tools/
└── kitty-launcher/              # ← The project
    ├── src/main.rs              # Main code (~310 lines)
    ├── Cargo.toml               # Project metadata
    ├── README.md                # Overview (read this first!)
    ├── LEARNING_GUIDE.md        # For learning Rust
    ├── INSTALL.md               # Installation guide
    ├── PROJECT_SUMMARY.md       # What was built
    ├── kitty-launcher.png       # Icon (256x256)
    ├── kitty-launcher-icon.svg  # Icon (vector)
    └── debian/                  # Debian package files
```

## Commands Cheat Sheet

```bash
# Building
cargo build              # Debug build
cargo build --release   # Optimized build
cargo clean             # Clean build artifacts

# Testing
cargo test              # Run unit tests
cargo clippy            # Check for improvements
cargo fmt               # Format code

# Running
./target/debug/kitty-launcher dev      # Debug binary
./target/release/kitty-launcher dev    # Release binary

# Debian Package
dpkg-buildpackage -us -uc  # Build .deb file
sudo dpkg -i *.deb         # Install package
sudo apt-get remove kitty-launcher  # Uninstall
```

## Configuration Paths (in order)

1. `./etc/kitty/` - Current directory
2. `~/.local/etc/kitty/` - User home
3. `/opt/etc/kitty/` - System-wide
4. `~/.config/kitty/` - Kitty standard

Use whichever location makes sense for your use case.

## Error Reference

| Error | Meaning | Solution |
|-------|---------|----------|
| `Session name cannot be empty` | No session name provided | `kitty-launcher dev` |
| `Invalid session name` | Invalid characters used | Use only letters, numbers, `-`, `_`, `.` |
| `Configuration file not found` | Session config doesn't exist | Create session file in one of the search paths |
| `Failed to launch kitty` | Kitty not installed | `sudo apt-get install kitty` |

## Key Files Explained

### src/main.rs
The entire application in one file (~310 lines):
- **Lines 1-26**: Module documentation
- **Lines 27-30**: Imports
- **Lines 33-38**: LauncherConfig struct
- **Lines 41-75**: validate_session_name() - Input validation
- **Lines 78-119**: find_config_file() - Search for config
- **Lines 122-126**: get_home_dir() - Get home directory
- **Lines 129-157**: load_config() - Load and validate config
- **Lines 160-188**: launch_kitty() - Launch the terminal
- **Lines 191-221**: main() - Entry point
- **Lines 223-256**: Unit tests

### Cargo.toml
Project metadata - defines name, version, dependencies, etc.

### README.md
Start here! Explains what the project does and how to use it.

## Understanding Error Handling

The app uses `Result<T, E>` for error handling:

```rust
// This type means "either Ok(value) or Err(error)"
fn risky_operation() -> Result<String, String> {
    if problem {
        Err("something went wrong".to_string())
    } else {
        Ok("success".to_string())
    }
}

// You must handle both cases
match risky_operation() {
    Ok(value) => println!("Got: {}", value),
    Err(e) => println!("Error: {}", e),
}
```

All functions in kitty-launcher use this pattern for safety.

## Common Tasks

### Add a Session

```bash
# Option 1: Create empty session
mkdir -p ~/.local/etc/kitty
touch ~/.local/etc/kitty/mysession

# Option 2: Copy from kitty's config
cp ~/.config/kitty/sessions/default ~/.local/etc/kitty/mysession
```

### Test Error Handling

```bash
kitty-launcher                # No args → shows help
kitty-launcher ../etc         # Invalid → security warning
kitty-launcher nonexistent    # Not found → helpful message
```

### Install Globally

```bash
# Build
cargo build --release

# Install
sudo cp target/release/kitty-launcher /usr/local/bin/
sudo chmod +x /usr/local/bin/kitty-launcher
```

## Learning Rust with This Project

**Level 1 (Complete Beginner)**
- Read README.md
- Understand what the program does
- Try running: `kitty-launcher dev`

**Level 2 (Getting Started)**
- Read LEARNING_GUIDE.md
- Understand core concepts: Ownership, Borrowing, Result types
- Review the comments in src/main.rs

**Level 3 (Intermediate)**
- Study each function in src/main.rs
- Try exercises from LEARNING_GUIDE.md
- Modify the code and recompile

**Level 4 (Advanced)**
- Add new features (see exercises)
- Extract code into a library
- Contribute improvements

## Debugging

### See what the program is doing
```bash
# Compile with debug symbols
cargo build

# Run with debug logging (add to main.rs if needed)
RUST_LOG=debug ./target/debug/kitty-launcher dev
```

### Check where it's looking for files
Look at the error message when session not found - it shows all paths searched.

### Test individual functions
The unit tests in src/main.rs test `validate_session_name()`:
```bash
cargo test -- --nocapture
```

## Security Notes

✅ **Safe from path traversal** - Rejects `../`, `/`, `\`, `.`, `..`
✅ **Safe from shell injection** - Uses `Command::new()` not `shell`
✅ **Validates all input** - No untrusted data passed to system

The strict validation ensures security while being helpful to users.

## Performance

- **Compile time**: ~0.7 seconds
- **Runtime**: <10ms typically
- **Binary size**: ~6MB debug, ~2.5MB release
- **Memory usage**: ~2MB typical

## Documentation Files

| File | Purpose | For Whom |
|------|---------|----------|
| README.md | Overview and features | Everyone |
| LEARNING_GUIDE.md | Deep Rust explanations | Rust learners |
| INSTALL.md | Installation steps | Users/Admins |
| PROJECT_SUMMARY.md | What was built | Project managers |
| QUICK_REFERENCE.md | This file | Quick lookup |

---

**Get Started**: Read README.md, then run `cargo build --release`!
