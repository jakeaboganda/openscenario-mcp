use openscenario_mcp::handlers::handle_get_real_world_road;
use openscenario_mcp::server::ServerState;
use std::sync::{Arc, Mutex};

fn main() {
    println!("🧪 Testing MCP Integration: get_real_world_road");
    println!("{}", "=".repeat(60));
    println!();

    // Create server state
    let state = Arc::new(Mutex::new(ServerState::new()));

    // Test 1: Get Nihonbashi roads
    println!("📍 Test 1: Nihonbashi (pre-configured location)");
    println!("{}", "-".repeat(60));

    match handle_get_real_world_road(state.clone(), "nihonbashi".to_string(), None) {
        Ok(result) => {
            // Parse JSON response
            match serde_json::from_str::<serde_json::Value>(&result) {
                Ok(json) => {
                    println!("✅ Success!");
                    println!();
                    println!("Response:");
                    println!("{}", serde_json::to_string_pretty(&json).unwrap());
                    println!();

                    // Check key fields
                    if let Some(status) = json.get("status").and_then(|v| v.as_str()) {
                        assert_eq!(status, "success", "Status should be success");
                    }

                    if let Some(total) = json.get("total_roads").and_then(|v| v.as_u64()) {
                        println!("📊 Found {} total roads", total);
                    }

                    if let Some(good) = json.get("good_roads").and_then(|v| v.as_u64()) {
                        println!("✅ {} good roads (>50m with lanes)", good);
                    }

                    if let Some(quality) = json.get("quality") {
                        if let Some(score) = quality.get("score").and_then(|v| v.as_u64()) {
                            println!("📈 Quality score: {}/100", score);
                        }
                    }

                    if let Some(recommended) = json.get("recommended_road") {
                        println!();
                        println!("🎯 Recommended road:");
                        println!("{}", serde_json::to_string_pretty(recommended).unwrap());
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse JSON response: {}", e);
                    eprintln!("Raw response: {}", result);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Test failed: {}", e);
            std::process::exit(1);
        }
    }

    println!();
    println!("{}", "=".repeat(60));
    println!("✅ All MCP integration tests passed!");
    println!("{}", "=".repeat(60));
    println!();
    println!("🎉 The get_real_world_road MCP tool is working!");
    println!();
    println!("Next steps:");
    println!("  1. Start MCP server: cargo run -p openscenario-mcp");
    println!("  2. Connect via Claude Desktop or other MCP client");
    println!("  3. Call: get_real_world_road(location='nihonbashi')");
}
