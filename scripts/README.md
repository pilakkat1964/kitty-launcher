# Build Scripts for Kitty Launcher

This directory contains build automation scripts for local development and CI/CD.

## Scripts

### `build.sh` - General Build Wrapper

Provides a unified interface for building the kitty-launcher binary locally.

**Usage:**
```bash
./scripts/build.sh [OPTIONS]
```

**Options:**
- `--release` - Build optimized release binary (recommended for production)
- `--test` - Run unit tests after building
- `--strip` - Strip debug symbols from binary (requires `--release`)
- `-h, --help` - Show help message

**Examples:**

```bash
# Build debug binary
./scripts/build.sh

# Build release binary with optimizations
./scripts/build.sh --release

# Build release and run tests
./scripts/build.sh --release --test

# Build and strip symbols for smaller binary
./scripts/build.sh --release --strip
```

**Output:**
- Debug: `target/debug/kitty-launcher`
- Release: `target/release/kitty-launcher`

---

### `build-deb.sh` - Debian Package Builder

Builds a complete Debian package (.deb) for distribution and installation.

**Usage:**
```bash
./scripts/build-deb.sh [OPTIONS]
```

**Options:**
- `--clean` - Clean all build artifacts before building fresh
- `--signed` - Sign the package with GPG key (requires configured GPG)
- `-h, --help` - Show help message

**Examples:**

```bash
# Build unsigned Debian package
./scripts/build-deb.sh

# Clean and build fresh
./scripts/build-deb.sh --clean

# Build with signature
./scripts/build-deb.sh --signed --clean
```

**Output:**
- Debian package: `kitty-launcher_*.deb`
- Also creates: `.build`, `.changes`, `.buildinfo` files

**Installation:**
```bash
sudo dpkg -i kitty-launcher_*.deb
```

---

## Build Flow

### Local Development

1. **For Testing:**
   ```bash
   ./scripts/build.sh --test
   ```

2. **For Release Candidate:**
   ```bash
   ./scripts/build.sh --release --test
   ```

3. **For Distribution (Debian):**
   ```bash
   ./scripts/build-deb.sh --clean
   ```

### CI/CD Automation

The GitHub Actions workflows automatically call these scripts:

- **build-and-test.yml** - Uses `./scripts/build.sh --release --test`
- **release.yml** - Uses `./scripts/build-deb.sh --clean` and `./scripts/build.sh --release --strip --test`

---

## Dependencies

### Required for Building
- `cargo` - Rust package manager
- `rustc` - Rust compiler

### Required for Debian Packaging
```bash
sudo apt-get install build-essential debhelper dpkg-dev
```

### Optional for Signing
- `gpg` - For signing packages
- `gnupg` - GPG implementation

---

## Environment Variables

These scripts respect standard Rust build environment variables:

- `RUST_BACKTRACE` - Set to `1` for debugging
- `RUST_LOG` - Set logging level

Example:
```bash
RUST_BACKTRACE=1 ./scripts/build.sh --release
```

---

## Troubleshooting

### Build fails with "command not found"
- Ensure Rust is installed: `rustup install stable`
- For Debian builds, install: `sudo apt-get install build-essential debhelper dpkg-dev`

### Permission denied on build scripts
```bash
chmod +x scripts/build.sh scripts/build-deb.sh
```

### Debian package build fails
- Check that you're in the project root directory
- Verify `debian/` directory exists with proper files
- Try cleaning with `--clean` flag

---

## Integration with CI/CD

These scripts are designed to work seamlessly with GitHub Actions:

```yaml
- name: Build with wrapper
  run: ./scripts/build.sh --release --test

- name: Build Debian package
  run: ./scripts/build-deb.sh --clean
```

---

## Contributing

To modify or extend these scripts:

1. Follow the existing error handling patterns
2. Use the color codes for consistent output
3. Ensure backward compatibility
4. Test both locally and in CI/CD

---

## License

These build scripts are part of kitty-launcher and are licensed under MIT.
