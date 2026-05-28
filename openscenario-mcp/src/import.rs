//! Import scenario handler for loading external .xosc files

use crate::server::ServerState;
use anyhow::{anyhow, Result};
use serde_json::json;
use std::fs;
use std::sync::{Arc, Mutex};

/// Import an existing OpenSCENARIO .xosc file into the current session.
///
/// Parses the XML file and loads it as a Scenario that can be inspected and modified.
///
/// # Arguments
/// * `state` - Server state
/// * `xosc_path` - Path to .xosc file
/// * `scenario_name` - Optional name for the scenario (defaults to filename)
///
/// # Returns
/// * JSON with scenario_id and import summary
pub fn handle_import_scenario(
    state: Arc<Mutex<ServerState>>,
    xosc_path: String,
    scenario_name: Option<String>,
) -> Result<String> {
    // Read XML file
    let xml_content = fs::read_to_string(&xosc_path)
        .map_err(|e| anyhow!("Failed to read {}: {}", xosc_path, e))?;

    // Parse OpenSCENARIO XML
    let scenario = openscenario::Scenario::from_xml(&xml_content)
        .map_err(|e| anyhow!("Failed to parse OpenSCENARIO XML: {}", e))?;

    // Generate scenario ID
    let scenario_id = scenario_name.unwrap_or_else(|| {
        std::path::Path::new(&xosc_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("imported_{}", uuid::Uuid::new_v4()))
    });

    // Store in state
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    // Get summary info
    let entity_count = scenario.entity_count();
    let story_count = scenario.story_count();
    let has_road_network = scenario.get_road_network().is_some();

    state_lock
        .scenarios
        .insert(scenario_id.clone(), scenario);

    Ok(json!({
        "scenario_id": scenario_id,
        "source_file": xosc_path,
        "entity_count": entity_count,
        "story_count": story_count,
        "has_road_network": has_road_network,
        "message": format!("Successfully imported scenario from {}", xosc_path)
    })
    .to_string())
}
