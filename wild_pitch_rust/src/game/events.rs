use crate::players::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayResult {
    Ball,
    Strike,
    FoulBall,
    Hit(HitType),
    Walk,
    Strikeout,
    HitByPitch,
    SacrificeHit,
    SacrificeFly,
    FieldersChoice,
    Error(Position),
    DoublePlay,
    TriplePlay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HitType {
    Single(Option<Position>),   // Single, optional fielder who made the play
    Double(Option<Position>),   // Double, optional fielder
    Triple(Option<Position>),   // Triple, optional fielder
    HomeRun,                    // Home run
    GroundOut(Position),        // Ground out to specific position
    FlyOut(Position),           // Fly out to specific position
    LineOut(Position),          // Line out to specific position
    PopOut(Position),           // Pop out to specific position
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BaseRunningEvent {
    StolenBase { runner_id: String, base: crate::game::state::Base },
    CaughtStealing { runner_id: String, base: crate::game::state::Base },
    WildPitch,
    PassedBall,
    Balk,
    RunnerAdvances { runner_id: String, from: crate::game::state::Base, to: Option<crate::game::state::Base> },
    RunnerScores { runner_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManagerAction {
    PinchHit { old_player_id: String, new_player_id: String },
    PinchRun { old_player_id: String, new_player_id: String },
    DefensiveSubstitution { old_player_id: String, new_player_id: String, position: Position },
    PitchingChange { old_pitcher_id: String, new_pitcher_id: String },
    StealsAttempt { runner_id: String, base: crate::game::state::Base },
    HitAndRun { runner_id: String },
    Bunt,
    IntentionalWalk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub inning: u8,
    pub inning_half: crate::game::state::InningHalf,
    pub outs: u8,
    pub batter_id: String,
    pub pitcher_id: String,
    pub result: PlayResult,
    pub base_running: Vec<BaseRunningEvent>,
    pub manager_actions: Vec<ManagerAction>,
    pub description: String,
    pub runs_scored: u8,
}

impl GameEvent {
    pub fn new(
        inning: u8,
        inning_half: crate::game::state::InningHalf,
        outs: u8,
        batter_id: String,
        pitcher_id: String,
        result: PlayResult,
    ) -> Self {
        Self {
            inning,
            inning_half,
            outs,
            batter_id,
            pitcher_id,
            result,
            base_running: Vec::new(),
            manager_actions: Vec::new(),
            description: String::new(),
            runs_scored: 0,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_base_running(mut self, events: Vec<BaseRunningEvent>) -> Self {
        self.base_running = events;
        self
    }

    pub fn with_runs_scored(mut self, runs: u8) -> Self {
        self.runs_scored = runs;
        self
    }

    pub fn add_manager_action(&mut self, action: ManagerAction) {
        self.manager_actions.push(action);
    }

    pub fn is_scoring_play(&self) -> bool {
        self.runs_scored > 0
    }

    pub fn is_out(&self) -> bool {
        matches!(self.result, 
            PlayResult::Strikeout |
            PlayResult::Hit(HitType::GroundOut(_)) |
            PlayResult::Hit(HitType::FlyOut(_)) |
            PlayResult::Hit(HitType::LineOut(_)) |
            PlayResult::Hit(HitType::PopOut(_)) |
            PlayResult::FieldersChoice |
            PlayResult::DoublePlay |
            PlayResult::TriplePlay
        )
    }

    pub fn is_hit(&self) -> bool {
        matches!(self.result,
            PlayResult::Hit(HitType::Single(_)) |
            PlayResult::Hit(HitType::Double(_)) |
            PlayResult::Hit(HitType::Triple(_)) |
            PlayResult::Hit(HitType::HomeRun)
        )
    }

    pub fn outs_recorded(&self) -> u8 {
        match &self.result {
            PlayResult::DoublePlay => 2,
            PlayResult::TriplePlay => 3,
            _ if self.is_out() => 1,
            _ => 0,
        }
    }

    pub fn format_play_description(&self, batter_name: &str) -> String {
        if !self.description.is_empty() {
            return self.description.clone();
        }

        let base_result = match &self.result {
            PlayResult::Ball => "Ball".to_string(),
            PlayResult::Strike => "Strike".to_string(),
            PlayResult::FoulBall => "Foul ball".to_string(),
            PlayResult::Walk => format!("{} walks", batter_name),
            PlayResult::Strikeout => format!("{} strikes out", batter_name),
            PlayResult::HitByPitch => format!("{} hit by pitch", batter_name),
            PlayResult::SacrificeHit => format!("{} sacrifice hit", batter_name),
            PlayResult::SacrificeFly => format!("{} sacrifice fly", batter_name),
            PlayResult::FieldersChoice => format!("{} fielder's choice", batter_name),
            PlayResult::Error(pos) => format!("{} reaches on error by {}", batter_name, pos.abbreviation()),
            PlayResult::DoublePlay => format!("{} grounds into double play", batter_name),
            PlayResult::TriplePlay => format!("{} hits into triple play", batter_name),
            PlayResult::Hit(hit_type) => {
                match hit_type {
                    HitType::Single(pos) => {
                        if let Some(fielder) = pos {
                            format!("{} singles to {}", batter_name, fielder.abbreviation())
                        } else {
                            format!("{} singles", batter_name)
                        }
                    },
                    HitType::Double(pos) => {
                        if let Some(fielder) = pos {
                            format!("{} doubles to {}", batter_name, fielder.abbreviation())
                        } else {
                            format!("{} doubles", batter_name)
                        }
                    },
                    HitType::Triple(pos) => {
                        if let Some(fielder) = pos {
                            format!("{} triples to {}", batter_name, fielder.abbreviation())
                        } else {
                            format!("{} triples", batter_name)
                        }
                    },
                    HitType::HomeRun => format!("{} home run!", batter_name),
                    HitType::GroundOut(pos) => format!("{} grounds out to {}", batter_name, pos.abbreviation()),
                    HitType::FlyOut(pos) => format!("{} flies out to {}", batter_name, pos.abbreviation()),
                    HitType::LineOut(pos) => format!("{} lines out to {}", batter_name, pos.abbreviation()),
                    HitType::PopOut(pos) => format!("{} pops out to {}", batter_name, pos.abbreviation()),
                }
            },
        };

        if self.runs_scored > 0 {
            format!("{}. {} run{} score{}.", 
                base_result, 
                self.runs_scored,
                if self.runs_scored == 1 { "" } else { "s" },
                if self.runs_scored == 1 { "s" } else { "" }
            )
        } else {
            base_result
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InningEvents {
    pub inning: u8,
    pub inning_half: crate::game::state::InningHalf,
    pub events: Vec<GameEvent>,
    pub runs_scored: u8,
    pub hits: u8,
    pub errors: u8,
    pub left_on_base: u8,
}

impl InningEvents {
    pub fn new(inning: u8, inning_half: crate::game::state::InningHalf) -> Self {
        Self {
            inning,
            inning_half,
            events: Vec::new(),
            runs_scored: 0,
            hits: 0,
            errors: 0,
            left_on_base: 0,
        }
    }

    pub fn add_event(&mut self, event: GameEvent) {
        self.runs_scored += event.runs_scored;
        
        if event.is_hit() {
            self.hits += 1;
        }
        
        if matches!(event.result, PlayResult::Error(_)) {
            self.errors += 1;
        }
        
        self.events.push(event);
    }

    pub fn total_outs(&self) -> u8 {
        self.events.iter().map(|e| e.outs_recorded()).sum()
    }

    pub fn is_complete(&self) -> bool {
        self.total_outs() >= 3
    }
}