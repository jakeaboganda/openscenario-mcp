//! Emergency Braking - Vehicle performs emergency stop
//!
//! Demonstrates:
//! - Lane-based positioning
//! - Initial speeds for both vehicles
//! - Collision condition detection
//! - Acceleration action (negative for braking)
//! - Safety-critical scenario testing
//!
//! Scenario: Follower vehicle approaching lead vehicle must brake to avoid collision
//!
//! Run with: cargo run --example emergency_braking

use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::Rule;
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

    // Lead vehicle (ahead, traveling slower)
    scenario.add_vehicle("lead_vehicle", vehicle_params.clone())?;
    scenario.set_initial_state(
        "lead_vehicle",
        Position::lane("1", -1, 100.0, 0.0, None), // Lane -1 (right lane), 100m ahead
        Some(20.0),                                // 20 m/s ≈ 72 km/h
    )?;

    // Following vehicle (behind, traveling faster)
    scenario.add_vehicle("follower_vehicle", vehicle_params)?;
    scenario.set_initial_state(
        "follower_vehicle",
        Position::lane("1", -1, 50.0, 0.0, None), // Same lane, 50m behind lead
        Some(30.0),                               // 30 m/s ≈ 108 km/h (faster, will catch up)
    )?;

    // Create story structure
    scenario.add_story("emergency_story")?;
    scenario.add_act("emergency_story", "brake_act")?;
    scenario.add_maneuver_group("emergency_story", "brake_act", "follower_group")?;
    scenario.add_actor(
        "emergency_story",
        "brake_act",
        "follower_group",
        "follower_vehicle",
    )?;
    scenario.add_maneuver(
        "emergency_story",
        "brake_act",
        "follower_group",
        "emergency_brake_maneuver",
    )?;

    // Condition: Time-to-collision < 3 seconds (collision imminent)
    scenario.add_event_with_ttc_condition(
        "emergency_story",
        "brake_act",
        "follower_group",
        "emergency_brake_maneuver",
        "collision_risk",
        "follower_vehicle", // Entity being monitored
        "lead_vehicle",     // Target entity
        3.0,                // TTC threshold: 3 seconds
        Rule::LessThan,     // Trigger when TTC < 3s
    )?;

    // Action: Emergency brake (strong deceleration: -8 m/s²)
    scenario.add_acceleration_action(
        "emergency_story",
        "brake_act",
        "follower_group",
        "emergency_brake_maneuver",
        "collision_risk", // Same event name links condition to action
        -8.0,             // Deceleration: -8 m/s² (emergency braking)
        2.0,              // Duration: 2 seconds
        None,             // No specific dynamics
    )?;

    // Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("emergency_braking.xosc", xml)?;

    println!("✅ Emergency braking scenario exported to emergency_braking.xosc");
    println!("   - Lead vehicle: Lane -1, s=100m, 20 m/s (72 km/h)");
    println!("   - Follower vehicle: Lane -1, s=50m, 30 m/s (108 km/h)");
    println!("   - Gap: 50m, closing at 10 m/s");
    println!("   - Condition: Time-to-collision < 3 seconds");
    println!("   - Action: Emergency brake at -8 m/s² for 2s");
    println!();
    println!("Visualization:");
    println!("  Follower (fast) approaches lead (slow) in same lane.");
    println!("  When collision risk detected, follower brakes hard.");
    println!();
    println!("To test in esmini:");
    println!("  esmini --osc emergency_braking.xosc");

    Ok(())
}
