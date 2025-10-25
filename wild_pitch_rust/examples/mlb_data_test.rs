use wild_pitch::data::MLBTestData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Wild Pitch MLB Data Integration Test ===\n");

    // Test creating MLB teams
    println!("Creating Yankees and Dodgers teams from Baseball Savant data...");
    match MLBTestData::create_mlb_teams() {
        Ok((yankees, dodgers)) => {
            println!("✅ Successfully created MLB teams!\n");

            println!("📊 Team Information:");
            println!("Yankees: {} - {}", yankees.full_name(), yankees.ballpark_name);
            println!("  Active Players: {}", yankees.get_active_players().len());
            println!("  Team Colors: {:?}", yankees.colors);
            
            println!("\nDodgers: {} - {}", dodgers.full_name(), dodgers.ballpark_name);
            println!("  Active Players: {}", dodgers.get_active_players().len());
            println!("  Team Colors: {:?}", dodgers.colors);

            // Show some player examples
            println!("\n🏟️ Sample Yankees Players:");
            for (i, player) in yankees.get_active_players().iter().take(3).enumerate() {
                println!("  {}. {} #{} ({})", 
                    i + 1, 
                    player.name, 
                    player.jersey_number, 
                    player.primary_position().abbreviation()
                );
                if let Some(ref batter) = player.batter {
                    println!("     Contact: {:.3}, Power: {:.3}, Speed: {:.3}",
                        batter.tendencies.contact_rate,
                        batter.tendencies.power_rating,
                        batter.tendencies.speed_rating
                    );
                }
            }

            println!("\n🏟️ Sample Dodgers Players:");
            for (i, player) in dodgers.get_active_players().iter().take(3).enumerate() {
                println!("  {}. {} #{} ({})", 
                    i + 1, 
                    player.name, 
                    player.jersey_number, 
                    player.primary_position().abbreviation()
                );
                if let Some(ref batter) = player.batter {
                    println!("     Contact: {:.3}, Power: {:.3}, Speed: {:.3}",
                        batter.tendencies.contact_rate,
                        batter.tendencies.power_rating,
                        batter.tendencies.speed_rating
                    );
                }
            }

            println!("\n📈 Data Conversion Analysis:");
            println!("{}", MLBTestData::analyze_player_conversion());

        },
        Err(e) => {
            println!("❌ Failed to create MLB teams: {}", e);
            return Err(e.into());
        }
    }

    println!("🎮 Test completed successfully!");
    println!("\n💡 Tips:");
    println!("- Run the main game with: cargo run");
    println!("- Start a new game to see Yankees vs Dodgers");
    println!("- Check Settings > MLB Data Analysis for detailed conversion info");
    println!("- Double key press issue has been fixed!");

    Ok(())
}