//! Highway Merge - Vehicle merging onto highway from on-ramp
//!
//! Demonstrates:
//! - Lane-based initial position
//! - Initial speed
//! - Lane change maneuver
//!
//! Scenario: A vehicle merges from right lane into center lane
//!
//! Run with: cargo run --example highway_merge

use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::TransitionShape;
use openscenario::{OpenScenarioVersion, Position, Scenario};

fn main() -> Result<(), openscenario::ScenarioError> {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Set road network
    scenario.set_road_network("roads/simple_highway.xodr")?;

    // Add merging vehicle
    let vehicle_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("merging_vehicle", vehicle_params)?;

    // Start in far-right lane (lane -2) at 25 m/s (~90 km/h)
    // Using lane-based position: road "1", lane -2, s=10m along road
    scenario.set_initial_state(
        "merging_vehicle",
        Position::lane("1", -2, 10.0, 0.0, None),
        Some(25.0), // 25 m/s ≈ 90 km/h
    )?;

    // Create story structure
    scenario.add_story("merge_story")?;
    scenario.add_act("merge_story", "merge_act")?;
    scenario.add_maneuver_group("merge_story", "merge_act", "merge_group")?;
    scenario.add_actor("merge_story", "merge_act", "merge_group", "merging_vehicle")?;
    scenario.add_maneuver("merge_story", "merge_act", "merge_group", "merge_maneuver")?;

    // Merge left from lane -2 to lane -1 (one lane to the left)
    scenario.add_lane_change_action(
        "merge_story",
        "merge_act",
        "merge_group",
        "merge_maneuver",
        "merge_left",
        1.0,  // Move one lane to the left (relative: +1)
        4.0,  // Take 4 seconds to merge
        TransitionShape::Sinusoidal,
    )?;

    // Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("highway_merge.xosc", xml)?;

    println!("✅ Highway merge scenario exported to highway_merge.xosc");
    println!("   - Vehicle: merging_vehicle");
    println!("   - Initial position: Lane -2 (far right), s=10m");
    println!("   - Initial speed: 25 m/s (90 km/h)");
    println!("   - Action: Merge left into lane -1 over 4s");
    println!();
    println!("Visualization:");
    println!("  The vehicle starts in the far-right lane traveling at 90 km/h");
    println!("  and smoothly merges one lane to the left using sinusoidal dynamics.");
    println!();
    println!("To test in esmini:");
    println!("  esmini --osc highway_merge.xosc --odr roads/simple_highway.xodr");

    Ok(())
}
