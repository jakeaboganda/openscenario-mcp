use openscenario::opendrive_validator::OpenDriveValidator;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --example find_good_roads <path-to-xodr>");
        std::process::exit(1);
    }

    let road_path = Path::new(&args[1]);

    println!("🔍 Finding good roads in: {:?}\n", road_path);

    let validator = match OpenDriveValidator::load(road_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("❌ Failed to load: {}", e);
            std::process::exit(1);
        }
    };

    let roads = validator.list_roads();

    // Filter for good roads: >50m, has lanes
    let mut good_roads: Vec<_> = roads
        .iter()
        .filter(|r| r.length > 50.0 && r.lane_count > 1)
        .collect();

    // Sort by length descending
    good_roads.sort_by(|a, b| b.length.partial_cmp(&a.length).unwrap());

    println!("📊 Statistics:");
    println!("  Total roads: {}", roads.len());
    println!("  Good roads (>50m, lanes>1): {}\n", good_roads.len());

    if good_roads.is_empty() {
        println!("❌ No roads >50m found!");
        return;
    }

    println!("🏆 Top 20 Longest Roads:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    for (i, road) in good_roads.iter().take(20).enumerate() {
        println!("{}. Road '{}'", i + 1, road.id);
        println!(
            "   Length: {:.1}m ({:.2}km)",
            road.length,
            road.length / 1000.0
        );
        println!("   Lanes: {}", road.lane_count);

        if let Some(info) = validator.get_road_info(&road.id) {
            let driving_lanes = info
                .lanes
                .iter()
                .filter(|l| l.lane_type == "driving")
                .count();
            println!("   Driving lanes: {}", driving_lanes);
        }

        if let Some(name) = &road.name {
            if !name.is_empty() {
                println!("   Name: {}", name);
            }
        }

        // Show sample spawn points
        if let Ok(points) = validator.suggest_spawn_points(&road.id, 3) {
            if !points.is_empty() {
                println!("   ✅ Can spawn {} vehicles", points.len());
            }
        }

        println!();
    }

    // Pick the best road for testing
    if let Some(best) = good_roads.first() {
        println!("\n🎯 Recommended Test Road: '{}'", best.id);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!(
            "  Length: {:.1}m ({:.2}km)",
            best.length,
            best.length / 1000.0
        );
        println!("  Lanes: {}", best.lane_count);

        match validator.suggest_spawn_points(&best.id, 5) {
            Ok(points) => {
                println!("\n  Spawn points for 5 vehicles:");
                for (i, point) in points.iter().enumerate() {
                    println!(
                        "    {}. lane={}, s={:.1}m ({})",
                        i + 1,
                        point.lane_id,
                        point.s,
                        point.description
                    );
                }
            }
            Err(e) => println!("  ⚠️  Spawn point error: {}", e),
        }
    }
}
