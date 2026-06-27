---
name: writing-skills
description: Use when creating new skills, editing existing skills, or verifying skills work before deployment
---

# Writing Skills

## Overview

**Writing skills IS Test-Driven Development applied to process documentation.**

You write test cases (pressure scenarios with subagents), watch them fail (baseline behavior), write the skill (documentation), watch tests pass (agents comply), and refactor (close loopholes).

**Core principle:** "If you didn't watch an agent fail without the skill, you don't know if the skill teaches the right thing."

**REQUIRED BACKGROUND:** You MUST understand superpowers:test-driven-development before using this skill.

## What is a Skill?

A **skill** is a reference guide for proven techniques, patterns, or tools that help future agents find and apply effective approaches.

**Skills are:** Reusable techniques, patterns, tools, reference guides

**Skills are NOT:** Narratives about how you solved a problem once

## TDD Mapping for Skills

| TDD Concept | Skill Creation |
|-------------|----------------|
| Test case | Pressure scenario with subagent |
| Production code | Skill document (SKILL.md) |
| Test fails (RED) | Agent violates rule without skill (baseline) |
| Test passes (GREEN) | Agent complies with skill present |
| Refactor | Close loopholes while maintaining compliance |

## When to Create a Skill

**Create when:**
- Technique wasn't intuitively obvious to you
- You'd reference this again across projects
- Pattern applies broadly (not project-specific)
- Others would benefit

**Don't create for:**
- One-off solutions
- Standard practices well-documented elsewhere
- Project-specific conventions (put in CLAUDE.md)
- Mechanical constraints (if enforceable with regex/validation, automate it)

## SKILL.md Structure

**Frontmatter (YAML):**
- Required fields: `name` and `description`
- `name`: letters, numbers, and hyphens only
- `description`: Third-person, starts with "Use when..." — describes ONLY triggering conditions, NOT what the skill does

```markdown
---
name: skill-name
description: Use when [specific triggering conditions and symptoms]
---

# Skill Name

## Overview
What is this? Core principle in 1-2 sentences.

## When to Use
Bullet list with symptoms and use cases. When NOT to use.

## Core Pattern
Code examples or step-by-step process.

## Quick Reference
Table or bullets for scanning.

## Common Mistakes
What goes wrong + fixes.
```

## Critical: Description = When to Use, NOT What the Skill Does

The description should ONLY describe triggering conditions. Do NOT summarize the skill's process or workflow.

**Why this matters:** When a description summarizes the skill's workflow, an agent may follow the description instead of reading the full skill content — taking shortcuts through the process.

```yaml
# BAD: Summarizes workflow
description: Use when executing plans - dispatches subagent per task with code review between tasks

# GOOD: Just triggering conditions
description: Use when executing implementation plans with independent tasks
```

## The Iron Law (Same as TDD)

```
NO SKILL WITHOUT A FAILING TEST FIRST
```

Write skill before testing? Delete it. Start over.

## RED-GREEN-REFACTOR for Skills

### RED: Write Failing Test (Baseline)
Run pressure scenario with subagent WITHOUT the skill. Document exact behavior and rationalizations used.

### GREEN: Write Minimal Skill
Write skill addressing those specific rationalizations. Run same scenarios WITH skill. Agent should now comply.

### REFACTOR: Close Loopholes
Agent found new rationalization? Add explicit counter. Re-test until bulletproof.

## Bulletproofing Against Rationalization

For discipline-enforcing skills (rules/requirements):

**Close every loophole explicitly:**
```markdown
Write code before test? Delete it. Start over.

**No exceptions:**
- Don't keep it as "reference"
- Don't "adapt" it while writing tests
- Delete means delete
```

**Address "Spirit vs Letter" arguments:**
```markdown
**Violating the letter of the rules is violating the spirit of the rules.**
```

**Build a rationalization table** from baseline testing — every excuse agents make goes in the table.

**Create a red flags list** so agents can self-check when rationalizing.

## Match the Form to the Failure

| Baseline failure | Right form | Wrong form |
|---|---|---|
| Skips rule under pressure | Prohibition + rationalization table + red flags | Soft guidance |
| Output has wrong shape | Positive recipe: state what the output IS | Prohibition list |
| Omits required element | Structural: REQUIRED field in template | Prose reminders |
| Behavior depends on condition | Conditional keyed to observable predicate | Unconditional rule |

## Common Mistakes

**Narrative example** — "In session X, we found..." — Too specific, not reusable.

**Multi-language dilution** — same example in 5 languages — mediocre quality, maintenance burden.

**Generic labels** — helper1, step3 — Labels should have semantic meaning.

**Sediment** — stale layers that settle because adding feels safe. Prune regularly.

## Skill Creation Checklist

**RED Phase:**
- [ ] Create pressure scenarios (3+ combined pressures for discipline skills)
- [ ] Run scenarios WITHOUT skill — document baseline behavior verbatim
- [ ] Identify patterns in rationalizations/failures

**GREEN Phase:**
- [ ] `name` uses only letters, numbers, hyphens
- [ ] `description` starts with "Use when..." — triggering conditions only, no workflow summary
- [ ] Keywords for search (errors, symptoms, tools)
- [ ] Address specific baseline failures identified in RED
- [ ] Run scenarios WITH skill — verify agents now comply

**REFACTOR Phase:**
- [ ] Identify NEW rationalizations from testing
- [ ] Add explicit counters
- [ ] Build rationalization table from all test iterations
- [ ] Create red flags list
- [ ] Re-test until bulletproof
