# OpenSCENARIO MCP Server - Fresh Installation Guide

**Complete setup guide for a new machine**

---

## Prerequisites

### 1. Install Rust

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts (default installation is fine)
# Then reload shell:
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

**Expected output**: `rustc 1.70+` and `cargo 1.70+`

---

### 2. Install Python 3.8+

**Ubuntu/Debian**:
```bash
sudo apt update
sudo apt install python3 python3-pip
```

**Fedora**:
```bash
sudo dnf install python3 python3-pip
```

**macOS**:
```bash
brew install python3
```

**Verify**:
```bash
python3 --version  # Should be 3.8 or higher
```

---

### 3. Install SUMO (Required for OSM conversion)

SUMO provides the `netconvert` tool needed to convert OpenStreetMap data to OpenDRIVE format.

**Ubuntu/Debian**:
```bash
sudo add-apt-repository ppa:sumo/stable
sudo apt update
sudo apt install sumo sumo-tools
```

**Fedora**:
```bash
sudo dnf install sumo sumo-tools
```

**macOS**:
```bash
brew install sumo
```

**Verify installation**:
```bash
netconvert --version
```

**Expected output**: `Eclipse SUMO netconvert Version 1.21.0` (or higher)

---

## Installation

### Step 1: Clone the Repository

```bash
# Choose a location (example uses home directory)
cd ~
git clone https://github.com/jakeaboganda/osc-mcp.git
cd osc-mcp
```

**Or if you have the code as a tarball**:
```bash
tar -xzf osc-mcp.tar.gz
cd osc-mcp
```

---

### Step 2: Install Python Dependencies

```bash
pip3 install requests
```

This is needed for the OSM download script.

---

### Step 3: Build the MCP Server

```bash
# Build in release mode (optimized)
cargo build --release

# This takes 2-5 minutes on first build
# Binary will be created at: target/release/openscenario-mcp
```

**Verify build**:
```bash
ls -lh target/release/openscenario-mcp
```

You should see the compiled binary (~10-20MB).

---

### Step 4: Test the Installation

```bash
# Test 1: Run a quick example
cargo run --example test_get_real_world_road

# Expected: Downloads Nihonbashi roads, shows analysis
# Takes ~20 seconds on first run
```

**Expected output**:
```
🧪 Testing MCP Integration: get_real_world_road
============================================================
🌍 Fetching real-world road: nihonbashi
✅ Success!
Response: {
  "location": "nihonbashi",
  "good_roads": 84,
  "total_roads": 1150,
  ...
}
```

---

## Configuration

### For Claude Desktop

**1. Install Claude Desktop**:
- Download from: https://claude.ai/download
- Or: `sudo snap install claude-desktop` (if available)

**2. Create configuration file**:

```bash
# Create config directory
mkdir -p ~/.config/Claude

# Create config file
nano ~/.config/Claude/claude_desktop_config.json
```

**3. Add this configuration** (replace `/path/to/` with your actual path):

```json
{
  "mcpServers": {
    "openscenario": {
      "command": "cargo",
      "args": [
        "run",
        "--release",
        "--manifest-path",
        "/home/YOUR_USERNAME/osc-mcp/openscenario-mcp/Cargo.toml"
      ],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

**Important**: Replace `/home/YOUR_USERNAME/` with your actual path!

**Quick way to get the path**:
```bash
cd ~/osc-mcp/openscenario-mcp
pwd  # Copy this output
```

**4. Restart Claude Desktop**

**5. Test it**:
Open Claude Desktop and say:
```
"Create a lane change scenario on Nihonbashi"
```

Claude should automatically call the MCP tools and generate a scenario!

---

### For VS Code (Optional)

**1. Install VS Code**:
- Download from: https://code.visualstudio.com/
- Or: `sudo snap install code --classic`

**2. Install GitHub Copilot**:
- In VS Code: Extensions → Search "GitHub Copilot"
- Install "GitHub Copilot" and "GitHub Copilot Chat"

**3. Open the project**:
```bash
code ~/osc-mcp
```

**4. Copilot is pre-configured!**

The `.github/copilot-instructions.md` file teaches Copilot about the MCP tools automatically.

**5. Test it**:
- Press `Ctrl+Shift+I` (Copilot Chat)
- Ask: "Show me how to use the MCP tools"

---

## Directory Structure After Installation

```
~/osc-mcp/
├── openscenario/              # Core Rust library
├── openscenario-mcp/          # MCP server
│   ├── Cargo.toml
│   ├── src/
│   └── examples/
├── tools/
│   └── osm/
│       └── osm_to_opendrive.py  # OSM downloader
├── cache/
│   └── osm/                   # Downloaded roads (auto-created)
├── target/
│   └── release/
│       └── openscenario-mcp   # Compiled binary
├── CLAUDE_USAGE.md            # Usage guide
├── VSCODE_USAGE.md
├── USAGE.md
└── README.md
```

---

## Troubleshooting

### "netconvert not found"

**Problem**: SUMO not installed or not in PATH

**Solution**:
```bash
# Install SUMO (see prerequisites above)

# If installed but not in PATH:
export SUMO_HOME=/usr/share/sumo
export PATH=$PATH:$SUMO_HOME/bin

# Make permanent:
echo 'export SUMO_HOME=/usr/share/sumo' >> ~/.bashrc
echo 'export PATH=$PATH:$SUMO_HOME/bin' >> ~/.bashrc
source ~/.bashrc
```

---

### "cargo: command not found"

**Problem**: Rust not installed or not in PATH

**Solution**:
```bash
# Install Rust (see prerequisites)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell
source $HOME/.cargo/env
```

---

### Build fails with "linker not found"

**Problem**: Missing build tools

**Solution**:

**Ubuntu/Debian**:
```bash
sudo apt install build-essential
```

**Fedora**:
```bash
sudo dnf groupinstall "Development Tools"
```

**macOS**:
```bash
xcode-select --install
```

---

### Python script fails: "ModuleNotFoundError: No module named 'requests'"

**Problem**: Missing Python dependency

**Solution**:
```bash
pip3 install requests
```

---

### Claude Desktop doesn't see the MCP server

**Problem**: Configuration file not loaded or wrong path

**Solutions**:

1. **Verify config file exists**:
```bash
cat ~/.config/Claude/claude_desktop_config.json
```

2. **Check the path is absolute**:
```json
{
  "mcpServers": {
    "openscenario": {
      "command": "cargo",
      "args": [
        "run",
        "--release",
        "--manifest-path",
        "/absolute/path/to/osc-mcp/openscenario-mcp/Cargo.toml"
      ]
    }
  }
}
```

3. **Restart Claude Desktop** (fully quit and reopen)

4. **Check Claude Desktop logs** (if available):
   - Look for MCP server connection messages
   - Check for error messages about the server

---

### First OSM download is slow

**This is normal!** First download includes:
- Querying OpenStreetMap Overpass API
- Converting with SUMO netconvert
- Analyzing road network

Takes 20-30 seconds. Subsequent requests use cached data.

---

## Verification Checklist

After installation, verify everything works:

- [ ] Rust installed: `cargo --version`
- [ ] Python installed: `python3 --version`
- [ ] SUMO installed: `netconvert --version`
- [ ] Requests module: `python3 -c "import requests; print('OK')"`
- [ ] Project cloned: `ls ~/osc-mcp/README.md`
- [ ] Build succeeds: `cargo build --release`
- [ ] Test passes: `cargo run --example test_get_real_world_road`
- [ ] Claude Desktop configured (if using)
- [ ] VS Code opened (if using)

---

## Quick Start Commands

After installation is complete:

### Test the Server
```bash
cd ~/osc-mcp
cargo run --example test_get_real_world_road
```

### Run the MCP Server Manually
```bash
cd ~/osc-mcp/openscenario-mcp
cargo run --release
```

### Use with Claude Desktop
Just open Claude and talk:
```
"Create a lane change scenario on Tokyo roads"
```

### Use with VS Code
```bash
code ~/osc-mcp
# Press Ctrl+Shift+I (Copilot Chat)
# Ask: "How do I use the MCP tools?"
```

---

## Environment Variables (Optional)

For convenience, you can add these to `~/.bashrc` or `~/.zshrc`:

```bash
# SUMO
export SUMO_HOME=/usr/share/sumo
export PATH=$PATH:$SUMO_HOME/bin

# Rust
export PATH=$PATH:$HOME/.cargo/bin

# OSC-MCP shortcut (optional)
alias osc-mcp='cd ~/osc-mcp && cargo run --release --bin openscenario-mcp'
```

Reload shell:
```bash
source ~/.bashrc  # or source ~/.zshrc
```

---

## Platform-Specific Notes

### Ubuntu 22.04 / 24.04
All packages available in official repos. Use `apt` as shown above.

### Fedora 40+
All packages available in official repos. Use `dnf` as shown above.

### macOS
Use Homebrew for all dependencies:
```bash
brew install rust python3 sumo
```

### Windows (WSL2)
Install WSL2, then follow Ubuntu instructions inside WSL.

---

## What to Do Next

After successful installation:

1. **Read the usage guide**: `~/osc-mcp/CLAUDE_USAGE.md` (recommended)
2. **Try generating a scenario**: Open Claude Desktop and experiment
3. **Explore examples**: `cargo run --example <example_name>`
4. **Check documentation**: All guides in `~/osc-mcp/*.md`

---

## Getting Help

**Documentation**:
- `CLAUDE_USAGE.md` - Complete Claude Desktop guide
- `VSCODE_USAGE.md` - VS Code guide
- `QUICKSTART.md` - Technical reference
- `README.md` - Project overview

**Common Issues**:
- Check the troubleshooting section above
- Verify all prerequisites are installed
- Make sure paths are absolute in config files

---

## Summary

**Minimum installation** (command sequence):
```bash
# Prerequisites
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
sudo apt install python3 python3-pip sumo sumo-tools  # Ubuntu
pip3 install requests

# Install
cd ~
git clone https://github.com/jakeaboganda/osc-mcp.git
cd osc-mcp
cargo build --release

# Test
cargo run --example test_get_real_world_road

# Configure Claude Desktop (edit paths!)
mkdir -p ~/.config/Claude
nano ~/.config/Claude/claude_desktop_config.json
# (paste config, save, restart Claude)
```

**Total time**: 15-20 minutes (including downloads)

---

**You're ready!** Open Claude Desktop and start generating scenarios! 🚀
