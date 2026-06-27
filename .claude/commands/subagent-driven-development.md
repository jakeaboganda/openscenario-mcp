---
name: subagent-driven-development
description: Use when executing implementation plans with independent tasks — dispatches a fresh subagent per task with review between tasks and a final whole-branch review
---

# Subagent-Driven Development

## Overview

Execute implementation plans by dispatching fresh implementer subagents per task, reviewing each task's output, and conducting a comprehensive final review.

**Core principle:** Fresh subagent per task + task review (spec + quality) + broad final review = high quality, fast iteration.

**Announce at start:** "I'm using the subagent-driven-development skill to execute this plan."

## Process Flow

1. **Pre-flight scan** — read the plan for contradictions before Task 1 dispatches
2. **Per-task cycle:** implementer dispatch → answer questions/provide context → implementation with testing → generate review package → task review → fix if needed → mark complete
3. **Continue without pausing** between tasks until all complete
4. **Final review** — dispatch whole-branch code review on the most capable available model
5. **Branch completion** — use finishing-a-development-branch skill

## Model Selection

- **Mechanical tasks** (isolated functions, 1-2 files, clear specs): cheapest tier
- **Integration work** (multi-file coordination, pattern matching): standard model
- **Architecture decisions**: most capable model
- **Final whole-branch review**: most capable tier, explicitly specified
- **Review tasks**: scale model to diff complexity and risk

**Key rule:** Turn count beats token price. Cheaper models often require 2-3× iterations on complex work, raising total cost despite lower per-token rates.

## Implementer Status Handling

Four outcomes trigger different responses:

**DONE** → generate a review package (diff from BASE to HEAD) before dispatching the task reviewer.

**DONE_WITH_CONCERNS** → read flagged doubts. Correctness/scope issues warrant pre-review resolution; observations proceed to review.

**NEEDS_CONTEXT** → provide missing information and re-dispatch.

**BLOCKED** → assess: context problems get re-dispatch with same model; reasoning-heavy tasks escalate to more capable models; oversized tasks subdivide; fundamentally wrong plans escalate to human review.

## Artifact Handling via Files

Rather than pasting artifacts into prompts:
- Extract task requirements into a brief file before implementer dispatch
- Name report files consistently: `task-N-brief.md` → `task-N-report.md`
- Generate review packages before task reviewer dispatch
- Task reviewers receive three inputs: brief, report, and review package diff
- Avoid pasting accumulated summaries into subsequent dispatches

## Reviewer Responsibilities

Task reviewers validate two dimensions: **spec compliance** AND **code quality**. Both verdicts required.

Do not pre-judge findings or instruct reviewers to ignore potential issues. Items marked "⚠️ Cannot verify from diff" don't block review but require controller verification.

## Progress Tracking

Use a durable ledger file to survive context compaction:
- Check `"$(git rev-parse --show-toplevel)/.superpowers/sdd/progress.md"` at skill start
- Append completion lines: "Task N: complete (commits `<base7>..<head7>`, review clean)"
- After compaction, trust git log and ledger over memory

## Red Flags

**Never:**
- Skip task reviews
- Proceed with unfixed Critical/Important issues
- Pre-judge reviewer findings or tell them what to ignore
- Dispatch multiple implementers simultaneously for the same task
- Make subagents read entire plans (use task briefs instead)
- Start implementation on primary branches without explicit user consent
- Trust agent success reports without verifying via VCS diff

**Always:**
- Re-review after fixes complete
- Provide diff files to reviewers
- Use file-based artifacts for handoffs
