use crate::players::{Player, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineupSpot {
    pub player_id: String,
    pub batting_order: u8, // 1-9
    pub position: Position,
}

impl LineupSpot {
    pub fn new(player_id: String, batting_order: u8, position: Position) -> Self {
        Self {
            player_id,
            batting_order,
            position,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lineup {
    pub spots: Vec<LineupSpot>,
    pub designated_hitter: Option<String>, // Player ID
    pub starting_pitcher_id: String,
    pub bench_players: Vec<String>, // Player IDs
}

impl Lineup {
    pub fn new() -> Self {
        Self {
            spots: Vec::new(),
            designated_hitter: None,
            starting_pitcher_id: String::new(),
            bench_players: Vec::new(),
        }
    }

    pub fn add_batter(&mut self, player_id: String, position: Position) -> Result<(), String> {
        if self.spots.len() >= 9 {
            return Err("Lineup is full".to_string());
        }

        // Check if position is already filled (except DH)
        if position != Position::DesignatedHitter {
            if self.spots.iter().any(|spot| spot.position == position) {
                return Err(format!("Position {} already filled", position.abbreviation()));
            }
        }

        let batting_order = (self.spots.len() + 1) as u8;
        self.spots.push(LineupSpot::new(player_id, batting_order, position));
        Ok(())
    }

    pub fn set_starting_pitcher(&mut self, player_id: String) {
        self.starting_pitcher_id = player_id;
    }

    pub fn set_designated_hitter(&mut self, player_id: String) {
        self.designated_hitter = Some(player_id);
    }

    pub fn add_bench_player(&mut self, player_id: String) {
        if !self.bench_players.contains(&player_id) {
            self.bench_players.push(player_id);
        }
    }

    pub fn remove_from_lineup(&mut self, player_id: &str) -> Option<LineupSpot> {
        if let Some(pos) = self.spots.iter().position(|spot| spot.player_id == player_id) {
            let removed_spot = self.spots.remove(pos);
            
            // Reorder batting order
            for (i, spot) in self.spots.iter_mut().enumerate() {
                spot.batting_order = (i + 1) as u8;
            }
            
            Some(removed_spot)
        } else {
            None
        }
    }

    pub fn substitute_player(&mut self, old_player_id: &str, new_player_id: String, new_position: Option<Position>) -> Result<(), String> {
        // Remove old player from lineup
        if let Some(mut spot) = self.remove_from_lineup(old_player_id) {
            // Update with new player
            spot.player_id = new_player_id.clone();
            if let Some(pos) = new_position {
                spot.position = pos;
            }
            
            // Insert back in correct batting order position
            let batting_order = spot.batting_order;
            self.spots.insert((batting_order - 1) as usize, spot);
            
            Ok(())
        } else if self.designated_hitter.as_ref() == Some(&old_player_id.to_string()) {
            // Handle DH substitution
            self.designated_hitter = Some(new_player_id);
            Ok(())
        } else if self.starting_pitcher_id == old_player_id {
            // Handle pitcher substitution
            self.starting_pitcher_id = new_player_id;
            Ok(())
        } else {
            Err("Player not found in lineup".to_string())
        }
    }

    pub fn move_batting_order(&mut self, player_id: &str, new_order: u8) -> Result<(), String> {
        if new_order < 1 || new_order > 9 {
            return Err("Batting order must be between 1 and 9".to_string());
        }

        if let Some(spot) = self.spots.iter_mut().find(|spot| spot.player_id == player_id) {
            let old_order = spot.batting_order;
            spot.batting_order = new_order;
            
            // Adjust other players' batting orders
            for other_spot in self.spots.iter_mut() {
                if other_spot.player_id != player_id {
                    if old_order < new_order && other_spot.batting_order > old_order && other_spot.batting_order <= new_order {
                        other_spot.batting_order -= 1;
                    } else if old_order > new_order && other_spot.batting_order >= new_order && other_spot.batting_order < old_order {
                        other_spot.batting_order += 1;
                    }
                }
            }
            
            // Sort spots by batting order
            self.spots.sort_by_key(|spot| spot.batting_order);
            Ok(())
        } else {
            Err("Player not found in lineup".to_string())
        }
    }

    pub fn get_batter_by_order(&self, batting_order: u8) -> Option<&LineupSpot> {
        self.spots.iter().find(|spot| spot.batting_order == batting_order)
    }

    pub fn get_player_at_position(&self, position: Position) -> Option<&LineupSpot> {
        self.spots.iter().find(|spot| spot.position == position)
    }

    pub fn get_batting_order(&self, player_id: &str) -> Option<u8> {
        self.spots.iter()
            .find(|spot| spot.player_id == player_id)
            .map(|spot| spot.batting_order)
    }

    pub fn is_complete(&self) -> bool {
        self.spots.len() == 9 && !self.starting_pitcher_id.is_empty()
    }

    pub fn has_designated_hitter(&self) -> bool {
        self.designated_hitter.is_some()
    }

    pub fn validate_lineup(&self, roster_players: &[&Player]) -> Vec<String> {
        let mut errors = Vec::new();

        // Check if lineup is complete
        if !self.is_complete() {
            errors.push("Lineup is not complete".to_string());
        }

        // Check if all positions are filled (except DH in AL)
        let required_positions = [
            Position::Catcher, Position::FirstBase, Position::SecondBase, Position::ThirdBase,
            Position::Shortstop, Position::LeftField, Position::CenterField, Position::RightField
        ];

        for &pos in &required_positions {
            if !self.spots.iter().any(|spot| spot.position == pos) {
                errors.push(format!("No player assigned to {}", pos.full_name()));
            }
        }

        // Check if players can play their assigned positions
        for spot in &self.spots {
            if let Some(player) = roster_players.iter().find(|p| p.id == spot.player_id) {
                if !player.can_play_position(spot.position) {
                    errors.push(format!("{} cannot play {}", player.name, spot.position.full_name()));
                }
            } else {
                errors.push(format!("Player {} not found in roster", spot.player_id));
            }
        }

        // Check starting pitcher
        if let Some(pitcher) = roster_players.iter().find(|p| p.id == self.starting_pitcher_id) {
            if !pitcher.is_pitcher() {
                errors.push(format!("{} is not a pitcher", pitcher.name));
            }
        } else {
            errors.push("Starting pitcher not found in roster".to_string());
        }

        errors
    }

    pub fn display_lineup(&self, roster_players: &[&Player]) -> Vec<String> {
        let mut display = Vec::new();
        
        display.push("BATTING ORDER:".to_string());
        for spot in &self.spots {
            if let Some(player) = roster_players.iter().find(|p| p.id == spot.player_id) {
                display.push(format!(
                    "{}. {} {} - {}",
                    spot.batting_order,
                    player.name,
                    player.jersey_number,
                    spot.position.abbreviation()
                ));
            }
        }

        if let Some(dh_id) = &self.designated_hitter {
            if let Some(dh_player) = roster_players.iter().find(|p| p.id == *dh_id) {
                display.push(format!("DH: {} {}", dh_player.name, dh_player.jersey_number));
            }
        }

        if let Some(pitcher) = roster_players.iter().find(|p| p.id == self.starting_pitcher_id) {
            display.push(format!("SP: {} {}", pitcher.name, pitcher.jersey_number));
        }

        display
    }
}