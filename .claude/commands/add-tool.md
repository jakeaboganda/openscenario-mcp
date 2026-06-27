Scaffold a new MCP tool for this project following the established 5-step pattern.

The tool name is: $ARGUMENTS

If no name was given, ask for it before proceeding.

## Steps to execute

### 1. Read context first
Before writing any code, read:
- `openscenario-mcp/src/tools.rs` — to see the JSON schema pattern
- `openscenario-mcp/src/handlers.rs` — to see the handler pattern (pick a similar existing tool as a model)
- `openscenario-mcp/src/server.rs` — to see how tools are registered

### 2. Ask clarifying questions if needed
- What does this tool do? (if not obvious from the name)
- What parameters does it take?
- Does it need new core library functionality, or does it compose existing methods?
- Which existing tool is most similar in shape?

### 3. Implement in this order

**a) Handler** — add `handle_<tool_name>` function in `openscenario-mcp/src/handlers.rs`
- Follow the existing pattern: extract params from `serde_json::Value`, lock `ServerState`, call core library, return `serde_json::Value`
- Return descriptive error strings (these surface to the AI)

**b) Schema** — add tool definition in `openscenario-mcp/src/tools.rs`
- Follow the existing JSON schema pattern
- Parameter descriptions should be clear enough for an AI to use without docs

**c) Registration** — add the tool to the match arm in `openscenario-mcp/src/server.rs`

**d) Core library** (if needed) — if the tool requires new core functionality:
  1. Add enum variant in `openscenario/src/storyboard.rs`
  2. Add public method in `openscenario/src/scenario.rs` with fail-fast validation
  3. Add XML serialization in `openscenario/src/xml.rs`
  4. Add rustdoc with a compilable example

**e) Tests** — write tests in `openscenario-mcp/tests/` (and `openscenario/tests/` if core library changed)
  - Minimum: happy path, missing required param, invalid param value
  - If core library changed: 9–12 tests per new feature

### 4. Verify
Run `cargo build --workspace` and `cargo test --workspace` and fix any issues before reporting done.

### 5. Report
Summarize: what was added, which files changed, what tests were written.
