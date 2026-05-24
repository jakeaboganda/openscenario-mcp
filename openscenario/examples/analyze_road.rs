use openscenario::opendrive_validator::OpenDriveValidator;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --example analyze_road <path-to-xodr>");
        eprintln!("Example: cargo run --example analyze_road cache/osm/nihonbashi_real.xodr");
        std::process::exit(1);
    }

    let road_path = Path::new(&args[1]);

    println!("🔍 Analyzing OpenDRIVE file: {:?}\n", road_path);

    let validator = match OpenDriveValidator::load(road_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("❌ Failed to load: {}", e);
            std::process::exit(1);
        }
    };

    println!("✅ OpenDRIVE loaded successfully!\n");

    // Quality assessment first
    println!("📊 Quality Assessment:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let quality = validator.assess_quality();
    println!("  Score: {}/100", quality.score);
    println!("  Has lanes: {}", quality.has_lanes);
    println!("  Has geometry: {}", quality.has_geometry);
    println!("  Valid lengths: {}", quality.has_valid_length);
    if !quality.issues.is_empty() {
        println!("  ⚠️  Issues found:");
        for issue in &quality.issues {
            println!("    • {}", issue);
        }
    }
    println!();

    // List all roads
    println!("🗺️  Road Network:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let roads = validator.list_roads();
    println!("  Total roads: {}", roads.len());

    // Show first 10 roads with details
    let display_count = roads.len().min(10);
    println!("  Showing {} of {} roads:\n", display_count, roads.len());

    for (i, road) in roads.iter().take(display_count).enumerate() {
        println!("  {}. Road '{}'", i + 1, road.id);
        println!("     Length: {:.1}m", road.length);
        println!("     Lanes: {}", road.lane_count);
        if let Some(name) = &road.name {
            if !name.is_empty() {
                println!("     Name: {}", name);
            }
        }

        // Get detailed info
        if let Some(info) = validator.get_road_info(&road.id) {
            print!("     Lane IDs: ");
            let lane_ids: Vec<String> = info
                .lanes
                .iter()
                .map(|l| format!("{} ({})", l.id, l.lane_type))
                .collect();
            println!("{}", lane_ids.join(", "));
        }
        println!();
    }

    if roads.len() > 10 {
        println!("  ... and {} more roads", roads.len() - 10);
        println!();
    }

    // Test spawn point generation on first road
    if let Some(first_road) = roads.first() {
        println!("🎯 Spawn Point Test (Road '{}'):", first_road.id);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        match validator.suggest_spawn_points(&first_road.id, 5) {
            Ok(points) => {
                if points.is_empty() {
                    println!("  ⚠️  No driving lanes found on this road");
                } else {
                    println!("  Suggested {} spawn points:\n", points.len());
                    for (i, point) in points.iter().enumerate() {
                        println!("  {}. {}", i + 1, point.description);
                        println!(
                            "     road_id='{}', lane_id={}, s={:.1}m",
                            point.road_id, point.lane_id, point.s
                        );
                    }
                }
            }
            Err(e) => eprintln!("  ❌ Error: {}", e),
        }
        println!();
    }

    println!("✅ Analysis complete!");
}
