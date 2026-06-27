---
name: using-git-worktrees
description: Use when starting feature work that needs isolation from current workspace or before executing implementation plans - ensures an isolated workspace exists via native tools or git worktree fallback
---

# Using Git Worktrees

## Overview

Ensure work happens in an isolated workspace. Prefer your platform's native worktree tools. Fall back to manual git worktrees only when no native tool is available.

**Core principle:** Detect existing isolation first. Then use native tools. Then fall back to git. Never fight the harness.

**Announce at start:** "I'm using the using-git-worktrees skill to set up an isolated workspace."

## Step 0: Detect Existing Isolation

**Before creating anything, check if you are already in an isolated workspace.**

```bash
GIT_DIR=$(cd "$(git rev-parse --git-dir)" 2>/dev/null && pwd -P)
GIT_COMMON=$(cd "$(git rev-parse --git-common-dir)" 2>/dev/null && pwd -P)
BRANCH=$(git branch --show-current)
```

**Submodule guard:** `GIT_DIR != GIT_COMMON` is also true inside git submodules. Verify you are not in a submodule:

```bash
# If this returns a path, you're in a submodule, not a worktree — treat as normal repo
git rev-parse --show-superproject-working-tree 2>/dev/null
```

**If `GIT_DIR != GIT_COMMON` (and not a submodule):** You are already in a linked worktree. Skip to Step 2. Do NOT create another worktree.

**If `GIT_DIR == GIT_COMMON` (or in a submodule):** You are in a normal repo checkout.

Ask for consent before creating a worktree:

> "Would you like me to set up an isolated worktree? It protects your current branch from changes."

If the user declines, work in place and skip to Step 2.

## Step 1: Create Isolated Workspace

### 1a. Native Worktree Tools (preferred)

Do you have a way to create a worktree? It might be a tool named `EnterWorktree`, `WorktreeCreate`, a `/worktree` command, or a `--worktree` flag. If you do, use it and skip to Step 2.

Only proceed to Step 1b if you have no native worktree tool available.

### 1b. Git Worktree Fallback

**Only use this if Step 1a does not apply.**

**Directory Selection** (follow priority order):

1. Check your instructions for a declared worktree directory preference.
2. Check for existing project-local worktree directory: `.worktrees` (preferred) or `worktrees`
3. Default to `.worktrees/` at the project root.

**Safety Verification (project-local directories only):**

```bash
git check-ignore -q .worktrees 2>/dev/null || git check-ignore -q worktrees 2>/dev/null
```

**If NOT ignored:** Add to .gitignore, commit the change, then proceed.

**Create the Worktree:**

```bash
path="$LOCATION/$BRANCH_NAME"
git worktree add "$path" -b "$BRANCH_NAME"
cd "$path"
```

**Sandbox fallback:** If `git worktree add` fails with a permission error, tell the user the sandbox blocked worktree creation and you're working in the current directory instead.

## Step 2: Project Setup

Auto-detect and run appropriate setup:

```bash
if [ -f Cargo.toml ]; then cargo build; fi
if [ -f package.json ]; then npm install; fi
if [ -f requirements.txt ]; then pip install -r requirements.txt; fi
if [ -f go.mod ]; then go mod download; fi
```

## Step 3: Verify Clean Baseline

```bash
cargo test / npm test / pytest / go test ./...
```

**If tests fail:** Report failures, ask whether to proceed or investigate.

**If tests pass:** Report ready.

```
Worktree ready at <full-path>
Tests passing (<N> tests, 0 failures)
Ready to implement <feature-name>
```

## Quick Reference

| Situation | Action |
|-----------|--------|
| Already in linked worktree | Skip creation (Step 0) |
| In a submodule | Treat as normal repo (Step 0 guard) |
| Native worktree tool available | Use it (Step 1a) |
| No native tool | Git worktree fallback (Step 1b) |
| `.worktrees/` exists | Use it (verify ignored) |
| `worktrees/` exists | Use it (verify ignored) |
| Neither exists | Default `.worktrees/` |
| Directory not ignored | Add to .gitignore + commit |
| Permission error on create | Work in place |
| Tests fail during baseline | Report failures + ask |

## Common Mistakes

- **Fighting the harness** — Using `git worktree add` when the platform already provides isolation
- **Skipping detection** — Creating a nested worktree inside an existing one
- **Skipping ignore verification** — Worktree contents get tracked, pollute git status
- **Proceeding with failing tests** — Can't distinguish new bugs from pre-existing issues

## Red Flags

**Never:**
- Create a worktree when Step 0 detects existing isolation
- Use `git worktree add` when you have a native worktree tool (e.g., `EnterWorktree`)
- Create worktree without verifying it's ignored (project-local)
- Skip baseline test verification
- Proceed with failing tests without asking

**Always:**
- Run Step 0 detection first
- Prefer native tools over git fallback
- Verify directory is ignored for project-local
- Verify clean test baseline
