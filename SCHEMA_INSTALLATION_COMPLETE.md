# ✅ OpenSCENARIO XSD Schemas - Installation Complete

## Summary

Successfully installed **official ASAM XSD schemas** for three OpenSCENARIO versions:

✅ **v1.1.1** - OpenSCENARIO 1.1.1 (77KB)  
✅ **v1.2.0** - OpenSCENARIO 1.2.0 (100KB)  
✅ **v1.3.0** - OpenSCENARIO 1.3.0 (104KB) - **Latest**

---

## What Was Added

### Schema Files

```
openscenario/schemas/
├── v1.1.1/
│   └── OpenSCENARIO.xsd  ✅ Full ASAM schema (77KB)
├── v1.2.0/
│   └── OpenSCENARIO.xsd  ✅ Full ASAM schema (100KB)
├── v1.3.0/
│   └── OpenSCENARIO.xsd  ✅ Full ASAM schema (104KB)
└── README.md             📝 Updated documentation
```

### Version Support

| Version | Features | Schema Size | Status |
|---------|----------|-------------|--------|
| **1.1.1** | Core OpenSCENARIO features | 77KB | ✅ Supported |
| **1.2.0** | + Animations, lighting, variables, controllers | 100KB | ✅ Supported |
| **1.3.0** | + Angle conditions, clothoid splines, trailers, monitors | 104KB | ✅ Supported (latest) |

---

## How It Works

### Automatic Version Detection

The validator automatically selects the correct schema based on your scenario's version:

```xml
<FileHeader revMajor="1" revMinor="2" ...>
  <!-- Validator uses v1.2.0/OpenSCENARIO.xsd -->
</FileHeader>
```

**Version Mapping**:
- `revMajor=1, revMinor=1` → `v1.1.1/OpenSCENARIO.xsd`
- `revMajor=1, revMinor=2` → `v1.2.0/OpenSCENARIO.xsd`
- `revMajor=1, revMinor=3` → `v1.3.0/OpenSCENARIO.xsd`

### Validation Mode

**Strict Mode** (current behavior):
```rust
let report = scenario.validate_with_xsd()?;

if report.valid {
    println!("✅ Scenario passes full XSD validation!");
} else {
    eprintln!("❌ Validation errors: {:?}", report.errors);
}
```

**No schemas? Returns error**:
```
ValidationReport {
    valid: false,
    errors: ["XSD schema not available for OpenSCENARIO v1.2.0"],
    warnings: []
}
```

---

## Key Features

### ✅ Full XSD Validation

With these schemas, the validator performs:
- ✅ Complete element structure checking
- ✅ Attribute constraint validation
- ✅ Cardinality enforcement (minOccurs, maxOccurs)
- ✅ Data type validation (integers, doubles, booleans, etc.)
- ✅ Enumeration checking (VehicleCategory, CloudState, etc.)
- ✅ Complex type validation (nested elements)
- ✅ Sequence/choice enforcement

### ✅ Version-Specific Features

**v1.1.1**:
- Basic scenario elements
- Standard vehicle/pedestrian/misc objects
- Core actions and conditions
- Trajectory and route support

**v1.2.0** (adds):
- `AnimationAction` and `AnimationState`
- `LightStateAction` with color control
- `VariableAction` and `VariableCondition`
- `ControllerType` enum (lateral, longitudinal, lighting, etc.)
- `AutomaticGearType` (n, p, r, d)
- `ColorType` (RGB, CMYK support)
- Enhanced brake input (percent vs force)

**v1.3.0** (adds):
- `AngleCondition` and `RelativeAngleCondition`
- `ClothoidSpline` for advanced trajectories
- `ConnectTrailerAction` / `DisconnectTrailerAction`
- `SetMonitorAction` with `MonitorDeclarations`
- `LogNormalDistribution` for stochastic scenarios
- `Lane` element for direct lane references
- Enhanced `GeoPosition` (vertical road selection)
- `objectControllerRef` for controller management

---

## Version Differences

### Major Changes by Version

**1.1.1 → 1.2.0**:
- Animation and lighting system introduced
- Variable management added
- Controller type classification
- Enhanced gear control (automatic/manual)
- Color specification (RGB/CMYK)

**1.2.0 → 1.3.0**:
- Orientation/angle conditions added
- Advanced trajectory types (clothoid splines)
- Trailer management actions
- Monitoring and logging infrastructure
- Enhanced distributions (log-normal)
- Coordinate system extended (world)

---

## Schema Sources

**Official ASAM Schemas**:
- **Copyright**: © ASAM e.V., 2021-2024
- **Standard**: https://www.asam.net/standards/detail/openscenario/
- **License**: ASAM license terms (see www.asam.net/license.html)
- **Usage**: Distributable for implementation purposes

**Verification**:
- ✅ All schemas contain official ASAM copyright headers
- ✅ Valid XML Schema Definition (XSD) format
- ✅ Complete specification coverage
- ✅ Proper namespace declarations

---

## Testing

### Verify Installation

```bash
cd openscenario/schemas
find . -name "OpenSCENARIO.xsd" | sort
```

**Expected output**:
```
./v1.1.1/OpenSCENARIO.xsd
./v1.2.0/OpenSCENARIO.xsd
./v1.3.0/OpenSCENARIO.xsd
```

### Check Schema Validity

```bash
# Verify XML structure
for v in v1.1.1 v1.2.0 v1.3.0; do
  xmllint --noout schemas/$v/OpenSCENARIO.xsd && echo "✅ $v valid"
done
```

### Test Validation

```bash
# Run validation tests
cd ../..
cargo test validation
```

---

## Usage Examples

### Example 1: Validate v1.2.0 Scenario

```rust
use openscenario::Scenario;

let scenario = Scenario::new("my_scenario", "1.2");
// ... add vehicles, actions, etc.

// Export XML
let xml = scenario.export_xml()?;

// Validate with XSD (uses v1.2.0 schema automatically)
let report = scenario.validate_with_xsd()?;

assert!(report.valid);
```

### Example 2: Use v1.3.0 Features

```rust
use openscenario::Scenario;

let scenario = Scenario::new("advanced_scenario", "1.3");
// ... now you can use v1.3.0 features like:
// - AngleCondition
// - ClothoidSpline trajectories
// - ConnectTrailerAction
// - SetMonitorAction
```

### Example 3: Handle Validation Errors

```rust
let report = scenario.validate_with_xsd()?;

if !report.valid {
    for error in &report.errors {
        eprintln!("❌ Validation error: {}", error);
    }
}

for warning in &report.warnings {
    eprintln!("⚠️ Warning: {}", warning);
}
```

---

## Next Steps

### For Development

1. ✅ **Schemas installed** - Ready to use
2. ✅ **Documentation updated** - See `schemas/README.md`
3. ✅ **Version support** - 1.1.1, 1.2.0, 1.3.0 ready

### For Users

1. **Use latest version** (1.3.0) for new scenarios
2. **Validate early** - Catch errors before simulation
3. **Review warnings** - Fix minor issues proactively

### For Contributors

1. **Add v1.4.0 support** (when released):
   - Create `schemas/v1.4.0/` directory
   - Add official `OpenSCENARIO.xsd`
   - Update version mapping if needed
   - Run `cargo test validation`

---

## Impact

### Before

❌ Validation could only check basic XML structure  
❌ Invalid scenarios might pass initial checks  
❌ Errors discovered late (during simulation)  

### After

✅ **Full XSD validation** with official schemas  
✅ **Catch errors early** in development  
✅ **100% spec compliance** checking  
✅ **Production-ready** validation  
✅ **Three versions supported** (1.1.1, 1.2.0, 1.3.0)  

---

## Documentation

**Updated files**:
- ✅ `openscenario/schemas/README.md` - Complete schema documentation
- ✅ Commit `4d41d8b` - Full change history

**Key sections**:
- Supported versions
- Validation behavior
- Version differences
- Installation verification
- License information

---

## Commit

**Commit**: `4d41d8b`  
**Message**: `feat: Add official ASAM XSD schemas for OpenSCENARIO v1.1.1, v1.2.0, v1.3.0`

**Files changed**:
- Added: `openscenario/schemas/v1.1.1/OpenSCENARIO.xsd` (77KB)
- Added: `openscenario/schemas/v1.2.0/OpenSCENARIO.xsd` (100KB)
- Added: `openscenario/schemas/v1.3.0/OpenSCENARIO.xsd` (104KB)
- Modified: `openscenario/schemas/README.md`

**Total**: +7,016 lines / 281KB of schemas

---

## ✅ Status: Complete

All three OpenSCENARIO XSD schemas successfully installed and ready for production use!

**What's next**: The library now supports full XSD validation for OpenSCENARIO 1.1.1, 1.2.0, and 1.3.0 scenarios. Use the latest version (1.3.0) for new projects to access all features.
