# Contributing to z-kitty-launcher

Thank you for your interest in contributing to z-kitty-launcher! This document provides guidelines and instructions for contributing code, documentation, bug reports, and feature suggestions.

## Code of Conduct

Be respectful, inclusive, and professional. All contributors are expected to maintain a welcoming environment for everyone.

## How to Contribute

### 1. Reporting Bugs

**Before submitting a bug report:**
- Check existing issues to avoid duplicates
- Verify the bug still exists on the latest version
- Gather relevant information (version, system, error messages)

**When submitting a bug report, include:**
- Clear title summarizing the issue
- Detailed description with steps to reproduce
- Expected vs. actual behavior
- System information (OS, Rust version, architecture)
- Error messages, logs, or stack traces
- Output of `kitty-launcher --version`

### 2. Suggesting Features

**Feature suggestions should include:**
- Clear use case explaining why this feature is needed
- Detailed description of expected behavior
- Examples or mockups if applicable
- Discussion of potential implementation approaches
- Links to related discussions or features

### 3. Submitting Code Changes

#### Setup Your Development Environment

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone the repository
git clone git@github.com:pilakkat1964/z-kitty-launcher.git
cd z-kitty-launcher

# Build the project
cargo build --release

# Run tests
cargo test

# Verify setup
rustc --version
cargo --version
```

#### Development Workflow

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   # or for bug fixes:
   git checkout -b fix/issue-number-description
   ```

2. **Make your changes:**
   - Write clear, focused commits with descriptive messages
   - Follow Rust conventions and idioms
   - Add tests for new functionality
   - Update documentation as needed

3. **Run tests and checks:**
   ```bash
   # Run all checks
   cargo test               # Run tests
   cargo clippy             # Lint checking
   cargo fmt --check        # Check formatting
   cargo build --release    # Release build
   cargo doc --open         # Generate and view docs
   ```

4. **Commit your changes:**
   ```bash
   git add [files]
   git commit -m "type: brief description

   More detailed explanation if needed.
   - Use bullet points for multiple changes
   - Reference issue numbers: fixes #123"
   ```

   **Commit message guidelines:**
   - Use conventional commits: `feat:`, `fix:`, `docs:`, `test:`, `refactor:`, `style:`, `chore:`
   - First line should be concise (50 chars or less)
   - Provide detailed explanation in the body
   - Reference related issues or PRs

5. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request:**
   - Clear title describing the change
   - Reference related issues (e.g., "Fixes #123")
   - Describe what changed and why
   - List any breaking changes
   - Include output from `cargo test` if relevant

#### Code Style Guidelines

**z-kitty-launcher follows these standards:**
- Follow Rust conventions and idioms
- Use `cargo fmt` for formatting (mandatory)
- Address all `cargo clippy` warnings
- Write tests for public APIs
- Document public items with doc comments
- Use meaningful error messages with `anyhow::Context`
- Keep error handling ergonomic with Result types

**Important standards:**
- No `unwrap()` in library code (use `?` operator)
- Proper error context with `.context("message")`
- Document safety invariants for unsafe code
- Use type hints for public functions

#### Testing Requirements

- Write unit tests for new functions
- Ensure all tests pass: `cargo test`
- Test error handling and edge cases
- Use meaningful assertions with context
- Add documentation examples in doc comments
- Test both Linux platforms (if possible)
- Test session name validation thoroughly

### 4. Documentation Contributions

Documentation improvements are valuable! You can contribute by:

- **Fixing typos and clarifying text** in existing docs
- **Adding examples** to API documentation or guides
- **Creating new guides** for common use cases
- **Improving architecture documentation** for developers
- **Adding learning resources** for Rust beginners
- **Adding code comments** for complex sections

**Documentation guidelines:**
- Use clear, accessible language
- Include examples and code snippets
- Keep examples tested and up-to-date
- Use consistent formatting and terminology
- Link related documentation sections
- Maintain YAML front matter for Jekyll pages

## Project Structure

```
z-kitty-launcher/
├── src/
│   └── main.rs           # Main implementation (957 lines)
├── Cargo.toml            # Project manifest (v0.4.0)
├── Cargo.lock            # Dependency lock
├── README.md             # Project overview (614 lines)
├── scripts/
│   ├── build.sh         # Build wrapper
│   ├── build-deb.sh     # Debian build wrapper
│   ├── install-completions.sh  # Shell completion installer
│   └── README.md        # Build documentation
├── debian/              # Debian packaging
├── docs/                # User documentation
├── data/
│   ├── kitty-launcher.desktop
│   ├── kitty-launcher.png
│   └── kitty-launcher-icon.svg
├── kitty-launcher.1     # Man page
├── kitty-launcher.info  # Info page
├── LEARNING_GUIDE.md    # Rust learning guide
├── INSTALL.md           # Installation guide
├── .github/workflows/   # CI/CD
├── README.md            # Project overview
└── AGENTS.md            # Agent documentation
```

## Building and Testing Locally

```bash
# Build debug binary (fast, larger)
cargo build

# Build release binary (optimized, ~509 KB)
cargo build --release

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt --check

# Generate documentation
cargo doc --open

# Build Debian package
./scripts/build-deb.sh --clean

# Run binary directly
./target/release/kitty-launcher --version
./target/release/kitty-launcher -h
```

## Understanding the Codebase

### Main Components (src/main.rs - 957 lines)

1. **Session Management**
   - `validate_session_name()` - Input validation and security
   - `find_config_file()` - Configuration file discovery
   - `create_session_file()` - Session file creation

2. **Desktop Integration**
   - `create_launcher_file()` - .desktop file generation
   - `create_system_launcher()` - System registration

3. **Shell Completions** (v0.4.0)
   - `generate_bash_completion()` - Bash completion script
   - `generate_zsh_completion()` - Zsh completion script

4. **Launching**
   - `launch_kitty()` - Session launching with kitty

5. **CLI Interface**
   - `print_help()` - Help system
   - `print_version()` - Version information
   - `main()` - Entry point and control flow

### Configuration Paths (Priority Order)

1. `./etc/kitty/` (current directory, for project-local sessions)
2. `~/.local/etc/kitty/` (user directory)
3. `/opt/etc/kitty/` (system-wide)
4. `~/.config/kitty/` (kitty standard location)

### Error Handling

Uses `anyhow::Result<T>` for ergonomic error handling:
- Proper error context with `.context("message")`
- Exit codes: 0 (success), 1 (runtime error), 2 (config error)
- Clear, actionable error messages for users

## Release Process

### Versioning

Uses [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes to API or CLI
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

**Version bumps typically follow:**
- v0.1.0 → v0.2.0: Major feature additions
- v0.4.0 → v0.4.1: Bug fixes and patches

### Creating a Release

1. **Update version** in `Cargo.toml`
2. **Update changelog** in debian/changelog or git history
3. **Commit changes**: `git commit -m "chore: bump version to X.Y.Z"`
4. **Create git tag**: `git tag vX.Y.Z -m "Release vX.Y.Z"`
5. **Push to remote**: `git push origin master && git push origin vX.Y.Z`
6. **GitHub Actions will automatically:**
   - Run tests on Rust stable
   - Run clippy linting and security audit
   - Build for AMD64 (native) and ARM64 (cross-compile)
   - Build Debian packages for both architectures
   - Create GitHub Release with all artifacts

See `AGENTS.md` for detailed release procedures.

## Getting Help

- **Documentation**: See `docs/` folder
- **README**: Comprehensive overview and usage examples
- **Man page**: `man ./kitty-launcher.1`
- **Learning Guide**: See `LEARNING_GUIDE.md` for Rust concepts
- **Installation**: See `INSTALL.md`
- **Architecture**: See `AGENTS.md`
- **Issues**: Check existing issues or create a new one
- **Discussions**: Use GitHub Discussions for questions

## Important Notes

### Security Considerations
- Input validation is strict - session names must be alphanumeric, hyphens, underscores, or dots
- Path traversal attacks are prevented
- Shell injection is prevented through proper argument passing

### Compatibility
- Requires kitty terminal to be installed and in PATH
- Tested on Linux (X11 and Wayland)
- Multi-architecture support (AMD64, ARM64)

### Performance
- Binary size: ~509 KB (release, stripped)
- Build time: ~1.5 seconds
- Startup latency: Negligible
- Minimal dependencies (7 crates)

## License

By contributing to z-kitty-launcher, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors are valued and recognized! We may mention contributors in:
- Release notes
- Project README
- Contributor list

Thank you for contributing to z-kitty-launcher! 🙏
