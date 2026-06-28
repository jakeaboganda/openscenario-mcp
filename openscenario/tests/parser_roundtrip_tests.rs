use openscenario::entities::{MiscObjectParams, PedestrianParams, VehicleCategory, VehicleParams};
use openscenario::storyboard::{
    Action, ByValueCondition, Condition, ConditionEdge, ConditionGroup, ConditionKind,
    DynamicsDimension, DynamicsShape, ParameterCondition, Rule, TransitionDynamics,
    TransitionShape, Trigger,
};
use openscenario::{OpenScenarioVersion, ParameterType, Position, Scenario};

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

// XML entity unescaping tests

#[test]
fn entity_name_with_xml_entities_is_unescaped() {
    // Build a scenario with a normal name, then manually inject XML-escaped chars into the XML
    // to verify the parser unescapes attribute values correctly.
    let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<OpenSCENARIO xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="OpenSCENARIO.xsd">
  <FileHeader description="" author="" date="2024-01-01T00:00:00" revMajor="1" revMinor="2"/>
  <ParameterDeclarations/>
  <CatalogLocations/>
  <RoadNetwork><LogicFile filepath="map.xodr"/></RoadNetwork>
  <Entities>
    <ScenarioObject name="Ego&amp;Hero">
      <Vehicle name="Ego&amp;Hero" vehicleCategory="car">
        <BoundingBox><Center x="0" y="0" z="0"/><Dimensions width="2" length="4" height="1.5"/></BoundingBox>
        <Performance maxSpeed="50" maxAcceleration="10" maxDeceleration="10"/>
        <Axles><FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.5" positionX="2.5" positionZ="0.3"/><RearAxle maxSteering="0" wheelDiameter="0.6" trackWidth="1.5" positionX="0" positionZ="0.3"/></Axles>
        <Properties/>
      </Vehicle>
    </ScenarioObject>
  </Entities>
  <Storyboard>
    <Init><Actions><Private entityRef="Ego&amp;Hero">
      <PrivateAction><TeleportAction><Position>
        <WorldPosition x="0.0" y="0.0" z="0.0" h="0.0"/>
      </Position></TeleportAction></PrivateAction>
    </Private></Actions></Init>
    <Story name="s"><Act name="a"><ManeuverGroup name="mg" maximumExecutionCount="1"><Actors selectTriggeringEntities="false"/><Maneuver name="m"><Event name="e" priority="overwrite"><Action name="a"><GlobalAction><EntityAction entityRef="ego"><AddEntityAction><Position><WorldPosition x="0" y="0" z="0" h="0"/></Position></AddEntityAction></EntityAction></GlobalAction></Action><StartTrigger/></Event></Maneuver></ManeuverGroup><StartTrigger><ConditionGroup><Condition name="c" delay="0" conditionEdge="none"><ByValueCondition><SimulationTimeCondition value="0" rule="greaterThan"/></ByValueCondition></Condition></ConditionGroup></StartTrigger></Act></Story>
    <StopTrigger/>
  </Storyboard>
</OpenSCENARIO>"#;

    let parsed = Scenario::from_xml(xml).unwrap();
    assert!(
        parsed.get_entity("Ego&Hero").is_some(),
        "entity name 'Ego&Hero' should be unescaped from 'Ego&amp;Hero' in XML"
    );
    assert!(
        parsed.get_initial_position("Ego&Hero").is_some(),
        "initial position should be associated with unescaped name 'Ego&Hero'"
    );
}

// CatalogReference entity type roundtrip tests

#[test]
fn roundtrip_vehicle_with_catalog_stays_vehicle() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_vehicle(
        "ego",
        VehicleParams {
            catalog: Some(openscenario::entities::CatalogReference {
                path: "VehicleCatalog.xosc".to_string(),
                entry_name: "sedan".to_string(),
            }),
            vehicle_category: VehicleCategory::Car,
            properties: None,
        },
    )
    .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let entity = parsed
        .get_entity("ego")
        .expect("ego should survive roundtrip");
    assert!(
        matches!(entity, openscenario::entities::Entity::Vehicle(_)),
        "vehicle with catalog should remain a Vehicle, got {:?}",
        entity
    );
}

#[test]
fn roundtrip_pedestrian_with_catalog_stays_pedestrian() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_pedestrian(
        "walker",
        PedestrianParams {
            catalog: Some(openscenario::entities::CatalogReference {
                path: "PedestrianCatalog.xosc".to_string(),
                entry_name: "adult_male".to_string(),
            }),
            model: None,
            mass: None,
        },
    )
    .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let entity = parsed
        .get_entity("walker")
        .expect("walker should survive roundtrip");
    assert!(
        matches!(entity, openscenario::entities::Entity::Pedestrian(_)),
        "pedestrian with catalog should remain a Pedestrian, got {:?}",
        entity
    );
}

#[test]
fn roundtrip_misc_object_with_catalog_stays_misc_object() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_misc_object(
        "cone",
        MiscObjectParams {
            catalog: Some(openscenario::entities::CatalogReference {
                path: "MiscObjectCatalog.xosc".to_string(),
                entry_name: "traffic_cone".to_string(),
            }),
            category: None,
            mass: None,
        },
    )
    .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let entity = parsed
        .get_entity("cone")
        .expect("cone should survive roundtrip");
    assert!(
        matches!(entity, openscenario::entities::Entity::MiscObject(_)),
        "misc object with catalog should remain a MiscObject, got {:?}",
        entity
    );
}

// Rule::GreaterOrEqual / LessOrEqual roundtrip tests

#[test]
fn roundtrip_greater_or_equal_rule_preserved() {
    let mut s = build_basic_scenario_with_speed_action(10.0);
    let trigger = Trigger::with_groups(vec![ConditionGroup {
        conditions: vec![Condition {
            name: "t".to_string(),
            delay: 0.0,
            condition_edge: ConditionEdge::None,
            kind: ConditionKind::ByValue(ByValueCondition::SimulationTime {
                value: 5.0,
                rule: Rule::GreaterOrEqual,
            }),
        }],
    }]);
    s.set_act_start_trigger("story1", "act1", trigger).unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let act = parsed
        .storyboard()
        .stories
        .get("story1")
        .unwrap()
        .acts
        .get("act1")
        .unwrap();
    let cond = &act.start_trigger.as_ref().unwrap().condition_groups[0].conditions[0];
    match &cond.kind {
        ConditionKind::ByValue(ByValueCondition::SimulationTime { rule, value }) => {
            assert_eq!(
                *rule,
                Rule::GreaterOrEqual,
                "greaterOrEqual should round-trip"
            );
            assert!((value - 5.0).abs() < 1e-6);
        }
        other => panic!("expected SimulationTime, got {:?}", other),
    }
}

#[test]
fn roundtrip_less_or_equal_rule_preserved() {
    let mut s = build_basic_scenario_with_speed_action(10.0);
    let trigger = Trigger::with_groups(vec![ConditionGroup {
        conditions: vec![Condition {
            name: "t".to_string(),
            delay: 0.0,
            condition_edge: ConditionEdge::None,
            kind: ConditionKind::ByValue(ByValueCondition::SimulationTime {
                value: 3.0,
                rule: Rule::LessOrEqual,
            }),
        }],
    }]);
    s.set_act_start_trigger("story1", "act1", trigger).unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let act = parsed
        .storyboard()
        .stories
        .get("story1")
        .unwrap()
        .acts
        .get("act1")
        .unwrap();
    let cond = &act.start_trigger.as_ref().unwrap().condition_groups[0].conditions[0];
    match &cond.kind {
        ConditionKind::ByValue(ByValueCondition::SimulationTime { rule, value }) => {
            assert_eq!(*rule, Rule::LessOrEqual, "lessOrEqual should round-trip");
            assert!((value - 3.0).abs() < 1e-6);
        }
        other => panic!("expected SimulationTime, got {:?}", other),
    }
}

// MiscObject / Pedestrian roundtrip tests

#[test]
fn roundtrip_misc_object_category_survives() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_misc_object(
        "cone1",
        MiscObjectParams {
            catalog: None,
            category: Some("obstacle".to_string()),
            mass: None,
        },
    )
    .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let entity = parsed
        .get_entity("cone1")
        .expect("cone1 should survive roundtrip");
    match entity {
        openscenario::entities::Entity::MiscObject(mo) => {
            assert_eq!(
                mo.params.category.as_deref(),
                Some("obstacle"),
                "miscObjectCategory should round-trip"
            );
        }
        other => panic!("expected MiscObject, got {:?}", other),
    }
}

#[test]
fn roundtrip_misc_object_mass_survives() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_misc_object(
        "barrier",
        MiscObjectParams {
            catalog: None,
            category: Some("barrier".to_string()),
            mass: Some(150.0),
        },
    )
    .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let entity = parsed
        .get_entity("barrier")
        .expect("barrier should survive roundtrip");
    match entity {
        openscenario::entities::Entity::MiscObject(mo) => {
            let mass = mo.params.mass.expect("mass should round-trip");
            assert!(
                (mass - 150.0).abs() < 1e-6,
                "mass should be 150.0, got {}",
                mass
            );
        }
        other => panic!("expected MiscObject, got {:?}", other),
    }
}

#[test]
fn roundtrip_pedestrian_model_survives() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_pedestrian(
        "walker1",
        PedestrianParams {
            catalog: None,
            model: Some("Adult_Male.fbx".to_string()),
            mass: None,
        },
    )
    .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let entity = parsed
        .get_entity("walker1")
        .expect("walker1 should survive roundtrip");
    match entity {
        openscenario::entities::Entity::Pedestrian(p) => {
            assert_eq!(
                p.params.model.as_deref(),
                Some("Adult_Male.fbx"),
                "model should round-trip"
            );
        }
        other => panic!("expected Pedestrian, got {:?}", other),
    }
}

#[test]
fn roundtrip_pedestrian_mass_survives() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_pedestrian(
        "walker2",
        PedestrianParams {
            catalog: None,
            model: None,
            mass: Some(70.0),
        },
    )
    .unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let entity = parsed
        .get_entity("walker2")
        .expect("walker2 should survive roundtrip");
    match entity {
        openscenario::entities::Entity::Pedestrian(p) => {
            let mass = p.params.mass.expect("mass should round-trip");
            assert!(
                (mass - 70.0).abs() < 1e-6,
                "mass should be 70.0, got {}",
                mass
            );
        }
        other => panic!("expected Pedestrian, got {:?}", other),
    }
}

// Malformed position attribute error tests

#[test]
fn malformed_world_position_x_returns_error() {
    let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<OpenSCENARIO xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="OpenSCENARIO.xsd">
  <FileHeader description="" author="" date="2024-01-01T00:00:00" revMajor="1" revMinor="2"/>
  <ParameterDeclarations/>
  <CatalogLocations/>
  <RoadNetwork><LogicFile filepath="map.xodr"/></RoadNetwork>
  <Entities>
    <ScenarioObject name="ego">
      <Vehicle name="ego" vehicleCategory="car">
        <BoundingBox><Center x="0" y="0" z="0"/><Dimensions width="2" length="4" height="1.5"/></BoundingBox>
        <Performance maxSpeed="50" maxAcceleration="10" maxDeceleration="10"/>
        <Axles><FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.5" positionX="2.5" positionZ="0.3"/><RearAxle maxSteering="0" wheelDiameter="0.6" trackWidth="1.5" positionX="0" positionZ="0.3"/></Axles>
        <Properties/>
      </Vehicle>
    </ScenarioObject>
  </Entities>
  <Storyboard>
    <Init><Actions><Private entityRef="ego">
      <PrivateAction><TeleportAction><Position>
        <WorldPosition x="not_a_number" y="10.0" z="0.0" h="0.0"/>
      </Position></TeleportAction></PrivateAction>
    </Private></Actions></Init>
    <Story name="s"><Act name="a"><ManeuverGroup name="mg" maximumExecutionCount="1"><Actors selectTriggeringEntities="false"/><Maneuver name="m"><Event name="e" priority="overwrite"><Action name="a"><GlobalAction><EntityAction entityRef="ego"><AddEntityAction><Position><WorldPosition x="0" y="0" z="0" h="0"/></Position></AddEntityAction></EntityAction></GlobalAction></Action><StartTrigger/></Event></Maneuver></ManeuverGroup><StartTrigger><ConditionGroup><Condition name="c" delay="0" conditionEdge="none"><ByValueCondition><SimulationTimeCondition value="0" rule="greaterThan"/></ByValueCondition></Condition></ConditionGroup></StartTrigger></Act></Story>
    <StopTrigger/>
  </Storyboard>
</OpenSCENARIO>"#;
    let result = Scenario::from_xml(xml);
    assert!(
        result.is_err(),
        "malformed x attribute should return an error, got Ok"
    );
}

// Truncated XML error tests

#[test]
fn truncated_xml_inside_story_returns_error() {
    let s = build_basic_scenario_with_speed_action(10.0);
    let xml = s.to_xml().unwrap();
    // Cut off before </Story>
    let truncated = xml.split("</Story>").next().unwrap();
    let result = Scenario::from_xml(truncated);
    assert!(
        result.is_err(),
        "truncated XML (missing </Story>) should return an error, got Ok"
    );
}

#[test]
fn truncated_xml_inside_act_returns_error() {
    let s = build_basic_scenario_with_speed_action(10.0);
    let xml = s.to_xml().unwrap();
    let truncated = xml.split("</Act>").next().unwrap();
    let result = Scenario::from_xml(truncated);
    assert!(
        result.is_err(),
        "truncated XML (missing </Act>) should return an error, got Ok"
    );
}

#[test]
fn truncated_xml_inside_maneuver_group_returns_error() {
    let s = build_basic_scenario_with_speed_action(10.0);
    let xml = s.to_xml().unwrap();
    let truncated = xml.split("</ManeuverGroup>").next().unwrap();
    let result = Scenario::from_xml(truncated);
    assert!(
        result.is_err(),
        "truncated XML (missing </ManeuverGroup>) should return an error, got Ok"
    );
}

// ParameterCondition roundtrip tests

#[test]
fn roundtrip_parameter_condition_trigger() {
    let mut s = build_basic_scenario_with_speed_action(10.0);
    s.add_parameter("MyParam", ParameterType::String, "active")
        .unwrap();

    let trigger = Trigger::with_groups(vec![ConditionGroup {
        conditions: vec![Condition {
            name: "param_cond".to_string(),
            delay: 0.0,
            condition_edge: ConditionEdge::None,
            kind: ConditionKind::ByValue(ByValueCondition::Parameter(ParameterCondition {
                parameter_ref: "MyParam".to_string(),
                value: "active".to_string(),
                rule: Rule::EqualTo,
            })),
        }],
    }]);
    s.set_act_start_trigger("story1", "act1", trigger).unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let act = parsed
        .storyboard()
        .stories
        .get("story1")
        .expect("story1 missing")
        .acts
        .get("act1")
        .expect("act1 missing");
    let trigger = act.start_trigger.as_ref().expect("start trigger missing");
    assert_eq!(trigger.condition_groups.len(), 1);
    let cond = &trigger.condition_groups[0].conditions[0];
    match &cond.kind {
        ConditionKind::ByValue(ByValueCondition::Parameter(pc)) => {
            assert_eq!(pc.parameter_ref, "MyParam");
            assert_eq!(pc.value, "active");
            assert_eq!(pc.rule, Rule::EqualTo);
        }
        other => panic!("expected ParameterCondition, got {:?}", other),
    }
}

#[test]
fn roundtrip_parameter_condition_preserves_rule_and_value() {
    let mut s = build_basic_scenario_with_speed_action(10.0);
    s.add_parameter("speed_limit", ParameterType::Double, "30.0")
        .unwrap();

    let trigger = Trigger::with_groups(vec![ConditionGroup {
        conditions: vec![Condition {
            name: "p".to_string(),
            delay: 0.0,
            condition_edge: ConditionEdge::None,
            kind: ConditionKind::ByValue(ByValueCondition::Parameter(ParameterCondition {
                parameter_ref: "speed_limit".to_string(),
                value: "30.0".to_string(),
                rule: Rule::GreaterThan,
            })),
        }],
    }]);
    s.set_act_start_trigger("story1", "act1", trigger).unwrap();

    let xml = s.to_xml().unwrap();
    let parsed = Scenario::from_xml(&xml).unwrap();

    let act = parsed
        .storyboard()
        .stories
        .get("story1")
        .unwrap()
        .acts
        .get("act1")
        .unwrap();
    let cond = &act.start_trigger.as_ref().unwrap().condition_groups[0].conditions[0];
    match &cond.kind {
        ConditionKind::ByValue(ByValueCondition::Parameter(pc)) => {
            assert_eq!(pc.rule, Rule::GreaterThan);
            assert_eq!(pc.value, "30.0");
        }
        other => panic!("expected ParameterCondition, got {:?}", other),
    }
}
