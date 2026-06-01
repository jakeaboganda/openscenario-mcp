# set_trigger_time Tool Guide

## Overview

The `set_trigger_time` tool allows you to set time-based triggers for Acts and Events in OpenSCENARIO scenarios. This enables precise timing control for when scenario elements should activate.

---

## Basic Usage

### Setting an Act Start Trigger

```json
{
  "name": "set_trigger_time",
  "arguments": {
    "scenario_id": "scenario_abc123",
    "element_type": "Act",
    "story_name": "main",
    "act_name": "main_act",
    "time_seconds": 2.0
  }
}
```

**Result**: Act `main_act` in story `main` will start at simulation time **t=2.0 seconds**.

---

### Setting an Event Start Trigger

```json
{
  "name": "set_trigger_time",
  "arguments": {
    "scenario_id": "scenario_abc123",
    "element_type": "Event",
    "story_name": "main",
    "act_name": "main_act",
    "maneuver_group": "ego_mg",
    "maneuver": "ego_maneuver",
    "event_name": "speed_event",
    "time_seconds": 5.0,
    "delay_seconds": 0.5
  }
}
```

**Result**: Event `speed_event` will start at **t=5.5 seconds** (5.0s + 0.5s delay).

---

## Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `scenario_id` | string | ✅ | Target scenario ID |
| `element_type` | string | ✅ | `"Act"` or `"Event"` |
| `story_name` | string | ✅ | Name of parent story |
| `act_name` | string | ✅ | Name of parent act |
| `maneuver_group` | string | Event only | Name of maneuver group (Event triggers) |
| `maneuver` | string | Event only | Name of maneuver (Event triggers) |
| `event_name` | string | Event only | Name of event (Event triggers) |
| `time_seconds` | number | ✅ | Simulation time when trigger activates |
| `delay_seconds` | number | ❌ | Optional delay after condition met (default: 0.0) |

---

## Naming Conventions

When using `add_speed_action` or `add_lane_change_action`, the MCP server creates auto-generated names:

### Acts
```
Act name format: {story_name}_act
Example: "main" story → "main_act"
```

### Maneuver Groups
```
MG format: {entity_name}_mg
Example: "ego" vehicle → "ego_mg"
```

### Maneuvers
```
Maneuver format: {entity_name}_maneuver
Example: "ego" vehicle → "ego_maneuver"
```

### Events
```
Event names: "speed_event" or "lane_change_event"
(fixed names used by action creators)
```

---

## Complete Workflow Example

```json
// 1. Create scenario
{"name": "create_scenario", "arguments": {"name": "test", "version": "1.2"}}
// → Returns: "scenario_test_abc123"

// 2. Load road network
{"name": "load_road_network", "arguments": {"xodr_path": "/path/to/map.xodr"}}

// 3. Add vehicle
{"name": "add_vehicle", "arguments": {
  "scenario_id": "scenario_test_abc123",
  "name": "ego",
  "category": "Car"
}}

// 4. Set position
{"name": "set_lane_position", "arguments": {
  "scenario_id": "scenario_test_abc123",
  "entity_name": "ego",
  "road_id": "1",
  "lane_id": "-1",
  "s": 10.0
}}

// 5. Add speed action (creates story/act/maneuver/event structure)
{"name": "add_speed_action", "arguments": {
  "scenario_id": "scenario_test_abc123",
  "entity_name": "ego",
  "story_name": "main",
  "speed": 30.0,
  "duration": 5.0
}}

// 6. Set trigger time for Act to start at t=2.0s
{"name": "set_trigger_time", "arguments": {
  "scenario_id": "scenario_test_abc123",
  "element_type": "Act",
  "story_name": "main",
  "act_name": "main_act",
  "time_seconds": 2.0
}}

// 7. Export scenario
{"name": "export_xml", "arguments": {
  "scenario_id": "scenario_test_abc123",
  "output_path": "output/test.xosc"
}}
```

---

## Generated XML

The `set_trigger_time` tool generates XML like this:

```xml
<StartTrigger>
  <ConditionGroup>
    <Condition name="SimTime_2" delay="0" conditionEdge="rising">
      <ByValueCondition>
        <SimulationTimeCondition value="2.0" rule="greaterThan"/>
      </ByValueCondition>
    </Condition>
  </ConditionGroup>
</StartTrigger>
```

**Key Points:**
- `conditionEdge="rising"` - Triggers once when time crosses threshold
- `rule="greaterThan"` - Activates when simulation_time > time_seconds
- `delay` - Optional additional delay after condition met

---

## Common Use Cases

### Sequential Actions

```json
// Act 1 starts at t=0 (immediate)
// Act 2 starts at t=5.0
{"element_type": "Act", "act_name": "phase2_act", "time_seconds": 5.0}

// Act 3 starts at t=10.0
{"element_type": "Act", "act_name": "phase3_act", "time_seconds": 10.0}
```

### Delayed Event Activation

```json
// Event starts at t=3.5 (3.0 + 0.5 delay)
{
  "element_type": "Event",
  "event_name": "brake_event",
  "time_seconds": 3.0,
  "delay_seconds": 0.5
}
```

---

## Limitations & Notes

**Current Limitations:**
- ✅ Supports: Act start triggers
- ✅ Supports: Event start triggers
- ❌ Does NOT support: Stop triggers (use `set_stop_time` instead)
- ❌ Does NOT support: Condition-based triggers (only time-based)

**Best Practices:**
1. **Create scenario structure first** - Add entities, positions, actions before setting triggers
2. **Use consistent naming** - Follow the auto-generated naming conventions
3. **Test trigger times** - Ensure Acts start in logical sequence
4. **Delay for coordination** - Use `delay_seconds` for fine-grained timing

---

## Troubleshooting

### "Act not found" Error
**Cause**: Act doesn't exist yet  
**Fix**: Run `add_speed_action` or `add_lane_change_action` first to create the Act

### "Event not found" Error
**Cause**: Incorrect maneuver/event names  
**Fix**: Check auto-generated names:
- MG: `{entity}_mg`
- Maneuver: `{entity}_maneuver`
- Event: `speed_event` or `lane_change_event`

### "Missing parameters" Error (Event triggers)
**Cause**: Event triggers require `maneuver_group`, `maneuver`, and `event_name`  
**Fix**: Include all three parameters when `element_type="Event"`

---

## Architecture

**Implementation:**
- Handler: `openscenario-mcp/src/handlers.rs::handle_set_trigger_time`
- Server: `openscenario-mcp/src/server.rs` (tool registration)
- Library: Uses `Scenario::set_act_start_trigger` and `Scenario::set_event_start_trigger`

**Trigger Mechanics:**
- Creates `Condition::simulation_time(t, Rule::GreaterThan)`
- Sets `ConditionEdge::Rising` (one-time activation)
- Wraps in `ConditionGroup` → `Trigger`

---

**Version**: osc-mcp v0.2.0  
**Commit**: e8588d4
