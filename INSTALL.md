# Installation Guide

**📚 Navigation**: [Home](README.md) | **Install** | [Quick Start](QUICKSTART.md) | [Usage](USAGE.md)

Complete setup guide for OpenSCENARIO MCP Server.

---

## Prerequisites

### Required

1. **Rust** (1.70+)
2. **Python** (3.8+) - for OpenDRIVE conversion
3. **Git**

### Optional (Recommended)

4. **Claude Desktop** - for easiest usage ([guide](CLAUDE_USAGE.md))
5. **esmini** - for visualizing scenarios ([setup below](#optional-esmini-simulator))

---

## Installation Steps

### 1. Install Rust

```bash
# Linux/Mac
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell
source $HOME/.cargo/env

# Verify
rustc --version  # Should show 1.70+
cargo --version
```

**Windows**: Download from [rustup.rs](https://rustup.rs)

---

### 2. Install Python 3.8+

**Ubuntu/Debian**:
```bash
sudo apt update && sudo apt install python3 python3-pip
```

**Fedora**:
```bash
sudo dnf install python3 python3-pip
```

**macOS**:
```bash
brew install python3
```

**Windows**: Download from [python.org](https://python.org)

**Verify**:
```bash
python3 --version  # Should be 3.8+
```

---

### 3. Clone Repository

```bash
git clone https://github.com/yourusername/osc-mcp.git
cd osc-mcp
```

---

### 4. Install Python Dependencies

```bash
# Install scenic and dependencies
pip3 install --user scenic
pip3 install --user shapely numpy
```

**Verify**:
```bash
python3 -c "import scenic; print('Scenic OK')"
```

---

### 5. Build the Project

```bash
# Build in release mode (optimized)
cargo build --release

# Or build in dev mode (faster compile, slower runtime)
cargo build
```

**First build takes 5-10 minutes** (downloads and compiles dependencies).

**Expected output**:
```
   Compiling openscenario v0.1.0
   Compiling openscenario-mcp v0.1.0
    Finished release [optimized] target(s) in 8m 23s
```

---

### 6. Test Installation

```bash
# Quick test - download Tokyo road
cargo run --example test_get_real_world_road
```

**Expected output**:
```
✅ Downloaded Nihonbashi road network
   Saved to: cache/osm/nihonbashi.xodr
   Roads: 1150
   Quality: 90/100
```

**Success?** Installation complete! → [Next: Quick Start](QUICKSTART.md)

**Problems?** → See [Troubleshooting](#troubleshooting) below

---

## Configure for Usage

Now choose how you want to use it:

### Option 1: Claude Desktop (Recommended)

**Easiest and most natural interface**

1. Install Claude Desktop from [claude.ai](https://claude.ai/download)
2. Configure MCP server:

Edit `~/.config/Claude/claude_desktop_config.json`:
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

**Replace** `/absolute/path/to` with your actual path (e.g., `/home/username/projects/osc-mcp`).

3. Restart Claude Desktop
4. Say: "Create a lane change scenario"

**Detailed guide**: [CLAUDE_USAGE.md](CLAUDE_USAGE.md)

---

### Option 2: VS Code + GitHub Copilot

**Best for developers**

1. Open project in VS Code:
```bash
code /path/to/your/osc-mcp
```

2. Copilot is pre-configured via `.github/copilot-instructions.md`

3. Ask Copilot (Ctrl+Shift+I):
```
"How do I create a lane change scenario?"
"Generate a cut-in scenario"
```

**Detailed guide**: [VSCODE_USAGE.md](VSCODE_USAGE.md)

---

### Option 3: Direct MCP API

**For advanced users / custom integrations**

```bash
cd openscenario-mcp
cargo run --release
```

Server listens on stdio for MCP messages.

**Detailed guide**: [USAGE.md](USAGE.md)

---

## Optional: esmini Simulator

**Visualize your generated scenarios**

### Download esmini

```bash
# Create tools directory
mkdir -p ~/tools
cd ~/tools

# Download esmini (Linux example - adjust URL for your OS)
wget https://github.com/esmini/esmini/releases/download/v2.37.4/esmini-demo_Ubuntu_20.04.zip
unzip esmini-demo_Ubuntu_20.04.zip

# Verify
./esmini-demo/bin/esmini --version
```

**macOS/Windows**: Download from [esmini releases](https://github.com/esmini/esmini/releases)

### Test with a scenario

```bash
cd ~/tools/esmini-demo
./bin/esmini --osc /path/to/your/scenario.xosc
```

**Detailed guide**: [docs/using-with-esmini.md](docs/using-with-esmini.md)

---

## Troubleshooting

### Build Errors

**"error: linker 'cc' not found"** (Linux):
```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora
sudo dnf install gcc
```

**"failed to run custom build command for openssl-sys"**:
```bash
# Ubuntu/Debian
sudo apt install pkg-config libssl-dev

# Fedora
sudo dnf install openssl-devel

# macOS
brew install openssl
```

---

### Python Issues

**"No module named 'scenic'"**:
```bash
# Try with --break-system-packages on newer systems
pip3 install --user scenic --break-system-packages

# Or use virtual environment
python3 -m venv venv
source venv/bin/activate
pip install scenic
```

**"ImportError: cannot import name 'Mapping' from 'collections'"** (Python 3.10+):

This is a known scenic issue. Workaround:
```bash
# Use Python 3.9
pyenv install 3.9.18
pyenv local 3.9.18
pip install scenic
```

---

### Permission Issues

**"Permission denied"** during build:
```bash
# Fix cargo cache permissions
sudo chown -R $USER:$USER ~/.cargo
```

---

### Still Having Issues?

1. Check [GitHub Issues](https://github.com/yourusername/osc-mcp/issues)
2. Open a new issue with:
   - Your OS and version
   - Rust version (`rustc --version`)
   - Python version (`python3 --version`)
   - Full error message

---

## Verify Everything Works

Run the full test suite:

```bash
# 1. OSM road download
cargo run --example test_get_real_world_road

# 2. Custom XODR loading
cargo run --example test_custom_xodr

# 3. Scenario generation
cargo run --example test_scenario_templates

# All passing? ✅ You're ready!
```

---

## Next Steps

**Installation complete!** Choose your next step:

1. **[Quick Start Guide](QUICKSTART.md)** - 5-minute test run
2. **[Claude Desktop Usage](CLAUDE_USAGE.md)** - Start using with Claude
3. **[VS Code Usage](VSCODE_USAGE.md)** - Developer workflow
4. **[General Usage](USAGE.md)** - All interfaces and tools

---

**Questions?** See [CONTRIBUTING.md](CONTRIBUTING.md) or open an issue.
