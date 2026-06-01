# Using OpenSCENARIO MCP with Claude Desktop

**Recommended method for interactive scenario generation**

---

## Why Claude Desktop?

Claude Desktop is purpose-built for MCP servers:
- ✅ Native MCP protocol support
- ✅ Natural conversation interface
- ✅ No code needed - just talk to Claude
- ✅ Real-time tool execution
- ✅ Visual feedback

---

## Setup

### 1. Install Claude Desktop

**Download**: https://claude.ai/download

**Ubuntu/Debian** (if available via snap):
```bash
sudo snap install claude-desktop
```

**Or download the .deb/.AppImage** from the website.

### 2. Configure MCP Server

Create or edit the configuration file:

```bash
mkdir -p ~/.config/Claude
nano ~/.config/Claude/claude_desktop_config.json
```

Add this configuration:

**Recommended: Use pre-built binary** (fastest):
```json
{
  "mcpServers": {
    "openscenario": {
      "command": "/absolute/path/to/osc-mcp/target/release/openscenario-mcp",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

**Alternative: Build on-demand** (slower startup):
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
      ],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

**Alternative: Global install** (works from anywhere):
```bash
# First install:
cd osc-mcp/openscenario-mcp
cargo install --path .

# Then configure:
{
  "mcpServers": {
    "openscenario": {
      "command": "openscenario-mcp",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

> **Note**: Replace `/absolute/path/to` with where you cloned the repo (e.g., `/home/username/projects/osc-mcp`).
> **Tip**: Get your path: `cd osc-mcp && pwd`
```

**Save and close** (Ctrl+X, Y, Enter in nano)

### 3. Restart Claude Desktop

Close and reopen Claude Desktop. The MCP server will auto-load.

---

## Usage

### Natural Conversation

Just talk to Claude naturally! The MCP server enables Claude to:
- Download real Tokyo roads
- Generate test scenarios
- Export OpenSCENARIO files

### Example Conversations

#### 🚗 **Example 1: Quick Lane Change**

**You**: "I need a lane change scenario on Tokyo's Nihonbashi highway"

**Claude** (automatically):
1. Calls `get_real_world_road('nihonbashi')`
2. Calls `create_quick_scenario('lane_change')`
3. Calls `export_xml(...)`

**Claude responds**: "Done! I've created a lane change scenario on the Shuto Expressway Inner Circular (1.6km, 2 lanes). The ego vehicle starts at 267m in lane -1 and smoothly changes to lane -2 over 5 seconds at 90 km/h. Another vehicle is positioned at 357m for interaction. Saved as `lane_change_nihonbashi.xosc`."

---

#### ⚠️ **Example 2: Safety Testing**

**You**: "Create a dangerous cut-in scenario for testing emergency braking"

**Claude**:
1. Calls `get_real_world_road('nihonbashi')`
2. Calls `create_quick_scenario('cutin', vehicle_count=2)`
3. Exports the result

**Claude responds**: "Created an aggressive cut-in scenario. The other vehicle starts in the adjacent lane 30m ahead traveling at 23 m/s, then cuts sharply in front of the ego vehicle (25 m/s) when they're 15m apart. The 2.5-second cubic transition creates a realistic emergency braking situation. Scenario saved."

---

#### 🚚 **Example 3: Convoy Testing**

**You**: "I want to test ACC with a 5-vehicle platoon on a Tokyo highway"

**Claude**:
1. Finds best highway via `get_real_world_road`
2. Calls `create_quick_scenario('platoon', vehicle_count=5)`
3. Exports result

**Claude responds**: "Generated a 5-vehicle platoon scenario on the Shuto Expressway. Vehicles are spaced 40m apart traveling at 90 km/h (25 m/s). This tests Adaptive Cruise Control and convoy stability. Ready for simulation."

---

#### 🗺️ **Example 4: Exploring Roads**

**You**: "What roads are available in Shibuya? Which is best for testing?"

**Claude**:
1. Calls `get_real_world_road('shibuya')`
2. Calls `list_roads()`
3. Analyzes results

**Claude responds**: "I found 234 roads in Shibuya. The best for testing is Route 246 (Aoyama-dori) - 890m long, 3 lanes, quality score 85/100. It has consistent lane structure and good spawn points at 100m, 250m, 400m, 650m, and 800m."

---

#### 🔧 **Example 5: Custom Scenario**

**You**: "Create a lane change scenario where the ego vehicle starts at 200m on road 5402, lane -2, traveling at 30 m/s, and changes to lane -3 over 6 seconds. Put another vehicle at 300m in lane -3 going 25 m/s."

**Claude**:
1. Validates road exists
2. Calls `create_lane_change_scenario(...)` with exact parameters
3. Exports result

**Claude responds**: "Custom scenario created with your exact specifications. The ego vehicle will change lanes over 6 seconds while approaching the slower vehicle ahead. This tests overtaking decision-making. Scenario saved as `custom_lane_change.xosc`."

---

#### 🗂️ **Example 6: Custom XODR File** ⭐ New!

**You**: "I have my own test track at /home/user/my_track.xodr. Load it and create a lane change scenario."

**Claude**:
1. Calls `load_road_network('/home/user/my_track.xodr')`
2. Analyzes the road network
3. Calls `create_quick_scenario('lane_change')`
4. Exports result

**Claude responds**: "Loaded your custom track! Found 8 roads with quality score 92/100. The main straight is 1.5km with 3 lanes - perfect for testing. Created a lane change scenario where the ego vehicle smoothly changes from lane -1 to lane -2 over 5 seconds at 90 km/h. Another vehicle is positioned ahead for realistic interaction. Scenario saved as `lane_change_custom_track.xosc`."

**You**: "Great! Now create a platoon of 5 vehicles on the same track."

**Claude**:
1. Calls `create_quick_scenario('platoon', vehicle_count=5)`
2. Uses the already-loaded custom track

**Claude responds**: "Created a 5-vehicle platoon on your track's main straight. Vehicles are spaced 40m apart, all traveling at 90 km/h. Perfect for testing ACC and convoy behavior. Ready to export!"

---

## Available Requests

You can ask Claude to:

### Road Discovery
- "What roads are available in [location]?"
- "Find the longest highway in Ginza"
- "Show me spawn points for road 5402"
- "Is position 250m on road 5402, lane -1 valid?"

### Quick Scenarios
- "Create a lane change scenario on [location]"
- "Generate a cut-in test for safety testing"
- "Make a 7-vehicle platoon scenario"
- "Test overtaking on Roppongi roads"

### Custom Scenarios
- "Create a scenario where..." (describe in detail)
- "I need ego at [position] and other vehicle at [position]"
- "Generate a scenario with specific speeds and lanes"

### Custom XODR Files ⭐ New!
- "Load my custom road network from /path/to/track.xodr"
- "I have a test track at ~/my_track.xodr, create scenarios on it"
- "Use my CARLA Town03 XODR file"
- "Load the circuit from /data/spa.xodr and create a platoon"

See **[CUSTOM_XODR.md](CUSTOM_XODR.md)** for complete guide.

### Batch Generation
- "Create 5 different lane change scenarios"
- "Generate test scenarios for all Tokyo locations"
- "Make variations of the same scenario"

### Information
- "Explain the difference between lane_change and cutin"
- "What's in the Nihonbashi road network?"
- "Show me scenario parameters"

---

## Tokyo Locations

Pre-configured locations you can request:

| Location | Description | Best For |
|----------|-------------|----------|
| `nihonbashi` | Mixed urban, expressway segments | Highway testing |
| `tokyo_station` | Central Tokyo, dense road network | Urban scenarios |
| `ginza` | Shopping district, wide roads | Multi-lane testing |
| `shinjuku` | Commercial area | Complex intersections |
| `shibuya` | Famous crossing area | Pedestrian interaction |
| `roppongi` | Entertainment district | Mixed traffic |

---

## Scenario Types

### Lane Change
- **Use case**: Overtaking, highway merging
- **Duration**: 5 seconds (smooth transition)
- **Speed**: 90 km/h typical
- **Test**: Lane keeping, trajectory planning

### Cut-In
- **Use case**: Safety testing, emergency response
- **Duration**: 2.5 seconds (aggressive)
- **Trigger**: When vehicles close (15m)
- **Test**: Emergency braking, collision avoidance

### Platoon
- **Use case**: ACC, convoy following
- **Vehicles**: 2-10 in formation
- **Spacing**: 40m default
- **Test**: Following distance, speed matching

---

## Troubleshooting

### "MCP server not found"
- Check `~/.config/Claude/claude_desktop_config.json` exists
- Verify path to Cargo.toml is correct
- Restart Claude Desktop

### "No road network loaded"
Claude needs to download roads first:
```
"Get the Nihonbashi roads first"
```

### "SUMO netconvert not found"
Install SUMO:
```bash
sudo apt install sumo sumo-tools
```

### Slow initial requests
First road download takes ~20 seconds. Subsequent requests use cached data.

---

## Advanced Usage

### Custom Locations

**You**: "Download roads from this area: 139.7671,35.6812,139.7771,35.6912"

**Claude**: Will use your custom bounding box (lon1,lat1,lon2,lat2)

### Scenario Chains

**You**: "Create 3 scenarios: first a lane change, then a cut-in, then a platoon. All on the same road."

**Claude**: Will generate all three with proper sequencing

### Export Multiple Formats

**You**: "Export this scenario as both .xosc and show me the JSON structure"

**Claude**: Can export files and explain structure

---

## Tips for Best Results

### 1. Be Specific When Needed
❌ "Make a scenario"  
✅ "Create a lane change scenario on Nihonbashi"

### 2. Let Claude Choose Defaults
✅ "Create a lane change scenario on Shibuya"  
(Claude will pick best road, realistic speeds, good spawn points)

### 3. Ask for Explanations
✅ "What parameters did you use for that scenario?"  
✅ "Why did you choose that road?"

### 4. Request Variations
✅ "Create 5 variations with different speeds"  
✅ "Make the same scenario but more aggressive"

### 5. Iterate
✅ "That's good, but make the ego vehicle faster"  
✅ "Move the other vehicle 50m further"

---

## Workflow Examples

### Workflow 1: Quick Testing
```
You: "I need to test lane change on Tokyo roads"
Claude: [downloads Nihonbashi] "Which specific area?"
You: "Use the best one"
Claude: [generates scenario] "Done! Shuto Expressway, saved as test.xosc"
```

### Workflow 2: Precise Control
```
You: "List roads in Ginza"
Claude: [shows 10 roads with details]
You: "Use road 3421, create lane change from -1 to -2"
Claude: [creates specific scenario] "Done with your parameters"
```

### Workflow 3: Batch Generation
```
You: "Create 10 different scenarios across Tokyo for regression testing"
Claude: [generates scenarios for different locations]
        "Created 10 scenarios: 3 lane_change, 4 cutin, 3 platoon"
```

---

## Visualization

After Claude generates a scenario:

```bash
# Visualize in esmini
esmini --window 60 60 800 400 \
       --osc lane_change_nihonbashi.xosc \
       --road cache/osm/nihonbashi.xodr
```

Or ask Claude:
```
"Show me the esmini command to visualize this"
```

---

## Why Claude Desktop?

**Natural Language Interface**:
- ✅ Just talk naturally - no code required
- ✅ Full conversation with back-and-forth
- ✅ Claude automatically chains multiple tools
- ✅ Error handling built-in
- ✅ Explanations provided automatically

**Perfect for**:
- Creating test scenarios quickly
- Exploring different configurations
- Non-developers who need scenarios
- Rapid prototyping and testing

**Alternative**: For programmatic access or custom integrations, see [USAGE.md](USAGE.md)

---

## Example Session Transcript

```
You: Hi! I need to test lane change scenarios on Tokyo highways.

Claude: I can help with that! Let me download some Tokyo highway data. 
        Which area would you like? I have pre-configured locations like 
        Nihonbashi, Ginza, or Shibuya, or I can use a custom area.

You: Let's use Nihonbashi.

Claude: [calls get_real_world_road('nihonbashi')]
        Great! I found the Nihonbashi area with 1150 roads. The best for 
        testing is the Shuto Expressway Inner Circular - it's 1.6km long 
        with 2 lanes and high quality. Would you like me to create a lane 
        change scenario on it?

You: Yes, make it realistic.

Claude: [calls create_quick_scenario('lane_change')]
        Done! I've created a lane change scenario where:
        - Ego vehicle starts at 267m in lane -1, traveling 90 km/h
        - Changes to lane -2 over 5 seconds
        - Another vehicle at 357m in lane -1 traveling 72 km/h
        
        The scenario is saved as lane_change_nihonbashi.xosc
        Would you like me to create more scenarios, or modify this one?

You: Can you make a more aggressive version?

Claude: [calls create_quick_scenario('cutin')]
        Created an aggressive cut-in version! Now the other vehicle 
        starts ahead in lane -2 and cuts into lane -1 sharply (2.5s) 
        when you're 15m apart. This tests emergency braking.
        Saved as cutin_nihonbashi.xosc

You: Perfect! How do I visualize these?

Claude: You can use esmini:
        
        esmini --osc lane_change_nihonbashi.xosc \
               --road cache/osm/nihonbashi.xodr
        
        Want me to create more test scenarios?
```

---

## Summary

**Claude Desktop** is the best way to use this MCP server because:
- ✅ Natural conversation
- ✅ No coding required
- ✅ Automatic tool chaining
- ✅ Interactive refinement
- ✅ Built for MCP

**Just talk to Claude like a colleague!** 🚀

---

## Next Steps

1. Install Claude Desktop
2. Configure MCP server (copy the JSON above)
3. Restart Claude Desktop
4. Say: "Create a lane change scenario on Nihonbashi"
5. Watch the magic happen! ✨

**Ready to start?** Open Claude Desktop and say hi! 😊
