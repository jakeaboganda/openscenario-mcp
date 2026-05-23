//! Platooning - Multiple vehicles following in coordinated formation
//!
//! Demonstrates:
//! - Multi-vehicle coordination
//! - Lane-based positioning
//! - Initial speeds for convoy
//! - Time headway conditions for multiple followers
//! - Platoon formation with consistent spacing
//!
//! Scenario: Three vehicles maintain 1.5-second time headway in a platoon
//!
//! Run with: cargo run --example platooning

use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::Rule;
use openscenario::{OpenScenarioVersion, Position, Scenario};

fn main() -> Result<(), openscenario::ScenarioError> {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Set road network
    scenario.set_road_network("roads/simple_highway.xodr")?;

    let vehicle_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    // Platoon leader (front vehicle)
    scenario.add_vehicle("leader", vehicle_params.clone())?;
    scenario.set_initial_state(
        "leader",
        Position::lane("1", -1, 80.0, 0.0, None),  // Lane -1, s=80m
        Some(25.0),  // 25 m/s ≈ 90 km/h
    )?;

    // Follower 1 (middle vehicle, follows leader at ~40m = 1.6s @ 25 m/s)
    scenario.add_vehicle("follower1", vehicle_params.clone())?;
    scenario.set_initial_state(
        "follower1",
        Position::lane("1", -1, 40.0, 0.0, None),  // Lane -1, s=40m
        Some(25.0),  // 25 m/s ≈ 90 km/h
    )?;

    // Follower 2 (rear vehicle, follows follower1 at ~40m)
    scenario.add_vehicle("follower2", vehicle_params)?;
    scenario.set_initial_state(
        "follower2",
        Position::lane("1", -1, 0.0, 0.0, None),  // Lane -1, s=0m (start)
        Some(25.0),  // 25 m/s ≈ 90 km/h
    )?;

    // Story structure
    scenario.add_story("platoon_story")?;
    scenario.add_act("platoon_story", "follow_act")?;

    // Follower 1 group
    scenario.add_maneuver_group("platoon_story", "follow_act", "follower1_group")?;
    scenario.add_actor("platoon_story", "follow_act", "follower1_group", "follower1")?;
    scenario.add_maneuver(
        "platoon_story",
        "follow_act",
        "follower1_group",
        "follow1_maneuver",
    )?;

    // Follower 1: Maintain 1.5s headway from leader
    scenario.add_event_with_time_headway_condition(
        "platoon_story",
        "follow_act",
        "follower1_group",
        "follow1_maneuver",
        "maintain_headway1",
        "follower1",
        "leader",
        1.5,
        Rule::LessThan,
        true,
    )?;

    scenario.add_speed_profile_action(
        "platoon_story",
        "follow_act",
        "follower1_group",
        "follow1_maneuver",
        "maintain_headway1",
        vec![(0.0, 25.0), (2.0, 22.0)], // Adjust speed to maintain distance
        true,
    )?;

    // Follower 2 group
    scenario.add_maneuver_group("platoon_story", "follow_act", "follower2_group")?;
    scenario.add_actor("platoon_story", "follow_act", "follower2_group", "follower2")?;
    scenario.add_maneuver(
        "platoon_story",
        "follow_act",
        "follower2_group",
        "follow2_maneuver",
    )?;

    // Follower 2: Maintain 1.5s headway from follower1
    scenario.add_event_with_time_headway_condition(
        "platoon_story",
        "follow_act",
        "follower2_group",
        "follow2_maneuver",
        "maintain_headway2",
        "follower2",
        "follower1",
        1.5,
        Rule::LessThan,
        true,
    )?;

    scenario.add_speed_profile_action(
        "platoon_story",
        "follow_act",
        "follower2_group",
        "follow2_maneuver",
        "maintain_headway2",
        vec![(0.0, 25.0), (2.0, 22.0)], // Adjust speed to maintain distance
        true,
    )?;

    // Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("platooning.xosc", xml)?;

    println!("✅ Platooning scenario exported to platooning.xosc");
    println!("   - Leader: Lane -1, s=80m, 25 m/s (90 km/h)");
    println!("   - Follower 1: Lane -1, s=40m, 25 m/s (90 km/h) - 40m gap");
    println!("   - Follower 2: Lane -1, s=0m, 25 m/s (90 km/h) - 40m gap");
    println!("   - Formation: 40m spacing (≈1.6s time headway @ 25 m/s)");
    println!();
    println!("Visualization:");
    println!("  Three vehicles travel in convoy formation in the right lane.");
    println!("  All maintain the same speed with consistent spacing.");
    println!("  Time headway conditions trigger adjustments if spacing changes.");
    println!();
    println!("To test in esmini:");
    println!("  esmini --osc platooning.xosc");

    Ok(())
}
