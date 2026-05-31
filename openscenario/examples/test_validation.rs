use openscenario::validation::XsdValidator;

fn main() {
    // Test 1: Valid minimal OpenSCENARIO 1.2
    let valid_xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader revMajor="1" revMinor="2" date="2024-01-01T00:00:00" description="Test" author="Test"/>
    <RoadNetwork>
        <LogicFile filepath="test.xodr"/>
    </RoadNetwork>
    <Entities>
        <ScenarioObject name="Ego">
            <Vehicle name="car1" vehicleCategory="car">
                <BoundingBox>
                    <Center x="0.0" y="0.0" z="0.0"/>
                    <Dimensions width="2.0" length="5.0" height="1.5"/>
                </BoundingBox>
                <Performance maxSpeed="50.0" maxAcceleration="3.0" maxDeceleration="5.0"/>
                <Axles>
                    <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.5" positionX="2.5" positionZ="0.3"/>
                    <RearAxle maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.5" positionX="0.0" positionZ="0.3"/>
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

    println!("\n=== Test 1: Valid OpenSCENARIO 1.2 ===");
    let validator = XsdValidator::new("1.2");
    let report = validator.validate(valid_xml);
    println!("Valid: {}", report.valid);
    if !report.errors.is_empty() {
        println!("Errors:");
        for error in &report.errors {
            println!("  - {}", error);
        }
    }

    // Test 2: Missing required attribute
    let invalid_xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader revMajor="1" revMinor="2" date="2024-01-01T00:00:00" description="Test" author="Test"/>
    <RoadNetwork>
        <LogicFile filepath="test.xodr"/>
    </RoadNetwork>
    <Entities>
        <ScenarioObject name="Ego">
            <Vehicle name="car1">
                <BoundingBox>
                    <Center x="0.0" y="0.0" z="0.0"/>
                    <Dimensions width="2.0" length="5.0" height="1.5"/>
                </BoundingBox>
                <Performance maxSpeed="50.0" maxAcceleration="3.0" maxDeceleration="5.0"/>
                <Axles>
                    <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.5" positionX="2.5" positionZ="0.3"/>
                    <RearAxle maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.5" positionX="0.0" positionZ="0.3"/>
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

    println!("\n=== Test 2: Invalid (missing vehicleCategory) ===");
    let report = validator.validate(invalid_xml);
    println!("Valid: {}", report.valid);
    if !report.errors.is_empty() {
        println!("Errors:");
        for error in &report.errors {
            println!("  - {}", error);
        }
    }

    // Test 3: Wrong element order
    let wrong_order_xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <RoadNetwork>
        <LogicFile filepath="test.xodr"/>
    </RoadNetwork>
    <FileHeader revMajor="1" revMinor="2" date="2024-01-01T00:00:00" description="Test" author="Test"/>
    <Entities>
        <ScenarioObject name="Ego">
            <Vehicle name="car1" vehicleCategory="car">
                <BoundingBox>
                    <Center x="0.0" y="0.0" z="0.0"/>
                    <Dimensions width="2.0" length="5.0" height="1.5"/>
                </BoundingBox>
                <Performance maxSpeed="50.0" maxAcceleration="3.0" maxDeceleration="5.0"/>
                <Axles>
                    <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.5" positionX="2.5" positionZ="0.3"/>
                    <RearAxle maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.5" positionX="0.0" positionZ="0.3"/>
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

    println!("\n=== Test 3: Invalid (wrong element order) ===");
    let report = validator.validate(wrong_order_xml);
    println!("Valid: {}", report.valid);
    if !report.errors.is_empty() {
        println!("Errors:");
        for error in &report.errors {
            println!("  - {}", error);
        }
    }
}
