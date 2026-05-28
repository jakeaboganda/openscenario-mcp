//! Scenario inspection handlers for MCP tools.
//!
//! Provides tools for AI agents to inspect and validate scenarios.

use crate::server::ServerState;
use anyhow::{anyhow, Result};
use serde_json::json;
use std::sync::{Arc, Mutex};

/// List all scenarios in the current state.
///
/// Returns basic information about all loaded scenarios.
/// TODO: Add pagination support for large numbers of scenarios.
pub fn handle_list_scenarios(state: Arc<Mutex<ServerState>>) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let scenarios: Vec<_> = state_lock
        .scenarios
        .iter()
        .map(|(id, scenario)| {
            json!({
                "scenario_id": id,
                "version": format!("{:?}", scenario.version()),
                "entity_count": scenario.entity_count(),
                "story_count": scenario.story_count(),
                "has_road_network": scenario.get_road_network().is_some(),
            })
        })
        .collect();

    Ok(json!({
        "scenarios": scenarios,
        "total": scenarios.len(),
    })
    .to_string())
}

/// Inspect a scenario and return comprehensive JSON structure.
///
/// Includes:
/// - All entities with their initial conditions (position, speed)
/// - Action triggers and conditions
/// - Timing information
/// - Parameter details
pub fn handle_inspect_scenario(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let scenario = state_lock.scenarios.get(&scenario_id).ok_or_else(|| {
        anyhow!(
            "Scenario '{}' not found.\nRun list_scenarios() to see available scenarios.",
            scenario_id
        )
    })?;

    // Collect entities with full details
    let entities: Vec<_> = scenario
        .entities()
        .map(|entity| {
            let name = entity.name();
            json!({
                "name": name,
                "type": match entity {
                    openscenario::entities::Entity::Vehicle(_) => "vehicle",
                    openscenario::entities::Entity::Pedestrian(_) => "pedestrian",
                    openscenario::entities::Entity::MiscObject(_) => "misc_object",
                },
                "details": format!("{:#?}", entity),
                "initial_position": scenario.get_initial_position(name).map(|pos| {
                    format!("{:?}", pos)
                }),
                "initial_speed": scenario.get_initial_speed(name),
            })
        })
        .collect();

    // Collect stories - use Debug format for now
    let storyboard_debug = format!("{:#?}", scenario.storyboard());

    // Collect parameters
    let parameters: Vec<_> = scenario
        .parameters()
        .iter()
        .map(|param| {
            json!({
                "name": &param.name,
                "type": format!("{:?}", param.parameter_type),
                "value": &param.value,
            })
        })
        .collect();

    Ok(json!({
        "scenario_id": scenario_id,
        "version": format!("{:?}", scenario.version()),
        "road_network": scenario.get_road_network(),
        "parameters": parameters,
        "entities": entities,
        "storyboard_debug": storyboard_debug,
    })
    .to_string())
}

/// Describe a scenario in human-readable Markdown format.
///
/// Provides a natural language summary suitable for AI understanding.
pub fn handle_describe_scenario(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let scenario = state_lock.scenarios.get(&scenario_id).ok_or_else(|| {
        anyhow!(
            "Scenario '{}' not found.\nRun list_scenarios() to see available scenarios.",
            scenario_id
        )
    })?;

    let mut description = String::new();

    // Header
    description.push_str(&format!("# Scenario: {}\n\n", scenario_id));
    description.push_str(&format!(
        "**Version**: OpenSCENARIO {:?}\n\n",
        scenario.version()
    ));

    if let Some(road) = scenario.get_road_network() {
        description.push_str(&format!("**Road Network**: `{}`\n\n", road));
    } else {
        description.push_str("**Road Network**: ⚠️ None\n\n");
    }

    // Parameters
    if !scenario.parameters().is_empty() {
        description.push_str("## Parameters\n\n");
        for param in scenario.parameters() {
            description.push_str(&format!(
                "- **{}** ({:?}): `{}`\n",
                param.name, param.parameter_type, param.value
            ));
        }
        description.push('\n');
    }

    // Entities
    description.push_str(&format!("## Entities ({})\n\n", scenario.entity_count()));

    for entity in scenario.entities() {
        let name = entity.name();
        let entity_type = match entity {
            openscenario::entities::Entity::Vehicle(v) => {
                format!("Vehicle ({:?})", v.params.vehicle_category)
            }
            openscenario::entities::Entity::Pedestrian(_) => "Pedestrian".to_string(),
            openscenario::entities::Entity::MiscObject(obj) => {
                format!("MiscObject ({:?})", obj.params.category)
            }
        };

        description.push_str(&format!("### {}\n\n", name));
        description.push_str(&format!("- **Type**: {}\n", entity_type));

        if let Some(pos) = scenario.get_initial_position(name) {
            description.push_str(&format!("- **Initial Position**: {:?}\n", pos));
        }

        if let Some(speed) = scenario.get_initial_speed(name) {
            description.push_str(&format!("- **Initial Speed**: {} m/s\n", speed));
        }

        description.push('\n');
    }

    // Storyboard
    description.push_str(&format!(
        "## Storyboard ({} stories)\n\n",
        scenario.story_count()
    ));

    for story in scenario.stories() {
        description.push_str(&format!("### Story: {}\n\n", story.name));

        for act in story.acts.values() {
            description.push_str(&format!("#### Act: {}\n\n", act.name));

            if let Some(trigger) = &act.start_trigger {
                description.push_str(&format!("**Start Trigger**: `{:?}`\n\n", trigger));
            }

            for mg in act.maneuver_groups.values() {
                description.push_str(&format!(
                    "**Maneuver Group**: {} (Actors: {})\n\n",
                    mg.name,
                    mg.actors.join(", ")
                ));

                for maneuver in &mg.maneuvers {
                    description.push_str(&format!("- **Maneuver**: {}\n", maneuver.name));

                    for event in &maneuver.events {
                        description.push_str(&format!("  - **Event**: {}\n", event.name));

                        if let Some(trigger) = &event.start_trigger {
                            description.push_str(&format!("    - **Trigger**: `{:?}`\n", trigger));
                        }

                        for action in &event.actions {
                            description.push_str(&format!("    - **Action**: `{:?}`\n", action));
                        }
                    }
                }

                description.push('\n');
            }
        }
    }

    Ok(description)
}

/// Validate a scenario and provide helpful suggestions.
///
/// Checks for common issues and provides actionable feedback.
pub fn handle_check_scenario(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let scenario = state_lock.scenarios.get(&scenario_id).ok_or_else(|| {
        anyhow!(
            "Scenario '{}' not found.\nRun list_scenarios() to see available scenarios.",
            scenario_id
        )
    })?;

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Check: Road network required
    if scenario.get_road_network().is_none() {
        errors.push(json!({
            "code": "MISSING_ROAD_NETWORK",
            "severity": "error",
            "message": "No road network set",
            "suggestion": "Use load_road_network() or get_real_world_road() to load a road network"
        }));
    }

    // Check: At least one entity
    if scenario.entity_count() == 0 {
        warnings.push(json!({
            "code": "NO_ENTITIES",
            "severity": "warning",
            "message": "Scenario has no entities",
            "suggestion": "Add vehicles using add_vehicle() or pedestrians using add_pedestrian()"
        }));
    }

    // Check: Entities have initial positions
    for entity in scenario.entities() {
        let name = entity.name();
        if scenario.get_initial_position(name).is_none() {
            errors.push(json!({
                "code": "MISSING_INITIAL_POSITION",
                "entity": name,
                "severity": "error",
                "message": format!("Entity '{}' has no initial position", name),
                "suggestion": "Use set_initial_position() or set_lane_position()"
            }));
        }
    }

    // Check: Entities have initial speeds
    for entity in scenario.entities() {
        let name = entity.name();
        if scenario.get_initial_speed(name).is_none() {
            warnings.push(json!({
                "code": "MISSING_INITIAL_SPEED",
                "entity": name,
                "severity": "warning",
                "message": format!("Entity '{}' has no initial speed (defaults to 0)", name),
                "suggestion": "Use set_initial_speed() to set the initial speed"
            }));
        }
    }

    // Check: At least one story
    if scenario.story_count() == 0 {
        warnings.push(json!({
            "code": "NO_STORIES",
            "severity": "warning",
            "message": "Scenario has no stories or actions",
            "suggestion": "Add stories with add_story() and actions like add_lane_change_action()"
        }));
    }

    let valid = errors.is_empty();

    Ok(json!({
        "valid": valid,
        "errors": errors,
        "warnings": warnings,
        "summary": if valid {
            format!("Scenario is valid (0 errors, {} warnings)", warnings.len())
        } else {
            format!("Scenario has {} errors and {} warnings", errors.len(), warnings.len())
        }
    })
    .to_string())
}
