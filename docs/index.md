---
layout: default
title: Z-Kitty Launcher Documentation
---

# Z-Kitty Launcher

**A robust, lightning-fast Rust-based terminal session manager for the Kitty terminal emulator.**

Perfect for power users who manage multiple terminal configurations, developers working on diverse projects, and system administrators handling complex workflows.

## Quick Links

- **[GitHub Repository](https://github.com/pilakkat1964/z-kitty-launcher)** - Source code and issue tracking
- **[Latest Release](https://github.com/pilakkat1964/z-kitty-launcher/releases)** - Download v0.5.2

---

## 📚 Documentation

### Getting Started
- **[Installation Guide](./INSTALL.md)** - Install Z-Kitty Launcher on your system
- **[Quick Reference](./QUICK_REFERENCE.md)** - Common commands and options at a glance

### Detailed Documentation  
- **[Project Summary](./PROJECT_SUMMARY.md)** - Comprehensive project overview
- **[Manifest](./MANIFEST.md)** - Detailed feature list and capabilities
- **[Learning Guide](./LEARNING_GUIDE.md)** - Rust implementation guide for learners

### Reference
- **[Man Page](./kitty-launcher.1)** - Full manual page
- **[Info Page](./kitty-launcher.info)** - Info format documentation

---

## Key Features

✨ **Session Management**
- Launch terminal sessions with custom configurations
- Create and manage multiple session presets
- Automatic configuration discovery

🎯 **Desktop Integration**
- Create .desktop launcher files
- KDE Plasma folder view support
- System-wide launcher installation

🔧 **Shell Completions**
- Bash completion support
- Zsh completion support
- Auto-discovery of available sessions

🛡️ **Security & Performance**
- Path traversal attack prevention
- Input sanitization
- Written in pure Rust with zero external dependencies
- ~509 KB release binary

---

## Installation

### From Debian Package (Recommended)

**AMD64:**
```bash
sudo dpkg -i kitty-launcher_0.5.2-1_amd64.deb
```

**ARM64 (Raspberry Pi, etc.):**
```bash
sudo dpkg -i kitty-launcher_0.5.2-1_arm64.deb
```

### From Precompiled Binary

```bash
wget https://github.com/pilakkat1964/z-kitty-launcher/releases/download/v0.5.2/kitty-launcher-v0.5.2-linux-amd64
chmod +x kitty-launcher-v0.5.2-linux-amd64
sudo cp kitty-launcher-v0.5.2-linux-amd64 /usr/local/bin/kitty-launcher
```

### From Source

```bash
git clone https://github.com/pilakkat1964/z-kitty-launcher.git
cd z-kitty-launcher
cargo build --release
sudo cp target/release/kitty-launcher /usr/local/bin/
```

---

## Basic Usage

### Launch a Session
```bash
kitty-launcher session-name
```

### Create a New Session
```bash
kitty-launcher -c my-project
$EDITOR ~/.local/etc/kitty/my-project.session
```

### View Help
```bash
kitty-launcher --help
```

### View Version
```bash
kitty-launcher --version
```

---

## Version Information

- **Current Version:** v0.5.2
- **Build Status:** ✅ Production Ready
- **Test Coverage:** 7/7 tests passing
- **License:** MIT

---

## Support & Contribution

- **Issues:** [Report bugs on GitHub](https://github.com/pilakkat1964/z-kitty-launcher/issues)
- **Discussions:** [GitHub Discussions](https://github.com/pilakkat1964/z-kitty-launcher/discussions)
- **License:** MIT - Feel free to use and modify

---

## 🔗 Related Z-Tools Projects

**Explore other tools in the z-tools ecosystem:**

- **[Z-Edit](http://pilakkat.mywire.org/z-edit/)** — Opens files with the right editor
- **[Z-Open](http://pilakkat.mywire.org/z-open/)** — Opens files/URLs with the right application
- **[RClone Mount Applete](https://pilakkat.mywire.org/z-rclone-mount-applete/)** — System tray manager for cloud storage

**[→ View Master Index](http://pilakkat.mywire.org/master-index/)** — Complete guide to all z-tools projects

---

## Build Information

This project is built with:
- **Language:** Rust (100% pure, zero external dependencies)
- **Binary Size:** ~509 KB (release, stripped)
- **Build Time:** ~1.5 seconds
- **Test Suite:** 7 unit tests

Learn more about the Rust implementation in the [Learning Guide](./LEARNING_GUIDE.md).

---

*Last Updated: April 16, 2026*
