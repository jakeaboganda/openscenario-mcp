# Using Custom XODR Files

**Load your own OpenDRIVE road networks**

---

## Overview

In addition to downloading real-world roads from OpenStreetMap, you can load your own custom OpenDRIVE (`.xodr`) files. This is useful for:

- **Custom test tracks** - Your own designed road networks
- **Simulation environments** - Roads from other simulators (CARLA, LGSVL, etc.)
- **Proprietary tracks** - Internal company test tracks
- **Generated roads** - Procedurally generated or hand-crafted networks

---

## Quick Start

### With Claude Desktop

Just tell Claude you have a custom XODR file:

```
You: "Load my custom road network from /home/user/my_track.xodr"

Claude: [calls load_road_network]
        "Loaded successfully! Found 15 roads with quality score 95/100.
         Would you like me to create a scenario on it?"

You: "Yes, create a lane change scenario"

Claude: [calls create_quick_scenario]
        "Done! Created lane change scenario on Road 1 (800m, 3 lanes)."
```

### With VS Code / Code

```rust
use openscenario_mcp::handlers::handle_load_road_network;
use openscenario_mcp::scenario_templates::handle_create_quick_scenario;

// Load custom XODR
let result = handle_load_road_network(
    state.clone(),
    "/path/to/your/track.xodr".to_string(),
)?;

// Create scenario
let scenario = handle_create_quick_scenario(
    state.clone(),
    "lane_change".to_string(),
    None,
)?;
```

---

## MCP Tool: `load_road_network`

**Description**: Load and analyze an OpenDRIVE road network file

**Parameters**:
- `xodr_path` (string, required) - Path to `.xodr` file (absolute or relative)

**Returns**:
```json
{
  "status": "success",
  "file": "/path/to/track.xodr",
  "road_count": 15,
  "roads": [
    {
      "id": "1",
      "length": 850.5,
      "lane_count": 3,
      "name": "Main Straight"
    },
    ...
  ],
  "quality": {
    "score": 95,
    "has_lanes": true,
    "has_geometry": true,
    "issues": []
  }
}
```

---

## Workflow

### 1. Prepare Your XODR File

**Requirements**:
- Valid OpenDRIVE format (1.4, 1.5, or 1.6)
- Contains road geometry
- Has lane sections defined
- Roads >50m for scenario generation

**Optional** (for better results):
- Lane markings
- Road names
- Multiple lanes (for lane change scenarios)
- Realistic dimensions

### 2. Load the Road Network

**Claude Desktop**:
```
"Load the XODR file at /path/to/track.xodr"
```

**Direct MCP call**:
```json
{
  "tool": "load_road_network",
  "arguments": {
    "xodr_path": "/path/to/track.xodr"
  }
}
```

### 3. Inspect Available Roads

**Claude Desktop**:
```
"What roads are available?"
"Show me the longest road"
"Which road has the most lanes?"
```

**Direct MCP call**:
```json
{
  "tool": "list_roads"
}
```

### 4. Create Scenarios

**Claude Desktop**:
```
"Create a lane change scenario"
"Generate a cut-in test on Road 3"
"Make a 5-vehicle platoon"
```

**Direct MCP call**:
```json
{
  "tool": "create_quick_scenario",
  "arguments": {
    "scenario_type": "lane_change"
  }
}
```

---

## Examples

### Example 1: Test Track

You have a custom test track `test_track.xodr`:

```
You: "Load test_track.xodr and create a lane change scenario"

Claude: [loads XODR, finds 5 roads, selects best one]
        "Loaded test track with 5 roads. Created lane change on 
         the main straight (1.2km, 3 lanes). Scenario saved."
```

### Example 2: CARLA Town

You exported a CARLA town as XODR:

```
You: "Load CARLA_Town03.xodr"

Claude: "Loaded CARLA Town03 - found 124 roads, quality 88/100.
         82 roads are suitable for testing (>50m with lanes)."

You: "Create a cutin scenario on the highway"

Claude: "Found the highway (Road 45, 2.5km, 4 lanes).
         Created aggressive cut-in scenario. Ready to test!"
```

### Example 3: Custom Circuit

You designed a race circuit:

```
You: "I have a race circuit at ~/circuits/spa.xodr. 
      Create a platoon of 10 vehicles following each other."

Claude: [loads circuit, finds 1 main loop]
        "Loaded your Spa circuit (7.0km loop, 2 lanes).
         Created 10-vehicle platoon spaced 40m apart,
         traveling at 200 km/h. Ready for convoy testing!"
```

---

## Path Resolution

The `xodr_path` parameter supports:

**Absolute paths**:
```
/home/user/tracks/track.xodr
/var/data/roads/highway.xodr
C:\Users\Jake\roads\test.xodr  (Windows)
```

**Relative paths** (from MCP server working directory):
```
roads/test_track.xodr
../shared/common_tracks.xodr
./data/my_road.xodr
```

**User home expansion**:
```
~/my_tracks/circuit.xodr
~jake/simulation/roads/highway.xodr
```

---

## Quality Scoring

The system analyzes your XODR file and provides a quality score (0-100):

| Score | Quality | Notes |
|-------|---------|-------|
| 90-100 | Excellent | All features present, ready for complex scenarios |
| 70-89 | Good | Suitable for most scenarios |
| 50-69 | Fair | Basic scenarios only |
| <50 | Poor | May have issues, limited scenario support |

**Score factors**:
- ✅ Has lane definitions (+30 points)
- ✅ Has road geometry (+30 points)
- ✅ Has multiple roads (+10 points)
- ✅ Roads >50m long (+10 points per road, max +20)
- ✅ Multi-lane roads (+10 points)

---

## Common Issues

### "Failed to load OpenDRIVE file"

**Causes**:
- File doesn't exist at specified path
- File is not valid OpenDRIVE XML
- File is corrupted or incomplete

**Solutions**:
- Verify file path is correct
- Check file exists: `ls /path/to/file.xodr`
- Validate with OpenDRIVE validator
- Try opening in RoadRunner or other tools

### "No suitable roads found"

**Causes**:
- Roads are too short (<50m)
- No lane sections defined
- Roads missing geometry

**Solutions**:
- Ensure roads have lane sections
- Roads should be >200m for lane changes
- Check OpenDRIVE has `<road>` elements with `<lanes>` and `<planView>`

### "Position validation failed"

**Causes**:
- Scenario tries to place vehicles beyond road length
- Lane IDs don't exist in XODR

**Solutions**:
- Check road lengths: use `list_roads` tool
- Verify lane IDs: inspect XODR file
- Use `validate_position` tool to check coordinates

---

## Tips

### 1. Check Quality First
```
"Load my_track.xodr and show me the quality report"
```

### 2. Inspect Roads Before Creating Scenarios
```
"List all roads with their lengths and lane counts"
```

### 3. Use Road Names (if available)
```
"Create a scenario on the 'Highway Section' road"
```

### 4. Test Position Validity
```
"Is position 500m on Road 1, lane -1 valid?"
```

### 5. Generate Multiple Scenarios
```
"Create 5 different scenarios on this track"
```

---

## Advanced: Multiple XODR Files

You can switch between different road networks:

```
You: "Load track_1.xodr"
Claude: [loads track 1]

You: "Create a scenario"
Claude: [creates scenario on track 1]

You: "Now load track_2.xodr"
Claude: [loads track 2, replaces track 1]

You: "Create another scenario"
Claude: [creates scenario on track 2]
```

**Note**: Only one road network is active at a time. Loading a new XODR replaces the current one.

---

## Integration with OSM Roads

You can mix custom XODR files with downloaded OpenStreetMap roads:

```
# Use OSM for one scenario
"Get roads for Nihonbashi"
"Create a lane change scenario"

# Switch to custom XODR for another
"Load my_test_track.xodr"
"Create a platoon scenario"

# Back to OSM
"Get roads for Shibuya"
"Create a cut-in scenario"
```

---

## Testing

Test the custom XODR loading:

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo run --example test_custom_xodr
```

**Expected output**:
- ✅ XODR file loaded
- ✅ Roads analyzed
- ✅ Quality score computed
- ✅ Scenario created
- ✅ Ready for export

---

## Example XODR Files

The project includes example XODR files for testing:

| File | Description | Roads | Length |
|------|-------------|-------|--------|
| `roads/simple_highway.xodr` | Basic 2-lane highway | 1 | 1000m |
| `cache/osm/nihonbashi.xodr` | Real Tokyo roads | ~1150 | Varies |

Generate more with:
- RoadRunner (MathWorks)
- OpenDRIVE Designer
- CARLA export
- SUMO netconvert (from OSM)

---

## Summary

**Custom XODR workflow**:
1. Prepare your `.xodr` file
2. Load with `load_road_network` tool
3. Inspect roads with `list_roads`
4. Create scenarios with `create_quick_scenario`
5. Export and test!

**Simple conversation**:
```
"Load my_track.xodr and create a lane change scenario"
```

That's it! The MCP server handles everything else automatically. 🚗🛣️

---

**Next**: See [CLAUDE_USAGE.md](CLAUDE_USAGE.md) for more conversation examples!
