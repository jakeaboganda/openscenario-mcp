# osc-mcp

Rust MCP server that lets AI assistants generate OpenSCENARIO `.xosc` test scenarios for autonomous vehicle simulation. Claude Desktop is the primary interface.

## Build & Test

```bash
cargo build --release          # MCP server binary → target/release/openscenario-mcp
cargo test --workspace         # all tests (436 total)
cargo clippy -- -D warnings    # CI enforces zero warnings
cargo fmt                      # CI enforces formatting
```

The binary speaks JSON-RPC 2.0 over stdio. Logs go to stderr; stdout is reserved for MCP protocol.

## Architecture

Two-crate workspace:

| Crate | Purpose | LOC |
|-------|---------|-----|
| `openscenario/` | Core library: Scenario builder, XML serialization, XSD validation, all domain types | ~1,800 |
| `openscenario-mcp/` | MCP server: 23 tool handlers, templates, import/inspect | ~4,100 |

### Key files

```
openscenario/src/
  scenario.rs           main builder API (136 public methods)
  storyboard.rs         Story → Act → ManeuverGroup → Maneuver → Event → Action/Condition types
  xml.rs                XML serialization, version-aware (OSC 1.0–1.3)
  validation.rs         XSD validation (Uppsala, pure Rust)
  opendrive_validator.rs road network validation

openscenario-mcp/src/
  handlers.rs           all 23 MCP tool implementations
  tools.rs              tool JSON schema definitions
  server.rs             ServerState, request routing, tool registration
  scenario_templates.rs template builders (lane_change, merge, cutin, platoon)
```

## Code Conventions

**Validation**: Fail-fast at construction time. Every public `&mut self` method returns `Result<()>`. Never defer validation to export.

**Error messages**: Include context — what failed, the specific value, and available alternatives. See `error.rs` for the pattern.

**Types**: Prefer enums with data over strings/ints for categorical values. Each enum variant carries its own parameters.

**XML isolation**: `xml.rs` is the only file that knows about XML. `Scenario` is a pure domain model.

**Testing**: TDD. Write 9–12 tests per feature before implementing: basic case, each variant/condition, invalid inputs, XML output check.

**Comments**: Only for non-obvious WHY — hidden constraints, workarounds, subtle invariants. Never describe what the code does.

## Adding a New MCP Tool

1. Add the handler function in `openscenario-mcp/src/handlers.rs`
2. Add the JSON schema in `openscenario-mcp/src/tools.rs`
3. Register the tool in `openscenario-mcp/src/server.rs`
4. Write tests in `openscenario-mcp/tests/`
5. If the tool calls the core library with new functionality → see "Adding a Core Library Feature" below

## Adding a Core Library Feature (Action / Condition / etc.)

1. Add enum variant in `openscenario/src/storyboard.rs`
2. Add public method in `openscenario/src/scenario.rs` with fail-fast validation
3. Add XML serialization in `openscenario/src/xml.rs`
4. Write 9–12 tests in `openscenario/tests/<feature>_tests.rs`
5. Add rustdoc with a compilable example on the public method

## Current Development Focus

**Scenario validation feedback loop** — the primary long-term investment.

Goal: AI interprets XSD and structural validation errors returned by `validate_scenario` / `validate_scenario_structure` and issues corrective tool calls, turning hard errors into a self-healing loop.

Relevant entry points:
- `openscenario/src/validation.rs` — `XsdValidator`, `ValidationReport`
- `openscenario/src/opendrive_validator.rs` — road network constraints
- `openscenario-mcp/src/handlers.rs` — `handle_validate_scenario`, `handle_validate_scenario_structure`

## Constraints

- A road network must be loaded (`load_road_network` or `get_real_world_road`) before any scenario can be created.
- `get_real_world_road` requires SUMO/netconvert installed at runtime.
- XSD strict mode: validation requires the official ASAM XSD files in `openscenario/schemas/`.
- `RUST_LOG=info` for normal operation; `debug`/`trace` for verbose output.
