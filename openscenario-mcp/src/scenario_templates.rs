/// Scenario template helpers for common autonomous vehicle testing scenarios
use crate::server::ServerState;
use anyhow::{anyhow, Result};
use openscenario::{
    entities::{VehicleCategory, VehicleParams},
    OpenScenarioVersion, Position, Scenario,
};
use serde_json::json;
use std::sync::{Arc, Mutex};

/// Helper to create VehicleParams
fn car_params() -> VehicleParams {
    VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    }
}

/// Generate a lane change scenario with ego vehicle and one other vehicle
pub fn handle_create_lane_change_scenario(
    state: Arc<Mutex<ServerState>>,
    road_id: String,
    lane_from: i32,
    lane_to: i32,
    ego_start_s: f64,
    other_start_s: f64,
    other_lane: i32,
    ego_speed: f64,
    other_speed: f64,
    scenario_name: Option<String>,
) -> Result<String> {
    let name = scenario_name.unwrap_or_else(|| "lane_change_scenario".to_string());

    // Create scenario
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add vehicles
    scenario.add_vehicle("ego", car_params())?;
    scenario.add_vehicle("other", car_params())?;

    // Set initial positions
    let _ = scenario.set_initial_position(
        "ego",
        Position::lane(road_id.clone(), lane_from, ego_start_s, 0.0, None),
    );

    let _ = scenario.set_initial_position(
        "other",
        Position::lane(road_id.clone(), other_lane, other_start_s, 0.0, None),
    );

    // Set initial speeds
    let _ = scenario.set_initial_speed("ego", ego_speed);
    let _ = scenario.set_initial_speed("other", other_speed);

    // Add lane change action for ego
    scenario.add_lane_change_action(
        "LaneChangeStory",
        "LaneChangeAct",
        "EgoGroup",
        "LaneChangeManeuver",
        "LaneChangeEvent",
        (lane_to - lane_from) as f64, // offset from current lane
        5.0,                          // 5 second duration
        openscenario::storyboard::TransitionShape::Linear,
    )?;

    // Store in state
    let scenario_id = uuid::Uuid::new_v4().to_string();
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    state_lock.scenarios.insert(scenario_id.clone(), scenario);

    Ok(json!({
        "scenario_id": scenario_id,
        "scenario_name": name,
        "type": "lane_change",
        "vehicles": ["ego", "other"],
        "ego_lane_change": {
            "from": lane_from,
            "to": lane_to,
            "duration_s": 5.0
        },
        "initial_speeds": {
            "ego": ego_speed,
            "other": other_speed
        }
    })
    .to_string())
}

/// Generate a highway merge scenario
pub fn handle_create_merge_scenario(
    state: Arc<Mutex<ServerState>>,
    main_road_id: String,
    merge_road_id: String,
    ego_start_s: f64,
    other_start_s: f64,
    target_lane: i32,
    ego_speed: f64,
    other_speed: f64,
    scenario_name: Option<String>,
) -> Result<String> {
    let name = scenario_name.unwrap_or_else(|| "merge_scenario".to_string());

    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add vehicles
    scenario.add_vehicle("ego", car_params())?;
    scenario.add_vehicle("other", car_params())?;

    // Ego starts on merge road
    let _ = scenario.set_initial_position(
        "ego",
        Position::lane(merge_road_id.clone(), -1, ego_start_s, 0.0, None),
    );

    // Other vehicle on main road
    let _ = scenario.set_initial_position(
        "other",
        Position::lane(main_road_id.clone(), -1, other_start_s, 0.0, None),
    );

    let _ = scenario.set_initial_speed("ego", ego_speed);
    let _ = scenario.set_initial_speed("other", other_speed);

    // Ego merges into target lane
    scenario.add_lane_change_action(
        "MergeStory",
        "MergeAct",
        "EgoGroup",
        "MergeManeuver",
        "MergeEvent",
        target_lane as f64 - (-1.0), // offset from current lane
        4.0,                         // 4 second merge
        openscenario::storyboard::TransitionShape::Sinusoidal,
    )?;

    let scenario_id = uuid::Uuid::new_v4().to_string();
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    state_lock.scenarios.insert(scenario_id.clone(), scenario);

    Ok(json!({
        "scenario_id": scenario_id,
        "scenario_name": name,
        "type": "merge",
        "vehicles": ["ego", "other"],
        "merge": {
            "from_road": merge_road_id,
            "to_road": main_road_id,
            "target_lane": target_lane,
            "duration_s": 4.0
        },
        "initial_speeds": {
            "ego": ego_speed,
            "other": other_speed
        }
    })
    .to_string())
}

/// Generate a cut-in scenario (other vehicle cuts in front of ego)
pub fn handle_create_cutin_scenario(
    state: Arc<Mutex<ServerState>>,
    road_id: String,
    ego_lane: i32,
    other_lane: i32,
    ego_start_s: f64,
    other_start_s: f64,
    ego_speed: f64,
    other_speed: f64,
    cutin_trigger_distance: f64,
    scenario_name: Option<String>,
) -> Result<String> {
    let name = scenario_name.unwrap_or_else(|| "cutin_scenario".to_string());

    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    scenario.add_vehicle("ego", car_params())?;
    scenario.add_vehicle("other", car_params())?;

    // Set positions
    let _ = scenario.set_initial_position(
        "ego",
        Position::lane(road_id.clone(), ego_lane, ego_start_s, 0.0, None),
    );

    let _ = scenario.set_initial_position(
        "other",
        Position::lane(road_id.clone(), other_lane, other_start_s, 0.0, None),
    );

    let _ = scenario.set_initial_speed("ego", ego_speed);
    let _ = scenario.set_initial_speed("other", other_speed);

    // Other vehicle cuts into ego's lane
    scenario.add_lane_change_action(
        "CutInStory",
        "CutInAct",
        "OtherGroup",
        "CutInManeuver",
        "CutInEvent",
        (ego_lane - other_lane) as f64, // offset to ego's lane
        2.5,                            // 2.5 second aggressive cut-in
        openscenario::storyboard::TransitionShape::Cubic,
    )?;

    let scenario_id = uuid::Uuid::new_v4().to_string();
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    state_lock.scenarios.insert(scenario_id.clone(), scenario);

    Ok(json!({
        "scenario_id": scenario_id,
        "scenario_name": name,
        "type": "cutin",
        "vehicles": ["ego", "other"],
        "cutin": {
            "from_lane": other_lane,
            "to_lane": ego_lane,
            "trigger_distance": cutin_trigger_distance,
            "duration_s": 2.5
        },
        "initial_speeds": {
            "ego": ego_speed,
            "other": other_speed
        }
    })
    .to_string())
}

/// Generate a platoon following scenario (multiple vehicles in convoy)
pub fn handle_create_platoon_scenario(
    state: Arc<Mutex<ServerState>>,
    road_id: String,
    lane_id: i32,
    vehicle_count: usize,
    start_s: f64,
    spacing: f64,
    speed: f64,
    scenario_name: Option<String>,
) -> Result<String> {
    if !(2..=10).contains(&vehicle_count) {
        return Err(anyhow!("Vehicle count must be between 2 and 10"));
    }

    let name = scenario_name.unwrap_or_else(|| "platoon_scenario".to_string());

    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    let mut vehicle_names = Vec::new();

    // Add vehicles
    for i in 0..vehicle_count {
        let vehicle_name = if i == 0 {
            "ego".to_string()
        } else {
            format!("vehicle_{}", i)
        };

        scenario.add_vehicle(&vehicle_name, car_params())?;

        // Position with spacing
        let s_position = start_s + (i as f64 * spacing);
        let _ = scenario.set_initial_position(
            &vehicle_name,
            Position::lane(road_id.clone(), lane_id, s_position, 0.0, None),
        );

        let _ = scenario.set_initial_speed(&vehicle_name, speed);
        vehicle_names.push(vehicle_name);
    }

    let scenario_id = uuid::Uuid::new_v4().to_string();
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    state_lock.scenarios.insert(scenario_id.clone(), scenario);

    Ok(json!({
        "scenario_id": scenario_id,
        "scenario_name": name,
        "type": "platoon",
        "vehicles": vehicle_names,
        "platoon": {
            "count": vehicle_count,
            "spacing_m": spacing,
            "lane": lane_id,
            "speed_mps": speed
        }
    })
    .to_string())
}

/// Generate a complete scenario on a recommended road
pub fn handle_create_quick_scenario(
    state: Arc<Mutex<ServerState>>,
    scenario_type: String,
    vehicle_count: Option<usize>,
) -> Result<String> {
    // Get the loaded road network
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let validator = state_lock.road_validator.as_ref().ok_or_else(|| {
        anyhow!("No road network loaded. Use get_real_world_road or load_road_network first.")
    })?;

    // Find a good road
    let roads = validator.list_roads();
    let mut good_roads: Vec<_> = roads
        .iter()
        .filter(|r| r.length > 200.0 && r.lane_count > 1)
        .collect();
    good_roads.sort_by(|a, b| b.length.partial_cmp(&a.length).unwrap());

    let best_road = good_roads
        .first()
        .ok_or_else(|| anyhow!("No suitable roads found. Road must be >200m with lanes."))?;

    let road_id = best_road.id.clone();

    // Get spawn points
    let count = vehicle_count.unwrap_or(3).min(5);
    let spawn_points = validator
        .suggest_spawn_points(&road_id, count)
        .map_err(|e| anyhow!("Failed to get spawn points: {}", e))?;

    if spawn_points.len() < 2 {
        return Err(anyhow!("Not enough spawn points available"));
    }

    // Release lock before calling other handlers
    drop(state_lock);

    // Generate scenario based on type
    match scenario_type.as_str() {
        "lane_change" => {
            let ego = &spawn_points[0];
            let other = &spawn_points[1];
            handle_create_lane_change_scenario(
                state,
                road_id,
                ego.lane_id,
                ego.lane_id + 1, // change to adjacent lane
                ego.s,
                other.s,
                other.lane_id,
                25.0, // 25 m/s (90 km/h)
                20.0, // 20 m/s (72 km/h)
                Some(format!("quick_{}_scenario", scenario_type)),
            )
        }
        "cutin" => {
            let ego = &spawn_points[0];
            let other = &spawn_points[1];
            handle_create_cutin_scenario(
                state,
                road_id,
                ego.lane_id,
                ego.lane_id + 1,
                ego.s,
                other.s + 30.0, // other starts 30m ahead
                25.0,
                23.0,
                15.0, // cut in when 15m apart
                Some(format!("quick_{}_scenario", scenario_type)),
            )
        }
        "platoon" => {
            let first = &spawn_points[0];
            handle_create_platoon_scenario(
                state,
                road_id,
                first.lane_id,
                count,
                first.s,
                40.0, // 40m spacing
                25.0,
                Some(format!("quick_{}_scenario", scenario_type)),
            )
        }
        _ => Err(anyhow!(
            "Unknown scenario type: {}. Use 'lane_change', 'cutin', or 'platoon'",
            scenario_type
        )),
    }
}
