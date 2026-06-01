# MCP Server Initialization Options

## ✅ Updated Documentation

All configuration docs now support **3 flexible initialization methods**:

---

## 🎯 Three Methods

### **1. Pre-built Binary** ⭐ Recommended

**Best for**: End users, production use  
**Speed**: Fastest (no build overhead)  
**Setup**: Build once, use forever

```bash
# Build the release binary
cd osc-mcp
cargo build --release
```

**Configuration**:
```json
{
  "mcpServers": {
    "openscenario": {
      "command": "/absolute/path/to/osc-mcp/target/release/openscenario-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

**Pros**:
- ⚡ Instant startup (~100ms)
- 🎯 No dependencies needed at runtime
- 💯 Production-ready

**Cons**:
- 📍 Requires absolute path
- 🔄 Must rebuild after code changes

---

### **2. Build On-Demand**

**Best for**: Developers, active development  
**Speed**: Slower startup (~5-10s first time, cached after)  
**Setup**: Works immediately from source

**Configuration**:
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

**Pros**:
- 🔄 Always uses latest code
- 🛠️ Great for development
- 📝 No separate build step

**Cons**:
- 🐌 Slower startup (builds on first run)
- 📍 Requires absolute path
- 🦀 Needs Rust toolchain installed

---

### **3. Global Install** 🌟 Cleanest

**Best for**: Regular users, system-wide access  
**Speed**: Fastest (like method 1)  
**Setup**: Install once, use anywhere

```bash
# Install to ~/.cargo/bin/
cd osc-mcp/openscenario-mcp
cargo install --path .
```

**Configuration**:
```json
{
  "mcpServers": {
    "openscenario": {
      "command": "openscenario-mcp",
      "args": []
    }
  }
}
```

**Pros**:
- ✨ No absolute paths needed
- 🌍 Works from any directory
- 🎯 Like any system CLI tool
- ⚡ Fast startup

**Cons**:
- 🔄 Must reinstall after updates
- 💾 Uses ~/.cargo/bin/ space

**Tip**: Update with `cargo install --path . --force`

---

## 📊 Comparison

| Method | Startup Speed | Path Required | Updates | Best For |
|--------|---------------|---------------|---------|----------|
| **Pre-built Binary** | ⚡⚡⚡ Instant | ✅ Absolute | Manual rebuild | End users |
| **Build On-Demand** | 🐌 5-10s | ✅ Absolute | Automatic | Developers |
| **Global Install** | ⚡⚡⚡ Instant | ❌ None | Reinstall | Regular use |

---

## 🎯 Recommendation by Use Case

### **Just want to use it** → Method 3 (Global Install)
```bash
cd osc-mcp/openscenario-mcp
cargo install --path .
# Configure with: "command": "openscenario-mcp"
```

### **Actively developing** → Method 2 (Build On-Demand)
```json
"command": "cargo",
"args": ["run", "--release", "--manifest-path", "..."]
```

### **Production deployment** → Method 1 (Pre-built Binary)
```bash
cargo build --release
# Use: target/release/openscenario-mcp
```

---

## 📝 Updated Files

- ✅ `INSTALL.md` - All 3 methods documented
- ✅ `CLAUDE_USAGE.md` - All 3 methods with examples
- ✅ `USAGE.md` - Quick setup updated
- ✅ `openscenario-mcp/README.md` - All 3 methods

---

## 💡 Tips

**Get your absolute path**:
```bash
cd osc-mcp && pwd  # Copy this!
```

**Verify installation** (method 3):
```bash
which openscenario-mcp
# Should show: /home/username/.cargo/bin/openscenario-mcp
```

**Check it works**:
```bash
# Method 1 or 3
/path/to/openscenario-mcp --version
# or
openscenario-mcp --version

# Method 2
cd osc-mcp/openscenario-mcp
cargo run --release -- --version
```

---

## 🚀 What Changed

**Before**:
- Only documented `cargo run` method
- Required absolute path to Cargo.toml
- Slow startup every time
- Not friendly for end users

**After**:
- 3 flexible methods for different needs
- Global install option (no paths!)
- Pre-built binary option (fast startup)
- Clear recommendations by use case
- Better developer experience

---

**Commit**: `ab358a6` - docs: Add multiple MCP server initialization options
