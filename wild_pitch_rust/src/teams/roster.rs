use crate::players::{Player, Position, PitcherRole};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roster {
    pub players: HashMap<String, Player>,
    pub active_roster: Vec<String>, // Player IDs on active roster
    pub injured_list: Vec<String>,  // Player IDs on injured list
    pub max_roster_size: usize,
    pub max_active_size: usize,
}

impl Roster {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            active_roster: Vec::new(),
            injured_list: Vec::new(),
            max_roster_size: 40,
            max_active_size: 25,
        }
    }

    pub fn add_player(&mut self, player: Player) -> Result<(), String> {
        if self.players.len() >= self.max_roster_size {
            return Err("Roster is full".to_string());
        }

        let player_id = player.id.clone();
        self.players.insert(player_id.clone(), player);
        
        if self.active_roster.len() < self.max_active_size {
            self.active_roster.push(player_id);
        }

        Ok(())
    }

    pub fn remove_player(&mut self, player_id: &str) -> Result<Player, String> {
        if let Some(player) = self.players.remove(player_id) {
            self.active_roster.retain(|id| id != player_id);
            self.injured_list.retain(|id| id != player_id);
            Ok(player)
        } else {
            Err("Player not found".to_string())
        }
    }

    pub fn get_player(&self, player_id: &str) -> Option<&Player> {
        self.players.get(player_id)
    }

    pub fn get_player_mut(&mut self, player_id: &str) -> Option<&mut Player> {
        self.players.get_mut(player_id)
    }

    pub fn activate_player(&mut self, player_id: &str) -> Result<(), String> {
        if !self.players.contains_key(player_id) {
            return Err("Player not found".to_string());
        }

        if self.active_roster.contains(&player_id.to_string()) {
            return Err("Player already active".to_string());
        }

        if self.active_roster.len() >= self.max_active_size {
            return Err("Active roster is full".to_string());
        }

        self.active_roster.push(player_id.to_string());
        self.injured_list.retain(|id| id != player_id);
        Ok(())
    }

    pub fn deactivate_player(&mut self, player_id: &str) -> Result<(), String> {
        if !self.active_roster.contains(&player_id.to_string()) {
            return Err("Player not on active roster".to_string());
        }

        self.active_roster.retain(|id| id != player_id);
        Ok(())
    }

    pub fn move_to_injured_list(&mut self, player_id: &str) -> Result<(), String> {
        if !self.players.contains_key(player_id) {
            return Err("Player not found".to_string());
        }

        if let Some(player) = self.players.get_mut(player_id) {
            player.fielder.is_injured = true;
            if let Some(ref mut batter) = player.batter {
                batter.is_injured = true;
            }
            if let Some(ref mut pitcher) = player.pitcher {
                pitcher.is_injured = true;
            }
        }

        self.active_roster.retain(|id| id != player_id);
        if !self.injured_list.contains(&player_id.to_string()) {
            self.injured_list.push(player_id.to_string());
        }

        Ok(())
    }

    pub fn activate_from_injured_list(&mut self, player_id: &str) -> Result<(), String> {
        if !self.injured_list.contains(&player_id.to_string()) {
            return Err("Player not on injured list".to_string());
        }

        if self.active_roster.len() >= self.max_active_size {
            return Err("Active roster is full".to_string());
        }

        if let Some(player) = self.players.get_mut(player_id) {
            player.fielder.is_injured = false;
            if let Some(ref mut batter) = player.batter {
                batter.is_injured = false;
            }
            if let Some(ref mut pitcher) = player.pitcher {
                pitcher.is_injured = false;
            }
        }

        self.injured_list.retain(|id| id != player_id);
        self.active_roster.push(player_id.to_string());

        Ok(())
    }

    pub fn get_active_players(&self) -> Vec<&Player> {
        self.active_roster
            .iter()
            .filter_map(|id| self.players.get(id))
            .collect()
    }

    pub fn get_injured_players(&self) -> Vec<&Player> {
        self.injured_list
            .iter()
            .filter_map(|id| self.players.get(id))
            .collect()
    }

    pub fn get_players_by_position(&self, position: Position) -> Vec<&Player> {
        self.get_active_players()
            .into_iter()
            .filter(|player| player.can_play_position(position))
            .collect()
    }

    pub fn get_pitchers(&self) -> Vec<&Player> {
        self.get_active_players()
            .into_iter()
            .filter(|player| player.is_pitcher())
            .collect()
    }

    pub fn get_starters(&self) -> Vec<&Player> {
        self.get_pitchers()
            .into_iter()
            .filter(|player| {
                player.pitcher.as_ref()
                    .map_or(false, |p| p.role == PitcherRole::Starter)
            })
            .collect()
    }

    pub fn get_relievers(&self) -> Vec<&Player> {
        self.get_pitchers()
            .into_iter()
            .filter(|player| {
                player.pitcher.as_ref()
                    .map_or(false, |p| matches!(p.role, PitcherRole::Reliever | PitcherRole::Closer))
            })
            .collect()
    }

    pub fn get_position_players(&self) -> Vec<&Player> {
        self.get_active_players()
            .into_iter()
            .filter(|player| player.is_position_player())
            .collect()
    }

    pub fn is_roster_legal(&self) -> bool {
        let active_count = self.active_roster.len();
        let pitcher_count = self.get_pitchers().len();
        let position_player_count = self.get_position_players().len();

        active_count <= self.max_active_size &&
        pitcher_count >= 5 && // Minimum pitchers
        position_player_count >= 8 // Minimum position players (one for each position)
    }

    pub fn roster_summary(&self) -> String {
        let total = self.players.len();
        let active = self.active_roster.len();
        let injured = self.injured_list.len();
        let pitchers = self.get_pitchers().len();
        let position_players = self.get_position_players().len();

        format!(
            "Roster: {}/{} total, {}/{} active, {} injured, {} pitchers, {} position players",
            total, self.max_roster_size,
            active, self.max_active_size,
            injured, pitchers, position_players
        )
    }
}