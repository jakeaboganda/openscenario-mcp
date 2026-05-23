use openscenario::storyboard::ConditionEdge;
use openscenario::{OpenScenarioVersion, Position, Scenario, ScenarioError};

#[test]
fn test_reach_position_condition_basic() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add vehicle
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    // Add story structure
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    // Add event with ReachPositionCondition
    let target_position = Position::world(100.0, 50.0, 0.0, 0.0);
    let result = scenario.add_event_with_reach_position_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        target_position,
        2.0, // tolerance in meters
    );

    assert!(result.is_ok());
}

#[test]
fn test_reach_position_condition_with_edge() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    let target_position = Position::world(100.0, 50.0, 0.0, 0.0);

    // Test with rising edge
    let result = scenario.add_event_with_reach_position_condition_advanced(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        target_position,
        2.0,
        ConditionEdge::Rising,
        0.0, // no delay
    );

    assert!(result.is_ok());
}

#[test]
fn test_reach_position_condition_negative_tolerance_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    let target_position = Position::world(100.0, 50.0, 0.0, 0.0);

    // Negative tolerance should fail
    let result = scenario.add_event_with_reach_position_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        target_position,
        -1.0, // invalid
    );

    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidValue { field, .. }) => {
            assert!(field.contains("tolerance"));
        }
        _ => panic!("Expected InvalidValue error"),
    }
}

#[test]
fn test_reach_position_condition_zero_tolerance() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    let target_position = Position::world(100.0, 50.0, 0.0, 0.0);

    // Zero tolerance should be valid (exact position match)
    let result = scenario.add_event_with_reach_position_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        target_position,
        0.0,
    );

    assert!(result.is_ok());
}

#[test]
fn test_reach_position_condition_nonexistent_entity_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    let target_position = Position::world(100.0, 50.0, 0.0, 0.0);

    // Reference non-existent entity
    let result = scenario.add_event_with_reach_position_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "nonexistent", // Not in scenario
        target_position,
        2.0,
    );

    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidEntityRef { .. }) => (),
        _ => panic!("Expected InvalidEntityRef error"),
    }
}

#[test]
fn test_reach_position_condition_different_position_types() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    // Test World position
    let world_pos = Position::world(100.0, 50.0, 0.0, 0.0);
    let result1 = scenario.add_event_with_reach_position_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event_world",
        "ego",
        world_pos,
        2.0,
    );
    assert!(result1.is_ok());

    // Test Lane position
    let lane_pos = Position::lane("road1".to_string(), 1, 100.0, 0.0, None);
    let result2 = scenario.add_event_with_reach_position_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event_lane",
        "ego",
        lane_pos,
        2.0,
    );
    assert!(result2.is_ok());

    // Test Road position
    let road_pos = Position::road("road1".to_string(), 100.0, 1.5, None);
    let result3 = scenario.add_event_with_reach_position_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event_road",
        "ego",
        road_pos,
        2.0,
    );
    assert!(result3.is_ok());
}

#[test]
fn test_reach_position_condition_xml_export() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    let target_position = Position::world(100.0, 50.0, 0.0, 0.0);
    scenario
        .add_event_with_reach_position_condition(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "event1",
            "ego",
            target_position,
            2.0,
        )
        .unwrap();

    let xml = scenario.to_xml().unwrap();

    // Check for ReachPositionCondition presence
    assert!(xml.contains("<ByEntityCondition>"));
    assert!(xml.contains("<ReachPositionCondition"));
    assert!(xml.contains("tolerance=\"2\""));
    assert!(xml.contains("<Position>"));
    assert!(xml.contains("<WorldPosition"));
    assert!(xml.contains("x=\"100\""));
    assert!(xml.contains("y=\"50\""));
}

#[test]
fn test_reach_position_condition_multiple_conditions() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    // Add multiple events with position conditions
    let pos1 = Position::world(50.0, 0.0, 0.0, 0.0);
    scenario
        .add_event_with_reach_position_condition(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "checkpoint1",
            "ego",
            pos1,
            1.0,
        )
        .unwrap();

    let pos2 = Position::world(100.0, 0.0, 0.0, 0.0);
    scenario
        .add_event_with_reach_position_condition(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "checkpoint2",
            "ego",
            pos2,
            1.0,
        )
        .unwrap();

    let pos3 = Position::world(150.0, 0.0, 0.0, 0.0);
    scenario
        .add_event_with_reach_position_condition(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "checkpoint3",
            "ego",
            pos3,
            1.0,
        )
        .unwrap();

    let xml = scenario.to_xml().unwrap();

    // Should contain all three checkpoints
    assert!(xml.contains("checkpoint1"));
    assert!(xml.contains("checkpoint2"));
    assert!(xml.contains("checkpoint3"));
}

#[test]
fn test_reach_position_condition_all_edges() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    let target_position = Position::world(100.0, 50.0, 0.0, 0.0);

    // Test all edge types
    let edges = vec![
        ConditionEdge::None,
        ConditionEdge::Rising,
        ConditionEdge::Falling,
        ConditionEdge::RisingOrFalling,
    ];

    for edge in edges.iter() {
        let result = scenario.add_event_with_reach_position_condition_advanced(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            &format!("event_{:?}", edge),
            "ego",
            target_position.clone(),
            2.0,
            *edge,
            0.0,
        );

        assert!(result.is_ok(), "Failed for edge: {:?}", edge);
    }
}

#[test]
fn test_reach_position_condition_with_delay() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    let target_position = Position::world(100.0, 50.0, 0.0, 0.0);

    // Add condition with 2.5 second delay
    let result = scenario.add_event_with_reach_position_condition_advanced(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        target_position,
        2.0,
        ConditionEdge::Rising,
        2.5, // delay in seconds
    );

    assert!(result.is_ok());

    let xml = scenario.to_xml().unwrap();
    assert!(xml.contains("delay=\"2.5\""));
}
