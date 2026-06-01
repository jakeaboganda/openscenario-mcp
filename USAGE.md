# Usage Guide

**📚 Navigation**: [Home](README.md) | [Install](INSTALL.md) | [Quick Start](QUICKSTART.md) | **Usage**

How to use OpenSCENARIO MCP Server with different interfaces.

---

## Choose Your Interface

### 🎯 Option 1: Claude Desktop (Recommended)

**Best for**: Natural language scenario generation  
**Setup time**: 5 minutes  
**Experience**: Just talk naturally

**What you get**:
- Talk to Claude like a human: *"Create a lane change scenario on Tokyo roads"*
- Claude automatically calls the right MCP tools
- Get back production-ready `.xosc` files
- No coding required

**Setup**: See detailed guide → **[CLAUDE_USAGE.md](CLAUDE_USAGE.md)**

**Quick setup**:
1. Install Claude Desktop
2. Build the server: `cd osc-mcp && cargo build --release`
3. Add to `~/.config/Claude/claude_desktop_config.json`:
   ```json
   {
     "mcpServers": {
       "openscenario": {
         "command": "/absolute/path/to/osc-mcp/target/release/openscenario-mcp",
         "args": []
       }
     }
   }
   ```
   Or install globally: `cargo install --path openscenario-mcp` then use `"command": "openscenario-mcp"`
4. Restart Claude
5. Say: "Create a scenario"

---

### 🔧 Option 2: Direct MCP API

**Best for**: Advanced users, custom integrations  
**Setup time**: 1 minute  
**Experience**: Full programmatic control

**What you get**:
- Direct access to all 18 MCP tools
- Build custom AI workflows
- Integrate with your own systems
- Stdio protocol communication

**Setup**: See detailed guide below → [Direct API Usage](#direct-api-usage)

---

## Common Workflows

### Workflow 1: Real-World Road Scenario

Generate a scenario on actual roads from OpenStreetMap:

**With Claude**:
```
"Create a lane change scenario on Nihonbashi highway in Tokyo"
```

**With API**:
```json
{"method": "tools/call", "params": {
  "name": "get_real_world_road",
  "arguments": {"location": "shibuya"}
}}
```

---

### Workflow 2: Custom XODR File

Use your own road network file:

**With Claude**:
```
"Load my custom track from /home/user/my_track.xodr and create a platoon scenario"
```

**With API**:
```json
{"method": "tools/call", "params": {
  "name": "load_road_network",
  "arguments": {"xodr_path": "/path/to/track.xodr"}
}}
```

**Detailed guide**: [CUSTOM_XODR.md](CUSTOM_XODR.md)

---

### Workflow 3: Multi-Entity Scenarios

Add vehicles, pedestrians, obstacles:

**With Claude**:
```
"Create a scenario with 3 vehicles and a pedestrian crossing at 100m"
```

**With API**:
```json
// 1. Create scenario
{"method": "tools/call", "params": {
  "name": "create_scenario",
  "arguments": {"name": "crossing_scenario"}
}}

// 2. Add entities
{"method": "tools/call", "params": {
  "name": "add_vehicle",
  "arguments": {"scenario_id": "...", "name": "vehicle1"}
}}

{"method": "tools/call", "params": {
  "name": "add_pedestrian",
  "arguments": {"scenario_id": "...", "name": "ped1"}
}}

// 3. Position them
{"method": "tools/call", "params": {
  "name": "set_lane_position",
  "arguments": {"entity_name": "ped1", "road_id": "1", "lane_id": -1, "s": 100, "offset": 0}
}}
```

---

## Available Tools Reference

### Road Network Tools

| Tool | Purpose | Example |
|------|---------|---------|
| `get_real_world_road` | Download OSM roads | `location: "tokyo"` |
| `load_road_network` | Load custom `.xodr` | `xodr_path: "/path/to/file.xodr"` |
| `list_roads` | List available roads | (no args) |
| `get_road_info` | Get road details | `road_id: "5402"` |

---

### Scenario Creation Tools

| Tool | Purpose | Example |
|------|---------|---------|
| `create_scenario` | Create empty scenario | `name: "my_scenario"` |
| `create_quick_scenario` | Auto-generate scenario | `scenario_type: "lane_change"` |
| `validate_scenario` | Check scenario validity | `scenario_id: "..."` |
| `export_xml` | Export to `.xosc` file | `filename: "output.xosc"` |

---

### Entity Tools

| Tool | Purpose | Example |
|------|---------|---------|
| `add_vehicle` | Add car/truck | `name: "ego"` |
| `add_pedestrian` | Add pedestrian | `name: "ped1", mass: 70.0` |
| `add_misc_object` | Add obstacle/barrier | `category: "barrier", mass: 500` |

---

### Positioning Tools

| Tool | Purpose | Example |
|------|---------|---------|
| `set_position` | Set world position | `x: 100, y: 50, z: 0` |
| `set_lane_position` | Set lane position | `road_id: "1", lane_id: -2, s: 100` |
| `validate_position` | Check if valid | `road_id: "1", s: 500` |
| `suggest_spawn_points` | Find good positions | `road_id: "1", count: 5` |

---

### Action Tools

| Tool | Purpose | Example |
|------|---------|---------|
| `add_speed_action` | Set speed | `entity: "ego", speed: 25.0` |
| `add_lane_change_action` | Change lanes | `target_lane_offset: 1` |

---

### Trigger Tools

| Tool | Purpose | Example |
|------|---------|---------|
| `set_stop_time` | Stop at time | `time: 10.0` |
| `set_stop_on_element` | Stop on condition | `element_type: "SpeedCondition"` |

**Full API documentation**: Each tool returns JSON with results or errors.

---

## Direct API Usage

### Starting the Server

```bash
cd openscenario-mcp
cargo run --release
```

Server listens on **stdin/stdout** for MCP protocol messages.

---

### MCP Protocol Format

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "tool_name",
    "arguments": {
      "arg1": "value1",
      "arg2": "value2"
    }
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {"type": "text", "text": "Result data here"}
    ]
  }
}
```

---

### Example API Session

**1. List available tools**:
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | cargo run --release
```

**2. Download roads**:
```bash
echo '{
  "jsonrpc":"2.0",
  "id":2,
  "method":"tools/call",
  "params":{
    "name":"get_real_world_road",
    "arguments":{"location":"tokyo"}
  }
}' | cargo run --release
```

**3. Create scenario**:
```bash
echo '{
  "jsonrpc":"2.0",
  "id":3,
  "method":"tools/call",
  "params":{
    "name":"create_quick_scenario",
    "arguments":{"scenario_type":"lane_change"}
  }
}' | cargo run --release
```

---

## Output Files

### Generated Files

Scenarios are saved in the project directory:

```
osc-mcp/
├── lane_change_nihonbashi.xosc    # Generated scenario
├── cutin_scenario.xosc             # Another scenario
└── cache/
    └── osm/
        └── nihonbashi.xodr         # Downloaded road data
```

### File Format

All scenarios are OpenSCENARIO 1.2 XML format:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader description="Generated scenario"/>
  <RoadNetwork>
    <LogicFile filepath="nihonbashi.xodr"/>
  </RoadNetwork>
  <Entities>...</Entities>
  <Storyboard>...</Storyboard>
</OpenSCENARIO>
```

**Use with**: esmini, CARLA, VTD, IPG, or any OSC-compatible simulator.

---

## Tips & Best Practices

### Road Selection

✅ **Do**:
- Use roads >200m for lane changes
- Check road quality score (aim for 70+)
- Use `suggest_spawn_points` for good positions

❌ **Don't**:
- Place vehicles beyond road length
- Use lane_id = 0 (center line, invalid)
- Ignore position validation errors

---

### Scenario Design

✅ **Do**:
- Validate scenarios before export
- Use realistic speeds (20-30 m/s typical)
- Test in esmini before production use

❌ **Don't**:
- Skip validation step
- Use extreme values (negative speeds, huge masses)
- Assume all roads have multiple lanes

---

### Performance

**First run**: Slow (compiles Rust code)  
**Subsequent runs**: Fast (uses cached build)

**Optimize**:
```bash
# Use release mode for production
cargo run --release

# Cache OSM downloads
# (automatically saved in cache/osm/)
```

---

## Troubleshooting

### "Road not found"

**Cause**: OSM location name not recognized

**Fix**:
- Try different name: "tokyo_station" instead of "tokyo"
- Use nearby landmark
- Check [OpenStreetMap](https://openstreetmap.org) for exact names

---

### "No suitable roads"

**Cause**: Downloaded roads too short or no lanes

**Fix**:
- Download larger area: "shibuya" instead of specific street
- Use `list_roads` to see what's available
- Check quality score (`get_road_info`)

---

### "Position validation failed"

**Cause**: Trying to place entity off-road or in invalid lane

**Fix**:
- Use `validate_position` first
- Use `suggest_spawn_points` to find valid positions
- Check road length with `get_road_info`

---

### "Scenario export failed"

**Cause**: Invalid scenario structure

**Fix**:
- Run `validate_scenario` first
- Check all entities have positions set
- Verify road network is loaded

---

## Advanced Topics

### Custom Integrations

Build your own AI tools that use the MCP server:

```python
# Python example
import subprocess
import json

def call_mcp_tool(tool_name, arguments):
    request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {"name": tool_name, "arguments": arguments}
    }
    
    result = subprocess.run(
        ["cargo", "run", "--release"],
        input=json.dumps(request),
        capture_output=True,
        text=True
    )
    
    return json.loads(result.stdout)

# Use it
result = call_mcp_tool("get_real_world_road", {"location": "tokyo"})
print(result)
```

---

### Batch Processing

Generate multiple scenarios:

```bash
# Create script
for location in tokyo osaka kyoto; do
  echo "Generating scenarios for $location..."
  echo '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"tools/call",
    "params":{
      "name":"get_real_world_road",
      "arguments":{"location":"'$location'"}
    }
  }' | cargo run --release
done
```

---

### CI/CD Integration

Use in automated testing:

```yaml
# GitHub Actions example
- name: Generate test scenarios
  run: |
    cd osc-mcp
    cargo run --example test_scenario_templates
    
- name: Validate scenarios
  run: |
    for file in *.xosc; do
      ~/tools/esmini-demo/bin/esmini --osc $file --headless
    done
```

---

## Next Steps

**You now know**:
- ✅ How to choose an interface
- ✅ Common workflows
- ✅ All available tools
- ✅ API format

**Go deeper**:
- [Claude Desktop guide](CLAUDE_USAGE.md) - Detailed examples
- [Custom XODR guide](CUSTOM_XODR.md) - Use your own roads
- [Contributing](CONTRIBUTING.md) - Add features

---

**Questions?** Open an issue on GitHub or see [CONTRIBUTING.md](CONTRIBUTING.md)
