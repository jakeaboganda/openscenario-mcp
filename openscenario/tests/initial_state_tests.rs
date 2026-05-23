use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::{OpenScenarioVersion, Position, Scenario};

// ============================================================================
// Initial Speed Tests
// ============================================================================

#[test]
fn test_set_initial_speed() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let result = scenario.set_initial_speed("ego", 30.0);

    assert!(result.is_ok());
    assert_eq!(scenario.get_initial_speed("ego"), Some(&30.0));
}

#[test]
fn test_set_initial_speed_entity_not_found() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let result = scenario.set_initial_speed("nonexistent", 30.0);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("nonexistent"));
    assert!(err_msg.contains("not found"));
}

#[test]
fn test_set_initial_speed_empty_entity_name() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let result = scenario.set_initial_speed("", 30.0);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("empty"));
}

#[test]
fn test_set_initial_speed_whitespace_entity_name() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let result = scenario.set_initial_speed("   ", 30.0);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("empty") || err_msg.contains("whitespace"));
}

#[test]
fn test_set_initial_speed_negative_speed() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let result = scenario.set_initial_speed("ego", -10.0);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("speed"));
}

#[test]
fn test_set_initial_speed_zero_is_valid() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let result = scenario.set_initial_speed("ego", 0.0);

    assert!(result.is_ok());
    assert_eq!(scenario.get_initial_speed("ego"), Some(&0.0));
}

#[test]
fn test_get_initial_speed_not_set() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    assert_eq!(scenario.get_initial_speed("ego"), None);
}

#[test]
fn test_initial_speeds_iterator() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("car1", params.clone()).unwrap();
    scenario.add_vehicle("car2", params).unwrap();
    scenario.set_initial_speed("car1", 25.0).unwrap();
    scenario.set_initial_speed("car2", 30.0).unwrap();

    let speeds: Vec<_> = scenario.initial_speeds().collect();
    assert_eq!(speeds.len(), 2);
}

#[test]
fn test_set_initial_speed_overwrites() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    scenario.set_initial_speed("ego", 20.0).unwrap();
    scenario.set_initial_speed("ego", 30.0).unwrap();

    assert_eq!(scenario.get_initial_speed("ego"), Some(&30.0));
}

// ============================================================================
// Initial State Tests (Combined Position + Speed)
// ============================================================================

#[test]
fn test_set_initial_state_with_speed() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let position = Position::world(0.0, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_state("ego", position.clone(), Some(30.0));

    assert!(result.is_ok());
    assert_eq!(scenario.get_initial_position("ego"), Some(&position));
    assert_eq!(scenario.get_initial_speed("ego"), Some(&30.0));
}

#[test]
fn test_set_initial_state_without_speed() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let position = Position::world(0.0, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_state("ego", position.clone(), None);

    assert!(result.is_ok());
    assert_eq!(scenario.get_initial_position("ego"), Some(&position));
    assert_eq!(scenario.get_initial_speed("ego"), None);
}

#[test]
fn test_set_initial_state_entity_not_found() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let position = Position::world(0.0, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_state("nonexistent", position, Some(30.0));

    assert!(result.is_err());
}

#[test]
fn test_set_initial_state_negative_speed_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let position = Position::world(0.0, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_state("ego", position.clone(), Some(-10.0));

    assert!(result.is_err());
    // Note: Position IS set because set_initial_position() succeeds first,
    // then set_initial_speed() fails. This is acceptable behavior.
    assert_eq!(scenario.get_initial_position("ego"), Some(&position));
    assert_eq!(scenario.get_initial_speed("ego"), None);
}

// ============================================================================
// Road Network Tests
// ============================================================================

#[test]
fn test_set_road_network() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let result = scenario.set_road_network("roads/highway.xodr");

    assert!(result.is_ok());
    assert_eq!(
        scenario.get_road_network(),
        Some(&"roads/highway.xodr".to_string())
    );
}

#[test]
fn test_set_road_network_empty_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let result = scenario.set_road_network("");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("empty"));
}

#[test]
fn test_set_road_network_whitespace_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let result = scenario.set_road_network("   ");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("empty") || err_msg.contains("whitespace"));
}

#[test]
fn test_set_road_network_trims_whitespace() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    scenario
        .set_road_network("  roads/highway.xodr  ")
        .unwrap();

    assert_eq!(
        scenario.get_road_network(),
        Some(&"roads/highway.xodr".to_string())
    );
}

#[test]
fn test_get_road_network_not_set() {
    let scenario = Scenario::new(OpenScenarioVersion::V1_2);
    assert_eq!(scenario.get_road_network(), None);
}

#[test]
fn test_set_road_network_overwrites() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    scenario.set_road_network("roads/first.xodr").unwrap();
    scenario.set_road_network("roads/second.xodr").unwrap();

    assert_eq!(
        scenario.get_road_network(),
        Some(&"roads/second.xodr".to_string())
    );
}

// ============================================================================
// XML Generation Tests
// ============================================================================

#[test]
fn test_xml_with_initial_speed() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    scenario.set_initial_speed("ego", 25.0).unwrap();

    let xml = scenario.to_xml().unwrap();

    // Check that SpeedAction is present
    assert!(xml.contains("<SpeedAction>"));
    assert!(xml.contains("<AbsoluteTargetSpeed value=\"25\""));
    assert!(xml.contains("dynamicsShape=\"step\""));
}

#[test]
fn test_xml_speed_action_before_teleport_action() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    scenario.set_initial_speed("ego", 25.0).unwrap();

    let xml = scenario.to_xml().unwrap();

    // Find positions of SpeedAction and TeleportAction
    let speed_pos = xml.find("<SpeedAction>").unwrap();
    let teleport_pos = xml.find("<TeleportAction>").unwrap();

    // SpeedAction must come before TeleportAction
    assert!(
        speed_pos < teleport_pos,
        "SpeedAction must come before TeleportAction in Init section"
    );
}

#[test]
fn test_xml_with_road_network() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    scenario.set_road_network("roads/highway.xodr").unwrap();

    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("<RoadNetwork>"));
    assert!(xml.contains("<LogicFile filepath=\"roads/highway.xodr\""));
}

#[test]
fn test_xml_without_road_network() {
    let scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("<RoadNetwork/>"));
    assert!(!xml.contains("<LogicFile"));
}

#[test]
fn test_xml_with_lane_position_and_speed() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    scenario.set_road_network("roads/highway.xodr").unwrap();
    scenario
        .set_initial_state("ego", Position::lane("1", -2, 10.0, 0.0, None), Some(25.0))
        .unwrap();

    let xml = scenario.to_xml().unwrap();

    // Check all critical elements are present
    assert!(xml.contains("<LogicFile filepath=\"roads/highway.xodr\""));
    assert!(xml.contains("<LanePosition"));
    assert!(xml.contains("roadId=\"1\""));
    assert!(xml.contains("laneId=\"-2\""));
    assert!(xml.contains("<SpeedAction>"));
    assert!(xml.contains("<AbsoluteTargetSpeed value=\"25\""));
}
