# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-05-31

### Added
- **XSD Validation**: Full W3C-conformant XSD validation using Uppsala pure-Rust validator
  - Lazy-loaded schema caching with `lazy_static`
  - Strict validation: fails without official XSD files (no graceful fallback)
  - Separate `warnings` field in `ValidationReport`
- **Vehicle Categories**: Added 3 missing categories to `VehicleCategory` enum
  - `Semitrailer` - Articulated trailer
  - `Train` - Railway vehicle
  - `Tram` - Streetcar
- **Position Types**: Added `Route` variant to `Position` enum for route-based positioning
  - ⚠️ **UNVERIFIED**: Not yet tested against official ASAM XSD
- **Parameter Types**: Added 3 new types to `ParameterType` enum
  - `UnsignedInt` - Unsigned integer
  - `UnsignedShort` - Unsigned short integer
  - `DateTime` - Date and time values
- **API**: `VehicleCategory` now re-exported in crate root for ergonomics

### Changed
- **BREAKING**: `ValidationReport` now has `warnings: Vec<String>` field
  - XSD-missing errors moved from fallback warnings to strict errors
  - **Migration**: Check both `report.errors` and `report.warnings`
- **BREAKING**: Validation now **fails** without XSD files (no graceful fallback)
  - Previous: `valid: true` with warning when XSD missing
  - Now: `valid: false` with error message
  - **Migration**: Ensure official ASAM XSD files are installed before validation
- XML serialization now always emits `<Properties></Properties>` element (XSD compliance)

### Fixed
- Properties element now always present in Vehicle/Pedestrian/MiscObject (XSD requirement)
- Vehicle category attribute correctly parsed from XML
- Parameter types correctly serialized to XSD-compliant strings

### Breaking Changes Summary

**Enum Variants**:
- **VehicleCategory**: 7 → 10 variants (+Semitrailer, Train, Tram)
- **Position**: 7 → 8 variants (+Route, unverified)
- **ParameterType**: 4 → 7 variants (+UnsignedInt, UnsignedShort, DateTime)

These enums are **NOT** marked `#[non_exhaustive]` - breaking changes expected in 0.x series.

**API Changes**:
- `ValidationReport` gained `warnings` field
- `VehicleCategory::as_xml_str()` method added (public API)
- Validation now **fails** without XSD (was graceful fallback)

**Semver**: This is a **0.x MINOR** release (breaking changes allowed in 0.x per semver).
Upgrading from 0.1.x will require code changes for exhaustive enum matches and validation behavior.

---

## [0.1.0] - 2026-04-30

### Added
- Initial release
- OpenSCENARIO scenario creation and export
- Catalog support for vehicles, pedestrians, and misc objects
- Road network validation
- Storyboard with actions and conditions
- Position specifications (World, Lane, Road, Relative)
- Basic XML generation and parsing

---

## Migration Guide: 0.1.x → 0.2.0

### 1. Update Validation Behavior

**Validation now requires XSD files**:

```rust
// Before: Validation passes with warning when XSD missing
let report = validator.validate(xml);
assert!(report.valid); // true even without XSD

// After: Validation fails without XSD
let report = validator.validate(xml);
assert!(!report.valid); // false, error message tells you to install XSD
assert!(report.errors[0].contains("XSD schema not available"));
```

**Action**: Install official ASAM XSD files. Run `./check-schemas.sh` for instructions.

### 2. Update ValidationReport Usage

Check warnings in addition to errors:

```rust
let report = validator.validate(xml);

// Before:
if !report.errors.is_empty() { /* ... */ }

// After:
if !report.errors.is_empty() {
    eprintln!("Errors: {:?}", report.errors);
}
if !report.warnings.is_empty() {
    eprintln!("Warnings: {:?}", report.warnings);
}
```

### 3. Update Enum Matches

No wildcard required (enums not `#[non_exhaustive]`):

```rust
// VehicleCategory: Handle new variants explicitly
match vehicle.category {
    VehicleCategory::Car => { /* ... */ },
    VehicleCategory::Truck => { /* ... */ },
    VehicleCategory::Semitrailer => { /* ... */ },  // NEW
    VehicleCategory::Train => { /* ... */ },        // NEW
    VehicleCategory::Tram => { /* ... */ },         // NEW
    // ... handle all 10 variants
}

// Position: Handle Route variant
match position {
    Position::World { .. } => { /* ... */ },
    Position::Route { .. } => {  // NEW (UNVERIFIED)
        eprintln!("Warning: Route position not XSD-verified");
        // ...
    },
    // ... handle all variants
}

// ParameterType: Handle new types
match param_type {
    ParameterType::Integer => { /* ... */ },
    ParameterType::UnsignedInt => { /* ... */ },     // NEW
    ParameterType::UnsignedShort => { /* ... */ },   // NEW
    ParameterType::DateTime => { /* ... */ },        // NEW
    // ... handle all 7 types
}
```

### 4. Update Imports (Optional)

You can now import `VehicleCategory` from crate root:

```rust
// Before:
use openscenario::entities::VehicleCategory;

// After (shorter):
use openscenario::VehicleCategory;
```

---

[0.2.0]: https://github.com/jakeaboganda/osc-mcp/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jakeaboganda/osc-mcp/releases/tag/v0.1.0
