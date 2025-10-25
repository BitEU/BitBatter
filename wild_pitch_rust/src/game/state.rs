use crate::teams::Team;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Base {
    First,
    Second, 
    Third,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseRunners {
    pub first: Option<String>,  // Player ID
    pub second: Option<String>, // Player ID
    pub third: Option<String>,  // Player ID
}

impl BaseRunners {
    pub fn new() -> Self {
        Self {
            first: None,
            second: None,
            third: None,
        }
    }

    pub fn clear(&mut self) {
        self.first = None;
        self.second = None;
        self.third = None;
    }

    pub fn get_runner(&self, base: Base) -> Option<&String> {
        match base {
            Base::First => self.first.as_ref(),
            Base::Second => self.second.as_ref(),
            Base::Third => self.third.as_ref(),
        }
    }

    pub fn set_runner(&mut self, base: Base, player_id: Option<String>) {
        match base {
            Base::First => self.first = player_id,
            Base::Second => self.second = player_id,
            Base::Third => self.third = player_id,
        }
    }

    pub fn advance_runner(&mut self, from_base: Base, to_base: Option<Base>) -> Option<String> {
        let runner = match from_base {
            Base::First => self.first.take(),
            Base::Second => self.second.take(),
            Base::Third => self.third.take(),
        };

        if let Some(to) = to_base {
            self.set_runner(to, runner.clone());
        }

        runner
    }

    pub fn runners_on_base(&self) -> Vec<Base> {
        let mut runners = Vec::new();
        if self.first.is_some() { runners.push(Base::First); }
        if self.second.is_some() { runners.push(Base::Second); }
        if self.third.is_some() { runners.push(Base::Third); }
        runners
    }

    pub fn count_runners(&self) -> u8 {
        self.runners_on_base().len() as u8
    }

    pub fn is_bases_loaded(&self) -> bool {
        self.first.is_some() && self.second.is_some() && self.third.is_some()
    }

    pub fn is_scoring_position(&self) -> bool {
        self.second.is_some() || self.third.is_some()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum GamePhase {
    PreGame,
    Playing,
    PitchingChange,
    ManagerDecision,
    GameOver,
    Paused,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum InningHalf {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Count {
    pub balls: u8,
    pub strikes: u8,
}

impl Count {
    pub fn new() -> Self {
        Self { balls: 0, strikes: 0 }
    }

    pub fn is_full_count(&self) -> bool {
        self.balls == 3 && self.strikes == 2
    }

    pub fn add_ball(&mut self) -> bool {
        self.balls += 1;
        self.balls >= 4
    }

    pub fn add_strike(&mut self) -> bool {
        self.strikes += 1;
        self.strikes >= 3
    }

    pub fn reset(&mut self) {
        self.balls = 0;
        self.strikes = 0;
    }

    pub fn display(&self) -> String {
        format!("{}-{}", self.balls, self.strikes)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSituation {
    pub inning: u8,
    pub inning_half: InningHalf,
    pub outs: u8,
    pub count: Count,
    pub runners: BaseRunners,
    pub current_batter_id: String,
    pub current_pitcher_id: String,
    pub batter_number: u8, // 1-9 in batting order
}

impl GameSituation {
    pub fn new() -> Self {
        Self {
            inning: 1,
            inning_half: InningHalf::Top,
            outs: 0,
            count: Count::new(),
            runners: BaseRunners::new(),
            current_batter_id: String::new(),
            current_pitcher_id: String::new(),
            batter_number: 1,
        }
    }

    pub fn is_inning_over(&self) -> bool {
        self.outs >= 3
    }

    pub fn advance_inning(&mut self) {
        match self.inning_half {
            InningHalf::Top => {
                self.inning_half = InningHalf::Bottom;
            },
            InningHalf::Bottom => {
                self.inning += 1;
                self.inning_half = InningHalf::Top;
            },
        }
        self.outs = 0;
        self.runners.clear();
        self.count.reset();
    }

    pub fn advance_batter(&mut self) {
        self.batter_number += 1;
        if self.batter_number > 9 {
            self.batter_number = 1;
        }
        self.count.reset();
    }

    pub fn add_out(&mut self) {
        self.outs += 1;
        self.count.reset();
    }

    pub fn is_top_inning(&self) -> bool {
        matches!(self.inning_half, InningHalf::Top)
    }

    pub fn is_bottom_inning(&self) -> bool {
        matches!(self.inning_half, InningHalf::Bottom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub visitor: u32,
    pub home: u32,
    pub visitor_runs_by_inning: Vec<u32>,
    pub home_runs_by_inning: Vec<u32>,
}

impl Score {
    pub fn new() -> Self {
        Self {
            visitor: 0,
            home: 0,
            visitor_runs_by_inning: vec![0; 9],
            home_runs_by_inning: vec![0; 9],
        }
    }

    pub fn add_run(&mut self, is_home_team: bool, inning: u8) {
        let inning_index = (inning - 1) as usize;
        
        if is_home_team {
            self.home += 1;
            if inning_index < self.home_runs_by_inning.len() {
                self.home_runs_by_inning[inning_index] += 1;
            } else {
                // Extra innings
                while self.home_runs_by_inning.len() <= inning_index {
                    self.home_runs_by_inning.push(0);
                    self.visitor_runs_by_inning.push(0);
                }
                self.home_runs_by_inning[inning_index] += 1;
            }
        } else {
            self.visitor += 1;
            if inning_index < self.visitor_runs_by_inning.len() {
                self.visitor_runs_by_inning[inning_index] += 1;
            } else {
                // Extra innings
                while self.visitor_runs_by_inning.len() <= inning_index {
                    self.visitor_runs_by_inning.push(0);
                    self.home_runs_by_inning.push(0);
                }
                self.visitor_runs_by_inning[inning_index] += 1;
            }
        }
    }

    pub fn get_winning_team(&self) -> Option<bool> {
        if self.visitor > self.home {
            Some(false) // Visitor wins
        } else if self.home > self.visitor {
            Some(true)  // Home wins
        } else {
            None // Tie
        }
    }

    pub fn get_margin(&self) -> u32 {
        if self.visitor > self.home {
            self.visitor - self.home
        } else {
            self.home - self.visitor
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub game_id: String,
    pub visitor_team: Team,
    pub home_team: Team,
    pub situation: GameSituation,
    pub score: Score,
    pub phase: GamePhase,
    pub play_by_play: Vec<String>,
    pub inning_summary: Vec<String>,
    pub game_start_time: String,
    pub weather: String,
    pub attendance: u32,
}

impl GameState {
    pub fn new(game_id: String, visitor_team: Team, home_team: Team) -> Self {
        Self {
            game_id,
            visitor_team,
            home_team,
            situation: GameSituation::new(),
            score: Score::new(),
            phase: GamePhase::PreGame,
            play_by_play: Vec::new(),
            inning_summary: Vec::new(),
            game_start_time: chrono::Utc::now().format("%H:%M").to_string(),
            weather: "Clear, 72°F".to_string(),
            attendance: 35000,
        }
    }

    pub fn current_batting_team(&self) -> &Team {
        if self.situation.is_top_inning() {
            &self.visitor_team
        } else {
            &self.home_team
        }
    }

    pub fn current_pitching_team(&self) -> &Team {
        if self.situation.is_top_inning() {
            &self.home_team
        } else {
            &self.visitor_team
        }
    }

    pub fn is_game_over(&self) -> bool {
        match self.phase {
            GamePhase::GameOver => true,
            _ => {
                // Game ends after 9 innings if home team is ahead
                // or after visitor bats in extra innings if they take the lead
                if self.situation.inning >= 9 {
                    if self.situation.is_bottom_inning() && self.score.home > self.score.visitor {
                        // Home team wins, no need to finish the inning
                        true
                    } else if self.situation.inning > 9 && self.situation.is_top_inning() && self.score.visitor > self.score.home {
                        // Visitor takes lead in extra innings
                        false // Let home team bat
                    } else if self.situation.inning > 9 && self.situation.is_bottom_inning() && self.score.home >= self.score.visitor {
                        // Home team ties or takes lead in extra innings
                        true
                    } else if self.situation.inning == 9 && self.situation.is_top_inning() && self.situation.outs >= 3 {
                        // End of 9th, check if home needs to bat
                        self.score.home > self.score.visitor
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    pub fn add_play(&mut self, description: String) {
        let inning_text = if self.situation.is_top_inning() { "T" } else { "B" };
        let play_text = format!("{}{}: {}", inning_text, self.situation.inning, description);
        self.play_by_play.push(play_text);
    }

    pub fn start_game(&mut self) {
        self.phase = GamePhase::Playing;
        self.add_play("GAME STARTED".to_string());
        
        // Set initial batter and pitcher
        if let Some(lineup_spot) = self.visitor_team.lineup.get_batter_by_order(1) {
            self.situation.current_batter_id = lineup_spot.player_id.clone();
        }
        self.situation.current_pitcher_id = self.home_team.lineup.starting_pitcher_id.clone();
    }

    pub fn advance_to_next_batter(&mut self) {
        self.situation.advance_batter();
        
        let current_team = self.current_batting_team();
        if let Some(lineup_spot) = current_team.lineup.get_batter_by_order(self.situation.batter_number) {
            self.situation.current_batter_id = lineup_spot.player_id.clone();
        }
    }

    pub fn end_inning(&mut self) {
        let inning_text = if self.situation.is_top_inning() {
            format!("END TOP {}", self.situation.inning)
        } else {
            format!("END BOTTOM {}", self.situation.inning)
        };
        self.add_play(inning_text);
        
        self.situation.advance_inning();
        self.situation.batter_number = 1;
        
        // Update current batter for new inning
        let current_team = self.current_batting_team();
        if let Some(lineup_spot) = current_team.lineup.get_batter_by_order(1) {
            self.situation.current_batter_id = lineup_spot.player_id.clone();
        }
        
        // Update current pitcher (simplified - in reality this would involve more complex logic)
        let pitching_team = self.current_pitching_team();
        self.situation.current_pitcher_id = pitching_team.lineup.starting_pitcher_id.clone();
    }

    pub fn check_game_end(&mut self) {
        if self.is_game_over() {
            self.phase = GamePhase::GameOver;
            let winner = if self.score.home > self.score.visitor {
                format!("{} wins", self.home_team.full_name())
            } else if self.score.visitor > self.score.home {
                format!("{} wins", self.visitor_team.full_name())
            } else {
                "Game tied".to_string()
            };
            self.add_play(format!("FINAL: {}", winner));
        }
    }

    // Convenience getters matching the original UI code
    pub fn is_top_inning(&self) -> bool {
        self.situation.is_top_inning()
    }

    pub fn inning(&self) -> u8 {
        self.situation.inning
    }

    pub fn outs(&self) -> u8 {
        self.situation.outs
    }

    pub fn visitor_score(&self) -> u32 {
        self.score.visitor
    }

    pub fn home_score(&self) -> u32 {
        self.score.home
    }
}