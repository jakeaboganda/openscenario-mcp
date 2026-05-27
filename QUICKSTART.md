# Quick Start Guide

**📚 Navigation**: [Home](README.md) | [Install](INSTALL.md) | **Quick Start** | [Usage](USAGE.md)

5-minute test to verify everything works.

---

## Prerequisites

✅ Already [installed](INSTALL.md)? Great! Let's test it.

❌ Not installed yet? → Go to [Installation Guide](INSTALL.md) first.

---

## Test 1: Download Real Road (1 minute)

Download actual road network from Tokyo:

```bash
cd /path/to/your/osc-mcp  # Replace with your path
cargo run --example test_get_real_world_road
```

**Expected output**:
```
🌏 Testing Real World Road Download
============================================================

📍 Downloading road for: nihonbashi (Tokyo)
✅ Downloaded OpenStreetMap data (3.2 KB)
✅ Converted to OpenDRIVE format
✅ Saved to: cache/osm/nihonbashi.xodr

📊 Road Network Analysis:
   Total roads: 1150
   Suitable for scenarios: 6 roads (>200m, multi-lane)
   Quality score: 90/100

Best roads for testing:
   1. Road 5271 - 1512m, 3 lanes - 首都高速都心環状線
   2. Road 5402 - 1606m, 3 lanes - 首都高速都心環状線
```

✅ **Success?** Road data downloaded and validated! → Continue to Test 2

❌ **Failed?** Check [Troubleshooting](INSTALL.md#troubleshooting)

---

## Test 2: Generate Scenario (2 minutes)

Create a test scenario using the downloaded road:

```bash
cargo run --example test_scenario_templates
```

**Expected output**:
```
🧪 Testing Scenario Templates
============================================================

✅ Loading road network: cache/osm/nihonbashi.xodr
✅ Found suitable roads for testing

📝 Creating lane change scenario...
   Road: 5402 (1606m, 3 lanes)
   Ego vehicle: lane -1 → lane -2 (5 seconds)
   Other vehicle: positioned ahead

✅ Scenario created: lane_change_nihonbashi.xosc

📝 Creating cut-in scenario...
✅ Scenario created: cutin_nihonbashi.xosc

📝 Creating platoon scenario...
✅ Scenario created: platoon_nihonbashi.xosc
```

✅ **Success?** Scenarios generated! → Continue to Test 3

---

## Test 3: Run MCP Server (2 minutes)

Start the MCP server and verify it responds:

```bash
cd openscenario-mcp
cargo run --release
```

**Expected**: Server starts and waits for MCP messages on stdin.

```
OpenSCENARIO MCP Server v0.1.0
Listening for MCP protocol messages on stdio...
Ready.
```

**Test it**: Send a simple MCP message:
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | cargo run --release
```

**Expected response** (JSON with tool list):
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tools": [
      {"name": "create_scenario", ...},
      {"name": "get_real_world_road", ...},
      ...
    ]
  }
}
```

✅ **Success?** Server responds to MCP! → You're ready!

**Stop the server**: Press `Ctrl+C`

---

## Test 4 (Optional): Visualize with esmini

If you [installed esmini](INSTALL.md#optional-esmini-simulator):

```bash
# View the generated scenario
~/tools/esmini-demo/bin/esmini --osc lane_change_nihonbashi.xosc
```

**Expected**: esmini opens showing the scenario with moving vehicles on the real Tokyo road network.

---

## ✅ All Tests Passed?

**Congratulations!** Everything is working. Now choose how you want to use it:

### Next Steps

**Recommended**: Use with Claude Desktop (most natural)
1. Configure Claude Desktop ([guide](CLAUDE_USAGE.md#setup))
2. Talk naturally: *"Create a lane change scenario"*
3. Get back: production-ready `.xosc` files

**Advanced**: Use the MCP API directly
1. Read the [Usage Guide](USAGE.md)
2. Send MCP messages via stdio
3. Build custom integrations

---

## ❌ Tests Failed?

### Common Issues

**Test 1 failed - network error**:
- Check internet connection
- Try different location: `test_get_real_world_road("tokyo_station")`

**Test 2 failed - no suitable roads**:
- This is OK if using simple test roads
- Try downloading a larger area: `get_real_world_road("shibuya")`

**Test 3 failed - server won't start**:
- Check port not in use: `lsof -i :stdio`
- Try dev build: `cargo run` (without `--release`)

**esmini won't open**:
- Check esmini installed correctly: `~/tools/esmini-demo/bin/esmini --version`
- Verify XOSC file exists: `ls *.xosc`
- Try with esmini's example: `~/tools/esmini-demo/bin/esmini --osc ~/tools/esmini-demo/resources/xosc/cut-in.xosc`

---

## What Just Happened?

You tested the full pipeline:

1. ✅ **Downloaded** real road data (OpenStreetMap → OpenDRIVE)
2. ✅ **Generated** OpenSCENARIO scenarios (lane change, cut-in, platoon)
3. ✅ **Started** MCP server (ready for AI assistants)
4. ✅ **Visualized** scenarios (optional, with esmini)

**Ready for real use!** → Choose your interface: [Claude Desktop](CLAUDE_USAGE.md) | [Direct API](USAGE.md)

---

## More Examples

Want to try more before committing to a full setup?

```bash
# Load custom XODR file
cargo run --example test_custom_xodr

# Test road network validation
./test_osm_conversion.sh

# Test catalog references
./test_catalog_validation.sh
```

---

**Ready to dive in?** → [Usage Guide](USAGE.md) | [Claude Desktop](CLAUDE_USAGE.md)
