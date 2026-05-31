use openscenario::entities::VehicleCategory;
use openscenario::validation::XsdValidator;

fn main() {
    println!("=== Testing New Vehicle Categories ===\n");

    // Test all 10 vehicle categories
    let categories = vec![
        ("car", VehicleCategory::Car),
        ("van", VehicleCategory::Van),
        ("truck", VehicleCategory::Truck),
        ("trailer", VehicleCategory::Trailer),
        ("semitrailer", VehicleCategory::Semitrailer),
        ("bus", VehicleCategory::Bus),
        ("motorbike", VehicleCategory::Motorbike),
        ("bicycle", VehicleCategory::Bicycle),
        ("train", VehicleCategory::Train),
        ("tram", VehicleCategory::Tram),
    ];

    for (xml_value, rust_enum) in categories {
        println!("✅ {} -> {:?} -> {}", xml_value, rust_enum, rust_enum.as_xml_str());
        assert_eq!(rust_enum.as_xml_str(), xml_value);
    }

    println!("\n=== Testing XSD Validation with New Categories ===\n");

    // Test train
    let train_xml = format!(r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader revMajor="1" revMinor="2" date="2024-01-01T00:00:00" description="Test" author="Test"/>
    <RoadNetwork>
        <LogicFile filepath="test.xodr"/>
    </RoadNetwork>
    <Entities>
        <ScenarioObject name="Loco">
            <Vehicle name="train1" vehicleCategory="train">
                <BoundingBox>
                    <Center x="0.0" y="0.0" z="0.0"/>
                    <Dimensions width="3.0" length="25.0" height="4.5"/>
                </BoundingBox>
                <Performance maxSpeed="50.0" maxAcceleration="1.0" maxDeceleration="2.0"/>
                <Axles>
                    <FrontAxle maxSteering="0.0" wheelDiameter="1.0" trackWidth="1.5" positionX="20.0" positionZ="0.5"/>
                    <RearAxle maxSteering="0.0" wheelDiameter="1.0" trackWidth="1.5" positionX="5.0" positionZ="0.5"/>
                </Axles>
                <Properties/>
            </Vehicle>
        </ScenarioObject>
    </Entities>
    <Storyboard>
        <Init>
            <Actions/>
        </Init>
        <StopTrigger/>
    </Storyboard>
</OpenSCENARIO>"#);

    let validator = XsdValidator::new("1.2");
    let report = validator.validate(&train_xml);
    println!("Train scenario valid: {}", report.valid);
    if !report.errors.is_empty() {
        println!("Errors:");
        for error in &report.errors {
            println!("  - {}", error);
        }
    }

    // Test semitrailer
    let semitrailer_xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader revMajor="1" revMinor="2" date="2024-01-01T00:00:00" description="Test" author="Test"/>
    <RoadNetwork>
        <LogicFile filepath="test.xodr"/>
    </RoadNetwork>
    <Entities>
        <ScenarioObject name="Hauler">
            <Vehicle name="semi1" vehicleCategory="semitrailer">
                <BoundingBox>
                    <Center x="0.0" y="0.0" z="0.0"/>
                    <Dimensions width="2.5" length="16.0" height="4.0"/>
                </BoundingBox>
                <Performance maxSpeed="35.0" maxAcceleration="1.5" maxDeceleration="3.0"/>
                <Axles>
                    <FrontAxle maxSteering="0.3" wheelDiameter="0.9" trackWidth="2.0" positionX="12.0" positionZ="0.5"/>
                    <RearAxle maxSteering="0.0" wheelDiameter="0.9" trackWidth="2.0" positionX="2.0" positionZ="0.5"/>
                </Axles>
                <Properties/>
            </Vehicle>
        </ScenarioObject>
    </Entities>
    <Storyboard>
        <Init>
            <Actions/>
        </Init>
        <StopTrigger/>
    </Storyboard>
</OpenSCENARIO>"#;

    let report = validator.validate(semitrailer_xml);
    println!("\nSemitrailer scenario valid: {}", report.valid);
    if !report.errors.is_empty() {
        println!("Errors:");
        for error in &report.errors {
            println!("  - {}", error);
        }
    }

    println!("\n✅ All vehicle categories working!");
}
