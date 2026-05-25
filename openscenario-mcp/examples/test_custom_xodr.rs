use openscenario_mcp::handlers::handle_load_road_network;
use openscenario_mcp::scenario_templates::handle_create_quick_scenario;
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
    
    // Example: Use the simple highway test file
    let xodr_path = "roads/simple_highway.xodr";
    
    match handle_load_road_network(state.clone(), xodr_path.to_string()) {
        Ok(result) => {
            match serde_json::from_str::<serde_json::Value>(&result) {
                Ok(json) => {
                    println!("✅ Custom XODR loaded successfully!");
                    if let Some(road_count) = json.get("road_count").and_then(|v| v.as_u64()) {
                        println!("   Roads found: {}", road_count);
                    }
                    if let Some(quality) = json.get("quality") {
                        if let Some(score) = quality.get("score").and_then(|v| v.as_u64()) {
                            println!("   Quality score: {}/100", score);
                        }
                    }
                    println!();
                    
                    // Show available roads
                    if let Some(roads) = json.get("roads").and_then(|v| v.as_array()) {
                        println!("Available roads:");
                        for (i, road) in roads.iter().take(5).enumerate() {
                            if let (Some(id), Some(length)) = (
                                road.get("id").and_then(|v| v.as_str()),
                                road.get("length").and_then(|v| v.as_f64())
                            ) {
                                println!("   {}. Road {} - {:.1}m", i + 1, id, length);
                            }
                        }
                        if roads.len() > 5 {
                            println!("   ... and {} more roads", roads.len() - 5);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse response: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to load XODR file: {}", e);
            eprintln!();
            eprintln!("Make sure the file exists at: {}", xodr_path);
            eprintln!("Or provide your own XODR file path.");
            std::process::exit(1);
        }
    }
    
    println!();
    
    // Step 2: Create a scenario on the custom road network
    println!("🚗 Step 2: Creating scenario on custom road...");
    println!("{}", "-".repeat(60));
    
    match handle_create_quick_scenario(
        state.clone(),
        "lane_change".to_string(),
        Some(2), // 2 vehicles
    ) {
        Ok(result) => {
            match serde_json::from_str::<serde_json::Value>(&result) {
                Ok(json) => {
                    println!("✅ Scenario created on custom road!");
                    println!();
                    println!("Scenario details:");
                    println!("{}", serde_json::to_string_pretty(&json).unwrap());
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse scenario: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create scenario: {}", e);
            eprintln!();
            eprintln!("This may happen if:");
            eprintln!("  - The road network has no suitable roads");
            eprintln!("  - Roads are too short (<200m)");
            eprintln!("  - No multi-lane roads available");
            std::process::exit(1);
        }
    }
    
    println!();
    println!("{}", "=".repeat(60));
    println!("✅ Custom XODR workflow complete!");
    println!("{}", "=".repeat(60));
    println!();
    println!("Summary:");
    println!("  1. ✅ Loaded custom XODR file");
    println!("  2. ✅ Analyzed road network");
    println!("  3. ✅ Created scenario on custom roads");
    println!();
    println!("Next steps:");
    println!("  - Export the scenario: export_xml(scenario_id, \"output.xosc\")");
    println!("  - Visualize: esmini --osc output.xosc --road {}", xodr_path);
    println!("  - Try different scenario types: 'cutin', 'platoon'");
}
