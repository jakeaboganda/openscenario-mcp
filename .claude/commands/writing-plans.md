---
name: writing-plans
description: Use when you have a spec or requirements for a multi-step task, before touching code
---

# Writing Plans

## Overview

Create thorough implementation plans for engineers with minimal codebase familiarity. Document file locations, code samples, testing approaches, and commit boundaries. Apply DRY, YAGNI, TDD principles with frequent commits.

**Announcement requirement:** "I'm using the writing-plans skill to create the implementation plan."

**Plan storage location:** `docs/superpowers/plans/YYYY-MM-DD-<feature-name>.md`
(User preferences override this default)

## Scope Check

If specs cover multiple independent subsystems, suggest breaking into separate plans — one per subsystem — each producing independently testable software.

## File Structure

Map created or modified files before defining tasks:

- Design units with clear boundaries and focused interfaces
- Each file maintains one clear responsibility
- Related files live together; split by responsibility, not technical layer
- Follow existing codebase patterns
- Include file splits in plans when existing files become unwieldy

## Task Right-Sizing

A task represents the smallest unit with its own test cycle, reviewable independently. Each task produces independently testable deliverables.

## Bite-Sized Task Granularity

**Each step takes 2-5 minutes:**
- Write the failing test
- Run it to confirm failure
- Implement minimal code for passing test
- Run tests to confirm passage
- Commit

## Plan Document Header

**Required format:**

```markdown
# [Feature Name] Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** [One sentence describing what this builds]

**Architecture:** [2-3 sentences about approach]

**Tech Stack:** [Key technologies/libraries]

## Global Constraints

[The spec's project-wide requirements — version floors, dependency limits,
naming and copy rules, platform requirements — one line each, with exact
values copied verbatim from the spec. Every task's requirements implicitly
include this section.]

---
```

## Task Structure

````markdown
### Task N: [Component Name]

**Files:**
- Create: `exact/path/to/file.rs`
- Modify: `exact/path/to/existing.rs:123-145`
- Test: `tests/exact/path/to/test.rs`

**Interfaces:**
- Consumes: [what this task uses from earlier tasks — exact signatures]
- Produces: [what later tasks rely on — exact function names, parameter and return types]

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn test_specific_behavior() {
    let result = function(input);
    assert_eq!(result, expected);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test test_specific_behavior`
Expected: FAIL with "function not defined"

- [ ] **Step 3: Write minimal implementation**

```rust
pub fn function(input: T) -> U {
    expected
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test test_specific_behavior`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add tests/path/test.rs src/path/file.rs
git commit -m "feat: add specific feature"
```
````

## No Placeholders

Every step requires actual engineer-ready content. Avoid:
- "TBD", "TODO", "implement later", "fill in details"
- "Add appropriate error handling" / "handle edge cases" (without showing how)
- "Write tests for the above" (without test code)
- "Similar to Task N" (repeat the code)
- References to undefined types or functions

## Requirements

- Exact file paths always
- Complete code in every step where code changes
- Exact commands with expected output
- DRY, YAGNI, TDD, frequent commits

## Self-Review Checklist

After completing the plan:

**1. Spec coverage:** Can you identify which task implements each requirement? List gaps.

**2. Placeholder scan:** Search for "TBD", "TODO", incomplete sections. Fix inline.

**3. Type consistency:** Do types, signatures, and function names match across tasks?

Fix issues inline. Add tasks for unaddressed spec requirements.

## Execution Handoff

After saving the plan, offer execution options:

**"Plan complete and saved to `docs/superpowers/plans/<filename>.md`. Two execution options:**

**1. Subagent-Driven (recommended)** — Dispatch fresh subagent per task, review between tasks

**2. Inline Execution** — Execute tasks in this session with checkpoints

**Which approach?"**

**If Subagent-Driven chosen:**
- **REQUIRED SUB-SKILL:** Use superpowers:subagent-driven-development

**If Inline Execution chosen:**
- **REQUIRED SUB-SKILL:** Use superpowers:executing-plans
