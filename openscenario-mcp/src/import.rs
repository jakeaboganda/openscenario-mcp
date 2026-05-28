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
    // Validate file extension
    if !xosc_path.ends_with(".xosc") {
        return Err(anyhow!(
            "Invalid file: '{}' does not have .xosc extension.\n\n\
            Expected: OpenSCENARIO XML file with .xosc extension.\n\n\
            Tip: Verify the file path is correct.",
            xosc_path
        ));
    }

    // Read XML file
    let xml_content = fs::read_to_string(&xosc_path).map_err(|e| {
        anyhow!(
            "Failed to read '{}': {}\n\n\
            Common causes:\n\
            - File does not exist\n\
            - Incorrect file path (use absolute path or relative to working directory)\n\
            - Insufficient read permissions\n\n\
            Tip: Check the file exists with: ls -la {}",
            xosc_path,
            e,
            xosc_path
        )
    })?;

    // Parse OpenSCENARIO XML
    let scenario = openscenario::Scenario::from_xml(&xml_content).map_err(|e| {
        anyhow!(
            "Failed to parse OpenSCENARIO XML from '{}':\n{}\n\n\
            Common causes:\n\
            - Malformed XML structure (unclosed tags, invalid syntax)\n\
            - Invalid OpenSCENARIO schema (must be 1.0, 1.1, or 1.2)\n\
            - Missing required elements (FileHeader, revMajor/revMinor)\n\
            - Invalid attribute values\n\n\
            Tips:\n\
            - Validate your .xosc file against the OpenSCENARIO XSD schema\n\
            - Check for XML syntax errors with: xmllint --noout {}\n\
            - Ensure FileHeader has revMajor and revMinor attributes",
            xosc_path,
            e,
            xosc_path
        )
    })?;

    // Generate scenario ID
    let scenario_id = scenario_name.unwrap_or_else(|| {
        std::path::Path::new(&xosc_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                eprintln!(
                    "Warning: Could not extract filename from '{}', using UUID",
                    xosc_path
                );
                format!("imported_{}", uuid::Uuid::new_v4())
            })
    });

    // Get summary info before moving scenario
    let version = scenario.version().to_string();
    let entity_count = scenario.entity_count();
    let entity_names: Vec<String> = scenario.entities().map(|e| e.name().to_string()).collect();
    let story_count = scenario.story_count();
    let story_names: Vec<String> = scenario
        .storyboard()
        .stories
        .keys()
        .map(|s| s.to_string())
        .collect();
    let road_network = scenario.get_road_network().cloned();

    // Check for road network mismatch
    let mut warnings = Vec::new();
    let mut suggestions = Vec::new();

    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    if let Some(imported_network) = &road_network {
        match &state_lock.current_road_network {
            Some(current) if current != imported_network => {
                warnings.push(format!(
                    "⚠️  Scenario references road network '{}' but '{}' is currently loaded.",
                    imported_network, current
                ));
                suggestions.push(format!(
                    "Load the correct road network: load_road_network(xodr_path='{}')",
                    imported_network
                ));
            }
            None => {
                suggestions.push(format!(
                    "Scenario requires road network '{}'. Load it with: load_road_network(xodr_path='{}')",
                    imported_network, imported_network
                ));
            }
            _ => {} // Network matches or no requirement
        }
    }

    // Store scenario
    state_lock.scenarios.insert(scenario_id.clone(), scenario);

    // Build next steps
    let next_steps = vec![
        format!(
            "inspect_scenario(scenario_id='{}') - View full JSON structure",
            scenario_id
        ),
        format!(
            "describe_scenario(scenario_id='{}') - Get human-readable summary",
            scenario_id
        ),
        format!(
            "check_scenario(scenario_id='{}') - Validate completeness and get suggestions",
            scenario_id
        ),
        format!(
            "export_xml(scenario_id='{}', output_path='modified.xosc') - Save changes",
            scenario_id
        ),
    ];

    // Build response
    let mut response = json!({
        "scenario_id": scenario_id,
        "source_file": xosc_path,
        "version": version,
        "entity_count": entity_count,
        "entities": entity_names,
        "story_count": story_count,
        "stories": story_names,
        "has_road_network": road_network.is_some(),
        "message": format!("Successfully imported {} v{} from {}", scenario_id, version, xosc_path),
        "next_steps": next_steps,
    });

    // Add optional fields
    if let Some(network_path) = road_network {
        response["road_network_path"] = json!(network_path);
    }
    if !warnings.is_empty() {
        response["warnings"] = json!(warnings);
    }
    if !suggestions.is_empty() {
        response["suggestions"] = json!(suggestions);
    }

    Ok(response.to_string())
}
