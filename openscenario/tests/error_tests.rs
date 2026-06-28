use openscenario::ScenarioError;

#[test]
fn test_entity_conflict_error() {
    let err = ScenarioError::EntityConflict {
        name: "car1".to_string(),
        existing_location: None,
    };

    assert!(err.to_string().contains("car1"));
    assert!(err.to_string().contains("already exists"));
}

#[test]
fn test_version_mismatch_error() {
    let err = ScenarioError::VersionMismatch {
        feature: "AppearanceAction".to_string(),
        required_version: "1.2".to_string(),
        current_version: "1.0".to_string(),
    };

    assert!(err.to_string().contains("AppearanceAction"));
    assert!(err.to_string().contains("1.2"));
}

#[test]
fn spawn_collision_error_message_includes_both_entity_names() {
    let err = ScenarioError::SpawnCollision {
        entity_a: "ego".to_string(),
        entity_b: "npc1".to_string(),
        distance: 1.5,
        min_clearance: 3.2,
    };
    let msg = err.to_string();
    assert!(msg.contains("ego"), "message missing entity_a: {}", msg);
    assert!(msg.contains("npc1"), "message missing entity_b: {}", msg);
}

#[test]
fn spawn_collision_error_message_includes_distances() {
    let err = ScenarioError::SpawnCollision {
        entity_a: "a".to_string(),
        entity_b: "b".to_string(),
        distance: 0.0,
        min_clearance: 4.72,
    };
    let msg = err.to_string();
    assert!(msg.contains("4.72") || msg.contains("4.7"), "clearance missing from: {}", msg);
}

#[test]
fn spawn_collision_is_scenario_error_variant() {
    let err: ScenarioError = ScenarioError::SpawnCollision {
        entity_a: "x".to_string(),
        entity_b: "y".to_string(),
        distance: 0.5,
        min_clearance: 2.0,
    };
    let _ = format!("{}", err);
}
