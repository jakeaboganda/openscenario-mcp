use openscenario::opendrive_validator::OpenDriveValidator;
use std::path::Path;

fn main() {
    let road_path = Path::new("roads/simple_highway.xodr");
    
    println!("🔍 Loading OpenDRIVE file: {:?}", road_path);
    
    let validator = match OpenDriveValidator::load(road_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("❌ Failed to load: {}", e);
            return;
        }
    };
    
    println!("✅ OpenDRIVE loaded successfully!\n");
    
    // Test 1: List all roads
    println!("📋 Roads in network:");
    let roads = validator.list_roads();
    for road in &roads {
        println!("  - Road '{}': {} lanes, {:.1}m long", 
                 road.id, road.lane_count, road.length);
        if let Some(name) = &road.name {
            println!("    Name: {}", name);
        }
    }
    println!();
    
    // Test 2: Get detailed info
    if let Some(first_road) = roads.first() {
        println!("🔍 Detailed info for road '{}':", first_road.id);
        if let Some(info) = validator.get_road_info(&first_road.id) {
            println!("  Length: {:.1}m", info.length);
            println!("  Lanes:");
            for lane in &info.lanes {
                println!("    - Lane {}: {}", lane.id, lane.lane_type);
            }
        }
        println!();
        
        // Test 3: Suggest spawn points
        println!("🎯 Suggested spawn points (3 vehicles):");
        match validator.suggest_spawn_points(&first_road.id, 3) {
            Ok(points) => {
                for (i, point) in points.iter().enumerate() {
                    println!("  {}. {}", i + 1, point.description);
                    println!("     Position: lane={}, s={:.1}m", point.lane_id, point.s);
                }
            }
            Err(e) => eprintln!("  ❌ Error: {}", e),
        }
        println!();
    }
    
    // Test 4: Quality assessment
    println!("📊 Data quality assessment:");
    let quality = validator.assess_quality();
    println!("  Score: {}/100", quality.score);
    println!("  Has lanes: {}", quality.has_lanes);
    println!("  Has geometry: {}", quality.has_geometry);
    println!("  Valid lengths: {}", quality.has_valid_length);
    if !quality.issues.is_empty() {
        println!("  Issues:");
        for issue in &quality.issues {
            println!("    - {}", issue);
        }
    }
    
    println!("\n✅ All tests completed!");
}
