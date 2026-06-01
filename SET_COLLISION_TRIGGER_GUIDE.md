# set_collision_trigger Tool Guide

## Overview

The `set_collision_trigger` tool allows you to set collision-based triggers for Acts and Events in OpenSCENARIO scenarios. This enables scenario elements to activate automatically when entities collide, perfect for emergency scenarios, obstacle detection, and interaction-based triggers.

---

## Basic Usage

### Trigger Event on Collision

```json
{
  "name": "set_collision_trigger",
  "arguments": {
    "scenario_id": "scenario_abc123",
    "element_type": "Event",
    "story_name": "main",
    "act_name": "main_act",
    "maneuver_group": "ego_mg",
    "maneuver": "ego_maneuver",
    "event_name": "brake_event",
    "entity_refs": ["ego"],
    "target_entity": "obstacle_1",
    "trigger_rule": "any"
  }
}
```

**Result**: Brake event triggers immediately when `ego` collides with `obstacle_1`.

---

### Monitor Multiple Entities

```json
{
  "name": "set_collision_trigger",
  "arguments": {
    "scenario_id": "scenario_abc123",
    "element_type": "Act",
    "story_name": "emergency",
    "act_name": "emergency_act",
    "entity_refs": ["vehicle1", "vehicle2", "vehicle3"],
    "target_entity": "barrier",
    "trigger_rule": "any",
    "delay_seconds": 0.5
  }
}
```

**Result**: Act triggers 0.5 seconds after **any** of the three vehicles collides with the barrier.

---

### Require All Entities to Collide

```json
{
  "name": "set_collision_trigger",
  "arguments": {
    "scenario_id": "scenario_abc123",
    "element_type": "Act",
    "story_name": "multi_collision",
    "act_name": "multi_collision_act",
    "entity_refs": ["vehicle1", "vehicle2"],
    "target_entity": "junction_center",
    "trigger_rule": "all"
  }
}
```

**Result**: Act only triggers after **both** vehicle1 **and** vehicle2 collide with junction_center.

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
| `entity_refs` | array[string] | ✅ | Entities to monitor (e.g., `["ego", "vehicle2"]`) |
| `target_entity` | string | ✅ | Entity to detect collisions with |
| `trigger_rule` | string | ✅ | `"any"` (at least one) or `"all"` (all must collide) |
| `delay_seconds` | number | ❌ | Optional delay after collision (default: 0.0) |

---

## Trigger Rules

### `"any"` Rule
**Behavior**: Triggers when **at least one** of the monitored entities collides

**Use Case**: Multiple vehicles, any collision should trigger response

```json
"entity_refs": ["car1", "car2", "car3"],
"target_entity": "obstacle",
"trigger_rule": "any"
```

**Result**: Triggers if car1 **OR** car2 **OR** car3 collides with obstacle

---

### `"all"` Rule  
**Behavior**: Triggers only when **all** monitored entities have collided

**Use Case**: Multi-vehicle pile-up, waiting for complete interaction

```json
"entity_refs": ["car1", "car2"],
"target_entity": "crash_zone",
"trigger_rule": "all"
```

**Result**: Triggers only after car1 **AND** car2 **both** collide with crash_zone

---

## Common Use Cases

### 1. Emergency Braking

```json
{
  "name": "set_collision_trigger",
  "arguments": {
    "element_type": "Event",
    "entity_refs": ["ego"],
    "target_entity": "pedestrian",
    "trigger_rule": "any",
    "event_name": "emergency_brake_event"
  }
}
```

**Scenario**: Ego vehicle brakes immediately upon collision with pedestrian

---

### 2. Obstacle Avoidance

```json
{
  "name": "set_collision_trigger",
  "arguments": {
    "element_type": "Event",
    "entity_refs": ["autonomous_vehicle"],
    "target_entity": "cone_array",
    "trigger_rule": "any",
    "event_name": "lane_change_event",
    "delay_seconds": 0.05
  }
}
```

**Scenario**: Vehicle initiates lane change 50ms after detecting cone collision

---

### 3. Multi-Vehicle Crash Coordination

```json
{
  "name": "set_collision_trigger",
  "arguments": {
    "element_type": "Act",
    "entity_refs": ["vehicle1", "vehicle2", "vehicle3"],
    "target_entity": "intersection_center",
    "trigger_rule": "any",
    "act_name": "emergency_response_act"
  }
}
```

**Scenario**: Emergency response activates when any vehicle hits intersection center

---

### 4. Sequential Collision Detection

**Setup**: Combine with time triggers for complex scenarios

```json
// Trigger 1: Detect first collision
{"name": "set_collision_trigger", "trigger_rule": "any"}

// Trigger 2: After 2 seconds, check if others collided
{"name": "set_trigger_time", "time_seconds": 2.0}
```

---

## Complete Workflow Example

```json
// 1. Create scenario + load road network
{"name": "create_scenario", "arguments": {"name": "collision_test", "version": "1.2"}}
{"name": "load_road_network", "arguments": {"xodr_path": "/path/map.xodr"}}

// 2. Add entities
{"name": "add_vehicle", "arguments": {"name": "ego", "category": "Car"}}
{"name": "add_misc_object", "arguments": {"name": "obstacle_1", "category": "obstacle"}}

// 3. Set positions
{"name": "set_lane_position", "arguments": {"entity_name": "ego", "road_id": "1", "lane_id": "-1", "s": 10.0}}
{"name": "set_lane_position", "arguments": {"entity_name": "obstacle_1", "road_id": "1", "lane_id": "-1", "s": 50.0}}

// 4. Add brake action
{"name": "add_speed_action", "arguments": {
  "entity_name": "ego",
  "story_name": "emergency",
  "speed": 0.0,
  "duration": 1.0
}}

// 5. Set collision trigger for brake action
{"name": "set_collision_trigger", "arguments": {
  "scenario_id": "scenario_collision_test_xyz",
  "element_type": "Act",
  "story_name": "emergency",
  "act_name": "emergency_act",
  "entity_refs": ["ego"],
  "target_entity": "obstacle_1",
  "trigger_rule": "any",
  "delay_seconds": 0.0
}}

// 6. Export scenario
{"name": "export_xml", "arguments": {
  "scenario_id": "scenario_collision_test_xyz",
  "output_path": "output/collision_test.xosc"
}}
```

---

## Generated XML Structure

```xml
<StartTrigger>
  <ConditionGroup>
    <Condition name="Collision_ego_obstacle_1" delay="0" conditionEdge="rising">
      <ByEntityCondition>
        <TriggeringEntities triggeringEntitiesRule="any">
          <EntityRef entityRef="ego"/>
        </TriggeringEntities>
        <EntityCondition>
          <CollisionCondition>
            <EntityRef entityRef="obstacle_1"/>
          </CollisionCondition>
        </EntityCondition>
      </ByEntityCondition>
    </Condition>
  </ConditionGroup>
</StartTrigger>
```

**Key Elements:**
- `triggeringEntitiesRule="any"` - At least one entity must collide
- `conditionEdge="rising"` - Triggers once on collision start (not continuously)
- `CollisionCondition` - References target entity to detect collision with

---

## Combining Triggers

### Collision OR Time

Use multiple condition groups to create OR logic:

```json
// Trigger 1: Collision-based
{"name": "set_collision_trigger", ...}

// Trigger 2: Time-based (fallback)
{"name": "set_trigger_time", "time_seconds": 10.0}
```

**Result**: Act triggers on collision **OR** after 10 seconds (whichever comes first)

---

### Collision AND Distance

Combine conditions in a single condition group (requires manual XML editing currently):

```xml
<ConditionGroup>
  <Condition ...><!-- Collision condition --></Condition>
  <Condition ...><!-- Distance condition --></Condition>
</ConditionGroup>
```

**Result**: Both collision **AND** distance conditions must be true

---

## Troubleshooting

### "Entity not found" Error
**Cause**: Referenced entity doesn't exist in scenario  
**Fix**: Ensure all entities in `entity_refs` and `target_entity` were added via `add_vehicle`, `add_pedestrian`, or `add_misc_object`

### "Invalid trigger_rule" Error
**Cause**: trigger_rule is not "any" or "all"  
**Fix**: Use exactly `"any"` or `"all"` (case-insensitive)

### Trigger Never Activates
**Possible Causes**:
1. Entities never actually collide (check positions/paths)
2. `trigger_rule="all"` but not all entities collide
3. Collision detection disabled in simulator

**Debug**:
- Verify entity bounding boxes overlap
- Check simulator collision detection settings
- Try `trigger_rule="any"` first for testing

### Delay Not Working
**Cause**: `delay_seconds` applies after collision detected, not before  
**Clarification**: `delay_seconds` is post-collision delay, not pre-collision warning

---

## Best Practices

1. **Start with "any" rule** - Easier to debug than "all"
2. **Use small delays** - 0.0-0.5s for realistic reaction times
3. **Test collision geometry** - Verify entities can actually collide
4. **Combine with time triggers** - Provide fallback if collision never occurs
5. **Monitor single entities** - Start simple (`entity_refs: ["ego"]`) before complex multi-entity scenarios

---

## Architecture

**Library Support:**
- Condition helper: `Condition::collision()`
- Auto-generates descriptive names: `Collision_{entities}_{target}`
- Uses `ConditionEdge::Rising` for one-time activation
- Wraps CollisionCondition in ByEntityCondition

**Handler**: `handle_set_collision_trigger()`
- Validates parameters and converts string rule to enum
- Creates collision condition with optional delay
- Reuses existing trigger-setting APIs

---

**Version**: osc-mcp v0.2.0  
**Commit**: e1d8e9e

---

## See Also

- `set_trigger_time` - Time-based triggers
- `add_speed_action` - Speed change actions
- `add_lane_change_action` - Lane change actions
