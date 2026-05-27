use openscenario_mcp::handlers::{handle_list_roads, handle_load_road_network};
use openscenario_mcp::server::ServerState;
use std::sync::{Arc, Mutex};

fn main() {
    println!("🧪 Testing Custom XODR File Loading");
    println!("{}", "=".repeat(60));
    println!();

    // Create server state
    let state = Arc::new(Mutex::new(ServerState::new()));

    // Step 1: Load a custom XODR file
    println!("📁 Step 1: Loading custom XODR file...");
    println!("{}", "-".repeat(60));

    // Example: Use a cached road network (from previous OSM download)
    // This demonstrates loading ANY .xodr file - could be your own!
    let xodr_path = "cache/osm/nihonbashi.xodr";

    // Other examples you could try:
    // let xodr_path = "roads/simple_highway.xodr";  // Simple test road
    // let xodr_path = "/path/to/your/custom/track.xodr";  // Your own XODR
    // let xodr_path = "~/my_tracks/circuit.xodr";  // User home directory

    match handle_load_road_network(state.clone(), xodr_path.to_string()) {
        Ok(result) => match serde_json::from_str::<serde_json::Value>(&result) {
            Ok(json) => {
                println!("✅ Custom XODR loaded successfully!");
                println!();
                if let Some(road_count) = json.get("road_count").and_then(|v| v.as_u64()) {
                    println!("📊 Statistics:");
                    println!("   Total roads: {}", road_count);
                }
                if let Some(quality) = json.get("quality") {
                    if let Some(score) = quality.get("score").and_then(|v| v.as_u64()) {
                        println!("   Quality score: {}/100", score);
                    }
                    if let Some(has_lanes) = quality.get("has_lanes").and_then(|v| v.as_bool()) {
                        println!("   Has lane info: {}", if has_lanes { "✅" } else { "❌" });
                    }
                }
                println!();
            }
            Err(e) => {
                eprintln!("❌ Failed to parse response: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("❌ Failed to load XODR file: {}", e);
            eprintln!();
            eprintln!("Possible reasons:");
            eprintln!("  - File doesn't exist at: {}", xodr_path);
            eprintln!("  - File is not valid OpenDRIVE format");
            eprintln!("  - No roads with lanes defined");
            eprintln!();
            eprintln!("Try:");
            eprintln!("  1. Check the file exists: ls {}", xodr_path);
            eprintln!("  2. Use get_real_world_road to download: cargo run --example test_get_real_world_road");
            eprintln!("  3. Provide your own XODR file path");
            std::process::exit(1);
        }
    }

    // Step 2: List roads to see what's available
    println!("📋 Step 2: Listing available roads...");
    println!("{}", "-".repeat(60));

    match handle_list_roads(state.clone()) {
        Ok(result) => {
            match serde_json::from_str::<serde_json::Value>(&result) {
                Ok(json) => {
                    if let Some(roads) = json.get("roads").and_then(|v| v.as_array()) {
                        // Filter for good roads (>200m with multiple lanes)
                        let good_roads: Vec<_> = roads
                            .iter()
                            .filter(|r| {
                                let length =
                                    r.get("length").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let lanes =
                                    r.get("lane_count").and_then(|v| v.as_u64()).unwrap_or(0);
                                length > 200.0 && lanes > 1
                            })
                            .collect();

                        println!(
                            "✅ Found {} roads suitable for scenarios (>200m, multi-lane):",
                            good_roads.len()
                        );
                        println!();

                        for (i, road) in good_roads.iter().take(10).enumerate() {
                            if let (Some(id), Some(length), Some(lanes)) = (
                                road.get("id").and_then(|v| v.as_str()),
                                road.get("length").and_then(|v| v.as_f64()),
                                road.get("lane_count").and_then(|v| v.as_u64()),
                            ) {
                                let name = road
                                    .get("name")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("(unnamed)");
                                println!(
                                    "   {}. Road {} - {:.0}m, {} lanes - {}",
                                    i + 1,
                                    id,
                                    length,
                                    lanes,
                                    name
                                );
                            }
                        }

                        if good_roads.len() > 10 {
                            println!("   ... and {} more suitable roads", good_roads.len() - 10);
                        }

                        if good_roads.is_empty() {
                            println!("   ⚠️  No roads >200m with multiple lanes found.");
                            println!("       Try a different XODR file for scenario generation.");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse road list: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list roads: {}", e);
            std::process::exit(1);
        }
    }

    println!();
    println!("{}", "=".repeat(60));
    println!("✅ Custom XODR workflow test complete!");
    println!("{}", "=".repeat(60));
    println!();
    println!("Summary:");
    println!("  1. ✅ Loaded custom XODR file: {}", xodr_path);
    println!("  2. ✅ Analyzed road network quality");
    println!("  3. ✅ Listed available roads for scenarios");
    println!();
    println!("Next steps:");
    println!("  - With Claude Desktop: 'Create a lane change scenario'");
    println!("  - Or use MCP tools: create_quick_scenario('lane_change')");
    println!(
        "  - Visualize: esmini --road {} --osc scenario.xosc",
        xodr_path
    );
    println!();
    println!("Try your own XODR:");
    println!("  - Edit this example to use your file path");
    println!("  - Any OpenDRIVE 1.4+ file works");
    println!("  - From RoadRunner, CARLA, SUMO, or custom tools");
}
