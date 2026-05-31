# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-05-31

### Added
- **XSD Validation**: Full W3C-conformant XSD validation using Uppsala pure-Rust validator
  - Lazy-loaded schema caching with `std::sync::OnceLock`
  - Graceful fallback when XSD files missing
  - Separate `warnings` field in `ValidationReport`
- **Vehicle Categories**: Added 3 missing categories to `VehicleCategory` enum
  - `Semitrailer` - Articulated trailer
  - `Train` - Railway vehicle
  - `Tram` - Streetcar
- **Position Types**: Added `Route` variant to `Position` enum for route-based positioning
- **Parameter Types**: Added 3 new types to `ParameterType` enum
  - `UnsignedInt` - Unsigned integer
  - `UnsignedShort` - Unsigned short integer
  - `DateTime` - Date and time values
- **API**: `VehicleCategory` now re-exported in crate root for ergonomics

### Changed
- **BREAKING**: `VehicleCategory`, `Position`, and `ParameterType` enums now marked `#[non_exhaustive]`
  - **Migration**: Add wildcard pattern `_ => {}` to exhaustive matches on these enums
  - Example:
    ```rust
    // Before (breaks in 0.2.0):
    match category {
        VehicleCategory::Car => {},
        // ... only 7 variants
    }
    
    // After (works in 0.2.0+):
    match category {
        VehicleCategory::Car => {},
        // ... handle variants you care about
        _ => {} // Required with #[non_exhaustive]
    }
    ```
- **BREAKING**: `ValidationReport` now has `warnings: Vec<String>` field
  - XSD-missing warnings moved from `errors` to `warnings`
  - **Migration**: Check both `report.errors` and `report.warnings`
- XML serialization now always emits `<Properties/>` element (XSD compliance)
- Replaced `lazy_static` with stdlib `std::sync::OnceLock` (zero external dependencies for validation)

### Fixed
- Properties element now always present in Vehicle/Pedestrian/MiscObject (XSD requirement)
- Vehicle category attribute correctly parsed from XML
- Parameter types correctly serialized to XSD-compliant strings

### Breaking Changes Summary

**Enum Exhaustiveness**:
- **VehicleCategory**: 7 → 10 variants (+Semitrailer, Train, Tram)
- **Position**: 7 → 8 variants (+Route)
- **ParameterType**: 4 → 7 variants (+UnsignedInt, UnsignedShort, DateTime)

All three enums now require wildcard patterns due to `#[non_exhaustive]`.

**API Changes**:
- `ValidationReport` gained `warnings` field
- `VehicleCategory::as_xml_str()` method added (public API)

**Semver**: This is a **0.x MINOR** release (breaking changes allowed in 0.x per semver).
Upgrading from 0.1.x will require code changes for exhaustive enum matches.

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

### 1. Update Enum Matches

Add wildcard patterns to exhaustive matches:

```rust
// VehicleCategory
match vehicle.category {
    VehicleCategory::Car => { /* ... */ },
    VehicleCategory::Truck => { /* ... */ },
    _ => { /* handle other categories */ }  // ← ADD THIS
}

// Position
match position {
    Position::World { .. } => { /* ... */ },
    Position::Lane { .. } => { /* ... */ },
    _ => { /* handle other positions */ }  // ← ADD THIS
}

// ParameterType
match param_type {
    ParameterType::Integer => { /* ... */ },
    ParameterType::String => { /* ... */ },
    _ => { /* handle other types */ }  // ← ADD THIS
}
```

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

### 3. Update Imports (Optional)

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
