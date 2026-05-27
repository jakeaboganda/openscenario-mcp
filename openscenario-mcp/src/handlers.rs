use crate::server::ServerState;
use anyhow::{anyhow, Result};
use openscenario::entities::{
    CatalogReference, MiscObjectParams, PedestrianParams, VehicleCategory, VehicleParams,
};
use openscenario::storyboard::{
    DynamicsDimension, DynamicsShape, TransitionDynamics, TransitionShape,
};
use openscenario::validation::XsdValidator;
use openscenario::Position;
use openscenario::{OpenScenarioVersion, Scenario};
use serde_json::json;
use std::fs;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Create a new OpenSCENARIO scenario
pub fn handle_create_scenario(
    state: Arc<Mutex<ServerState>>,
    name: String,
    version: String,
) -> Result<String> {
    // Parse version
    let osc_version = match version.as_str() {
        "1.0" => OpenScenarioVersion::V1_0,
        "1.1" => OpenScenarioVersion::V1_1,
        "1.2" => OpenScenarioVersion::V1_2,
        _ => {
            return Err(anyhow!(
                "Invalid version: {}. Must be 1.0, 1.1, or 1.2",
                version
            ))
        }
    };

    // Create scenario
    let scenario = Scenario::new(osc_version);

    // Generate unique ID
    let scenario_id = format!("{}_{}", name, Uuid::new_v4());

    // Store in state
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    state_lock.scenarios.insert(scenario_id.clone(), scenario);

    Ok(scenario_id)
}

/// Add a vehicle to a scenario
pub fn handle_add_vehicle(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    name: String,
    category: String,
    catalog: Option<String>,
) -> Result<String> {
    // Parse vehicle category (case-insensitive)
    let vehicle_category = match category.to_lowercase().as_str() {
        "car" => VehicleCategory::Car,
        "truck" => VehicleCategory::Truck,
        "bus" => VehicleCategory::Bus,
        "trailer" => VehicleCategory::Trailer,
        "van" => VehicleCategory::Van,
        "motorbike" => VehicleCategory::Motorbike,
        "bicycle" => VehicleCategory::Bicycle,
        _ => return Err(anyhow!("Invalid vehicle category: {}", category)),
    };

    // Parse catalog if provided
    let catalog_ref = catalog.map(|path| {
        // Simple format: "path:entry_name"
        let parts: Vec<&str> = path.split(':').collect();
        if parts.len() == 2 {
            CatalogReference {
                path: parts[0].to_string(),
                entry_name: parts[1].to_string(),
            }
        } else {
            CatalogReference {
                path: path.clone(),
                entry_name: name.clone(),
            }
        }
    });

    let params = VehicleParams {
        catalog: catalog_ref,
        vehicle_category,
        properties: None,
    };

    // Get scenario and add vehicle
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    scenario.add_vehicle(name.clone(), params)?;

    Ok(name)
}

/// Set initial position for an entity in a scenario
pub fn handle_set_position(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    entity_name: String,
    x: f64,
    y: f64,
    z: f64,
    h: f64,
) -> Result<String> {
    let position = Position::world(x, y, z, h);

    // Get scenario and set position
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    scenario.set_initial_position(entity_name.clone(), position)?;

    Ok(format!("Position set for entity: {}", entity_name))
}

/// Add a pedestrian to a scenario
pub fn handle_add_pedestrian(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    name: String,
    catalog: Option<String>,
    mass: Option<f64>,
) -> Result<String> {
    // Parse catalog reference if provided
    let catalog_ref = if let Some(ref path) = catalog {
        let parts: Vec<&str> = path.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow!(
                "Invalid catalog format: '{}'. Expected 'path:entry_name'",
                path
            ));
        }
        Some(CatalogReference {
            path: parts[0].to_string(),
            entry_name: parts[1].to_string(),
        })
    } else {
        None
    };

    // Validate mass if provided
    if let Some(m) = mass {
        if m <= 0.0 {
            return Err(anyhow!("Mass must be positive, got: {}", m));
        }
        if m > 500.0 {
            return Err(anyhow!(
                "Mass {} kg seems unrealistic for a pedestrian. Max: 500 kg",
                m
            ));
        }
    }

    let params = PedestrianParams {
        catalog: catalog_ref,
        model: None,
        mass: mass.or(Some(70.0)), // Default to 70kg if not provided
    };
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;
    scenario.add_pedestrian(name.clone(), params)?;
    Ok(name)
}

/// Add a misc object to a scenario
pub fn handle_add_misc_object(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    name: String,
    category: String,
    mass: f64,
) -> Result<String> {
    // Validate mass
    if mass <= 0.0 {
        return Err(anyhow!("Mass must be positive, got: {}", mass));
    }
    if mass > 100000.0 {
        return Err(anyhow!(
            "Mass {} kg seems unrealistic. Max: 100000 kg (100 tons)",
            mass
        ));
    }

    // Validate category
    const VALID_CATEGORIES: &[&str] = &[
        "barrier",
        "obstacle",
        "pole",
        "tree",
        "vegetation",
        "building",
        "vehicle",
        "none",
    ];
    if !VALID_CATEGORIES.contains(&category.as_str()) {
        return Err(anyhow!(
            "Invalid category '{}'. Valid categories: {}",
            category,
            VALID_CATEGORIES.join(", ")
        ));
    }

    let params = MiscObjectParams {
        catalog: None,
        category: Some(category),
        mass: Some(mass),
    };
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;
    scenario.add_misc_object(name.clone(), params)?;
    Ok(name)
}

/// Set initial lane position for an entity in a scenario
pub fn handle_set_lane_position(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    entity_name: String,
    road_id: String,
    lane_id: i32,
    s: f64,
    offset: f64,
) -> Result<String> {
    // Validate inputs
    if s < 0.0 {
        return Err(anyhow!("Position 's' cannot be negative, got: {}", s));
    }
    if lane_id == 0 {
        return Err(anyhow!("Lane ID cannot be 0 (center lane is invalid)"));
    }
    if offset.abs() > 10.0 {
        return Err(anyhow!(
            "Lateral offset {} m seems excessive. Typical range: ±10m",
            offset
        ));
    }

    let position = Position::lane(road_id, lane_id, s, offset, None);

    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    scenario.set_initial_position(entity_name.clone(), position)?;

    Ok(format!("Lane position set for entity: {}", entity_name))
}

/// Add a speed action to a scenario
/// Creates default story structure if it doesn't exist: story -> act -> maneuver_group -> maneuver -> event
pub fn handle_add_speed_action(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    entity_name: String,
    story_name: String,
    speed: f64,
    duration: f64,
) -> Result<String> {
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    // Ensure story structure exists
    let act_name = format!("{}_act", story_name);
    let mg_name = format!("{}_mg", entity_name);
    let maneuver_name = format!("{}_maneuver", entity_name);
    let event_name = "speed_event";

    // Try to create story structure (ignore errors if already exists)
    let _ = scenario.add_story(&story_name);
    let _ = scenario.add_act(&story_name, &act_name);
    let _ = scenario.add_maneuver_group(&story_name, &act_name, &mg_name);
    let _ = scenario.add_maneuver(&story_name, &act_name, &mg_name, &maneuver_name);

    // Ensure actor is added (try multiple times if needed)
    if let Err(e) = scenario.add_actor(&story_name, &act_name, &mg_name, entity_name.clone()) {
        // If it failed, log but continue - the actor might already exist
        eprintln!(
            "Note: add_actor returned error (may be ok if already exists): {}",
            e
        );
    }

    // Add speed action
    scenario.add_speed_action(
        &story_name,
        &act_name,
        &mg_name,
        &maneuver_name,
        event_name,
        speed,
        TransitionDynamics {
            shape: DynamicsShape::Linear,
            dimension: DynamicsDimension::Time,
            value: duration,
        },
    )?;

    Ok(format!(
        "Speed action added: {} m/s over {} seconds",
        speed, duration
    ))
}

/// Add a lane change action to a scenario
/// Creates default story structure if it doesn't exist: story -> act -> maneuver_group -> maneuver -> event
pub fn handle_add_lane_change_action(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    entity_name: String,
    story_name: String,
    target_lane: f64,
    duration: f64,
) -> Result<String> {
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    // Ensure story structure exists
    let act_name = format!("{}_act", story_name);
    let mg_name = format!("{}_mg", entity_name);
    let maneuver_name = format!("{}_maneuver", entity_name);
    let event_name = "lane_change_event";

    // Try to create story structure (ignore errors if already exists)
    let _ = scenario.add_story(&story_name);
    let _ = scenario.add_act(&story_name, &act_name);
    let _ = scenario.add_maneuver_group(&story_name, &act_name, &mg_name);
    let _ = scenario.add_maneuver(&story_name, &act_name, &mg_name, &maneuver_name);

    // Ensure actor is added
    if let Err(e) = scenario.add_actor(&story_name, &act_name, &mg_name, entity_name.clone()) {
        eprintln!(
            "Note: add_actor returned error (may be ok if already exists): {}",
            e
        );
    }

    // Add lane change action
    scenario.add_lane_change_action(
        &story_name,
        &act_name,
        &mg_name,
        &maneuver_name,
        event_name,
        target_lane,
        duration,
        TransitionShape::Linear,
    )?;

    Ok(format!(
        "Lane change action added: target lane offset {} over {} seconds",
        target_lane, duration
    ))
}

/// Export a scenario to an XML file
pub fn handle_export_xml(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    output_path: String,
) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    // Generate XML
    let xml_content = scenario.to_xml()?;

    // Write to file
    fs::write(&output_path, xml_content).map_err(|e| anyhow!("Failed to write XML file: {}", e))?;

    Ok(format!("Exported scenario to: {}", output_path))
}

/// Validate a scenario using XSD validation
pub fn handle_validate_scenario(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    // Generate XML
    let xml_content = scenario.to_xml()?;

    // Get version string
    let version_str = scenario.version().to_string();

    // Create validator and validate
    let validator = XsdValidator::new(version_str);
    let report = validator.validate(&xml_content);

    // Format as JSON report
    let json_report = json!({
        "valid": report.valid,
        "errors": report.errors
    });

    Ok(json_report.to_string())
}

pub fn handle_set_stop_time(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    seconds: f64,
) -> Result<String> {
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    scenario.set_stop_time(seconds);
    Ok(format!("Set stop time to {} seconds", seconds))
}

pub fn handle_set_stop_on_element(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    element_type: String,
    element_ref: String,
    state_name: String,
    delay: f64,
) -> Result<String> {
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    scenario.set_stop_on_element_state(
        element_type.clone(),
        element_ref.clone(),
        state_name.clone(),
        delay,
    );
    Ok(format!(
        "Set stop trigger on {} element '{}' reaching state '{}'",
        element_type, element_ref, state_name
    ))
}

/// Load and analyze an OpenDRIVE road network
pub fn handle_load_road_network(
    state: Arc<Mutex<ServerState>>,
    xodr_path: String,
) -> Result<String> {
    use openscenario::opendrive_validator::OpenDriveValidator;
    use std::path::Path;

    let path = Path::new(&xodr_path);
    let validator = OpenDriveValidator::load(path)
        .map_err(|e| anyhow!("Failed to load OpenDRIVE file: {}", e))?;

    // Get road information
    let roads = validator.list_roads();
    let quality = validator.assess_quality();

    // Store validator in state
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    state_lock.road_validator = Some(validator);
    state_lock.current_road_network = Some(xodr_path.clone());

    Ok(json!({
        "status": "success",
        "file": xodr_path,
        "road_count": roads.len(),
        "roads": roads,
        "quality": {
            "score": quality.score,
            "has_lanes": quality.has_lanes,
            "has_geometry": quality.has_geometry,
            "has_valid_length": quality.has_valid_length,
            "issues": quality.issues
        }
    })
    .to_string())
}

/// List all roads in the loaded network
pub fn handle_list_roads(state: Arc<Mutex<ServerState>>) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let validator = state_lock
        .road_validator
        .as_ref()
        .ok_or_else(|| anyhow!("No road network loaded. Use load_road_network first."))?;

    let roads = validator.list_roads();
    Ok(json!({
        "roads": roads,
        "count": roads.len()
    })
    .to_string())
}

/// Get detailed information about a specific road
pub fn handle_get_road_info(state: Arc<Mutex<ServerState>>, road_id: String) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let validator = state_lock
        .road_validator
        .as_ref()
        .ok_or_else(|| anyhow!("No road network loaded. Use load_road_network first."))?;

    let info = validator
        .get_road_info(&road_id)
        .ok_or_else(|| anyhow!("Road '{}' not found", road_id))?;

    Ok(json!(info).to_string())
}

/// Suggest valid spawn points for vehicles
pub fn handle_suggest_spawn_points(
    state: Arc<Mutex<ServerState>>,
    road_id: String,
    count: usize,
) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let validator = state_lock
        .road_validator
        .as_ref()
        .ok_or_else(|| anyhow!("No road network loaded. Use load_road_network first."))?;

    let points = validator
        .suggest_spawn_points(&road_id, count)
        .map_err(|e| anyhow!("Failed to generate spawn points: {}", e))?;

    Ok(json!({
        "spawn_points": points,
        "count": points.len(),
        "road_id": road_id
    })
    .to_string())
}

/// Validate a position against the loaded road network
pub fn handle_validate_position(
    state: Arc<Mutex<ServerState>>,
    road_id: String,
    lane_id: i32,
    s: f64,
) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let validator = state_lock
        .road_validator
        .as_ref()
        .ok_or_else(|| anyhow!("No road network loaded. Use load_road_network first."))?;

    // Validate road + s position
    validator
        .validate_road_position(&road_id, s)
        .map_err(|e| anyhow!("Position validation failed: {}", e))?;

    // Validate lane
    validator
        .validate_lane_position(&road_id, lane_id)
        .map_err(|e| anyhow!("Lane validation failed: {}", e))?;

    Ok(json!({
        "valid": true,
        "road_id": road_id,
        "lane_id": lane_id,
        "s": s,
        "message": "Position is valid"
    })
    .to_string())
}

/// Get a real-world road network from OpenStreetMap
pub fn handle_get_real_world_road(
    state: Arc<Mutex<ServerState>>,
    location: String,
    output_name: Option<String>,
) -> Result<String> {
    use std::process::Command;

    // Determine output name
    let name = output_name.unwrap_or_else(|| location.replace(' ', "_").to_lowercase());

    // Get workspace root (assuming we're in openscenario-mcp/)
    let workspace_root =
        std::env::current_dir().map_err(|e| anyhow!("Failed to get current directory: {}", e))?;

    let script_path = workspace_root.join("tools/osm/osm_to_opendrive.py");

    if !script_path.exists() {
        return Err(anyhow!(
            "OSM converter script not found at: {:?}",
            script_path
        ));
    }

    println!(
        "🌍 Fetching real-world road: {} (output: {})",
        location, name
    );

    // Run Python script
    let output = Command::new("python3")
        .arg(&script_path)
        .arg(&location)
        .arg("-o")
        .arg(&name)
        .current_dir(&workspace_root)
        .output()
        .map_err(|e| anyhow!("Failed to execute OSM converter: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(anyhow!(
            "OSM conversion failed:\nStdout: {}\nStderr: {}",
            stdout,
            stderr
        ));
    }

    // Expected output path
    let xodr_path = workspace_root.join(format!("cache/osm/{}.xodr", name));

    if !xodr_path.exists() {
        return Err(anyhow!("OpenDRIVE file not created: {:?}", xodr_path));
    }

    // Load the road network
    let validator = openscenario::opendrive_validator::OpenDriveValidator::load(&xodr_path)
        .map_err(|e| anyhow!("Failed to load generated OpenDRIVE: {}", e))?;

    // Analyze the network
    let quality = validator.assess_quality();
    let roads = validator.list_roads();

    // Find good roads (>50m, has lanes)
    let mut good_roads: Vec<_> = roads
        .iter()
        .filter(|r| r.length > 50.0 && r.lane_count > 1)
        .collect();
    good_roads.sort_by(|a, b| b.length.partial_cmp(&a.length).unwrap());

    // Get recommended road (longest good road)
    let recommended = good_roads.first().map(|r| {
        let spawn_points = validator.suggest_spawn_points(&r.id, 5).unwrap_or_default();
        json!({
            "road_id": r.id,
            "length": r.length,
            "lane_count": r.lane_count,
            "name": r.name,
            "spawn_points": spawn_points
        })
    });

    // Store validator in state
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    state_lock.road_validator = Some(validator);
    state_lock.current_road_network = Some(xodr_path.to_string_lossy().to_string());

    Ok(json!({
        "status": "success",
        "location": location,
        "xodr_path": xodr_path.to_string_lossy(),
        "total_roads": roads.len(),
        "good_roads": good_roads.len(),
        "quality": {
            "score": quality.score,
            "has_lanes": quality.has_lanes,
            "has_geometry": quality.has_geometry,
            "issues": quality.issues
        },
        "recommended_road": recommended,
        "top_roads": good_roads.iter().take(10).map(|r| json!({
            "id": r.id,
            "length": r.length,
            "lane_count": r.lane_count,
            "name": r.name
        })).collect::<Vec<_>>()
    })
    .to_string())
}
