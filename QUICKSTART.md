# Quick Start Guide: OpenSCENARIO MCP Server

**Last Updated**: 2026-05-23  
**Status**: Phases 1-3 Complete (Core infrastructure working)

---

## What This Does

AI can request real Tokyo roads and generate autonomous vehicle testing scenarios.

**Example**:
```
User: "I need to test lane change on Nihonbashi highway"
AI: [calls get_real_world_road('nihonbashi')]
AI: "I found the Shuto Expressway Inner Circular (1.6km, 2 lanes). 
     I'll create a scenario with your ego vehicle and 2 other cars..."
```

---

## Quick Test

```bash
cd ~/.openclaw/workspace/osc-mcp

# Test 1: Download real Tokyo road
./test_osm_conversion.sh nihonbashi -o test

# Test 2: Find usable roads
cargo run --example find_good_roads cache/osm/test.xodr

# Test 3: Test MCP tool
cargo run --example test_get_real_world_road
```

**Expected**: Shuto Expressway 1.6km road with 5 spawn points

---

## Available MCP Tools

### Road Network
- `get_real_world_road(location, output_name?)` - Download from OSM
- `load_road_network(xodr_path)` - Load local file
- `list_roads()` - Query all roads
- `get_road_info(road_id)` - Details for specific road
- `suggest_spawn_points(road_id, count)` - Vehicle placement
- `validate_position(road_id, lane_id, s)` - Check if valid

### Scenarios (Existing)
- `create_scenario(name, version)`
- `add_vehicle(scenario_id, name, category)`
- `set_position(...)` - Place vehicles
- `add_speed_action(...)` - Set speeds
- `add_lane_change_action(...)` - Maneuvers
- `export_xml(scenario_id, output_path)` - Generate XML

---

## Pre-Configured Locations

| Name | Description | Best For |
|------|-------------|----------|
| `nihonbashi` | Shuto C1 Inner Circular | Highway scenarios |
| `route1_nihonbashi` | Route 1 | Mixed highway |
| `shuto_c1` | Large expressway network | Complex scenarios |
| `tokyo_station` | Tokyo Station area | Urban + highway |
| `ginza` | Ginza district | Urban streets |

Custom: `"139.77,35.68,139.78,35.69"` (lon,lat,lon,lat)

---

## File Locations

**Project Root**: `~/.openclaw/workspace/osc-mcp`

**Key Files**:
- `tools/osm/osm_to_opendrive.py` - OSM download/conversion
- `openscenario/src/opendrive_validator.rs` - Road intelligence
- `openscenario-mcp/src/handlers.rs` - MCP tool handlers
- `openscenario-mcp/src/server.rs` - MCP server

**Generated**:
- `cache/osm/*.xodr` - Downloaded road networks (gitignored)

**Documentation**:
- `memory/2026-05-23.md` - Today's session summary
- `memory/phase1-complete.md` - Road intelligence
- `memory/phase2-complete.md` - OSM pipeline
- `memory/phase3-complete.md` - MCP integration

---

## Typical Workflow

1. **Get real road**: `get_real_world_road(location='nihonbashi')`
   - Returns: Road network, quality score, recommended roads, spawn points

2. **Create scenario**: `create_scenario(name='test', version='1.2')`

3. **Add vehicles**: `add_vehicle(scenario_id='...', name='ego', category='Car')`

4. **Place vehicles**: Use spawn points from step 1
   ```json
   set_position({
     "road_id": "5402",
     "lane_id": -1,
     "s": 267.7
   })
   ```

5. **Add actions**: Lane changes, speed changes, etc.

6. **Export**: `export_xml(scenario_id='...', output_path='scenario.xosc')`

7. **Visualize**: `esmini --osc scenario.xosc --odr cache/osm/nihonbashi.xodr`

---

## What's Complete

- ✅ Road intelligence (AI can query roads)
- ✅ OSM integration (real Tokyo roads)
- ✅ MCP tool (AI can request roads)
- ✅ Quality scoring (90/100 typical)
- ✅ Spawn points (automatic placement)
- ✅ Scenario generation (existing tools)

---

## What's Next (Optional)

**Phase 4** (1-2h): Scenario helpers
- Lane change templates
- Merge scenarios
- Cut-in maneuvers
- Batch generation

**Phase 5** (1h): Testing & docs
- Claude Desktop integration
- End-to-end testing
- User guide
- Example workflows

---

## Common Issues

**"netconvert not found"**:
- Install SUMO: `bash tools/install_sumo.sh`

**"No good roads found"**:
- Some areas have only short streets
- Try `nihonbashi` or `shuto_c1` for highways

**"Quality score low"**:
- OSM data varies by location
- >80 is good, >60 is usable

---

## Need Help?

Check documentation in `memory/`:
- `2026-05-23.md` - Today's full session
- `phase*-complete.md` - Detailed phase docs
- `tokyo-av-testing-implementation-plan.md` - Original plan

Or run examples:
```bash
cargo run --example find_good_roads cache/osm/nihonbashi.xodr
cargo run --example test_get_real_world_road
```

---

**Status**: Ready to use! 🚀  
**Next session**: Continue with Phase 4/5 or start using it!
