//! XML parsing for OpenSCENARIO files.
//!
//! Implements parsing of OpenSCENARIO XML into Scenario structs.

use crate::entities::{
    CatalogReference, Entity, MiscObject, MiscObjectParams, Pedestrian, PedestrianParams, Vehicle,
    VehicleCategory, VehicleParams,
};
use crate::position::Position;
use crate::scenario::{ParameterDeclaration, ParameterType, Scenario};
use crate::storyboard::{Act, Story, Storyboard};
use crate::{OpenScenarioVersion, Result, ScenarioError};
use quick_xml::events::{BytesStart, Event as XmlEvent};
use quick_xml::Reader;
use std::collections::HashMap;

impl Scenario {
    /// Parse OpenSCENARIO XML into a Scenario struct.
    ///
    /// Supports OpenSCENARIO versions 1.0, 1.1, and 1.2.
    ///
    /// # Arguments
    /// * `xml` - OpenSCENARIO XML string
    ///
    /// # Returns
    /// * `Ok(Scenario)` - Parsed scenario
    /// * `Err(ScenarioError)` - Parse error
    ///
    /// # Examples
    /// ```no_run
    /// use openscenario::Scenario;
    ///
    /// let xml = std::fs::read_to_string("scenario.xosc").unwrap();
    /// let scenario = Scenario::from_xml(&xml).unwrap();
    /// ```
    pub fn from_xml(xml: &str) -> Result<Self> {
        // Security: Validate file size to prevent DoS
        const MAX_XML_SIZE: usize = 100 * 1024 * 1024; // 100MB
        if xml.len() > MAX_XML_SIZE {
            return Err(ScenarioError::InvalidValue {
                field: "xml_size".to_string(),
                reason: format!(
                    "XML file too large ({} bytes). Maximum size is {} bytes (100MB)",
                    xml.len(),
                    MAX_XML_SIZE
                ),
            });
        }

        // Security: Check for empty input
        if xml.trim().is_empty() {
            return Err(ScenarioError::Parse(
                "XML file is empty or contains only whitespace".to_string(),
            ));
        }

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);
        reader.config_mut().check_end_names = true; // Validate closing tags

        let mut scenario = None;
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(XmlEvent::Start(e)) => {
                    if e.name().as_ref() == b"OpenSCENARIO" {
                        scenario = Some(parse_openscenario(&mut reader)?);
                        break;
                    }
                }
                Ok(XmlEvent::Eof) => break,
                Err(e) => return Err(ScenarioError::Xml(e)),
                _ => {}
            }
            buf.clear();
        }

        scenario
            .ok_or_else(|| ScenarioError::Parse("No OpenSCENARIO root element found".to_string()))
    }
}

fn parse_openscenario(reader: &mut Reader<&[u8]>) -> Result<Scenario> {
    let mut version = OpenScenarioVersion::V1_2; // Default
    let mut entities = HashMap::new();
    let mut initial_positions = HashMap::new();
    let mut initial_speeds = HashMap::new();
    let mut road_network = None;
    let mut parameters = Vec::new();
    let mut storyboard = Storyboard::default();

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"FileHeader" => {
                    version = parse_file_header(reader)?;
                }
                b"ParameterDeclarations" => {
                    parameters = parse_parameter_declarations(reader)?;
                }
                b"RoadNetwork" => {
                    road_network = parse_road_network(reader)?;
                }
                b"Entities" => {
                    entities = parse_entities(reader)?;
                }
                b"Storyboard" => {
                    (storyboard, initial_positions, initial_speeds) =
                        parse_storyboard(reader, &entities)?;
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"OpenSCENARIO" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Scenario {
        version,
        entities,
        initial_positions,
        initial_speeds,
        road_network,
        parameters,
        storyboard,
    })
}

fn parse_file_header(reader: &mut Reader<&[u8]>) -> Result<OpenScenarioVersion> {
    let mut major: Option<u8> = None;
    let mut minor: Option<u8> = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"FileHeader" => {
                for attr in e.attributes() {
                    let attr = attr.map_err(|e| ScenarioError::Parse(e.to_string()))?;
                    match attr.key.as_ref() {
                        b"revMajor" => {
                            let value = String::from_utf8_lossy(&attr.value);
                            major = Some(value.parse::<u8>().map_err(|_| {
                                ScenarioError::Parse(format!(
                                    "Invalid revMajor attribute: '{}'. Expected integer.",
                                    value
                                ))
                            })?);
                        }
                        b"revMinor" => {
                            let value = String::from_utf8_lossy(&attr.value);
                            minor = Some(value.parse::<u8>().map_err(|_| {
                                ScenarioError::Parse(format!(
                                    "Invalid revMinor attribute: '{}'. Expected integer.",
                                    value
                                ))
                            })?);
                        }
                        _ => {}
                    }
                }
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"FileHeader" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    // Validate version
    let major = major.ok_or_else(|| {
        ScenarioError::Parse("Missing revMajor attribute in FileHeader".to_string())
    })?;
    let minor = minor.ok_or_else(|| {
        ScenarioError::Parse("Missing revMinor attribute in FileHeader".to_string())
    })?;

    // Only support OpenSCENARIO 1.x
    if major != 1 {
        return Err(ScenarioError::Parse(format!(
            "Unsupported OpenSCENARIO version {}.{}. Only version 1.x is supported (1.0, 1.1, 1.2).",
            major, minor
        )));
    }

    let version = match minor {
        0 => OpenScenarioVersion::V1_0,
        1 => OpenScenarioVersion::V1_1,
        _ => OpenScenarioVersion::V1_2, // 1.2+ all map to V1_2
    };

    Ok(version)
}

fn parse_parameter_declarations(reader: &mut Reader<&[u8]>) -> Result<Vec<ParameterDeclaration>> {
    let mut parameters = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"ParameterDeclaration" => {
                let mut name = String::new();
                let mut parameter_type = ParameterType::String;
                let mut value = String::new();

                for attr in e.attributes() {
                    let attr = attr.map_err(|e| ScenarioError::Parse(e.to_string()))?;
                    match attr.key.as_ref() {
                        b"name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                        b"parameterType" => {
                            parameter_type = match attr.value.as_ref() {
                                b"integer" => ParameterType::Integer,
                                b"double" => ParameterType::Double,
                                b"string" => ParameterType::String,
                                b"boolean" => ParameterType::Boolean,
                                b"unsignedInt" => ParameterType::UnsignedInt,
                                b"unsignedShort" => ParameterType::UnsignedShort,
                                b"dateTime" => ParameterType::DateTime,
                                _ => ParameterType::String,
                            }
                        }
                        b"value" => value = String::from_utf8_lossy(&attr.value).to_string(),
                        _ => {}
                    }
                }

                parameters.push(ParameterDeclaration {
                    name,
                    parameter_type,
                    value,
                });
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"ParameterDeclarations" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(parameters)
}

fn parse_road_network(reader: &mut Reader<&[u8]>) -> Result<Option<String>> {
    let mut road_network = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"LogicFile" => {
                for attr in e.attributes() {
                    let attr = attr.map_err(|e| ScenarioError::Parse(e.to_string()))?;
                    if attr.key.as_ref() == b"filepath" {
                        road_network = Some(String::from_utf8_lossy(&attr.value).to_string());
                        break;
                    }
                }
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"RoadNetwork" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(road_network)
}

fn parse_entities(reader: &mut Reader<&[u8]>) -> Result<HashMap<String, Entity>> {
    const MAX_ENTITIES: usize = 10_000; // Security: Limit entity count
    let mut entities = HashMap::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => {
                if e.name().as_ref() == b"ScenarioObject" {
                    if entities.len() >= MAX_ENTITIES {
                        return Err(ScenarioError::Parse(format!(
                            "Too many entities (>{}). Possible malicious file.",
                            MAX_ENTITIES
                        )));
                    }
                    let (name, entity) = parse_scenario_object(reader)?;
                    entities.insert(name, entity);
                }
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Entities" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(entities)
}

fn parse_scenario_object(reader: &mut Reader<&[u8]>) -> Result<(String, Entity)> {
    let mut name = String::new();
    let mut entity = None;
    let mut buf = Vec::new();

    // Get name from ScenarioObject attributes
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"ScenarioObject" => {
                for attr in e.attributes() {
                    let attr = attr.map_err(|e| ScenarioError::Parse(e.to_string()))?;
                    if attr.key.as_ref() == b"name" {
                        name = String::from_utf8_lossy(&attr.value).to_string();
                        break;
                    }
                }
                break;
            }
            Ok(XmlEvent::Eof) => {
                return Err(ScenarioError::Parse(
                    "Unexpected EOF in ScenarioObject".to_string(),
                ))
            }
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    // Parse entity type
    buf.clear();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"Vehicle" => entity = Some(parse_vehicle(reader, &name, &e)?),
                b"Pedestrian" => entity = Some(parse_pedestrian(reader, &name)?),
                b"MiscObject" => entity = Some(parse_misc_object(reader, &name)?),
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"ScenarioObject" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    entity
        .map(|e| (name, e))
        .ok_or_else(|| ScenarioError::Parse("No entity found in ScenarioObject".to_string()))
}

fn parse_vehicle(
    reader: &mut Reader<&[u8]>,
    name: &str,
    start_elem: &BytesStart,
) -> Result<Entity> {
    let mut vehicle_category = VehicleCategory::Car;

    // Parse vehicleCategory attribute
    for attr_result in start_elem.attributes().flatten() {
        if attr_result.key.as_ref() == b"vehicleCategory" {
            let value = String::from_utf8_lossy(&attr_result.value).to_lowercase();
            vehicle_category = match value.as_str() {
                "car" => VehicleCategory::Car,
                "van" => VehicleCategory::Van,
                "truck" => VehicleCategory::Truck,
                "trailer" => VehicleCategory::Trailer,
                "semitrailer" => VehicleCategory::Semitrailer,
                "bus" => VehicleCategory::Bus,
                "motorbike" => VehicleCategory::Motorbike,
                "bicycle" => VehicleCategory::Bicycle,
                "train" => VehicleCategory::Train,
                "tram" => VehicleCategory::Tram,
                _ => VehicleCategory::Car, // Default fallback
            };
            break;
        }
    }

    let mut catalog = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"CatalogReference" => {
                    catalog = Some(parse_catalog_reference(reader)?);
                }
                b"Properties" => {
                    // Parse vehicle category from Properties
                    vehicle_category = parse_vehicle_properties(reader)?;
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"CatalogReference" => {
                catalog = Some(parse_catalog_reference_empty(&e)?);
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Vehicle" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Entity::Vehicle(Vehicle {
        name: name.to_string(),
        params: VehicleParams {
            catalog,
            vehicle_category,
            properties: None,
        },
    }))
}

fn parse_pedestrian(reader: &mut Reader<&[u8]>, name: &str) -> Result<Entity> {
    let mut catalog = None;
    let mut mass = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"CatalogReference" => {
                catalog = Some(parse_catalog_reference(reader)?);
            }
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"CatalogReference" => {
                catalog = Some(parse_catalog_reference_empty(&e)?);
            }
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"Properties" => {
                mass = Some(parse_pedestrian_properties(reader)?);
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Pedestrian" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Entity::Pedestrian(Pedestrian {
        name: name.to_string(),
        params: PedestrianParams {
            catalog,
            model: None,
            mass,
        },
    }))
}

fn parse_misc_object(reader: &mut Reader<&[u8]>, name: &str) -> Result<Entity> {
    let mut category = None;
    let mut mass = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"Properties" => {
                (category, mass) = parse_misc_object_properties(reader)?;
            }
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"MiscObject" => {
                for attr in e.attributes() {
                    let attr = attr.map_err(|e| ScenarioError::Parse(e.to_string()))?;
                    if attr.key.as_ref() == b"mass" {
                        let value = String::from_utf8_lossy(&attr.value);
                        mass = Some(value.parse::<f64>().map_err(|_| {
                            ScenarioError::Parse(format!(
                                "Invalid mass attribute on MiscObject '{}': '{}'. Expected number.",
                                name, value
                            ))
                        })?);
                    }
                    if attr.key.as_ref() == b"miscObjectCategory" {
                        category = Some(String::from_utf8_lossy(&attr.value).to_string());
                    }
                }
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"MiscObject" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Entity::MiscObject(MiscObject {
        name: name.to_string(),
        params: MiscObjectParams {
            catalog: None,
            category,
            mass,
        },
    }))
}

// Helper parsers

fn parse_catalog_reference(reader: &mut Reader<&[u8]>) -> Result<CatalogReference> {
    // Simplified - just skip for now
    skip_element(reader, b"CatalogReference")?;
    Ok(CatalogReference {
        path: "".to_string(),
        entry_name: "".to_string(),
    })
}

fn parse_catalog_reference_empty(e: &quick_xml::events::BytesStart) -> Result<CatalogReference> {
    let mut path = String::new();
    let mut entry_name = String::new();

    for attr in e.attributes() {
        let attr = attr.map_err(|e| ScenarioError::Parse(e.to_string()))?;
        match attr.key.as_ref() {
            b"catalogName" => path = String::from_utf8_lossy(&attr.value).to_string(),
            b"entryName" => entry_name = String::from_utf8_lossy(&attr.value).to_string(),
            _ => {}
        }
    }

    Ok(CatalogReference { path, entry_name })
}

fn parse_vehicle_properties(reader: &mut Reader<&[u8]>) -> Result<VehicleCategory> {
    skip_element(reader, b"Properties")?;
    Ok(VehicleCategory::Car) // Default
}

fn parse_pedestrian_properties(reader: &mut Reader<&[u8]>) -> Result<f64> {
    skip_element(reader, b"Properties")?;
    Ok(70.0) // Default mass
}

fn parse_misc_object_properties(
    reader: &mut Reader<&[u8]>,
) -> Result<(Option<String>, Option<f64>)> {
    skip_element(reader, b"Properties")?;
    Ok((None, None))
}

// Remove parse_misc_object_category since we don't need it

// Type alias for complex return type
type StoryboardResult = Result<(Storyboard, HashMap<String, Position>, HashMap<String, f64>)>;

fn parse_storyboard(
    reader: &mut Reader<&[u8]>,
    entities: &HashMap<String, Entity>,
) -> StoryboardResult {
    let mut stories = HashMap::new();
    let mut initial_positions = HashMap::new();
    let mut initial_speeds = HashMap::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"Init" => {
                    (initial_positions, initial_speeds) = parse_init(reader, entities)?;
                }
                b"Story" => {
                    let story = parse_story(reader)?;
                    stories.insert(story.name.clone(), story);
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Storyboard" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok((
        Storyboard {
            stories,
            stop_trigger: None,
        },
        initial_positions,
        initial_speeds,
    ))
}

fn parse_init(
    reader: &mut Reader<&[u8]>,
    _entities: &HashMap<String, Entity>,
) -> Result<(HashMap<String, Position>, HashMap<String, f64>)> {
    // Simplified - would need full implementation
    skip_element(reader, b"Init")?;
    Ok((HashMap::new(), HashMap::new()))
}

fn parse_story(reader: &mut Reader<&[u8]>) -> Result<Story> {
    let mut name = String::new();
    let mut acts = HashMap::new();
    let mut buf = Vec::new();

    // Get story name
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"Story" => {
                for attr in e.attributes() {
                    let attr = attr.map_err(|e| ScenarioError::Parse(e.to_string()))?;
                    if attr.key.as_ref() == b"name" {
                        name = String::from_utf8_lossy(&attr.value).to_string();
                        break;
                    }
                }
                break;
            }
            Ok(XmlEvent::Eof) => {
                return Err(ScenarioError::Parse("Unexpected EOF in Story".to_string()))
            }
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    // Parse acts
    buf.clear();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"Act" => {
                let act = parse_act(reader)?;
                acts.insert(act.name.clone(), act);
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Story" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Story { name, acts })
}

fn parse_act(reader: &mut Reader<&[u8]>) -> Result<Act> {
    // Simplified - would need full implementation
    let name = String::new();

    // This is a minimal stub - full implementation needed
    skip_to_end(reader, b"Act")?;

    Ok(Act {
        name,
        maneuver_groups: HashMap::new(),
        start_trigger: None,
    })
}

// Helper functions

fn skip_element(reader: &mut Reader<&[u8]>, element_name: &[u8]) -> Result<()> {
    skip_to_end(reader, element_name)
}

fn skip_to_end(reader: &mut Reader<&[u8]>, end_name: &[u8]) -> Result<()> {
    const MAX_DEPTH: usize = 100; // Security: Prevent stack overflow from deeply nested XML
    let mut depth = 1;
    let mut buf = Vec::new();

    loop {
        // Security check: Enforce maximum nesting depth
        if depth > MAX_DEPTH {
            return Err(ScenarioError::Parse(format!(
                "XML nesting too deep (>{} levels). Possible XML bomb attack or malformed file.",
                MAX_DEPTH
            )));
        }

        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == end_name => depth += 1,
            Ok(XmlEvent::End(e)) if e.name().as_ref() == end_name => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}
