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
    };
    let msg = err.to_string();
    assert!(msg.contains("ego"), "message missing entity_a: {}", msg);
    assert!(msg.contains("npc1"), "message missing entity_b: {}", msg);
}

#[test]
fn spawn_collision_is_scenario_error_variant() {
    let err: ScenarioError = ScenarioError::SpawnCollision {
        entity_a: "x".to_string(),
        entity_b: "y".to_string(),
    };
    let _ = format!("{}", err);
}
