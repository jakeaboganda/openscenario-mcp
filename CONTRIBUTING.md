# Contributing to osc-mcp

Thank you for your interest in contributing! This guide will help you understand the project structure and development workflow.

## Project Structure

```
osc-mcp/
├── openscenario/          # Core Rust library for OpenSCENARIO 1.2
│   ├── src/               # Library source code
│   ├── tests/             # Unit and integration tests
│   └── examples/          # Runnable scenario examples
│
├── openscenario-mcp/      # MCP (Model Context Protocol) server
│   └── src/               # MCP server implementation
│
├── opendrive/             # OpenDRIVE road network library
│   ├── src/               # Road geometry and lane logic
│   └── tests/             # Road network tests
│
├── esmini-tests/          # End-to-end testing with esmini
│   ├── scenarios/         # Generated test scenarios
│   └── *.py               # Python test runners
│
└── docs/                  # Documentation
    ├── *.md               # User guides
    └── archive/           # Historical design docs
```

## Development Workflow

### Building

```bash
# Build all crates
cargo build

# Build specific crate
cargo build -p openscenario
cargo build -p openscenario-mcp
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test -p openscenario
cargo test --test entity_tests

# Run with output
cargo test -- --nocapture
```

### Running Examples

```bash
# Generate scenario files
cargo run --example highway_merge
cargo run --example emergency_braking
cargo run --example lane_change_overtaking
cargo run --example platooning

# Output: *.xosc files in current directory
```

### Testing with esmini

See [`docs/using-with-esmini.md`](docs/using-with-esmini.md) for details.

```bash
# Visual playback
esmini --osc highway_merge.xosc

# Headless with CSV logging
esmini --osc highway_merge.xosc --headless --csv_logger output.csv
```

## Code Style

- **Rust**: Follow `rustfmt` defaults
- **Docs**: Every public API needs doc comments with examples
- **Tests**: Comprehensive unit tests for all new features
- **Examples**: Demonstrate real-world usage patterns

## Git Workflow

### Commit Messages

Follow conventional commits:

```
feat: add initial speed support
fix: correct lane change action ordering
test: add comprehensive initial state tests
docs: update README with new API methods
```

### Before Committing

```bash
# Format code
cargo fmt

# Run tests
cargo test

# Check for warnings
cargo clippy
```

## Adding New Features

### 1. Start with Tests (TDD)

```rust
#[test]
fn test_new_feature() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    // Test the desired behavior
    assert!(scenario.new_feature().is_ok());
}
```

### 2. Implement the Feature

Add to appropriate module in `openscenario/src/`.

### 3. Document with Examples

```rust
/// Sets the vehicle's initial speed.
///
/// # Examples
///
/// ```
/// use openscenario::Scenario;
///
/// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
/// scenario.set_initial_speed("ego", 30.0)?;
/// # Ok::<(), openscenario::ScenarioError>(())
/// ```
pub fn set_initial_speed(&mut self, entity: impl Into<String>, speed: f64) -> Result<()> {
    // Implementation
}
```

### 4. Add Integration Example

Create `openscenario/examples/feature_demo.rs` showing real-world usage.

### 5. Update Documentation

Add to relevant guide in `docs/`.

## File Hygiene

**Generated files** (not version-controlled):
- `*.xosc` - Scenario outputs from examples
- `*.csv` - esmini test data
- `target/` - Build artifacts
- `Cargo.lock` (for libraries)

**Documentation**:
- Active docs: `docs/*.md`
- Historical/planning: `docs/archive/`

## Testing Philosophy

### Unit Tests
- Test individual functions and methods
- Mock external dependencies
- Fast and focused

### Integration Tests
- Test complete workflows
- Use real scenario building
- Verify XML generation

### End-to-End Tests
- Test with esmini simulator
- Verify scenario execution
- Located in `esmini-tests/`

## Questions?

- Check existing issues on GitHub
- Read the documentation in `docs/`
- Look at examples in `openscenario/examples/`
- Review test patterns in `openscenario/tests/`

---

**Happy contributing!** 🦀🚗
