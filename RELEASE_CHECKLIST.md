# 0.2.0 Release Checklist

**Status**: 🔴 **BLOCKED** - Critical issues must be resolved

---

## ✅ Pre-Release Checklist

### Phase 1: Version & History (CRITICAL)
- [ ] **Decide**: Which commit represents actual 0.2.0?
  - [ ] Option A: Tag `73028ef` as v0.2.0 (simpler, reflects final decisions)
  - [ ] Option B: Tag `0c48976` as v0.2.0, bump `73028ef` to v0.3.0
  - [ ] Option C: Squash both commits, create clean v0.2.0
  - **Recommendation**: Option A (tag `73028ef`)

- [ ] **Git Tags**: Apply chosen version strategy
  ```bash
  git tag v0.2.0 73028ef
  git push origin v0.2.0
  ```

### Phase 2: CHANGELOG.md Fixes (CRITICAL)
- [ ] **Fix contradictions**:
  - [ ] Change "Lazy-loaded schema caching with `std::sync::OnceLock`" → "with `lazy_static`"
  - [ ] Remove references to `#[non_exhaustive]` (it's not present in final code)
  - [ ] Update "Removed lazy_static dependency" → "Uses lazy_static dependency"

- [ ] **Add missing sections**:
  ```markdown
  ### Breaking Changes Summary
  
  **Behavioral Breaking Changes** (NEW SECTION):
  - Validation now fails without XSD files (`valid: false` instead of `valid: true` + warning)
    - **Impact**: Code checking `if report.valid` may now reject scenarios
    - **Mitigation**: Install ASAM XSD files OR handle XSD-missing error explicitly
  
  **Unverified Features** (NEW SECTION):
  - `Position::Route` variant not tested against official ASAM XSD
    - **Status**: Use with caution; structure may change
    - **See**: `position.rs` doc comment for warning
  ```

- [ ] **Add migration example**:
  ```markdown
  ### 2. Update ValidationReport Struct Literals
  
  \`\`\`rust
  // Before (0.1.x):
  ValidationReport {
      valid: true,
      errors: vec![],
  }
  
  // After (0.2.0):
  ValidationReport {
      valid: true,
      errors: vec![],
      warnings: vec![],  // NEW REQUIRED FIELD
  }
  \`\`\`
  ```

- [ ] **Add design decisions section** (optional but recommended):
  ```markdown
  ### Design Decisions in 0.2.0
  
  **Strict Validation**: Validation now fails hard without XSD files (no graceful fallback)
  - Prevents false confidence in validation results
  - Forces proper XSD setup for production use
  
  **No #[non_exhaustive]**: Public enums are NOT marked `#[non_exhaustive]`
  - OpenSCENARIO spec versions are stable (1.0/1.1/1.2 finalized)
  - Breaking changes acceptable in 0.x series per semver
  - Will re-evaluate for 1.0 release
  
  **lazy_static Dependency**: Uses `lazy_static` for validator caching
  - Stable, well-tested dependency
  - Simpler than custom initialization logic
  ```

### Phase 3: Documentation Improvements (HIGH PRIORITY)
- [ ] **Add API contract to `validation.rs`**:
  ```rust
  /// Validation report containing results and any errors.
  ///
  /// # Invariants
  /// - `valid` is `true` if and only if `errors` is empty
  /// - `warnings` do not affect validity (informational only)
  /// - Empty `errors` with non-empty `warnings` indicates valid-but-with-issues
  ///
  /// # Strict Validation Mode (0.2.0+)
  /// Without official ASAM XSD files, validation fails with:
  /// - `valid: false`
  /// - `errors: ["XSD schema not available..."]`
  ///
  /// Install XSD files using `./check-schemas.sh` before validation.
  pub struct ValidationReport {
      pub valid: bool,
      pub errors: Vec<String>,
      pub warnings: Vec<String>,
  }
  ```

- [ ] **Add 0.x policy to README.md**:
  ```markdown
  ## Versioning & Stability (0.x series)
  
  This project follows [Semantic Versioning](https://semver.org/):
  - **0.x.y**: Pre-1.0 development - breaking changes allowed in MINOR versions
  - **1.x.y**: Stable API - breaking changes require MAJOR version bump
  
  **In 0.x releases**:
  - MINOR (0.2.0, 0.3.0): May include breaking changes
  - PATCH (0.2.1, 0.2.2): Bug fixes only, no breaking changes
  
  Before 1.0, expect:
  - New enum variants in MINOR versions (breaks exhaustive matches)
  - New struct fields in MINOR versions (breaks struct literals)
  - Behavioral changes in MINOR versions (document carefully)
  ```

### Phase 4: Code Improvements (MEDIUM PRIORITY)
- [ ] **Consider adding ValidationMode** (optional):
  ```rust
  pub enum ValidationMode {
      /// Fail validation without XSD files (default, 0.2.0+)
      Strict,
      /// Warn but pass validation without XSD (deprecated, for migration)
      #[deprecated(note = "Use Strict mode; will be removed in 0.3.0")]
      Lenient,
  }
  
  impl XsdValidator {
      pub fn new_with_mode(version: String, mode: ValidationMode) -> Self {
          Self { version, mode }
      }
  }
  ```

- [ ] **Consider deprecating Route position**:
  ```rust
  pub enum Position {
      // ... other variants
      
      /// Position along a predefined route
      ///
      /// ⚠️ **UNVERIFIED**: Not tested against official ASAM XSD schema.
      #[deprecated(
          note = "Unverified against official XSD; may change structure in future. \
                  See tracking issue #XXX"
      )]
      Route {
          route_ref: String,
          s: f64,
      },
  }
  ```

- [ ] **Reconsider lazy_static → OnceLock**:
  - OnceLock is stdlib (Rust 1.70+, released June 2023)
  - Project uses Rust 2021 edition (implies 1.56+)
  - **Check**: What is project's MSRV?
  - If MSRV ≥ 1.70: Prefer OnceLock (zero deps)
  - If MSRV < 1.70: Keep lazy_static (document MSRV)

### Phase 5: Testing & Verification (BEFORE TAG)
- [ ] **Run full test suite**:
  ```bash
  cargo test --workspace
  # Expected: 134/134 passing
  ```

- [ ] **Verify XSD validation**:
  ```bash
  ./check-schemas.sh  # Ensure schemas present
  cargo test --test catalog_validation_e2e
  cargo test --test validation_tests
  ```

- [ ] **Check for unintentional changes**:
  ```bash
  git diff v0.1.0..HEAD -- openscenario/src/lib.rs
  # Verify only expected exports changed
  ```

- [ ] **Build documentation**:
  ```bash
  cargo doc --workspace --no-deps --open
  # Manually review:
  # - VehicleCategory, Position, ParameterType enums
  # - ValidationReport struct
  # - XsdValidator::validate() method
  ```

### Phase 6: Release Execution
- [ ] **Update Cargo.toml versions**:
  ```bash
  # Already done: workspace version = 0.2.0
  grep version Cargo.toml
  grep version openscenario/Cargo.toml
  ```

- [ ] **Create GitHub release**:
  - Tag: `v0.2.0`
  - Title: `OpenSCENARIO 0.2.0 - XSD Validation & Enum Extensions`
  - Body: Copy from CHANGELOG.md (0.2.0 section)
  - Attach: `BREAKING_CHANGES_AUDIT.md` (for maintainer reference)

- [ ] **Publish to crates.io** (if applicable):
  ```bash
  cd openscenario
  cargo publish --dry-run
  cargo publish
  ```

- [ ] **Announce** (if applicable):
  - GitHub Discussions
  - Project Discord/Slack
  - OpenSCENARIO community forums

---

## 🔍 Verification Script

Save as `verify-0.2.0.sh`:

```bash
#!/bin/bash
set -e

echo "🔍 Verifying 0.2.0 release readiness..."

# Check version in Cargo.toml
echo "✅ Checking version..."
grep -q 'version = "0.2.0"' Cargo.toml || { echo "❌ Workspace version not 0.2.0"; exit 1; }
grep -q 'version = "0.2.0"' openscenario/Cargo.toml || { echo "❌ openscenario version not 0.2.0"; exit 1; }

# Check CHANGELOG mentions 0.2.0
echo "✅ Checking CHANGELOG..."
grep -q '## \[0.2.0\]' CHANGELOG.md || { echo "❌ CHANGELOG missing 0.2.0 section"; exit 1; }

# Check for lazy_static (not OnceLock)
echo "✅ Checking dependencies..."
grep -q 'lazy_static' openscenario/Cargo.toml || { echo "❌ lazy_static missing"; exit 1; }
grep -q 'OnceLock' openscenario/src/validation.rs && { echo "❌ OnceLock still referenced"; exit 1; }

# Check enums NOT #[non_exhaustive]
echo "✅ Checking enum attributes..."
grep -A5 'pub enum VehicleCategory' openscenario/src/entities.rs | grep -q '#\[non_exhaustive\]' && {
    echo "❌ VehicleCategory should NOT be #[non_exhaustive]"
    exit 1
}

# Run tests
echo "✅ Running tests..."
cargo test --workspace --quiet || { echo "❌ Tests failed"; exit 1; }

# Check test count
TEST_COUNT=$(cargo test --workspace 2>&1 | grep -oP '\d+(?= passed)' | tail -1)
if [ "$TEST_COUNT" != "134" ]; then
    echo "⚠️  Warning: Expected 134 tests, got $TEST_COUNT"
fi

echo ""
echo "✅ All checks passed!"
echo "📦 Ready to tag v0.2.0"
```

Run before tagging:
```bash
chmod +x verify-0.2.0.sh
./verify-0.2.0.sh
```

---

## 📊 Status Tracker

| Task | Status | Priority | Blocker? |
|------|--------|----------|----------|
| Choose version strategy | ⬜ TODO | CRITICAL | ✅ YES |
| Fix CHANGELOG contradictions | ⬜ TODO | CRITICAL | ✅ YES |
| Add ValidationReport migration | ⬜ TODO | CRITICAL | ✅ YES |
| Document Route unverified | ⬜ TODO | HIGH | ⚠️ YES |
| Add behavioral breaking change | ⬜ TODO | HIGH | ⚠️ SOFT |
| Add API contract docs | ⬜ TODO | MEDIUM | ❌ NO |
| Add 0.x policy to README | ⬜ TODO | MEDIUM | ❌ NO |
| Consider ValidationMode | ⬜ TODO | LOW | ❌ NO |
| Reconsider lazy_static | ⬜ TODO | LOW | ❌ NO |
| Run verification script | ⬜ TODO | CRITICAL | ✅ YES |
| Create GitHub release | ⬜ TODO | CRITICAL | ✅ YES |

---

## 🚀 Quick Path to Release (Minimum Viable)

If time is constrained, this minimal set unblocks release:

1. **Tag `73028ef` as v0.2.0** (5 minutes)
2. **Fix CHANGELOG contradictions** (15 minutes)
   - Change OnceLock → lazy_static
   - Remove #[non_exhaustive] references
3. **Add ValidationReport migration example** (10 minutes)
4. **Add Route unverified note to CHANGELOG** (5 minutes)
5. **Run verification script** (2 minutes)
6. **Create GitHub release** (10 minutes)

**Total time**: ~45 minutes

---

## 📞 Questions to Answer

Before proceeding, answer these:

1. **Which commit is the actual release?**
   - [ ] `0c48976` (OnceLock, #[non_exhaustive])
   - [ ] `73028ef` (lazy_static, no #[non_exhaustive]) ← **Recommended**
   - [ ] Squash and re-commit

2. **Is Position::Route ready for release?**
   - [ ] Yes, ship with UNVERIFIED warning
   - [ ] No, remove and defer to 0.3.0
   - [ ] Yes, but deprecate until verified

3. **Should we keep lazy_static or revert to OnceLock?**
   - [ ] Keep lazy_static (need to justify)
   - [ ] Revert to OnceLock (remove external dep)
   - [ ] Check MSRV first

4. **Is strict validation non-negotiable?**
   - [ ] Yes, enforce XSD requirement
   - [ ] No, add ValidationMode for gradual migration
   - [ ] Add feature flag for lenient mode

5. **When is the release deadline?**
   - [ ] Today (fast-track minimum viable)
   - [ ] This week (address all HIGH priority)
   - [ ] No rush (address everything properly)

---

**Once all CRITICAL tasks are ✅, proceed with release!**

---

Generated: 2026-05-31  
Source: `BREAKING_CHANGES_AUDIT.md`
