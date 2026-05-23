# GitHub Copilot Instructions for OpenSCENARIO MCP

## Project Overview

This project is an MCP (Model Context Protocol) server that provides tools for autonomous vehicle scenario generation using real-world roads from OpenStreetMap.

## Available MCP Tools

When users ask to work with scenarios or roads, use these MCP tool patterns:

### Road Network Tools

**get_real_world_road** - Download real roads from OpenStreetMap
```rust
// Example: Get Tokyo Nihonbashi roads
get_real_world_road("nihonbashi", None)
// Returns: Road analysis + recommended spawn points
```

**list_roads** - Query all roads in loaded network
```rust
list_roads()
// Returns: Vec<RoadInfo> with id, length, lanes
```

**get_road_info** - Get details for specific road
```rust
get_road_info("5402")
// Returns: RoadInfo with full details
```

**suggest_spawn_points** - Get vehicle starting positions
```rust
suggest_spawn_points("5402", 5)
// Returns: 5 spawn points with (road_id, lane_id, s, heading)
```

### Scenario Generation Tools

**create_quick_scenario** - Auto-generate complete scenario
```rust
create_quick_scenario("lane_change", Some(3))
// Automatically uses best road from loaded network
// Returns: scenario_id
```

**create_lane_change_scenario** - Custom lane change
```rust
create_lane_change_scenario(
    "5402",        // road_id
    -1,            // lane_from
    -2,            // lane_to
    100.0,         // ego_start_s
    150.0,         // other_start_s
    -1,            // other_lane
    25.0,          // ego_speed
    20.0,          // other_speed
    Some("test".to_string())
)
```

**create_cutin_scenario** - Safety testing cut-in
```rust
create_cutin_scenario(
    "5402", -1, -2,     // road, ego_lane, other_lane
    100.0, 130.0,       // start positions
    25.0, 23.0,         // speeds
    15.0,               // trigger distance
    None
)
```

**create_platoon_scenario** - Multi-vehicle convoy
```rust
create_platoon_scenario(
    "5402", -1,         // road, lane
    5,                  // vehicle count
    200.0,              // start_s
    40.0,               // spacing
    25.0,               // speed
    None
)
```

**export_xml** - Save scenario to file
```rust
export_xml(scenario_id, "output.xosc")
```

## Typical Workflows

### 1. Quick Scenario on Real Road (3 steps)
```rust
// User: "Create a lane change scenario on Nihonbashi"
let result = get_real_world_road("nihonbashi", None)?;
let scenario = create_quick_scenario("lane_change", None)?;
export_xml(scenario_id, "nihonbashi_lane_change.xosc")?;
```

### 2. Custom Scenario
```rust
// User: "Create a 5-vehicle platoon on Tokyo Station roads"
get_real_world_road("tokyo_station", None)?;
let roads = list_roads()?;
let best_road = roads.iter().max_by_key(|r| r.length)?;
create_platoon_scenario(
    &best_road.id, -1, 5, 100.0, 40.0, 25.0, None
)?;
```

## Code Patterns

### Error Handling
Always use `?` for Result propagation:
```rust
let result = handle_get_real_world_road(state, location, output)?;
```

### State Management
MCP server uses Arc<Mutex<ServerState>>:
```rust
let state_lock = state.lock()
    .map_err(|_| anyhow!("Failed to acquire lock"))?;
```

### Position Validation
Always validate positions before use:
```rust
validator.validate_road_position(&road_id, s)?;
validator.validate_lane_position(&road_id, lane_id)?;
```

### Lane Offsets
Use relative offsets, not absolute positions:
```rust
// Lane change from -1 to -2
let offset = lane_to - lane_from;  // -1
scenario.add_lane_change_action(..., offset, ...)?;
```

## Testing Commands

When users ask to test:
```bash
# Test real world road download
cargo run --example test_get_real_world_road

# Test scenario templates
cargo run --example test_scenario_templates

# Run MCP server
cd openscenario-mcp && cargo run
```

## Pre-configured Locations

Suggest these Tokyo locations:
- `nihonbashi` - Mixed urban, good highway segments
- `tokyo_station` - Central Tokyo
- `ginza` - Shopping district
- `shinjuku` - Commercial area
- `shibuya` - Famous crossing
- `roppongi` - Entertainment district

## Scenario Types

When users ask about scenarios:
- **lane_change** - Overtaking maneuver, 5s smooth transition
- **cutin** - Aggressive cut-in, 2.5s sharp maneuver, safety testing
- **platoon** - Multi-vehicle convoy, ACC testing, 2-10 vehicles

## Common Issues

**"No road network loaded"**
→ Call `get_real_world_road` or `load_road_network` first

**"No suitable roads found"**
→ Area too small or poor quality, try larger bbox

**"netconvert not found"**
→ Install SUMO: `sudo apt install sumo sumo-tools`

## Architecture

```
MCP Server (Rust)
├─ Road Network Intelligence (opendrive_validator.rs)
│  ├─ list_roads()
│  ├─ get_road_info()
│  ├─ suggest_spawn_points()
│  └─ validate_position()
│
├─ Scenario Templates (scenario_templates.rs)
│  ├─ create_quick_scenario()
│  ├─ create_lane_change_scenario()
│  ├─ create_cutin_scenario()
│  └─ create_platoon_scenario()
│
└─ OSM Integration (Python)
   └─ osm_to_opendrive.py
      ├─ Download from Overpass API
      └─ Convert via SUMO netconvert
```

## Code Style

- Use meaningful variable names (not `x`, `y`)
- Always handle Results with `?` or proper error messages
- Document public functions with `///` comments
- Use `anyhow::Result` for error handling
- Prefer `&str` parameters, convert to String inside functions

## Helpful Responses

When users ask vague questions, suggest concrete examples:
- "Try: get_real_world_road('nihonbashi')"
- "The quick_scenario tool can auto-generate this"
- "Here's a complete 3-step workflow..."

---

**Remember**: This is an MCP server - the tools are called via JSON-RPC, but you should explain them in natural Rust patterns for code assistance.
