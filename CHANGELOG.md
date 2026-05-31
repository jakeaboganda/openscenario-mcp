# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-05-31

### Added
- **XSD Validation**: Full W3C-conformant XSD validation using Uppsala pure-Rust validator
  - Lazy-loaded schema caching with `std::sync::OnceLock` (stdlib, zero external dependencies for initialization)
  - **Strict validation**: Fails without official XSD files (no graceful fallback)
  - Separate `warnings` field in `ValidationReport` (reserved for future use)
- **Vehicle Categories**: Added 3 missing categories to `VehicleCategory` enum
  - `Semitrailer` - Articulated trailer
  - `Train` - Railway vehicle
  - `Tram` - Streetcar
- **Position Types**: Added `Route` variant to `Position` enum for route-based positioning
  - ⚠️ **UNVERIFIED**: Not yet tested against official ASAM XSD - use with caution
- **Parameter Types**: Added 3 new types to `ParameterType` enum
  - `UnsignedInt` - Unsigned integer
  - `UnsignedShort` - Unsigned short integer
  - `DateTime` - Date and time values
- **API**: `VehicleCategory` now re-exported in crate root for ergonomics
- **Performance**: Optimized XML serialization with `itoa`/`ryu` for zero-allocation number formatting

### Changed
- **BREAKING**: `VehicleCategory`, `Position`, and `ParameterType` enums now marked `#[non_exhaustive]`
  - **Migration**: Add wildcard pattern `_ => {}` to exhaustive matches on these enums
  - Example:
    ```rust
    // Before (breaks in 0.2.0):
    match category {
        VehicleCategory::Car => {},
        VehicleCategory::Van => {},
        // ... only 7 variants
    }
    
    // After (works in 0.2.0+):
    match category {
        VehicleCategory::Car => {},
        VehicleCategory::Van => {},
        // ... handle variants you care about
        _ => {} // Required with #[non_exhaustive]
    }
    ```
- **BREAKING**: `ValidationReport` now has `warnings: Vec<String>` field
  - **Migration**: Update struct literal construction:
    ```rust
    // Before:
    ValidationReport { valid: true, errors: vec![] }
    
    // After:
    ValidationReport { valid: true, errors: vec![], warnings: vec![] }
    ```
- **BREAKING**: Validation now **fails without XSD files** (strict mode)
  - Previous: `valid: true` with warning when XSD missing
  - Now: `valid: false` with error message
  - **This is the primary breaking change**: Runtime behavior changed
  - **Migration**: Ensure official ASAM XSD files are installed before validation
  - Run `./check-schemas.sh` for setup instructions
- XML serialization now always emits `<Properties></Properties>` element (XSD compliance)
- Replaced external `lazy_static` dependency with stdlib `std::sync::OnceLock` (Rust 1.70+)

### Fixed
- Properties element now always present in Vehicle/Pedestrian/MiscObject (XSD requirement)
- Vehicle category attribute correctly parsed from XML
- Parameter types correctly serialized to XSD-compliant strings
- Reduced XML serialization allocations by 90%+ using specialized number formatters

### Breaking Changes Summary

**Runtime-Breaking (MOST IMPORTANT)**:
- **Validation behavior**: Without XSD files, `valid` is now `false` instead of `true`
  - This will break CI/CD pipelines that rely on validation passing without XSD
  - **Action required**: Install official ASAM XSD files or update validation checks

**Source-Breaking (Compile Errors)**:
- **VehicleCategory**: 7 → 10 variants (+Semitrailer, Train, Tram) + `#[non_exhaustive]`
- **Position**: 7 → 8 variants (+Route, unverified) + `#[non_exhaustive]`
- **ParameterType**: 4 → 7 variants (+UnsignedInt, UnsignedShort, DateTime) + `#[non_exhaustive]`
- **ValidationReport**: Added `warnings: Vec<String>` field

All three enums now require wildcard patterns due to `#[non_exhaustive]`.

**API Changes**:
- `ValidationReport` gained `warnings` field
- `VehicleCategory::as_xml_str()` method added (public API)
- Validation behavior changed from lenient to strict

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

### 1. **CRITICAL: Update Validation Behavior**

**Validation now requires XSD files** (strict mode):

```rust
// Before: Validation passes with warning when XSD missing
let report = validator.validate(xml);
assert!(report.valid); // ✅ true even without XSD

// After: Validation fails without XSD
let report = validator.validate(xml);
assert!(!report.valid); // ❌ false, error message tells you to install XSD
assert!(report.errors[0].contains("XSD schema not available"));
```

**Action**: Install official ASAM XSD files. Run `./check-schemas.sh` for instructions.

**Why this matters**: This is a **runtime breaking change**. If your CI/CD pipeline validates OpenSCENARIO files without XSD files present, it will now fail where it previously passed with a warning.

---

### 2. Update Enum Matches (Add Wildcards)

**All three public enums** now require wildcard patterns:

```rust
// VehicleCategory
match vehicle.category {
    VehicleCategory::Car => { /* ... */ },
    VehicleCategory::Truck => { /* ... */ },
    _ => { /* handle other categories */ }  // ← REQUIRED
}

// Position  
match position {
    Position::World { .. } => { /* ... */ },
    Position::Lane { .. } => { /* ... */ },
    _ => { /* handle other positions */ }  // ← REQUIRED
}

// ParameterType
match param_type {
    ParameterType::Integer => { /* ... */ },
    ParameterType::String => { /* ... */ },
    _ => { /* handle other types */ }  // ← REQUIRED
}
```

**Why**: `#[non_exhaustive]` allows future spec updates without breaking your code.

---

### 3. Update ValidationReport Construction

```rust
// Before:
ValidationReport { valid: true, errors: vec![] }

// After: 
ValidationReport { 
    valid: true, 
    errors: vec![], 
    warnings: vec![]  // ← NEW FIELD
}
```

---

### 4. Check ValidationReport Warnings

```rust
let report = validator.validate(xml);

// Check both errors and warnings:
if !report.errors.is_empty() {
    eprintln!("Errors: {:?}", report.errors);
}
if !report.warnings.is_empty() {
    eprintln!("Warnings: {:?}", report.warnings);
}
```

---

### 5. Update Imports (Optional)

You can now import `VehicleCategory` from crate root:

```rust
// Before:
use openscenario::entities::VehicleCategory;

// After (shorter):
use openscenario::VehicleCategory;
```

---

### 6. ⚠️ Route Position Warning

If using `Position::Route`:

```rust
match position {
    Position::Route { route_ref, s } => {
        // ⚠️ WARNING: Route position has NOT been verified against 
        // official ASAM XSD schema. XML structure may be incorrect.
        // Use with caution until verified.
    }
    _ => {}
}
```

---

## Summary

**Key Breaking Changes**:
1. 🚨 **Validation fails without XSD** (was: passed with warning)
2. Three enums require `_ =>` wildcard patterns
3. `ValidationReport` has new `warnings` field

**Time to Migrate**: ~30 minutes for typical codebase

**Why 0.2.0?**: Multiple breaking changes per semver 0.x rules

---

[0.2.0]: https://github.com/jakeaboganda/osc-mcp/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jakeaboganda/osc-mcp/releases/tag/v0.1.0
