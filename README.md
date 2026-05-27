# OpenSCENARIO MCP Server

**Generate OpenSCENARIO test scenarios using AI assistants (Claude, Copilot)**

Talk naturally to AI: *"Create a lane change scenario on Tokyo's Nihonbashi highway"*  
Get back: Production-ready `.xosc` file with real road data from OpenStreetMap.

---

## 🚀 Quick Start

1. **[Install](INSTALL.md)** (5 minutes)
2. **Choose your interface**:
   - **[Claude Desktop](CLAUDE_USAGE.md)** (recommended - most natural)
   - **[Direct API](USAGE.md)** (advanced users, custom integrations)
3. **[Test it works](QUICKSTART.md)** (5 minutes)

**First time here?** → Start with [Installation Guide](INSTALL.md)

---

## ✨ What You Can Do

**Talk to AI, get scenarios**:
```
"Create a lane change scenario"
"Add a cut-in for emergency braking tests"
"Generate a 5-vehicle platoon"
"Place a pedestrian at 50m on lane -2"
```

**Real roads from anywhere**:
- Download actual road networks from OpenStreetMap
- Use your own custom `.xodr` files ([guide](CUSTOM_XODR.md))
- Automatic quality validation

**Production ready**:
- OpenSCENARIO 1.2 format
- Works with esmini, CARLA, VTD, any OSC simulator
- Validates scenarios before export

---

## 🎯 Features

✅ **18 MCP tools** for scenario generation  
✅ **Real-world roads** from OpenStreetMap (Japan, US, Europe - anywhere!)  
✅ **Custom XODR support** - bring your own road networks  
✅ **Natural language** - talk to Claude/Copilot in plain English  
✅ **Scenario templates** - lane change, cut-in, platoon, merge  
✅ **Position validation** - catches errors before simulation  
✅ **Multi-entity** - vehicles, pedestrians, obstacles  

---

## 📚 Documentation

**Getting Started**:
- [Installation Guide](INSTALL.md) - Prerequisites, build, setup
- [Quick Start Test](QUICKSTART.md) - 5-minute proof it works

**Usage Guides**:
- [Claude Desktop Usage](CLAUDE_USAGE.md) - Recommended interface (detailed examples)
- [General Usage Guide](USAGE.md) - Direct API, tool reference, custom integrations

**Advanced Topics**:
- [Custom XODR Files](CUSTOM_XODR.md) - Use your own road networks
- [Contributing Guide](CONTRIBUTING.md) - Development setup

---

## 🤖 What is MCP?

**Model Context Protocol** - lets AI assistants use tools safely.

Think of it like giving Claude or Copilot a "toolbox":
- Claude can call `get_real_world_road("tokyo")` 
- Get back actual road data
- Generate scenarios on real roads
- All through natural conversation

**You**: "Create a scenario on Shibuya"  
**Claude** (behind the scenes):
1. Calls `get_real_world_road('shibuya')`
2. Calls `create_quick_scenario('lane_change')`
3. Calls `export_xml(...)`

**You** get: `lane_change_shibuya.xosc` file ready to use

[Learn more about MCP →](https://modelcontextprotocol.io)

---

## 🛠️ Available Tools

The MCP server provides 18 tools for scenario generation:

| Category | Tools |
|----------|-------|
| **Roads** | `get_real_world_road`, `load_road_network`, `list_roads`, `get_road_info` |
| **Scenarios** | `create_scenario`, `create_quick_scenario`, `validate_scenario`, `export_xml` |
| **Entities** | `add_vehicle`, `add_pedestrian`, `add_misc_object` |
| **Positioning** | `set_position`, `set_lane_position`, `validate_position`, `suggest_spawn_points` |
| **Actions** | `add_speed_action`, `add_lane_change_action` |
| **Triggers** | `set_stop_time`, `set_stop_on_element` |

See [USAGE.md](USAGE.md) for complete tool documentation.

---

## 💡 Example Use Cases

**Automotive Testing**:
- ADAS validation scenarios
- Emergency braking tests
- Lane keeping system tests
- ACC/convoy following tests

**Simulation**:
- CARLA scenario generation
- SUMO traffic scenarios
- Custom simulator setups

**Research**:
- Autonomous vehicle research
- Traffic simulation studies
- Road network analysis

---

## 🧪 Example Output

**You**: "Create a lane change scenario on Tokyo's Nihonbashi highway"

**Claude generates**:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader description="Lane change on 首都高速都心環状線"/>
  <RoadNetwork>
    <LogicFile filepath="nihonbashi.xodr"/>
  </RoadNetwork>
  <Entities>
    <ScenarioObject name="ego">
      <Vehicle vehicleCategory="car" mass="1500" .../>
    </ScenarioObject>
    ...
  </Entities>
  <Storyboard>
    <!-- Full scenario with lane change action -->
  </Storyboard>
</OpenSCENARIO>
```

Ready to run in esmini, CARLA, or any OpenSCENARIO simulator.

---

## 🌏 Supported Regions

**Road data available worldwide via OpenStreetMap**:
- 🇯🇵 Japan (Tokyo, Osaka, etc.)
- 🇺🇸 USA (all states)
- 🇪🇺 Europe (all countries)
- 🌍 Anywhere with OpenStreetMap coverage

Plus: [Use your own `.xodr` files](CUSTOM_XODR.md) from any source!

---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup
- Code structure
- Testing guidelines
- Pull request process

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

## 🔗 Links

- **Documentation**: All guides in this repository
- **MCP Specification**: [modelcontextprotocol.io](https://modelcontextprotocol.io)
- **OpenSCENARIO**: [openscenario.org](https://www.asam.net/standards/detail/openscenario/)
- **esmini Simulator**: [github.com/esmini/esmini](https://github.com/esmini/esmini)

---

**Ready to start?** → [Installation Guide](INSTALL.md)
