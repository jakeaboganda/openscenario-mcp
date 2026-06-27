use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::{
    Action, ByValueCondition, Condition, ConditionEdge, ConditionGroup, ConditionKind,
    DynamicsDimension, DynamicsShape, Rule, TransitionDynamics, TransitionShape, Trigger,
};
use openscenario::{OpenScenarioVersion, Position, Scenario};

fn car(name: &str) -> (&str, VehicleParams) {
    (
        name,
        VehicleParams {
            catalog: None,
            vehicle_category: VehicleCategory::Car,
            properties: None,
        },
    )
}

fn linear_dynamics(value: f64) -> TransitionDynamics {
    TransitionDynamics {
        shape: DynamicsShape::Linear,
        dimension: DynamicsDimension::Time,
        value,
    }
}

fn build_basic_scenario_with_speed_action(speed: f64) -> Scenario {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (name, params) = car("ego");
    s.add_vehicle(name, params).unwrap();
    s.set_initial_position("ego", Position::world(10.0, 20.0, 0.0, 1.57))
        .unwrap();
    s.set_initial_speed("ego", 15.0).unwrap();
    s.add_story("story1").unwrap();
    s.add_act("story1", "act1").unwrap();
    s.add_maneuver_group("story1", "act1", "ego_mg").unwrap();
    s.add_actor("story1", "act1", "ego_mg", "ego").unwrap();
    s.add_maneuver("story1", "act1", "ego_mg", "maneuver1")
        .unwrap();
    s.add_speed_action(
        "story1",
        "act1",
        "ego_mg",
        "maneuver1",
        "event1",
        speed,
        linear_dynamics(5.0),
    )
    .unwrap();
    s
}

// parse_init tests

#[test]
fn roundtrip_world_position_coordinates() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (name, params) = car("ego");
    s.add_vehicle(name, params).unwrap();
    s.set_initial_position("ego", Position::world(42.5, -10.0, 3.0, 0.785))
        .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let pos = parsed
        .get_initial_position("ego")
        .expect("ego should have initial position after import");
    match pos {
        Position::World { x, y, z, h, .. } => {
            assert!((x - 42.5).abs() < 1e-6, "x should be 42.5, got {}", x);
            assert!((y - (-10.0)).abs() < 1e-6, "y should be -10.0, got {}", y);
            assert!((z - 3.0).abs() < 1e-6, "z should be 3.0, got {}", z);
            assert!((h - 0.785).abs() < 1e-6, "h should be 0.785, got {}", h);
        }
        other => panic!("expected WorldPosition, got {:?}", other),
    }
}

#[test]
fn roundtrip_world_position_heading_pitch_roll() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (name, params) = car("ego");
    s.add_vehicle(name, params).unwrap();
    s.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 1.0))
        .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let pos = parsed
        .get_initial_position("ego")
        .expect("ego should have initial position");
    match pos {
        Position::World { h, .. } => {
            assert!((h - 1.0).abs() < 1e-6, "heading should be 1.0, got {}", h);
        }
        other => panic!("expected WorldPosition, got {:?}", other),
    }
}

#[test]
fn roundtrip_lane_position() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (name, params) = car("ego");
    s.add_vehicle(name, params).unwrap();
    s.set_initial_position(
        "ego",
        Position::Lane {
            road_id: "road42".to_string(),
            lane_id: -1,
            s: 25.5,
            offset: 0.3,
            orientation: None,
        },
    )
    .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let pos = parsed
        .get_initial_position("ego")
        .expect("ego should have initial lane position after import");
    match pos {
        Position::Lane {
            road_id,
            lane_id,
            s,
            offset,
            ..
        } => {
            assert_eq!(road_id, "road42");
            assert_eq!(*lane_id, -1);
            assert!((s - 25.5).abs() < 1e-6, "s should be 25.5, got {}", s);
            assert!(
                (offset - 0.3).abs() < 1e-6,
                "offset should be 0.3, got {}",
                offset
            );
        }
        other => panic!("expected LanePosition, got {:?}", other),
    }
}

#[test]
fn roundtrip_initial_speed() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (name, params) = car("ego");
    s.add_vehicle(name, params).unwrap();
    s.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    s.set_initial_speed("ego", 27.8).unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let speed = parsed
        .get_initial_speed("ego")
        .expect("ego should have initial speed after import");
    assert!(
        (speed - 27.8).abs() < 1e-6,
        "speed should be 27.8, got {}",
        speed
    );
}

#[test]
fn roundtrip_position_and_speed_together() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (name, params) = car("ego");
    s.add_vehicle(name, params).unwrap();
    s.set_initial_position("ego", Position::world(5.0, 5.0, 0.0, 0.0))
        .unwrap();
    s.set_initial_speed("ego", 10.0).unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    assert!(
        parsed.get_initial_position("ego").is_some(),
        "position should survive roundtrip"
    );
    let speed = parsed
        .get_initial_speed("ego")
        .expect("speed should survive roundtrip");
    assert!((speed - 10.0).abs() < 1e-6);
}

#[test]
fn roundtrip_two_entities_keep_distinct_positions() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (n, p) = car("ego");
    s.add_vehicle(n, p).unwrap();
    let (n, p) = car("npc");
    s.add_vehicle(n, p).unwrap();

    s.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    s.set_initial_position("npc", Position::world(50.0, 0.0, 0.0, 0.0))
        .unwrap();
    s.set_initial_speed("ego", 10.0).unwrap();
    s.set_initial_speed("npc", 20.0).unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let ego_pos = parsed
        .get_initial_position("ego")
        .expect("ego needs position");
    let npc_pos = parsed
        .get_initial_position("npc")
        .expect("npc needs position");

    match (ego_pos, npc_pos) {
        (Position::World { x: ex, .. }, Position::World { x: nx, .. }) => {
            assert!((ex - 0.0).abs() < 1e-6, "ego x should be 0.0");
            assert!((nx - 50.0).abs() < 1e-6, "npc x should be 50.0");
        }
        _ => panic!("expected WorldPositions"),
    }

    let ego_spd = parsed.get_initial_speed("ego").expect("ego needs speed");
    let npc_spd = parsed.get_initial_speed("npc").expect("npc needs speed");
    assert!((ego_spd - 10.0).abs() < 1e-6);
    assert!((npc_spd - 20.0).abs() < 1e-6);
}

#[test]
fn roundtrip_speed_only_no_position() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (name, params) = car("ego");
    s.add_vehicle(name, params).unwrap();
    s.set_initial_speed("ego", 5.0).unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let speed = parsed
        .get_initial_speed("ego")
        .expect("speed should survive roundtrip");
    assert!((speed - 5.0).abs() < 1e-6);
    assert!(
        parsed.get_initial_position("ego").is_none(),
        "position should remain None when not set"
    );
}

#[test]
fn roundtrip_position_only_no_speed() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (name, params) = car("ego");
    s.add_vehicle(name, params).unwrap();
    s.set_initial_position("ego", Position::world(1.0, 2.0, 0.0, 0.0))
        .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    assert!(
        parsed.get_initial_position("ego").is_some(),
        "position should survive roundtrip"
    );
    assert!(
        parsed.get_initial_speed("ego").is_none(),
        "speed should remain None when not set"
    );
}

#[test]
fn roundtrip_entity_not_in_init_has_no_position() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (n, p) = car("ego");
    s.add_vehicle(n, p).unwrap();
    let (n, p) = car("npc");
    s.add_vehicle(n, p).unwrap();

    s.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    assert!(
        parsed.get_initial_position("ego").is_some(),
        "ego should have position"
    );
    // npc has no Init section; position should be None, not a crash
    assert!(
        parsed.get_initial_position("npc").is_none(),
        "npc should have no position"
    );
}

// ─── parse_act tests ────────────────────────────────────────────────────────

#[test]
fn roundtrip_story_count_preserved() {
    let s = build_basic_scenario_with_speed_action(50.0);
    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    assert_eq!(
        parsed.story_count(),
        1,
        "one story should survive roundtrip"
    );
}

#[test]
fn roundtrip_act_name_preserved() {
    let s = build_basic_scenario_with_speed_action(50.0);
    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let story = &parsed.storyboard().stories["story1"];
    assert!(
        story.acts.contains_key("act1"),
        "act1 should be present; got keys: {:?}",
        story.acts.keys().collect::<Vec<_>>()
    );
}

#[test]
fn roundtrip_maneuver_group_actors_preserved() {
    let s = build_basic_scenario_with_speed_action(50.0);
    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let act = &parsed.storyboard().stories["story1"].acts["act1"];
    let mg = act
        .maneuver_groups
        .get("ego_mg")
        .expect("ego_mg should exist");
    assert!(
        mg.actors.contains(&"ego".to_string()),
        "ego should be in actors list; got {:?}",
        mg.actors
    );
}

#[test]
fn roundtrip_maneuver_name_preserved() {
    let s = build_basic_scenario_with_speed_action(50.0);
    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let act = &parsed.storyboard().stories["story1"].acts["act1"];
    let mg = &act.maneuver_groups["ego_mg"];
    assert_eq!(mg.maneuvers.len(), 1, "one maneuver expected");
    assert_eq!(
        mg.maneuvers[0].name, "maneuver1",
        "maneuver name should be preserved"
    );
}

#[test]
fn roundtrip_speed_action_target_preserved() {
    let s = build_basic_scenario_with_speed_action(72.5);
    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let act = &parsed.storyboard().stories["story1"].acts["act1"];
    let mg = &act.maneuver_groups["ego_mg"];
    let event = &mg.maneuvers[0].events[0];
    match &event.actions[0] {
        Action::Speed(sa) => {
            assert!(
                (sa.target_speed - 72.5).abs() < 1e-6,
                "target speed should be 72.5, got {}",
                sa.target_speed
            );
        }
        other => panic!("expected Action::Speed, got {:?}", other),
    }
}

#[test]
fn roundtrip_lane_change_action_target_preserved() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (name, params) = car("ego");
    s.add_vehicle(name, params).unwrap();
    s.add_story("story1").unwrap();
    s.add_act("story1", "act1").unwrap();
    s.add_maneuver_group("story1", "act1", "ego_mg").unwrap();
    s.add_actor("story1", "act1", "ego_mg", "ego").unwrap();
    s.add_maneuver("story1", "act1", "ego_mg", "maneuver1")
        .unwrap();
    s.add_lane_change_action(
        "story1",
        "act1",
        "ego_mg",
        "maneuver1",
        "event1",
        -1.0,
        3.0,
        TransitionShape::Sinusoidal,
    )
    .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let act = &parsed.storyboard().stories["story1"].acts["act1"];
    let event = &act.maneuver_groups["ego_mg"].maneuvers[0].events[0];
    match &event.actions[0] {
        Action::LaneChange(lc) => {
            assert!(
                (lc.target_lane_offset - (-1.0)).abs() < 1e-6,
                "target_lane_offset should be -1.0, got {}",
                lc.target_lane_offset
            );
        }
        other => panic!("expected Action::LaneChange, got {:?}", other),
    }
}

#[test]
fn roundtrip_multiple_maneuver_groups_both_present() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (n, p) = car("ego");
    s.add_vehicle(n, p).unwrap();
    let (n, p) = car("npc");
    s.add_vehicle(n, p).unwrap();

    s.add_story("story1").unwrap();
    s.add_act("story1", "act1").unwrap();
    s.add_maneuver_group("story1", "act1", "ego_mg").unwrap();
    s.add_actor("story1", "act1", "ego_mg", "ego").unwrap();
    s.add_maneuver("story1", "act1", "ego_mg", "maneuver1")
        .unwrap();
    s.add_speed_action(
        "story1",
        "act1",
        "ego_mg",
        "maneuver1",
        "event1",
        50.0,
        linear_dynamics(5.0),
    )
    .unwrap();

    s.add_maneuver_group("story1", "act1", "npc_mg").unwrap();
    s.add_actor("story1", "act1", "npc_mg", "npc").unwrap();
    s.add_maneuver("story1", "act1", "npc_mg", "maneuver2")
        .unwrap();
    s.add_speed_action(
        "story1",
        "act1",
        "npc_mg",
        "maneuver2",
        "event2",
        30.0,
        linear_dynamics(3.0),
    )
    .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let act = &parsed.storyboard().stories["story1"].acts["act1"];
    assert!(
        act.maneuver_groups.contains_key("ego_mg"),
        "ego_mg should be present"
    );
    assert!(
        act.maneuver_groups.contains_key("npc_mg"),
        "npc_mg should be present"
    );
}

#[test]
fn roundtrip_event_count_preserved() {
    let s = build_basic_scenario_with_speed_action(50.0);
    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let maneuver =
        &parsed.storyboard().stories["story1"].acts["act1"].maneuver_groups["ego_mg"].maneuvers[0];
    assert_eq!(
        maneuver.events.len(),
        1,
        "one event should survive roundtrip"
    );
}

#[test]
fn roundtrip_act_start_trigger_sim_time_value() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (name, params) = car("ego");
    s.add_vehicle(name, params).unwrap();
    s.add_story("story1").unwrap();
    s.add_act("story1", "act1").unwrap();
    s.add_maneuver_group("story1", "act1", "ego_mg").unwrap();
    s.add_actor("story1", "act1", "ego_mg", "ego").unwrap();
    s.add_maneuver("story1", "act1", "ego_mg", "maneuver1")
        .unwrap();
    s.add_speed_action(
        "story1",
        "act1",
        "ego_mg",
        "maneuver1",
        "event1",
        50.0,
        linear_dynamics(5.0),
    )
    .unwrap();
    let trigger = Trigger::new(ConditionGroup::new(vec![Condition {
        name: "trigger_cond".to_string(),
        delay: 0.0,
        condition_edge: ConditionEdge::None,
        kind: ConditionKind::ByValue(ByValueCondition::SimulationTime {
            value: 3.0,
            rule: Rule::GreaterThan,
        }),
    }]));
    s.set_act_start_trigger("story1", "act1", trigger).unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let act = &parsed.storyboard().stories["story1"].acts["act1"];
    let trigger = act
        .start_trigger
        .as_ref()
        .expect("start trigger should survive roundtrip");
    let group = &trigger.condition_groups[0];
    let cond = &group.conditions[0];
    match &cond.kind {
        ConditionKind::ByValue(ByValueCondition::SimulationTime { value, rule }) => {
            assert!(
                (value - 3.0).abs() < 1e-6,
                "trigger time should be 3.0, got {}",
                value
            );
            assert_eq!(*rule, Rule::GreaterThan);
        }
        other => panic!("expected SimulationTimeCondition, got {:?}", other),
    }
}

#[test]
fn roundtrip_multiple_stories_both_present() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let (n, p) = car("ego");
    s.add_vehicle(n, p).unwrap();

    s.add_story("story_a").unwrap();
    s.add_act("story_a", "act1").unwrap();
    s.add_maneuver_group("story_a", "act1", "mg1").unwrap();
    s.add_actor("story_a", "act1", "mg1", "ego").unwrap();
    s.add_maneuver("story_a", "act1", "mg1", "m1").unwrap();
    s.add_speed_action(
        "story_a",
        "act1",
        "mg1",
        "m1",
        "e1",
        10.0,
        linear_dynamics(2.0),
    )
    .unwrap();

    s.add_story("story_b").unwrap();
    s.add_act("story_b", "act1").unwrap();
    s.add_maneuver_group("story_b", "act1", "mg1").unwrap();
    s.add_actor("story_b", "act1", "mg1", "ego").unwrap();
    s.add_maneuver("story_b", "act1", "mg1", "m1").unwrap();
    s.add_speed_action(
        "story_b",
        "act1",
        "mg1",
        "m1",
        "e1",
        20.0,
        linear_dynamics(3.0),
    )
    .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    assert_eq!(
        parsed.story_count(),
        2,
        "two stories should survive roundtrip"
    );
    assert!(
        parsed.storyboard().stories.contains_key("story_a"),
        "story_a should be present"
    );
    assert!(
        parsed.storyboard().stories.contains_key("story_b"),
        "story_b should be present"
    );
}
