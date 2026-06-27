Print the following cheat sheet for this project, formatted cleanly. Do not add commentary beyond what is listed.

---

## osc-mcp superpowers

Things you can ask me to do in this project:

### Code generation
- **Add a new MCP tool** — `/add-tool <tool-name>`: scaffolds handler, schema, registration, and test stubs following the project pattern
- **Add a new action/condition type** — describe it and I'll implement all 5 steps (enum variant, method, XML, tests, rustdoc)
- **Add a scenario template** — give me a scenario type and I'll implement it in `scenario_templates.rs`

### Review & quality
- **`/grill-me`** — I'll ask hard design questions about whatever you're working on before you commit
- **`/code-review`** — review the current diff for bugs and simplification opportunities
- **`/simplify`** — apply reuse and efficiency cleanups to changed code

### Validation feedback (long-term focus)
- Analyze a `ValidationReport` output and explain what went wrong in plain English
- Suggest which MCP tool calls would fix a given validation error
- Design the error→correction mapping for the auto-repair loop

### Debugging
- Diagnose a failing test — paste the output and I'll find the root cause
- Explain an XSD validation error — paste the `ValidationReport` and the generated XML
- Trace a JSON-RPC request through the handler stack

### Exploration
- Explain any module or function: "what does `opendrive_validator.rs` do?"
- Map which MCP tools call which core library methods
- Show the call chain from a tool name to XML output

### Project tasks
- `cargo test --workspace` — run and interpret the full test suite
- Check what tests exist for a given feature
- Show me all tools that touch `ServerState`

### Onboarding
- Explain the Story→Act→ManeuverGroup→Maneuver→Event→Action hierarchy
- Explain the 7 position types and when to use each
- Walk through how a scenario goes from tool call to `.xosc` file
