# Installation Guide for Kitty Launcher

This guide covers installing kitty-launcher from source and as a Debian package.

## Prerequisites

- **Debian/Ubuntu system** (tested on Debian Bullseye and Ubuntu 20.04+)
- **Kitty terminal emulator** installed
- **Rust toolchain** (for building from source)

### Install Kitty

```bash
# Debian/Ubuntu
sudo apt-get install kitty

# Or compile from source (https://sw.kovidgoyal.net/kitty/build/)
```

## Option 1: Install from Source (Development)

### Prerequisites for Building

```bash
sudo apt-get install cargo rustc build-essential debhelper
```

### Build and Install

```bash
cd kitty-launcher
cargo build --release
sudo cp target/release/kitty-launcher /usr/local/bin/
sudo chmod +x /usr/local/bin/kitty-launcher
```

### Verify Installation

```bash
kitty-launcher --help
# or just check with:
which kitty-launcher
```

## Option 2: Build and Install Debian Package

### Prerequisites

```bash
sudo apt-get install cargo rustc build-essential debhelper dpkg-dev
```

### Build Package

```bash
cd kitty-launcher
dpkg-buildpackage -us -uc
```

This will create a `.deb` file in the parent directory.

### Install Package

```bash
sudo dpkg -i ../kitty-launcher_0.1.0-1_*.deb
```

### Verify Installation

```bash
kitty-launcher
# Should show usage message
```

## Setup Configuration Files

Before using kitty-launcher, you need to create session configuration files.

### Create a Session Configuration

Sessions should be files in one of these directories (checked in order):

1. `./etc/kitty/` (current directory)
2. `~/.local/etc/kitty/` (user's local config)
3. `/opt/etc/kitty/` (system-wide optional)
4. `~/.config/kitty/` (kitty standard location)

### Example: Create a Development Session

```bash
# Create directory if it doesn't exist
mkdir -p ~/.local/etc/kitty

# Create a session file (use kitty's session file format)
# You can copy an existing kitty session file or create one
cp ~/.config/kitty/sessions/dev ~/.local/etc/kitty/dev

# Or create a simple session file:
cat > ~/.local/etc/kitty/dev << 'EOF'
# Kitty session file for development
new_window
EOF
```

For more details on kitty session file format, see:
https://sw.kovidgoyal.net/kitty/launch/

### Example: Create System-wide Sessions

If you want to provide sessions for all users:

```bash
sudo mkdir -p /opt/etc/kitty
sudo cp /path/to/session/config /opt/etc/kitty/myapp
sudo chmod 644 /opt/etc/kitty/myapp
```

## Usage

Once configured, you can launch sessions with:

```bash
# Launch the 'dev' session
kitty-launcher dev

# Launch any configured session
kitty-launcher myapp
kitty-launcher default
```

## Uninstall

### If installed from source

```bash
sudo rm /usr/local/bin/kitty-launcher
```

### If installed from Debian package

```bash
sudo apt-get remove kitty-launcher
# Or
sudo dpkg -r kitty-launcher
```

## Troubleshooting

### "Session name cannot be empty"

You forgot to provide a session name:

```bash
# Wrong
kitty-launcher

# Correct
kitty-launcher dev
```

### "Configuration file not found"

The session file doesn't exist in any of the standard locations. Create it:

```bash
mkdir -p ~/.local/etc/kitty
# Create or copy your session configuration
# Then use: kitty-launcher yoursessionname
```

### "Failed to launch kitty"

Make sure kitty is installed:

```bash
which kitty
sudo apt-get install kitty
```

### "Invalid session name"

You tried to use special characters or paths. Session names can only contain:
- Alphanumeric characters (a-z, A-Z, 0-9)
- Hyphens (-)
- Underscores (_)
- Dots (.)

```bash
# Wrong
kitty-launcher ../../etc/passwd
kitty-launcher dev@home
kitty-launcher dev/work

# Correct
kitty-launcher dev
kitty-launcher dev_work
kitty-launcher dev-session
```

## Configuration Search Order

The program searches for configuration files in this exact order:

```
1. ./etc/kitty/                  (current working directory)
2. ~/.local/etc/kitty/            (user local config)
3. /opt/etc/kitty/                (system-wide optional)
4. ~/.config/kitty/               (kitty standard config location)
```

The first match is used. This allows:
- **Project-specific sessions** in `./etc/kitty/`
- **User sessions** in `~/.local/etc/kitty/`
- **System-wide sessions** in `/opt/etc/kitty/`
- **Fallback to kitty's default** location

## Advanced: Custom Build Options

### Debug Build (faster compilation)

```bash
cargo build
```

### Release Build with Optimizations

```bash
cargo build --release
```

## Getting Help

For issues or questions:

1. Check the [LEARNING_GUIDE.md](LEARNING_GUIDE.md) for understanding the code
2. Read [README.md](README.md) for overview
3. Check the source code comments - they're very detailed!
4. Run: `kitty-launcher` without arguments to see usage

## Next Steps

1. Read the [README.md](README.md) for feature overview
2. Check [LEARNING_GUIDE.md](LEARNING_GUIDE.md) to understand how it works
3. Create your session configuration files
4. Enjoy flexible kitty sessions!
