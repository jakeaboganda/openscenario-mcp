use openscenario_mcp::scenario_templates::handle_create_quick_scenario;
use openscenario_mcp::handlers::handle_get_real_world_road;
use openscenario_mcp::server::ServerState;
use std::sync::{Arc, Mutex};

fn main() {
    println!("🧪 Testing Phase 4: Scenario Helpers");
    println!("{}", "=".repeat(60));
    println!();
    
    // Create server state
    let state = Arc::new(Mutex::new(ServerState::new()));
    
    // Step 1: Load Nihonbashi roads
    println!("📍 Step 1: Loading Nihonbashi road network...");
    println!("{}", "-".repeat(60));
    
    match handle_get_real_world_road(
        state.clone(),
        "nihonbashi".to_string(),
        None,
    ) {
        Ok(result) => {
            match serde_json::from_str::<serde_json::Value>(&result) {
                Ok(json) => {
                    println!("✅ Road network loaded!");
                    if let Some(recommended) = json.get("recommended_road") {
                        if let Some(road_id) = recommended.get("road_id").and_then(|v| v.as_str()) {
                            println!("   Recommended road: {}", road_id);
                        }
                        if let Some(length) = recommended.get("length").and_then(|v| v.as_f64()) {
                            println!("   Length: {:.1}m", length);
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
            eprintln!("❌ Failed to load road network: {}", e);
            std::process::exit(1);
        }
    }
    
    println!();
    
    // Step 2: Create quick lane change scenario
    println!("🚗 Step 2: Creating lane change scenario...");
    println!("{}", "-".repeat(60));
    
    match handle_create_quick_scenario(
        state.clone(),
        "lane_change".to_string(),
        None, // default 3 vehicles
    ) {
        Ok(result) => {
            match serde_json::from_str::<serde_json::Value>(&result) {
                Ok(json) => {
                    println!("✅ Scenario created!");
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
            std::process::exit(1);
        }
    }
    
    println!();
    
    // Step 3: Create quick cut-in scenario
    println!("⚠️  Step 3: Creating cut-in scenario...");
    println!("{}", "-".repeat(60));
    
    match handle_create_quick_scenario(
        state.clone(),
        "cutin".to_string(),
        Some(2), // just 2 vehicles
    ) {
        Ok(result) => {
            match serde_json::from_str::<serde_json::Value>(&result) {
                Ok(json) => {
                    println!("✅ Cut-in scenario created!");
                    if let Some(scenario_type) = json.get("type").and_then(|v| v.as_str()) {
                        println!("   Type: {}", scenario_type);
                    }
                    if let Some(vehicles) = json.get("vehicles") {
                        println!("   Vehicles: {}", vehicles);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse scenario: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create scenario: {}", e);
            std::process::exit(1);
        }
    }
    
    println!();
    
    // Step 4: Create quick platoon scenario
    println!("🚚 Step 4: Creating platoon scenario...");
    println!("{}", "-".repeat(60));
    
    match handle_create_quick_scenario(
        state.clone(),
        "platoon".to_string(),
        Some(5), // 5 vehicles in convoy
    ) {
        Ok(result) => {
            match serde_json::from_str::<serde_json::Value>(&result) {
                Ok(json) => {
                    println!("✅ Platoon scenario created!");
                    if let Some(platoon) = json.get("platoon") {
                        println!();
                        println!("Platoon details:");
                        println!("{}", serde_json::to_string_pretty(platoon).unwrap());
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse scenario: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create scenario: {}", e);
            std::process::exit(1);
        }
    }
    
    println!();
    println!("{}", "=".repeat(60));
    println!("✅ All Phase 4 tests passed!");
    println!("{}", "=".repeat(60));
    println!();
    println!("🎉 Scenario templates are working!");
    println!();
    println!("Available MCP tools:");
    println!("  - create_quick_scenario(type, vehicle_count?)");
    println!("  - create_lane_change_scenario(...)");
    println!("  - create_cutin_scenario(...)");
    println!("  - create_platoon_scenario(...)");
    println!();
    println!("Next: Export scenarios and visualize in esmini!");
}
