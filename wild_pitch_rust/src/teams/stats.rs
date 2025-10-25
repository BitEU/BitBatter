use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamBattingStats {
    pub games_played: u32,
    pub at_bats: u32,
    pub runs: u32,
    pub hits: u32,
    pub doubles: u32,
    pub triples: u32,
    pub home_runs: u32,
    pub runs_batted_in: u32,
    pub walks: u32,
    pub strikeouts: u32,
    pub stolen_bases: u32,
    pub caught_stealing: u32,
    pub sacrifice_hits: u32,
    pub sacrifice_flies: u32,
    pub hit_by_pitch: u32,
    pub left_on_base: u32,
    pub double_plays_grounded_into: u32,
}

impl TeamBattingStats {
    pub fn new() -> Self {
        Self {
            games_played: 0,
            at_bats: 0,
            runs: 0,
            hits: 0,
            doubles: 0,
            triples: 0,
            home_runs: 0,
            runs_batted_in: 0,
            walks: 0,
            strikeouts: 0,
            stolen_bases: 0,
            caught_stealing: 0,
            sacrifice_hits: 0,
            sacrifice_flies: 0,
            hit_by_pitch: 0,
            left_on_base: 0,
            double_plays_grounded_into: 0,
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
            let singles = self.hits - self.doubles - self.triples - self.home_runs;
            let total_bases = singles + (self.doubles * 2) + (self.triples * 3) + (self.home_runs * 4);
            total_bases as f64 / self.at_bats as f64
        }
    }

    pub fn ops(&self) -> f64 {
        self.on_base_percentage() + self.slugging_percentage()
    }

    pub fn runs_per_game(&self) -> f64 {
        if self.games_played == 0 {
            0.0
        } else {
            self.runs as f64 / self.games_played as f64
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamPitchingStats {
    pub games_played: u32,
    pub games_started: u32,
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

impl TeamPitchingStats {
    pub fn new() -> Self {
        Self {
            games_played: 0,
            games_started: 0,
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

    pub fn runs_allowed_per_game(&self) -> f64 {
        if self.games_played == 0 {
            0.0
        } else {
            self.runs_allowed as f64 / self.games_played as f64
        }
    }

    pub fn winning_percentage(&self) -> f64 {
        let total_games = self.wins + self.losses;
        if total_games == 0 {
            0.0
        } else {
            self.wins as f64 / total_games as f64
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamFieldingStats {
    pub games_played: u32,
    pub putouts: u32,
    pub assists: u32,
    pub errors: u32,
    pub double_plays: u32,
    pub triple_plays: u32,
    pub passed_balls: u32,
    pub stolen_bases_allowed: u32,
    pub caught_stealing: u32,
}

impl TeamFieldingStats {
    pub fn new() -> Self {
        Self {
            games_played: 0,
            putouts: 0,
            assists: 0,
            errors: 0,
            double_plays: 0,
            triple_plays: 0,
            passed_balls: 0,
            stolen_bases_allowed: 0,
            caught_stealing: 0,
        }
    }

    pub fn fielding_percentage(&self) -> f64 {
        let total_chances = self.putouts + self.assists + self.errors;
        if total_chances == 0 {
            1.0
        } else {
            (self.putouts + self.assists) as f64 / total_chances as f64
        }
    }

    pub fn errors_per_game(&self) -> f64 {
        if self.games_played == 0 {
            0.0
        } else {
            self.errors as f64 / self.games_played as f64
        }
    }

    pub fn double_plays_per_game(&self) -> f64 {
        if self.games_played == 0 {
            0.0
        } else {
            self.double_plays as f64 / self.games_played as f64
        }
    }

    pub fn caught_stealing_percentage(&self) -> f64 {
        let attempts = self.stolen_bases_allowed + self.caught_stealing;
        if attempts == 0 {
            0.0
        } else {
            self.caught_stealing as f64 / attempts as f64
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStats {
    pub batting: TeamBattingStats,
    pub pitching: TeamPitchingStats,
    pub fielding: TeamFieldingStats,
    pub wins: u32,
    pub losses: u32,
    pub ties: u32,
}

impl TeamStats {
    pub fn new() -> Self {
        Self {
            batting: TeamBattingStats::new(),
            pitching: TeamPitchingStats::new(),
            fielding: TeamFieldingStats::new(),
            wins: 0,
            losses: 0,
            ties: 0,
        }
    }

    pub fn winning_percentage(&self) -> f64 {
        let total_games = self.wins + self.losses + self.ties;
        if total_games == 0 {
            0.0
        } else {
            (self.wins as f64 + (self.ties as f64 * 0.5)) / total_games as f64
        }
    }

    pub fn games_played(&self) -> u32 {
        self.wins + self.losses + self.ties
    }

    pub fn games_behind(&self, leader_wins: u32, leader_losses: u32) -> f64 {
        let leader_percentage = leader_wins as f64 / (leader_wins + leader_losses) as f64;
        let our_percentage = self.winning_percentage();
        let games_played_diff = (leader_wins + leader_losses) as f64 - (self.wins + self.losses) as f64;
        
        if our_percentage >= leader_percentage {
            0.0
        } else {
            let percentage_diff = leader_percentage - our_percentage;
            (percentage_diff * (self.wins + self.losses) as f64) + (games_played_diff * 0.5)
        }
    }

    pub fn run_differential(&self) -> i32 {
        self.batting.runs as i32 - self.pitching.runs_allowed as i32
    }

    pub fn pythagorean_winning_percentage(&self) -> f64 {
        let runs_scored = self.batting.runs as f64;
        let runs_allowed = self.pitching.runs_allowed as f64;
        
        if runs_scored == 0.0 && runs_allowed == 0.0 {
            0.5
        } else {
            let runs_scored_squared = runs_scored.powi(2);
            let runs_allowed_squared = runs_allowed.powi(2);
            runs_scored_squared / (runs_scored_squared + runs_allowed_squared)
        }
    }
}