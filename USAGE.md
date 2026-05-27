# How to Use OpenSCENARIO MCP Server

Two ways to use the OpenSCENARIO MCP server: **Claude Desktop** (recommended) and **VS Code with Copilot**.

---

## 🏆 Method 1: Claude Desktop (Recommended)

**Best for**: Interactive scenario generation, no coding needed

### Quick Setup

1. **Install Claude Desktop**: https://claude.ai/download

2. **Configure MCP server**:
```bash
mkdir -p ~/.config/Claude
cat > ~/.config/Claude/claude_desktop_config.json << 'EOF'
{
  "mcpServers": {
    "openscenario": {
      "command": "cargo",
      "args": [
        "run",
        "--release",
        "--manifest-path",
        "/absolute/path/to/osc-mcp/openscenario-mcp/Cargo.toml"
      ],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}

**Note**: Replace `/absolute/path/to` with your actual installation path.
EOF
```

3. **Restart Claude Desktop**

4. **Start talking**:
```
"Create a lane change scenario on Nihonbashi highway"
```

**Details**: See [CLAUDE_USAGE.md](./CLAUDE_USAGE.md)

---

## 💻 Method 2: VS Code with GitHub Copilot

**Best for**: Developing code, extending the tool, debugging

### Quick Setup

1. **Open project in VS Code**:
```bash
code /path/to/your/osc-mcp  # Replace with your actual path
```

2. **Copilot is already configured** (via `.github/copilot-instructions.md`)

3. **Ask Copilot** (Ctrl+Shift+I):
```
"Show me how to create a lane change scenario"
```

**Details**: See [VSCODE_USAGE.md](./VSCODE_USAGE.md)

---

## 🆚 Which Should You Use?

### Use Claude Desktop When:
- ✅ You want to generate scenarios quickly
- ✅ You prefer natural conversation
- ✅ No coding needed
- ✅ Interactive exploration
- ✅ Testing and experimentation

**Example**: "Create 5 different test scenarios on Tokyo highways"

### Use VS Code Copilot When:
- ✅ You're writing code that uses the MCP tools
- ✅ You want to extend the functionality
- ✅ You need precise control over parameters
- ✅ Debugging Rust code
- ✅ Building new features

**Example**: "Write a function that generates 100 test scenarios"

---

## 📚 Full Documentation

| Guide | Description |
|-------|-------------|
| [CLAUDE_USAGE.md](./CLAUDE_USAGE.md) | Complete Claude Desktop guide |
| [VSCODE_USAGE.md](./VSCODE_USAGE.md) | Complete VS Code + Copilot guide |
| [QUICKSTART.md](./QUICKSTART.md) | Technical reference |

---

## 🚀 Quick Examples

### Claude Desktop (No Code)
```
You: "I need to test lane change on Tokyo roads"
Claude: [downloads roads, generates scenario, exports]
Claude: "Done! Scenario saved as lane_change_nihonbashi.xosc"
```

### VS Code Copilot (With Code)
```rust
// Ask Copilot: "Generate a lane change scenario"
use openscenario_mcp::handlers::*;
use openscenario_mcp::scenario_templates::*;

let state = Arc::new(Mutex::new(ServerState::new()));
handle_get_real_world_road(state.clone(), "nihonbashi".into(), None)?;
handle_create_quick_scenario(state, "lane_change".into(), Some(3))?;
```

---

## 🎯 Recommendation

**Start with Claude Desktop!** It's:
- Faster to set up
- Easier to use
- No coding required
- More interactive

**Then use VS Code** when you want to:
- Build custom tools
- Integrate into larger systems
- Write automated tests
- Extend the MCP server

---

## 💡 Can You Use Both?

**Yes!** They work great together:

1. **Explore with Claude Desktop**: "What roads are available in Shibuya?"
2. **Build with VS Code**: Write code to automate what you learned
3. **Test with Claude Desktop**: Verify your new features work
4. **Deploy**: Use your extended MCP server

---

## 🔧 Prerequisites

Both methods require:
- ✅ Rust 1.70+
- ✅ Python 3.8+
- ✅ SUMO (for netconvert): `sudo apt install sumo sumo-tools`

---

## 📊 Feature Comparison

| Feature | Claude Desktop | VS Code Copilot |
|---------|----------------|-----------------|
| **Setup time** | 2 minutes | 5 minutes |
| **Learning curve** | Easy | Medium |
| **Natural language** | ✅ Full | ⚠️ Code-focused |
| **No coding** | ✅ Yes | ❌ No |
| **Code assistance** | ❌ No | ✅ Yes |
| **Interactive** | ✅ Yes | ⚠️ Limited |
| **Tool chaining** | ✅ Automatic | ❌ Manual |
| **Best for beginners** | ✅ Yes | ❌ No |
| **Best for developers** | ⚠️ Limited | ✅ Yes |

---

## 🎬 Getting Started

### Absolute Beginner?
→ **Use Claude Desktop**  
→ Read [CLAUDE_USAGE.md](./CLAUDE_USAGE.md)

### Experienced Developer?
→ **Use VS Code Copilot**  
→ Read [VSCODE_USAGE.md](./VSCODE_USAGE.md)

### Want Both?
→ **Set up Claude Desktop first** (quick wins)  
→ **Then add VS Code** (deeper integration)

---

## 🚀 Next Steps

**Choose your path**:

**Path A: Claude Desktop** (5 minutes)
1. Install Claude Desktop
2. Copy config JSON
3. Restart Claude
4. Say: "Create a scenario on Nihonbashi"

**Path B: VS Code** (10 minutes)
1. `code /path/to/your/osc-mcp`  # Replace with actual path
2. Open Copilot Chat (Ctrl+Shift+I)
3. Ask: "How do I use the MCP tools?"
4. Start coding!

**Path C: Both** (15 minutes)
1. Set up Claude Desktop (5 min)
2. Try generating a few scenarios
3. Open VS Code (5 min)
4. Build on what you learned (5 min)

---

## ✅ You're Ready!

Pick your method and start generating autonomous vehicle test scenarios on real Tokyo roads! 🚗🗺️

**Questions?** Check the detailed guides:
- [CLAUDE_USAGE.md](./CLAUDE_USAGE.md) - Claude Desktop
- [VSCODE_USAGE.md](./VSCODE_USAGE.md) - VS Code + Copilot
- [QUICKSTART.md](./QUICKSTART.md) - Technical reference

**Happy testing!** 🎉
