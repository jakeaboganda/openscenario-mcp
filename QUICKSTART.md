# Quick Start Guide: OpenSCENARIO MCP Server

**Real-world roads meet autonomous vehicle testing scenarios**

---

## What This Does

The OpenSCENARIO MCP Server lets you:
- Download real roads from OpenStreetMap (any location worldwide)
- Convert them to OpenDRIVE format
- Generate autonomous vehicle testing scenarios automatically
- Export to OpenSCENARIO XML for simulation

**Example**: "I need to test lane change on Tokyo's Shuto Expressway"
→ Get real road, generate scenario, export XML. Done in 3 MCP calls.

---

## Quick Test

```bash
cd /path/to/your/osc-mcp  # Replace with where you cloned the repo

# Test 1: Download real Tokyo road
cargo run --example test_get_real_world_road

# Test 2: Run MCP server
cd openscenario-mcp && cargo run
```

**Expected**: Road network loaded, MCP server starts on stdio

---

## Prerequisites

### Required
- Rust 1.70+
- Python 3.8+
- SUMO 1.21.0+ (for netconvert)

### Install SUMO

**Ubuntu/Debian**:
```bash
sudo add-apt-repository ppa:sumo/stable
sudo apt update
sudo apt install sumo sumo-tools
```

**macOS**:
```bash
brew install sumo
```

**Fedora**:
```bash
sudo dnf install sumo sumo-tools
```

Verify: `netconvert --version`

---

## Installation

```bash
# Navigate to project directory
cd /path/to/your/osc-mcp  # Replace with your actual path

# Build
cargo build --release

# Run server
cd openscenario-mcp
cargo run --release
```

---

## MCP Tools Available

### Road Network Tools

**`get_real_world_road`** ⭐  
Download and convert real roads from OpenStreetMap.

```json
{
  "location": "nihonbashi",
  "output_name": "tokyo_test"
}
```

Returns road analysis with recommended spawn points.

**`list_roads`**  
Query all roads in loaded network.

**`get_road_info`**  
Get detailed info for a specific road.

**`suggest_spawn_points`**  
Get recommended vehicle starting positions.

**`validate_position`**  
Check if a position is valid on the road network.

### Scenario Generation Tools

**`create_quick_scenario`** ⭐  
Auto-generate complete scenario on best available road.

```json
{
  "scenario_type": "lane_change",
  "vehicle_count": 3
}
```

Supported types: `lane_change`, `cutin`, `platoon`

**`create_lane_change_scenario`**  
Generate lane change scenario with custom parameters.

**`create_cutin_scenario`**  
Generate cut-in safety testing scenario.

**`create_platoon_scenario`**  
Generate multi-vehicle convoy scenario.

**`export_xml`**  
Export scenario to OpenSCENARIO XML file.

---

## Typical Workflow

### 1. Get Real Road
```json
get_real_world_road({
  "location": "nihonbashi"
})
```

### 2. Generate Scenario
```json
create_quick_scenario({
  "scenario_type": "lane_change"
})
```

### 3. Export
```json
export_xml({
  "scenario_id": "<from-step-2>",
  "output_path": "scenario.xosc"
})
```

**Done!** Scenario ready for visualization in esmini or other OpenSCENARIO tools.

---

## Pre-configured Locations

The tool includes several Tokyo locations with pre-calculated bounding boxes:

- `nihonbashi` - Nihonbashi district (mixed urban)
- `tokyo_station` - Tokyo Station area
- `ginza` - Ginza shopping district
- `shinjuku` - Shinjuku commercial area
- `shibuya` - Shibuya crossing area
- `roppongi` - Roppongi district

Or provide custom bounding box: `"135.0,35.0,135.1,35.1"` (lon1,lat1,lon2,lat2)

---

## Testing & Validation

### Test Real Road Download
```bash
cargo run --example test_get_real_world_road
```

**Output**: Road network analysis for Nihonbashi area

### Test Road Intelligence
```bash
cargo run --example find_good_roads cache/osm/nihonbashi.xodr
```

**Output**: List of usable roads with quality metrics

### Manual OSM Conversion
```bash
./test_osm_conversion.sh nihonbashi -o test_roads
```

**Output**: OpenDRIVE file in `cache/osm/test_roads.xodr`

---

## Scenario Types

### Lane Change
- Ego vehicle changes lanes
- One other vehicle for interaction
- 5-second smooth transition
- Realistic highway speeds (90 km/h)

### Cut-In
- Other vehicle cuts in front aggressively
- 2.5-second sharp maneuver
- Tests emergency braking
- Safety-critical scenario

### Platoon
- 2-10 vehicles in convoy
- 40m spacing between vehicles
- All vehicles same speed
- ACC/convoy testing

---

## Configuration

### Cache Directory
Downloaded roads cached in `cache/osm/`

### Python Dependencies
```bash
pip install requests
```

### SUMO Path
If netconvert not in PATH:
```bash
export SUMO_HOME=/usr/share/sumo
export PATH=$PATH:$SUMO_HOME/bin
```

---

## Visualization

Use esmini to visualize generated scenarios:

```bash
# Install esmini (https://github.com/esmini/esmini)

# Run scenario
esmini --window 60 60 800 400 \
       --osc scenario.xosc \
       --road cache/osm/nihonbashi.xodr
```

---

## Troubleshooting

### "netconvert not found"
Install SUMO or add to PATH (see Prerequisites)

### "No suitable roads found"
Road network may be too small or have poor quality. Try:
- Larger area
- Different location
- Check `cache/osm/*.xodr` file size (should be >100KB)

### "Position validation failed"
Road network not loaded. Call `get_real_world_road` or `load_road_network` first.

### Python script fails
Check:
```bash
python3 tools/osm/osm_to_opendrive.py --help
which netconvert
```

---

## Architecture

```
User/AI
   ↓
MCP Server (Rust)
   ↓
┌──────────────┬──────────────────┐
│ Road Network │ Scenario Builder │
├──────────────┼──────────────────┤
│ • List roads │ • Templates      │
│ • Query info │ • Vehicles       │
│ • Validate   │ • Actions        │
│ • Spawn pts  │ • Export         │
└──────────────┴──────────────────┘
         ↓              ↓
   OpenDRIVE    OpenSCENARIO XML
```

**Data flow**:
1. Python downloads OSM data
2. SUMO converts to OpenDRIVE
3. Rust validates and indexes
4. MCP tools query/generate
5. Export to OpenSCENARIO

---

## Examples

### Example 1: Quick Lane Change
```bash
# Via MCP:
get_real_world_road({"location": "nihonbashi"})
create_quick_scenario({"scenario_type": "lane_change"})
export_xml({"scenario_id": "...", "output_path": "test.xosc"})

# Visualize:
esmini --osc test.xosc --road cache/osm/nihonbashi.xodr
```

### Example 2: Custom Cut-In
```bash
create_cutin_scenario({
  "road_id": "5402",
  "ego_lane": -1,
  "other_lane": -2,
  "ego_start_s": 100.0,
  "other_start_s": 130.0,
  "ego_speed": 25.0,
  "other_speed": 23.0,
  "cutin_trigger_distance": 15.0
})
```

### Example 3: Platoon Convoy
```bash
create_platoon_scenario({
  "road_id": "5402",
  "lane_id": -1,
  "vehicle_count": 5,
  "start_s": 200.0,
  "spacing": 40.0,
  "speed": 25.0
})
```

---

## Development

### Run Tests
```bash
cargo test
```

### Build Docs
```bash
cargo doc --open
```

### Format Code
```bash
cargo fmt
cargo clippy
```

---

## References

- [OpenSCENARIO Specification](https://www.asam.net/standards/detail/openscenario/)
- [OpenDRIVE Specification](https://www.asam.net/standards/detail/opendrive/)
- [SUMO Documentation](https://sumo.dlr.de/docs/)
- [esmini](https://github.com/esmini/esmini)
- [Model Predictive Control Protocol](https://modelcontextprotocol.io/)

---

## License

See LICENSE file for details.

---

## Support

For issues or questions:
- Check troubleshooting section above
- Review example code in `openscenario-mcp/examples/`
- Consult OpenSCENARIO/OpenDRIVE specifications

---

**Ready to generate scenarios on real roads!** 🚀
