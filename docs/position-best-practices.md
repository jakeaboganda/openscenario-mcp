# Position Type Guidelines - Best Practices

## Core Principle: Avoid World Positions

**Rule**: Prefer `Position::lane()` or `Position::road()` over `Position::world()`

**Why**: World positions don't bind to road networks and cause visualization issues.

---

## When to Use Each Position Type

### ✅ Position::lane() - RECOMMENDED

**Use for**: Most scenarios with road networks

```rust
Position::lane(
    "1",      // road_id
    -1,       // lane_id (negative = right of center)
    50.0,     // s (distance along road in meters)
    0.0,      // offset (lateral offset from lane center)
    None      // orientation (optional)
)
```

**Advantages**:
- ✅ Binds to specific lane
- ✅ Follows road geometry (curves, elevation)
- ✅ Works with lane change actions
- ✅ Visualizes correctly in esmini
- ✅ Clear semantic meaning

**Use cases**:
- Highway scenarios
- Lane change maneuvers
- Following behavior
- Traffic scenarios
- Any scenario with roads

---

### ✅ Position::road() - GOOD ALTERNATIVE

**Use for**: Positions not tied to specific lanes

```rust
Position::road(
    "1",      // road_id
    50.0,     // s (distance along road)
    -3.5,     // t (lateral offset from road center)
    None      // orientation (optional)
)
```

**Advantages**:
- ✅ Binds to road
- ✅ More flexible than lane positions
- ✅ Good for off-lane positions (shoulders, sidewalks)

**Use cases**:
- Pedestrians on sidewalks
- Vehicles on road shoulders
- Parking scenarios
- Between-lane positions

---

### ⚠️ Position::world() - AVOID

**Use ONLY for**: Scenarios without road networks

```rust
Position::world(
    0.0,      // x
    0.0,      // y
    0.0,      // z
    0.0       // heading (radians)
)
```

**Problems**:
- ❌ Not bound to road network
- ❌ Doesn't follow road geometry
- ❌ Hard to align with lanes
- ❌ Lane change actions don't work properly
- ❌ No semantic meaning

**Only acceptable use cases**:
- Testing position systems
- Scenarios explicitly without roads
- Temporary prototypes (must convert later)

---

## Migration Strategy

### Before (❌ Wrong):
```rust
// World position - disconnected from road
scenario.set_initial_position("ego", 
    Position::world(0.0, 3.5, 0.0, 0.0))?;
```

### After (✅ Correct):
```rust
// Lane position - bound to road and lane
scenario.set_initial_position("ego", 
    Position::lane("1", -1, 0.0, 0.0, None))?;
```

---

## Position Selection Decision Tree

```
Start
  │
  ├─ Do you have a road network? ───No──→ Position::world() (temporary only)
  │                                        
  Yes
  │
  ├─ Is vehicle in a specific lane? ───Yes──→ Position::lane() ✅ RECOMMENDED
  │
  No (shoulder, sidewalk, etc.)
  │
  └─ Position::road() ✅ GOOD
```

---

## Common Patterns

### Pattern 1: Highway Traffic

```rust
// Right lane (normal traffic)
Position::lane("1", -1, 50.0, 0.0, None)

// Left lane (overtaking)
Position::lane("1", 1, 50.0, 0.0, None)

// Far right (slow/merging)
Position::lane("1", -2, 50.0, 0.0, None)
```

### Pattern 2: Following Vehicles

```rust
// Leader
Position::lane("1", -1, 100.0, 0.0, None)

// Follower (50m behind)
Position::lane("1", -1, 50.0, 0.0, None)
```

### Pattern 3: Two Lanes Apart

```rust
// Vehicle 1 in right lane
Position::lane("1", -1, 0.0, 0.0, None)

// Vehicle 2 in left lane
Position::lane("1", 1, 0.0, 0.0, None)
```

---

## Validation Rules

### For Examples and Documentation

**Before committing any example**:

1. ✅ Check: Does it use `Position::lane()` or `Position::road()`?
2. ✅ Check: No `Position::world()` unless explicitly justified
3. ✅ Check: Lane IDs match road network (use analyzer tool)
4. ✅ Check: s-coordinates are within road length
5. ✅ Test: Visualize in esmini to confirm correct placement

### For Library Users

Document in guides:
- Prefer lane/road positions
- World positions are legacy/testing only
- Always analyze road network first
- Use matcher tool for suggestions

---

## API Changes Considered

### Option 1: Deprecate World Positions

```rust
#[deprecated(since = "0.2.0", note = "Use Position::lane() or Position::road() instead")]
pub fn world(x: f64, y: f64, z: f64, h: f64) -> Self { ... }
```

**Pros**: Clear guidance
**Cons**: Breaking change

### Option 2: Add Warnings

```rust
impl Position {
    pub fn world(x: f64, y: f64, z: f64, h: f64) -> Self {
        eprintln!("Warning: Position::world() is not recommended. Use Position::lane() or Position::road() for scenarios with roads.");
        // ...
    }
}
```

**Pros**: Non-breaking
**Cons**: Noisy

### Option 3: Documentation Only

**Recommended approach**:
- Document best practices
- Update all examples to use lane positions
- Show world positions only in "advanced" section
- Emphasize in Getting Started guide

---

## Example Rewrites

### Highway Merge (Before)

```rust
// ❌ Wrong: World position
scenario.set_initial_position("merging_vehicle",
    Position::world(0.0, 3.5, 0.0, 0.0))?;
```

### Highway Merge (After)

```rust
// ✅ Correct: Lane position
scenario.set_initial_position("merging_vehicle",
    Position::lane("1", -2, 10.0, 0.0, None))?;
    // Start in far-right lane (simulates on-ramp)
```

### Emergency Braking (Before)

```rust
// ❌ Wrong: World positions
scenario.set_initial_position("lead",
    Position::world(50.0, 0.0, 0.0, 0.0))?;
scenario.set_initial_position("follower",
    Position::world(0.0, 0.0, 0.0, 0.0))?;
```

### Emergency Braking (After)

```rust
// ✅ Correct: Lane positions
scenario.set_initial_position("lead",
    Position::lane("1", -1, 100.0, 0.0, None))?;
scenario.set_initial_position("follower",
    Position::lane("1", -1, 50.0, 0.0, None))?;
    // 50m behind in same lane
```

---

## Tools Support

### Analyzer Tool

`analyze_opendrive.py` shows correct lane IDs:

```
Vehicle Position Guide:
  Lane -1: Position::lane("1", -1, s, 0.0, None)
           ↑ Right of center (normal traffic)
```

### Matcher Tool

`match_scenarios.py` generates lane-based code:

```rust
scenario.set_initial_position("merging_vehicle", 
    Position::lane("1", -2, 10.0, 0.0, None))?;
```

Both tools **never** suggest `Position::world()`.

---

## Documentation Updates Needed

### 1. Getting Started Guide

Add section: "Position Types: Always Use Lane/Road"

### 2. README Examples

Audit all code examples, replace world with lane positions

### 3. API Documentation

Add warning to `Position::world()` rustdoc:

```rust
/// # Warning
/// 
/// World positions are not bound to road networks and should be avoided
/// for scenarios with roads. Use [`Position::lane()`] or [`Position::road()`]
/// instead for proper road-based positioning.
```

### 4. Example Files

Rewrite all 7 examples:
- highway_merge.rs ✅ (use lane -2)
- lane_change_overtaking.rs ✅ (use lane -1 → lane 1)
- emergency_braking.rs ✅ (use lane -1, same lane)
- platooning.rs ✅ (use lane -1, same lane)
- adaptive_cruise_control.rs (audit)
- hello_world.rs (audit)
- lane_change.rs (audit)

---

## Testing Strategy

### For Each Example

1. Analyze road: `python tools/analyze_opendrive.py roads/file.xodr`
2. Get suggestions: `python tools/match_scenarios.py roads/file.xodr --template <scenario>`
3. Rewrite with lane positions
4. Generate .xosc
5. Visualize in esmini
6. Verify placement is correct

### Validation Checklist

- [ ] No `Position::world()` in examples
- [ ] All positions use `Position::lane()` or `Position::road()`
- [ ] Lane IDs match road network
- [ ] s-coordinates within road bounds
- [ ] Comments explain lane choices
- [ ] Visualizes correctly in esmini

---

## Future: Automatic Validation

Could add to library:

```rust
impl Scenario {
    /// Validate all positions against road network
    pub fn validate_positions(&self) -> Result<ValidationReport> {
        let mut warnings = Vec::new();
        
        for (entity, pos) in &self.initial_positions {
            match pos {
                Position::World(_) => {
                    warnings.push(format!(
                        "Entity '{}' uses world position (not recommended)",
                        entity
                    ));
                }
                Position::Lane(lane_pos) => {
                    // Check lane exists in road network
                    // Check s is within road length
                }
                _ => {}
            }
        }
        
        Ok(ValidationReport { warnings, errors: vec![] })
    }
}
```

---

## Summary

**Golden Rule**: 🌟 **Always use `Position::lane()` or `Position::road()` for scenarios with road networks** 🌟

**Never use** `Position::world()` unless:
- No road network exists
- Explicitly testing position systems
- Temporary prototype (mark TODO)

**Tools help**: Both analyzer and matcher suggest correct lane positions automatically.

---

## Action Items for Tomorrow

1. ✅ Rewrite all 4 highway examples with lane positions
2. ✅ Add warnings to Position::world() rustdoc
3. ✅ Update Getting Started guide
4. ✅ Audit all README code examples
5. ✅ Test all examples in esmini
6. ✅ Document position selection in guides

**Priority**: Examples first (user-facing), then docs, then API changes.
