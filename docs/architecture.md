# Architecture Guide

Internal architecture of the osc-mcp OpenSCENARIO library.

---

## Project Structure

Two-crate workspace:

```
openscenario/           # Core library — domain model, XML, validation
openscenario-mcp/       # MCP server — 31 tool handlers, templates, import/inspect
```

### Core library (`openscenario/src/`)

| File | Responsibility |
|------|---------------|
| `scenario.rs` | Builder API — the only file users interact with directly |
| `storyboard.rs` | All story/action/condition/trigger types |
| `entities.rs` | Vehicle, Pedestrian, MiscObject, BoundingBox |
| `position.rs` | World, Lane, Road, and Relative* position types |
| `error.rs` | All error variants (`#[non_exhaustive]`) |
| `xml.rs` | XML serialization — the only file that knows about XML |
| `validation.rs` | XSD validation via Uppsala (lazy-loaded schemas) |
| `opendrive_validator.rs` | OpenDRIVE road network parsing and lane queries |
| `catalog.rs` | Read-only catalog loading (Vehicle/Pedestrian/MiscObject) |
| `parser.rs` | Round-trip XML parsing for scenario import |
| `version.rs` | `OpenScenarioVersion` enum |

### MCP server (`openscenario-mcp/src/`)

| File | Responsibility |
|------|---------------|
| `server.rs` | `ServerState`, tool schema registration, request routing |
| `handlers.rs` | Primitive tool handlers (entity, position, action, trigger, road tools) |
| `scenario_templates.rs` | One-shot template tools: lane_change, cutin, platoon, quick |
| `import.rs` | `import_scenario` handler (.xosc → Scenario) |
| `inspection.rs` | `list/inspect/describe/check_scenario` handlers |
| `tools.rs` | JSON schema definitions |
| `main.rs` | Binary entry point (JSON-RPC 2.0 over stdio) |

---

## Key Design Rules

**Validation**: Fail-fast at construction time. Every public `&mut self` method returns `Result<()>`. Errors are never deferred to XML export.

**XML isolation**: `xml.rs` is the only file that serializes XML. `Scenario` is a pure domain model with no XML knowledge.

**Ordered vs named collections**: Stories and Acts use `HashMap` (lookup by name). Maneuvers and Events use `Vec` (order is semantically significant).

**Trigger logic**: `Trigger` = OR of `ConditionGroup`s; each `ConditionGroup` = AND of `Condition`s.

**Error messages**: Every error includes the failing value and, where applicable, a list of valid alternatives.

**MCP state**: All scenarios live in a single `Arc<Mutex<ServerState>>` for the process lifetime. A road network must be loaded before scenario creation.

---

## Extension Points

### New action or condition type (5 steps)

1. Add struct + enum variant in `storyboard.rs`
2. Add public method with fail-fast validation in `scenario.rs`
3. Add XML serialization case in `xml.rs`
4. Write 9–12 tests in `openscenario/tests/<feature>_tests.rs`
5. Add rustdoc with a compilable example on the public method

### New MCP tool (4 steps)

1. Implement handler in `handlers.rs` (or a new module for a new category)
2. Add JSON schema in `server.rs` → `register_tools()`
3. Add routing arm in `server.rs` → `handle_call_tool()`
4. Write tests in `openscenario-mcp/tests/`

---

## Supported Versions

OpenSCENARIO: 1.0, 1.1, 1.2 (structural differences handled in `xml.rs` and `validation.rs`)
