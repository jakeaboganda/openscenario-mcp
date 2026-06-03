use openscenario_mcp::handlers::{
    handle_add_lane_change_action, handle_add_speed_action, handle_add_vehicle,
    handle_create_scenario, handle_export_xml, handle_load_road_network, handle_set_position,
};
use openscenario_mcp::server::ServerState;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

// Minimal OpenDRIVE XML for testing
const MINIMAL_XODR: &str = r###"<?xml version="1.0" encoding="UTF-8"?>
<OpenDRIVE>
    <header revMajor="1" revMinor="6" name="test_road" version="1.0" date="2026-05-31T00:00:00"/>
    <road name="test_road" length="1000.0" id="1" junction="-1">
        <link/>
        <planView>
            <geometry s="0.0" x="0.0" y="0.0" hdg="0.0" length="1000.0">
                <line/>
            </geometry>
        </planView>
        <lanes>
            <laneSection s="0.0">
                <center>
                    <lane id="0" type="none" level="false">
                        <link/>
                    </lane>
                </center>
                <right>
                    <lane id="-1" type="driving" level="false">
                        <link/>
                        <width sOffset="0.0" a="3.5" b="0.0" c="0.0" d="0.0"/>
                    </lane>
                </right>
            </laneSection>
        </lanes>
    </road>
</OpenDRIVE>
"###;

fn setup_state() -> Arc<Mutex<ServerState>> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let state = Arc::new(Mutex::new(ServerState::new()));

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let xodr_path = format!("/tmp/test_mcp_road_{}.xodr", timestamp);
    fs::write(&xodr_path, MINIMAL_XODR).expect("Failed to write test XODR");

    let _ = handle_load_road_network(state.clone(), xodr_path.clone());
    let _ = fs::remove_file(&xodr_path);

    state
}

#[test]
fn test_add_speed_action_handler() {
    let state = setup_state();

    // Create scenario and add vehicle
    let scenario_id = handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string(),
    )
    .unwrap();

    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "Car".to_string(),
        None,
    )
    .unwrap();

    // Add speed action
    let result = handle_add_speed_action(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "test_story".to_string(),
        50.0,
        5.0,
        None,
    );

    assert!(result.is_ok());
    assert!(result.unwrap().contains("Speed action added"));
}

#[test]
fn test_add_lane_change_action_handler() {
    let state = setup_state();

    // Create scenario and add vehicle
    let scenario_id = handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string(),
    )
    .unwrap();

    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "Car".to_string(),
        None,
    )
    .unwrap();

    // Add lane change action
    let result = handle_add_lane_change_action(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "test_story".to_string(),
        -3.5,
        4.0,
        None,
    );

    assert!(result.is_ok());
    assert!(result.unwrap().contains("Lane change action added"));
}

#[test]
fn test_export_xml_handler() {
    let state = setup_state();

    // Create scenario and add vehicle
    let scenario_id = handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string(),
    )
    .unwrap();

    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "Car".to_string(),
        None,
    )
    .unwrap();

    handle_set_position(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        10.0,
        20.0,
        0.0,
        0.0,
    )
    .unwrap();

    // Export to XML
    let output_path = "/tmp/test_scenario_export.xosc";
    let result = handle_export_xml(state.clone(), scenario_id.clone(), output_path.to_string());

    assert!(result.is_ok());
    assert!(result.unwrap().contains("Exported scenario to"));

    // Verify file exists and contains valid XML
    assert!(Path::new(output_path).exists());
    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains("<?xml"));
    assert!(content.contains("OpenSCENARIO"));
    assert!(content.contains("ego_vehicle"));

    // Cleanup
    fs::remove_file(output_path).ok();
}
