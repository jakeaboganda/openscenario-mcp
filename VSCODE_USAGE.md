# Using OpenSCENARIO MCP in VS Code

## Setup Complete! ✅

Your VS Code is now configured to work with the OpenSCENARIO MCP server.

---

## How to Use

### 1. Open VS Code in the Project
```bash
cd ~/.openclaw/workspace/osc-mcp
code .
```

### 2. Start the MCP Server

**Option A: Via Tasks**
- Press `Ctrl+Shift+P` (Command Palette)
- Type "Tasks: Run Task"
- Select "Run OpenSCENARIO MCP Server"

**Option B: Via Terminal**
```bash
cd openscenario-mcp
cargo run --release
```

The server runs on stdio and waits for MCP protocol messages.

---

## 3. Use GitHub Copilot Chat

Open Copilot Chat (Ctrl+Shift+I) and ask questions like:

### Example Conversations

**Q:** "How do I get roads for Tokyo's Nihonbashi area?"

**Copilot will suggest:**
```rust
use openscenario_mcp::handlers::handle_get_real_world_road;

let result = handle_get_real_world_road(
    state.clone(),
    "nihonbashi".to_string(),
    None,
)?;
```

---

**Q:** "Create a lane change scenario on this road"

**Copilot will suggest:**
```rust
use openscenario_mcp::scenario_templates::handle_create_quick_scenario;

let scenario = handle_create_quick_scenario(
    state.clone(),
    "lane_change".to_string(),
    Some(3),
)?;
```

---

**Q:** "Show me how to generate a complete test scenario"

**Copilot will provide the 3-step workflow:**
```rust
// Step 1: Get real road
let road_result = handle_get_real_world_road(
    state.clone(),
    "nihonbashi".to_string(),
    None,
)?;

// Step 2: Auto-generate scenario
let scenario_result = handle_create_quick_scenario(
    state.clone(),
    "lane_change".to_string(),
    None,
)?;

// Step 3: Parse scenario_id and export
let scenario_data: Value = serde_json::from_str(&scenario_result)?;
let scenario_id = scenario_data["scenario_id"]
    .as_str()
    .ok_or_else(|| anyhow!("Missing scenario_id"))?;

let export_result = handle_export_xml(
    state.clone(),
    scenario_id.to_string(),
    "scenario.xosc".to_string(),
)?;
```

---

## 4. Ask Copilot to Explain MCP Tools

Try these questions in Copilot Chat:

- "What MCP tools are available in this project?"
- "Show me how to use create_quick_scenario"
- "What's the difference between lane_change and cutin scenarios?"
- "How do I validate a road position?"
- "Write a function that generates 10 test scenarios"

Copilot now has context from `.github/copilot-instructions.md`!

---

## 5. Test with Copilot's Help

Ask Copilot:
- "Run the test_get_real_world_road example"
- "Check if SUMO is installed"
- "Show me the cached roads"

It will suggest appropriate commands.

---

## Direct MCP Client (Alternative)

If you want to call the MCP server directly from code, here's a pattern Copilot can help you with:

**Ask:** "Write a Rust function that calls the MCP server's get_real_world_road tool"

**Copilot will provide:**
```rust
use std::process::{Command, Stdio};
use std::io::Write;
use serde_json::json;

fn call_mcp_tool(tool: &str, params: serde_json::Value) -> anyhow::Result<String> {
    let mut child = Command::new("cargo")
        .args(["run", "--release"])
        .current_dir("openscenario-mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": tool,
            "arguments": params
        }
    });
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(request.to_string().as_bytes())?;
    }
    
    let output = child.wait_with_output()?;
    Ok(String::from_utf8(output.stdout)?)
}

// Usage
let result = call_mcp_tool("get_real_world_road", json!({
    "location": "nihonbashi"
}))?;
```

---

## What Copilot Knows

GitHub Copilot now understands:
- ✅ All 15 MCP tools (6 road + 5 scenario + 4 templates)
- ✅ Typical workflows (3-step quick scenario)
- ✅ Code patterns (error handling, state management)
- ✅ Tokyo locations (nihonbashi, tokyo_station, etc.)
- ✅ Scenario types (lane_change, cutin, platoon)
- ✅ Testing commands
- ✅ Common issues and solutions

---

## VS Code Tasks Available

Press `Ctrl+Shift+P` → "Tasks: Run Task":
- **Run OpenSCENARIO MCP Server** - Starts MCP server
- **Test: Get Real World Road** - Runs example test

---

## Tips

1. **Ask Copilot to generate scenarios**:
   - "Generate a cut-in scenario on Shibuya roads"
   - "Create a 5-vehicle platoon on Ginza"

2. **Ask Copilot to explain**:
   - "How does the spawn point suggestion work?"
   - "What parameters does create_cutin_scenario need?"

3. **Ask Copilot to refactor**:
   - "Add error handling to this scenario generation"
   - "Make this function async"

4. **Ask Copilot to test**:
   - "Write a test for the lane change scenario"
   - "Add assertions to validate the output"

---

## What's Next?

Now you can:
- Write code with Copilot assistance that uses MCP tools
- Ask Copilot questions about the project
- Generate test scenarios with Copilot's help
- Get code suggestions specific to autonomous vehicle testing

**Try it**: Open Copilot Chat and say "Show me how to use this MCP server" 🚀
