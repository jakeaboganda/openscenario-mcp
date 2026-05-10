# Documentation Milestone Plan

**Goal**: Create comprehensive, user-friendly documentation for the osc-mcp OpenSCENARIO library.

**Date**: 2026-05-10  
**Status**: Planning Phase  
**Target**: Production-ready documentation for all implemented features

---

## 🎯 Documentation Goals

### Primary Goals
1. **Enable new users** to build their first scenario in <15 minutes
2. **Provide reference documentation** for all API methods
3. **Show real-world examples** of common use cases
4. **Guide integration** with simulators (esmini)
5. **Explain concepts** for users new to OpenSCENARIO

### Success Metrics
- ✅ README covers "getting started" in <5 minutes
- ✅ Every public method has doc comments with examples
- ✅ 5+ complete example scenarios (highway, urban, parking, etc.)
- ✅ Architecture guide explains design decisions
- ✅ Integration guide for esmini with screenshots/videos

---

## 📚 Documentation Structure

### 1. **README.md** (Root Level)
**Purpose**: First impression, quick start, feature overview

**Sections**:
- **Project Description** - What is osc-mcp? Why use it?
- **Features** - Bullet list of implemented features with checkboxes
- **Quick Start** - 10-line "Hello World" scenario
- **Installation** - `cargo add openscenario` + dependencies
- **Basic Usage** - Code snippet showing typical workflow
- **Examples** - Links to `/examples` directory
- **Documentation** - Links to docs, guides, API reference
- **Status** - Current implementation status (Milestones 1-3 complete)
- **Contributing** - How to contribute, code style, testing
- **License** - MIT/Apache dual license

**Target Length**: 200-400 lines (readable in 3-5 minutes)

---

### 2. **API Documentation** (Rustdoc Comments)
**Purpose**: Reference documentation for developers

**Coverage**:
- [ ] **Scenario module** - `scenario.rs` (30+ methods)
- [ ] **Storyboard module** - `storyboard.rs` (structs, enums)
- [ ] **Entities module** - `entities.rs` (Vehicle, Pedestrian, etc.)
- [ ] **Position module** - `position.rs` (7 position types)
- [ ] **XML module** - `xml.rs` (export functionality)
- [ ] **Error module** - `error.rs` (error types)

**Quality Standard**:
```rust
/// Add an event with a time headway condition trigger.
///
/// Creates an event that triggers when the time gap between the entity
/// and a lead vehicle meets the rule threshold.
///
/// # Arguments
/// * `story` - Story name
/// * `act` - Act name within the story
/// * `entity_ref` - Entity being monitored (follower)
/// * `lead_entity_ref` - Lead vehicle to measure gap to
/// * `time_headway_value` - Time gap threshold in seconds (must be positive)
/// * `rule` - Comparison rule (LessThan, GreaterThan, EqualTo)
/// * `freespace` - If true, measure to bounding box; if false, to reference point
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(ScenarioError)` if validation fails
///
/// # Examples
/// ```
/// use openscenario::{Scenario, OpenScenarioVersion};
/// use openscenario::storyboard::Rule;
///
/// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
/// // ... add entities and story structure ...
/// 
/// // Trigger when following too closely (< 2 seconds)
/// scenario.add_event_with_time_headway_condition(
///     "main_story", "act1", "mg1", "maneuver1", "too_close",
///     "ego", "lead_vehicle", 2.0, Rule::LessThan, true
/// )?;
/// # Ok::<(), openscenario::ScenarioError>(())
/// ```
///
/// # Errors
/// Returns `ScenarioError::InvalidValue` if `time_headway_value` <= 0.
/// Returns `ScenarioError::InvalidEntityRef` if either entity doesn't exist.
pub fn add_event_with_time_headway_condition(...)
```

**Estimated Work**: 2-3 hours (methodical pass through all public APIs)

---

### 3. **Examples Directory** (`examples/`)
**Purpose**: Concrete, runnable examples for learning

**Target Examples** (10-15 total):

#### **Basic Examples** (Getting Started)
1. `hello_world.rs` - Simplest possible scenario (1 vehicle, 1 action)
2. `basic_lane_change.rs` - Single lane change maneuver
3. `speed_control.rs` - Acceleration and speed actions
4. `position_reaching.rs` - Move to waypoint with condition

#### **Highway Scenarios**
5. `highway_merge.rs` - On-ramp merge with acceleration
6. `adaptive_cruise_control.rs` - ACC using TimeHeadway + SpeedProfile
7. `lane_change_overtaking.rs` - Overtake slower vehicle
8. `emergency_braking.rs` - Sudden deceleration with CollisionCondition

#### **Urban Scenarios**
9. `traffic_light_intersection.rs` - 4-way intersection with stops
10. `pedestrian_crossing.rs` - Pedestrian + vehicle interaction
11. `stop_and_go_traffic.rs` - StandStill conditions in congestion
12. `parking_maneuver.rs` - Parallel parking with position checks

#### **Advanced Examples**
13. `platooning.rs` - Multi-vehicle following with TimeHeadway
14. `collision_avoidance.rs` - TTC-based evasive maneuver
15. `multi_vehicle_coordination.rs` - Complex scenario with multiple actors

**Example Template**:
```rust
//! Highway Merge Scenario
//!
//! Demonstrates:
//! - AccelerationAction for speed adjustment
//! - RelativeDistanceCondition for merge decision
//! - LaneChangeAction for merging
//!
//! Expected behavior:
//! 1. Ego vehicle accelerates on on-ramp
//! 2. When gap is available, merge into highway lane
//! 3. Match highway traffic speed
//!
//! Run with: `cargo run --example highway_merge`

use openscenario::*;

fn main() -> Result<(), ScenarioError> {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    // 1. Setup entities
    // ... (well-commented code)
    
    // 2. Define story structure
    // ...
    
    // 3. Add behaviors
    // ...
    
    // 4. Export XML
    let xml = scenario.to_xml()?;
    std::fs::write("highway_merge.xosc", xml)?;
    println!("Scenario exported to highway_merge.xosc");
    
    Ok(())
}
```

**Estimated Work**: 8-12 hours (1-2 hours per example with testing)

---

### 4. **Guides** (`docs/guides/`)
**Purpose**: In-depth explanations and tutorials

#### **Getting Started Guide** (`docs/guides/getting-started.md`)
- Installation and setup
- Core concepts (Scenario, Story, Act, Maneuver)
- First scenario walkthrough
- Running in esmini
- Common pitfalls and solutions

#### **Architecture Guide** (`docs/guides/architecture.md`)
- Project structure overview
- Module responsibilities
- Design patterns used (builder, type-safe APIs)
- Error handling philosophy
- XML export pipeline
- Testing strategy

#### **OpenSCENARIO Primer** (`docs/guides/openscenario-primer.md`)
- What is OpenSCENARIO?
- Key concepts (Entities, Actions, Conditions, Triggers)
- Story hierarchy (Story → Act → ManeuverGroup → Maneuver → Event)
- Position types explained
- Coordinate systems
- When to use which feature

#### **Integration Guide** (`docs/guides/integration.md`)
- **esmini integration**
  - Installation
  - Running scenarios
  - Visualization
  - Command-line options
- **CARLA integration** (future)
- **Custom simulator integration** (guidance)

#### **Migration Guide** (`docs/guides/migration.md`)
- Upgrading from v0.1 to v0.2 (when needed)
- Breaking changes log
- Deprecation notices

**Estimated Work**: 6-8 hours (2 hours per major guide)

---

### 5. **Cookbook** (`docs/cookbook/`)
**Purpose**: Quick recipes for common tasks

**Recipes** (15-20 snippets):
1. "How to add a vehicle"
2. "How to trigger on speed threshold"
3. "How to implement ACC behavior"
4. "How to detect collisions"
5. "How to create custom positions"
6. "How to chain multiple events"
7. "How to synchronize multiple vehicles"
8. "How to export and validate XML"
9. "How to debug scenario issues"
10. "How to measure distances between entities"
... (more as needed)

**Format**:
```markdown
## How to implement ACC behavior

**Problem**: You want a vehicle to maintain safe following distance.

**Solution**:
\`\`\`rust
// Add TimeHeadway condition
scenario.add_event_with_time_headway_condition(
    "story", "act", "mg", "maneuver", "too_close",
    "ego", "lead", 2.0, Rule::LessThan, true
)?;

// Add speed adjustment action
scenario.add_speed_profile_action(
    "story", "act", "mg", "maneuver", "slow_down",
    vec![
        (0.0, 20.0),  // t=0s: 20 m/s
        (3.0, 15.0),  // t=3s: 15 m/s
    ],
    true  // time-based
)?;
\`\`\`

**Explanation**: TimeHeadway measures time gap. When < 2s, speed profile
reduces speed smoothly over 3 seconds.

**See also**: [examples/adaptive_cruise_control.rs](../../examples/adaptive_cruise_control.rs)
```

**Estimated Work**: 4-6 hours

---

### 6. **Testing Documentation** (`docs/testing.md`)
**Purpose**: Guide contributors on testing practices

**Sections**:
- Testing philosophy (TDD approach)
- Test structure (unit, integration, validation)
- Running tests (`cargo test`)
- Writing new tests (patterns, assertions)
- Test coverage goals (>90%)
- CI/CD setup (GitHub Actions)
- Validation with esmini

**Estimated Work**: 2 hours

---

### 7. **API Reference** (Generated via `cargo doc`)
**Purpose**: Auto-generated HTML documentation

**Setup**:
- [ ] Configure `Cargo.toml` with documentation metadata
- [ ] Add `#![doc = include_str!("../README.md")]` to `lib.rs`
- [ ] Set up GitHub Pages or docs.rs publishing
- [ ] Add badges to README (docs.rs, crates.io, CI status)

**Estimated Work**: 1 hour setup

---

## 🗓️ Implementation Phases

### **Phase 1: Foundation** (Week 1, ~10 hours)
**Goal**: Core documentation in place

- [x] Write comprehensive README.md
- [ ] Add rustdoc comments to 10 most-used methods
- [ ] Create 3 basic examples (hello_world, lane_change, speed_control)
- [ ] Write "Getting Started Guide"

**Deliverables**:
- Updated README with quick start
- 3 runnable examples
- Getting started guide (markdown)

---

### **Phase 2: Examples & Guides** (Week 2, ~15 hours)
**Goal**: Rich example library

- [ ] Create 7 more examples (highway, urban, advanced)
- [ ] Write Architecture Guide
- [ ] Write OpenSCENARIO Primer
- [ ] Write Integration Guide (esmini)
- [ ] Start cookbook with 10 recipes

**Deliverables**:
- 10 total examples covering major use cases
- 3 comprehensive guides
- 10 cookbook recipes

---

### **Phase 3: Polish & API Docs** (Week 3, ~8 hours)
**Goal**: Complete API reference

- [ ] Add rustdoc comments to all remaining public methods
- [ ] Add examples to every doc comment
- [ ] Finish cookbook (20 recipes total)
- [ ] Write testing documentation
- [ ] Set up docs.rs or GitHub Pages

**Deliverables**:
- 100% API documentation coverage
- Published HTML documentation
- Complete cookbook

---

### **Phase 4: Validation & Iteration** (Week 4, ~5 hours)
**Goal**: User feedback and improvements

- [ ] Test all examples in esmini
- [ ] Add screenshots/videos to integration guide
- [ ] Get feedback from 2-3 external users
- [ ] Fix documentation gaps
- [ ] Add FAQ section based on feedback

**Deliverables**:
- Validated examples (all run in esmini)
- User-tested documentation
- FAQ section

---

## 📊 Documentation Quality Checklist

### **Per Example**
- [ ] Has a clear doc comment explaining purpose
- [ ] Lists what it demonstrates
- [ ] Includes expected behavior description
- [ ] Compiles without warnings
- [ ] Runs successfully with `cargo run --example <name>`
- [ ] Generates valid OpenSCENARIO XML
- [ ] Validated in esmini (visual check)
- [ ] Code is well-commented (explain *why*, not *what*)

### **Per Guide**
- [ ] Clear table of contents
- [ ] Progressive difficulty (easy → advanced)
- [ ] Code examples are tested
- [ ] Links to relevant API docs
- [ ] Links to relevant examples
- [ ] Diagrams where helpful (optional)

### **Per API Method**
- [ ] Summary line (<80 chars)
- [ ] Detailed description
- [ ] All parameters documented
- [ ] Return value documented
- [ ] At least one code example
- [ ] Error cases documented
- [ ] Links to related methods

---

## 🛠️ Tools & Automation

### **Documentation Tools**
- **rustdoc** - Generate API reference
- **mdBook** - For guides (optional, markdown is fine for now)
- **GitHub Pages** - Host documentation
- **docs.rs** - Automatic crate documentation

### **Validation Tools**
- **cargo test --doc** - Test code in doc comments
- **cargo clippy** - Lint documentation
- **markdownlint** - Markdown style checking
- **esmini** - Validate generated scenarios

### **CI/CD Checks**
```yaml
# .github/workflows/docs.yml
- name: Check documentation
  run: |
    cargo doc --all-features --no-deps
    cargo test --doc
    
- name: Validate examples
  run: |
    cargo build --examples
    ./scripts/validate_examples.sh  # Run all examples, check output
```

---

## 📝 Writing Style Guide

### **Tone**
- **Friendly but professional** - "Let's build a scenario" not "The user shall..."
- **Direct and concise** - Avoid unnecessary jargon
- **Example-driven** - Show, don't just tell
- **Beginner-friendly** - Explain concepts, don't assume knowledge

### **Code Style**
- **Commented examples** - Explain *why* not *what*
- **Error handling** - Always show proper `Result` handling
- **Realistic scenarios** - Use meaningful names (not `foo`, `bar`)
- **Progressive complexity** - Start simple, add features

### **Formatting**
- **Markdown** - Use standard GitHub-flavored markdown
- **Code blocks** - Always specify language (\`\`\`rust)
- **Headings** - Use hierarchical structure (H1 → H2 → H3)
- **Links** - Use relative links for internal docs

---

## 🎯 Success Metrics (Definition of Done)

### **Minimum Viable Documentation (MVP)**
- [x] README with quick start
- [ ] 5 working examples
- [ ] Getting Started guide
- [ ] 50% of public methods have doc comments with examples

### **Complete Documentation (v1.0)**
- [ ] README with full feature list
- [ ] 10+ working examples (all validated in esmini)
- [ ] 4 comprehensive guides
- [ ] 15+ cookbook recipes
- [ ] 100% API documentation coverage
- [ ] Published docs (docs.rs or GitHub Pages)
- [ ] Testing documentation

### **Excellent Documentation (v2.0+)**
- [ ] Video tutorials (YouTube)
- [ ] Interactive examples (web-based)
- [ ] Community contributions (tutorials, blog posts)
- [ ] Translated guides (non-English)

---

## 📅 Milestones & Timeline

### **Milestone D1: Foundation** (Target: Week 1)
- README.md complete
- 3 basic examples working
- Getting Started guide published
- Initial rustdoc comments on key methods

### **Milestone D2: Examples** (Target: Week 2)
- 10 examples total
- All examples validated in esmini
- Architecture + OpenSCENARIO guides complete

### **Milestone D3: API Reference** (Target: Week 3)
- 100% API documentation
- Cookbook with 15+ recipes
- Testing documentation

### **Milestone D4: Polish** (Target: Week 4)
- External review completed
- Screenshots/videos in integration guide
- FAQ section added
- All documentation cross-linked

---

## 🚀 Next Steps

1. **Review this plan** with stakeholders
2. **Prioritize deliverables** (which examples/guides are most critical?)
3. **Start Phase 1** with README and basic examples
4. **Iterate based on feedback**

**Estimated Total Effort**: 38-41 hours over 4 weeks  
**Current Status**: ✅ PLAN COMPLETE, ready to execute

---

## 💡 Future Enhancements

- Interactive scenario builder (web UI)
- OpenSCENARIO visualizer (render scenarios as diagrams)
- Performance benchmarks documentation
- Comparison with other OpenSCENARIO libraries
- Integration examples for more simulators (CARLA, Scenic, etc.)

---

**Ready to begin?** Suggest starting with:
1. Update README.md with current feature status
2. Create 3 basic examples (hello_world, lane_change, acc)
3. Write Getting Started guide

This gives immediate value and builds foundation for deeper documentation.
