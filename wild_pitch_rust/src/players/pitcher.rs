use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PitcherStats {
    pub games_started: u32,
    pub games_finished: u32,
    pub complete_games: u32,
    pub shutouts: u32,
    pub saves: u32,
    pub wins: u32,
    pub losses: u32,
    pub innings_pitched: f64,
    pub hits_allowed: u32,
    pub runs_allowed: u32,
    pub earned_runs: u32,
    pub home_runs_allowed: u32,
    pub walks_issued: u32,
    pub strikeouts: u32,
    pub hit_batsmen: u32,
    pub wild_pitches: u32,
    pub balks: u32,
}

impl PitcherStats {
    pub fn new() -> Self {
        Self {
            games_started: 0,
            games_finished: 0,
            complete_games: 0,
            shutouts: 0,
            saves: 0,
            wins: 0,
            losses: 0,
            innings_pitched: 0.0,
            hits_allowed: 0,
            runs_allowed: 0,
            earned_runs: 0,
            home_runs_allowed: 0,
            walks_issued: 0,
            strikeouts: 0,
            hit_batsmen: 0,
            wild_pitches: 0,
            balks: 0,
        }
    }

    pub fn era(&self) -> f64 {
        if self.innings_pitched == 0.0 {
            0.0
        } else {
            (self.earned_runs as f64 * 9.0) / self.innings_pitched
        }
    }

    pub fn whip(&self) -> f64 {
        if self.innings_pitched == 0.0 {
            0.0
        } else {
            (self.walks_issued + self.hits_allowed) as f64 / self.innings_pitched
        }
    }

    pub fn strikeout_rate(&self) -> f64 {
        if self.innings_pitched == 0.0 {
            0.0
        } else {
            (self.strikeouts as f64 * 9.0) / self.innings_pitched
        }
    }

    pub fn walk_rate(&self) -> f64 {
        if self.innings_pitched == 0.0 {
            0.0
        } else {
            (self.walks_issued as f64 * 9.0) / self.innings_pitched
        }
    }

    pub fn strikeout_to_walk_ratio(&self) -> f64 {
        if self.walks_issued == 0 {
            if self.strikeouts == 0 {
                0.0
            } else {
                f64::INFINITY
            }
        } else {
            self.strikeouts as f64 / self.walks_issued as f64
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PitcherRole {
    Starter,
    Reliever,
    Closer,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Handedness {
    Left,
    Right,
    Switch, // For batters who can switch hit
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PitcherTendencies {
    // Pitching abilities (0.0 to 1.0)
    pub control_rating: f64,        // Strike throwing ability
    pub velocity_rating: f64,       // Fastball speed
    pub movement_rating: f64,       // Breaking ball quality
    pub stamina_rating: f64,        // Endurance
    pub composure_rating: f64,      // Performance under pressure
    
    // Pitch repertoire (0.0 to 1.0, should sum to 1.0)
    pub fastball_frequency: f64,
    pub curveball_frequency: f64,
    pub slider_frequency: f64,
    pub changeup_frequency: f64,
    pub other_frequency: f64,
    
    // Situational tendencies
    pub vs_lefty_modifier: f64,     // Performance vs left-handed batters
    pub vs_righty_modifier: f64,    // Performance vs right-handed batters
    pub with_runners_modifier: f64, // Performance with runners on base
}

impl Default for PitcherTendencies {
    fn default() -> Self {
        Self {
            control_rating: 0.6,
            velocity_rating: 0.6,
            movement_rating: 0.5,
            stamina_rating: 0.7,
            composure_rating: 0.5,
            fastball_frequency: 0.5,
            curveball_frequency: 0.2,
            slider_frequency: 0.15,
            changeup_frequency: 0.1,
            other_frequency: 0.05,
            vs_lefty_modifier: 1.0,
            vs_righty_modifier: 1.0,
            with_runners_modifier: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pitcher {
    pub id: String,
    pub name: String,
    pub jersey_number: u8,
    pub handedness: Handedness,
    pub role: PitcherRole,
    pub stats: PitcherStats,
    pub tendencies: PitcherTendencies,
    pub is_injured: bool,
    pub fatigue_level: f64,     // 0.0 = exhausted, 1.0 = fresh
    pub pitches_thrown: u32,    // Current game pitch count
    pub max_pitches: u32,       // Typical pitch count limit
}

impl Pitcher {
    pub fn new(id: String, name: String, jersey_number: u8, handedness: Handedness) -> Self {
        Self {
            id,
            name,
            jersey_number,
            handedness,
            role: PitcherRole::Starter,
            stats: PitcherStats::new(),
            tendencies: PitcherTendencies::default(),
            is_injured: false,
            fatigue_level: 1.0,
            pitches_thrown: 0,
            max_pitches: 100,
        }
    }

    pub fn with_role(mut self, role: PitcherRole) -> Self {
        self.role = role;
        // Adjust max pitches based on role
        self.max_pitches = match role {
            PitcherRole::Starter => 110,
            PitcherRole::Reliever => 40,
            PitcherRole::Closer => 30,
        };
        self
    }

    pub fn with_tendencies(mut self, tendencies: PitcherTendencies) -> Self {
        self.tendencies = tendencies;
        self
    }

    pub fn is_starter(&self) -> bool {
        matches!(self.role, PitcherRole::Starter)
    }

    pub fn is_reliever(&self) -> bool {
        matches!(self.role, PitcherRole::Reliever)
    }

    pub fn is_closer(&self) -> bool {
        matches!(self.role, PitcherRole::Closer)
    }

    pub fn fatigue_percentage(&self) -> f64 {
        if self.max_pitches == 0 {
            0.0
        } else {
            (self.pitches_thrown as f64 / self.max_pitches as f64).min(1.0)
        }
    }

    pub fn effective_control(&self) -> f64 {
        let fatigue_impact = 1.0 - (self.fatigue_percentage() * 0.3); // Max 30% reduction
        self.tendencies.control_rating * self.fatigue_level * fatigue_impact
    }

    pub fn effective_velocity(&self) -> f64 {
        let fatigue_impact = 1.0 - (self.fatigue_percentage() * 0.2); // Max 20% reduction
        self.tendencies.velocity_rating * self.fatigue_level * fatigue_impact
    }

    pub fn effective_movement(&self) -> f64 {
        let fatigue_impact = 1.0 - (self.fatigue_percentage() * 0.15); // Max 15% reduction
        self.tendencies.movement_rating * self.fatigue_level * fatigue_impact
    }

    pub fn add_pitch(&mut self) {
        self.pitches_thrown += 1;
        // Gradual fatigue increase based on pitch count
        if self.pitches_thrown > (self.max_pitches * 3 / 4) {
            self.fatigue_level = (self.fatigue_level - 0.01).max(0.3);
        }
    }

    pub fn reset_game_stats(&mut self) {
        self.pitches_thrown = 0;
        self.fatigue_level = 1.0;
    }
}