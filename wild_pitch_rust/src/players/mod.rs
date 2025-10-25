pub mod batter;
pub mod pitcher;
pub mod fielder;

pub use batter::*;
pub use pitcher::*;
pub use fielder::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub jersey_number: u8,
    pub batter: Option<Batter>,
    pub pitcher: Option<Pitcher>,
    pub fielder: Fielder,
    pub age: u8,
    pub height: String,
    pub weight: u16,
    pub throws: Handedness,
    pub bats: Handedness,
    pub salary: u32,
    pub years_pro: u8,
}

impl Player {
    pub fn new(
        id: String,
        name: String,
        jersey_number: u8,
        primary_position: Position,
        throws: Handedness,
        bats: Handedness,
    ) -> Self {
        let fielder = Fielder::new(id.clone(), name.clone(), jersey_number, primary_position);
        
        // Create batter for all non-pitcher positions
        let batter = if primary_position != Position::Pitcher {
            Some(Batter::new(id.clone(), name.clone(), jersey_number))
        } else {
            None
        };

        // Create pitcher only for pitcher position
        let pitcher = if primary_position == Position::Pitcher {
            Some(Pitcher::new(id.clone(), name.clone(), jersey_number, throws))
        } else {
            None
        };

        Self {
            id,
            name,
            jersey_number,
            batter,
            pitcher,
            fielder,
            age: 25,
            height: "6'0\"".to_string(),
            weight: 180,
            throws,
            bats,
            salary: 500000,
            years_pro: 1,
        }
    }

    pub fn position_player(
        id: String,
        name: String,
        jersey_number: u8,
        primary_position: Position,
        throws: Handedness,
        bats: Handedness,
    ) -> Self {
        assert_ne!(primary_position, Position::Pitcher);
        Self::new(id, name, jersey_number, primary_position, throws, bats)
    }

    pub fn pitcher(
        id: String,
        name: String,
        jersey_number: u8,
        throws: Handedness,
        role: PitcherRole,
    ) -> Self {
        let mut player = Self::new(id, name, jersey_number, Position::Pitcher, throws, throws);
        if let Some(ref mut pitcher) = player.pitcher {
            pitcher.role = role;
        }
        player
    }

    pub fn is_pitcher(&self) -> bool {
        self.pitcher.is_some()
    }

    pub fn is_position_player(&self) -> bool {
        self.batter.is_some()
    }

    pub fn can_pitch(&self) -> bool {
        self.pitcher.is_some()
    }

    pub fn can_bat(&self) -> bool {
        self.batter.is_some()
    }

    pub fn primary_position(&self) -> Position {
        self.fielder.primary_position
    }

    pub fn can_play_position(&self, position: Position) -> bool {
        self.fielder.can_play_position(position)
    }

    pub fn defensive_rating_at_position(&self, position: Position) -> f64 {
        self.fielder.defensive_rating(position)
    }

    pub fn batting_average(&self) -> f64 {
        self.batter.as_ref().map_or(0.0, |b| b.stats.batting_average())
    }

    pub fn era(&self) -> f64 {
        self.pitcher.as_ref().map_or(0.0, |p| p.stats.era())
    }

    pub fn is_injured(&self) -> bool {
        self.fielder.is_injured || 
        self.batter.as_ref().map_or(false, |b| b.is_injured) ||
        self.pitcher.as_ref().map_or(false, |p| p.is_injured)
    }

    pub fn overall_fatigue(&self) -> f64 {
        let fielding_fatigue = self.fielder.fatigue_level;
        let batting_fatigue = self.batter.as_ref().map_or(1.0, |b| b.fatigue_level);
        let pitching_fatigue = self.pitcher.as_ref().map_or(1.0, |p| p.fatigue_level);
        
        (fielding_fatigue + batting_fatigue + pitching_fatigue) / 3.0
    }

    pub fn display_name(&self) -> String {
        format!("{} ({})", self.name, self.primary_position().abbreviation())
    }
}