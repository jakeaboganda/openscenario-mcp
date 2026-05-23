//! Lane Change Overtaking - Vehicle overtaking a slower vehicle
//!
//! Demonstrates:
//! - Two-vehicle interaction
//! - Lane-based positioning
//! - Speed differential for overtaking
//! - Multi-step overtaking maneuver
//!
//! Scenario: A faster vehicle approaches a slower vehicle and overtakes it
//!
//! Run with: cargo run --example lane_change_overtaking

use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::TransitionShape;
use openscenario::{OpenScenarioVersion, Position, Scenario};

fn main() -> Result<(), openscenario::ScenarioError> {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Set road network
    scenario.set_road_network("roads/simple_highway.xodr")?;

    // Add two vehicles
    let vehicle_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    // Slow vehicle ahead in right lane (lane -1)
    scenario.add_vehicle("slow_vehicle", vehicle_params.clone())?;
    scenario.set_initial_state(
        "slow_vehicle",
        Position::lane("1", -1, 80.0, 0.0, None),  // Lane -1 (right), s=80m
        Some(20.0),  // 20 m/s ≈ 72 km/h
    )?;

    // Fast vehicle behind in right lane (lane -1)
    scenario.add_vehicle("fast_vehicle", vehicle_params)?;
    scenario.set_initial_state(
        "fast_vehicle",
        Position::lane("1", -1, 20.0, 0.0, None),  // Lane -1 (right), s=20m
        Some(30.0),  // 30 m/s ≈ 108 km/h
    )?;

    // Create story structure for overtaking vehicle
    scenario.add_story("overtake_story")?;
    scenario.add_act("overtake_story", "overtake_act")?;
    scenario.add_maneuver_group("overtake_story", "overtake_act", "fast_group")?;
    scenario.add_actor("overtake_story", "overtake_act", "fast_group", "fast_vehicle")?;
    scenario.add_maneuver("overtake_story", "overtake_act", "fast_group", "overtake_maneuver")?;

    // Step 1: Change to left lane to overtake
    scenario.add_lane_change_action(
        "overtake_story",
        "overtake_act",
        "fast_group",
        "overtake_maneuver",
        "move_left",
        1.0, // Move one lane to the left
        3.0, // Take 3 seconds
        TransitionShape::Sinusoidal,
    )?;

    // Step 2: Return to right lane after passing
    scenario.add_lane_change_action(
        "overtake_story",
        "overtake_act",
        "fast_group",
        "overtake_maneuver",
        "move_right",
        -1.0, // Move one lane to the right (back to original)
        3.0,  // Take 3 seconds
        TransitionShape::Sinusoidal,
    )?;

    // Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("lane_change_overtaking.xosc", xml)?;

    println!("✅ Overtaking scenario exported to lane_change_overtaking.xosc");
    println!("   - Slow vehicle: Lane -1, s=80m, 20 m/s (72 km/h)");
    println!("   - Fast vehicle: Lane -1, s=20m, 30 m/s (108 km/h)");
    println!("   - Gap: 60m, closing at 10 m/s");
    println!("   - Action 1: Fast vehicle moves left to lane 1 (3s)");
    println!("   - Action 2: Fast vehicle returns to lane -1 (3s)");
    println!();
    println!("Visualization:");
    println!("  Fast vehicle approaches slow vehicle in right lane,");
    println!("  changes to left lane to overtake, passes, then");
    println!("  returns to right lane after passing.");
    println!();
    println!("To test in esmini:");
    println!("  esmini --osc lane_change_overtaking.xosc");

    Ok(())
}
