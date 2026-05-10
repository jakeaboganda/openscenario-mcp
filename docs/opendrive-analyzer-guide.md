# OpenDRIVE Analyzer - Quick Reference

## Usage

```bash
python tools/analyze_opendrive.py <file.xodr>
```

## What It Shows

### 1. Road Geometry
- Length, starting position, heading
- Straight vs curved sections

### 2. Lane Layout
- Lane IDs (positive = left, negative = right)
- Lane types (driving, shoulder, etc.)
- Lane widths

### 3. ASCII Diagram
Visual cross-section of the road

### 4. Position Guide
Exact parameters for `Position::lane()` calls

### 5. World Coordinate Mapping
Approximate y-coordinates for each lane

## Example Output

For `simple_highway.xodr`:

```
Lane Layout:
  Lane  1: y ≈ +3.5m (left of center) - OVERTAKING
  Lane  0: y = 0.0m (center reference)
  Lane -1: y ≈ -3.5m (right) - NORMAL TRAFFIC
  Lane -2: y ≈ -7.0m (far right) - SLOW
```

## Using the Output

### For Lane-Based Positions

```rust
// Vehicle in normal traffic (right lane)
Position::lane("1", -1, 50.0, 0.0, None)
//             ^^^  ^^  ^^^^
//             |    |   distance along road (s)
//             |    lane ID from analyzer
//             road ID
```

### For World Positions

```rust
// Vehicle in normal traffic (right lane)
Position::world(50.0, -3.5, 0.0, 0.0)
//              ^^^^  ^^^^
//              x     y from analyzer output
```

## Common Patterns

### Highway Scenarios

**Right lane (normal traffic)**: Lane -1, y ≈ -3.5m
**Left lane (overtaking)**: Lane 1, y ≈ +3.5m
**Far right (slow/merging)**: Lane -2, y ≈ -7.0m

### Lane Changes

**Merge from far right to right**:
- Start: Lane -2 → Target: Lane -1

**Overtake (right to left)**:
- Start: Lane -1 → Target: Lane 1

**Return after overtake (left to right)**:
- Start: Lane 1 → Target: Lane -1

## Workflow

1. **Analyze road network**:
   ```bash
   python tools/analyze_opendrive.py roads/my_road.xodr
   ```

2. **Note lane IDs and y-coordinates**

3. **Write scenario with correct positions**:
   ```rust
   // Use lane IDs from analyzer
   Position::lane("1", -1, 0.0, 0.0, None)
   ```

4. **Generate scenario**:
   ```bash
   cargo run --example my_scenario
   ```

5. **Test in esmini**

6. **Iterate if needed**

## Tips

- **Always analyze the road first** before writing position code
- **Lane 0 is NOT drivable** - it's the reference line
- **Negative lane IDs** = right of center (normal traffic)
- **Positive lane IDs** = left of center (overtaking)
- **World y-coordinates** are approximate (use lane positions when possible)

## Integration with Examples

Before writing any scenario:

```bash
# 1. Analyze road
python tools/analyze_opendrive.py roads/simple_highway.xodr > road_analysis.txt

# 2. Read road_analysis.txt

# 3. Write scenario using correct lane IDs

# 4. Test
```

## Future Enhancements

- [ ] 3D visualization (matplotlib)
- [ ] Junction analysis
- [ ] Road connection graph
- [ ] Lane change validity checker
- [ ] Position validation (is this position valid?)
