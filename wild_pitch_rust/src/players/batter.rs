use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatterStats {
    pub at_bats: u32,
    pub hits: u32,
    pub doubles: u32,
    pub triples: u32,
    pub home_runs: u32,
    pub runs_batted_in: u32,
    pub runs_scored: u32,
    pub walks: u32,
    pub strikeouts: u32,
    pub stolen_bases: u32,
    pub caught_stealing: u32,
    pub sacrifice_hits: u32,
    pub sacrifice_flies: u32,
    pub hit_by_pitch: u32,
    pub errors: u32,
}

impl BatterStats {
    pub fn new() -> Self {
        Self {
            at_bats: 0,
            hits: 0,
            doubles: 0,
            triples: 0,
            home_runs: 0,
            runs_batted_in: 0,
            runs_scored: 0,
            walks: 0,
            strikeouts: 0,
            stolen_bases: 0,
            caught_stealing: 0,
            sacrifice_hits: 0,
            sacrifice_flies: 0,
            hit_by_pitch: 0,
            errors: 0,
        }
    }

    pub fn batting_average(&self) -> f64 {
        if self.at_bats == 0 {
            0.0
        } else {
            self.hits as f64 / self.at_bats as f64
        }
    }

    pub fn on_base_percentage(&self) -> f64 {
        let total_plate_appearances = self.at_bats + self.walks + self.hit_by_pitch + self.sacrifice_flies;
        if total_plate_appearances == 0 {
            0.0
        } else {
            (self.hits + self.walks + self.hit_by_pitch) as f64 / total_plate_appearances as f64
        }
    }

    pub fn slugging_percentage(&self) -> f64 {
        if self.at_bats == 0 {
            0.0
        } else {
            let total_bases = self.hits + self.doubles + (self.triples * 2) + (self.home_runs * 3);
            total_bases as f64 / self.at_bats as f64
        }
    }

    pub fn ops(&self) -> f64 {
        self.on_base_percentage() + self.slugging_percentage()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatterTendencies {
    // Hitting tendencies (0.0 to 1.0)
    pub contact_rate: f64,      // Likelihood of making contact
    pub power_rating: f64,      // Home run potential
    pub speed_rating: f64,      // Base running speed
    pub patience_rating: f64,   // Tendency to work counts/draw walks
    pub clutch_rating: f64,     // Performance in high-pressure situations
    
    // Situational hitting preferences
    pub vs_lefty_modifier: f64,  // Performance vs left-handed pitching
    pub vs_righty_modifier: f64, // Performance vs right-handed pitching
    pub with_runners_modifier: f64, // Performance with runners on base
}

impl Default for BatterTendencies {
    fn default() -> Self {
        Self {
            contact_rate: 0.7,
            power_rating: 0.5,
            speed_rating: 0.5,
            patience_rating: 0.5,
            clutch_rating: 0.5,
            vs_lefty_modifier: 1.0,
            vs_righty_modifier: 1.0,
            with_runners_modifier: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batter {
    pub id: String,
    pub name: String,
    pub jersey_number: u8,
    pub stats: BatterStats,
    pub tendencies: BatterTendencies,
    pub is_injured: bool,
    pub fatigue_level: f64, // 0.0 = exhausted, 1.0 = fresh
}

impl Batter {
    pub fn new(id: String, name: String, jersey_number: u8) -> Self {
        Self {
            id,
            name,
            jersey_number,
            stats: BatterStats::new(),
            tendencies: BatterTendencies::default(),
            is_injured: false,
            fatigue_level: 1.0,
        }
    }

    pub fn with_tendencies(mut self, tendencies: BatterTendencies) -> Self {
        self.tendencies = tendencies;
        self
    }

    pub fn singles(&self) -> u32 {
        self.stats.hits - self.stats.doubles - self.stats.triples - self.stats.home_runs
    }

    pub fn total_bases(&self) -> u32 {
        self.singles() + (self.stats.doubles * 2) + (self.stats.triples * 3) + (self.stats.home_runs * 4)
    }

    pub fn plate_appearances(&self) -> u32 {
        self.stats.at_bats + self.stats.walks + self.stats.hit_by_pitch + self.stats.sacrifice_flies + self.stats.sacrifice_hits
    }

    pub fn effective_contact_rate(&self) -> f64 {
        self.tendencies.contact_rate * self.fatigue_level
    }

    pub fn effective_power_rating(&self) -> f64 {
        self.tendencies.power_rating * self.fatigue_level
    }
}