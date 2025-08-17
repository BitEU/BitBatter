pub mod roster;
pub mod lineup;
pub mod stats;

pub use roster::*;
pub use lineup::*;
pub use stats::*;

use crate::players::Player;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub city: String,
    pub abbreviation: String,
    pub colors: (String, String), // Primary, Secondary
    pub roster: Roster,
    pub lineup: Lineup,
    pub stats: TeamStats,
    pub ballpark_name: String,
}

impl Team {
    pub fn new(id: String, name: String, city: String, abbreviation: String) -> Self {
        Self {
            id,
            name,
            city,
            abbreviation,
            colors: ("Blue".to_string(), "White".to_string()),
            roster: Roster::new(),
            lineup: Lineup::new(),
            stats: TeamStats::new(),
            ballpark_name: "Stadium".to_string(),
        }
    }

    pub fn with_colors(mut self, primary: String, secondary: String) -> Self {
        self.colors = (primary, secondary);
        self
    }

    pub fn with_ballpark(mut self, ballpark_name: String) -> Self {
        self.ballpark_name = ballpark_name;
        self
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.city, self.name)
    }

    pub fn add_player(&mut self, player: Player) -> Result<(), String> {
        self.roster.add_player(player)
    }

    pub fn get_player(&self, player_id: &str) -> Option<&Player> {
        self.roster.get_player(player_id)
    }

    pub fn get_active_players(&self) -> Vec<&Player> {
        self.roster.get_active_players()
    }

    pub fn set_lineup(&mut self, lineup: Lineup) -> Result<(), String> {
        let active_players = self.get_active_players();
        let errors = lineup.validate_lineup(&active_players);
        
        if errors.is_empty() {
            self.lineup = lineup;
            Ok(())
        } else {
            Err(format!("Invalid lineup: {}", errors.join(", ")))
        }
    }

    pub fn is_ready_to_play(&self) -> bool {
        self.lineup.is_complete() && self.roster.is_roster_legal()
    }

    pub fn team_summary(&self) -> String {
        format!(
            "{} - {} ({})\nRecord: {}-{}-{} ({:.3})\n{}",
            self.full_name(),
            self.ballpark_name,
            self.abbreviation,
            self.stats.wins,
            self.stats.losses,
            self.stats.ties,
            self.stats.winning_percentage(),
            self.roster.roster_summary()
        )
    }
}