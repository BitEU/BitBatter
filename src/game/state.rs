use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InningHalf {
    Top,
    Bottom,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PitchState {
    ChoosePitch,
    Aiming { pitch_type: usize },
    Pitching { frames_left: u8 },
    WaitingForBatter,
    Swinging { frames_left: u8 },
    BallInPlay { frames_left: u8 },
    ShowResult { result: PlayResult, frames_left: u8 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayResult {
    Strike,
    Ball,
    Foul,
    Hit(HitType),
    Out(OutType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum HitType {
    Single,
    Double,
    Triple,
    HomeRun,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutType {
    Strikeout,
    Groundout,
    Flyout,
    LineOut,
}

#[derive(Debug, Clone, Copy)]
pub enum PitchLocation {
    UpInside,
    Up,
    UpOutside,
    Inside,
    Middle,
    Outside,
    DownInside,
    Down,
    DownOutside,
}

impl PitchLocation {
    pub fn from_direction(up: bool, down: bool, left: bool, right: bool) -> Self {
        match (up, down, left, right) {
            (true, false, true, false) => PitchLocation::UpInside,
            (true, false, false, false) => PitchLocation::Up,
            (true, false, false, true) => PitchLocation::UpOutside,
            (false, false, true, false) => PitchLocation::Inside,
            (false, false, false, false) => PitchLocation::Middle,
            (false, false, false, true) => PitchLocation::Outside,
            (false, true, true, false) => PitchLocation::DownInside,
            (false, true, false, false) => PitchLocation::Down,
            (false, true, false, true) => PitchLocation::DownOutside,
            _ => PitchLocation::Middle, // Invalid combo defaults to middle
        }
    }

    pub fn is_strike(&self) -> bool {
        !matches!(self, PitchLocation::UpInside | PitchLocation::UpOutside | 
                       PitchLocation::DownInside | PitchLocation::DownOutside)
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub inning: u8,
    pub half: InningHalf,
    pub outs: u8,
    pub balls: u8,
    pub strikes: u8,
    pub home_score: u8,
    pub away_score: u8,
    pub bases: [bool; 3], // 1st, 2nd, 3rd
    pub current_batter_idx: usize,
    pub pitch_state: PitchState,
    pub pitch_location: Option<PitchLocation>,
    pub swing_location: Option<PitchLocation>,
    pub message: String,
    pub game_over: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            inning: 1,
            half: InningHalf::Top,
            outs: 0,
            balls: 0,
            strikes: 0,
            home_score: 0,
            away_score: 0,
            bases: [false, false, false],
            current_batter_idx: 0,
            pitch_state: PitchState::ChoosePitch,
            pitch_location: None,
            swing_location: None,
            message: "Choose your pitch!".to_string(),
            game_over: false,
        }
    }

    pub fn batting_team(&self) -> &str {
        match self.half {
            InningHalf::Top => "Away",
            InningHalf::Bottom => "Home",
        }
    }

    pub fn advance_batter(&mut self) {
        self.current_batter_idx = (self.current_batter_idx + 1) % 9;
        self.balls = 0;
        self.strikes = 0;
        self.pitch_state = PitchState::ChoosePitch;
        self.pitch_location = None;
        self.swing_location = None;
    }

    pub fn add_out(&mut self) {
        self.outs += 1;
        if self.outs >= 3 {
            self.end_half_inning();
        } else {
            self.advance_batter();
        }
    }

    pub fn end_half_inning(&mut self) {
        match self.half {
            InningHalf::Top => {
                self.half = InningHalf::Bottom;
            }
            InningHalf::Bottom => {
                if self.inning >= 9 && self.home_score != self.away_score {
                    self.game_over = true;
                    self.message = format!(
                        "Game Over! Final Score - Home: {} Away: {}",
                        self.home_score, self.away_score
                    );
                } else {
                    self.inning += 1;
                    self.half = InningHalf::Top;
                }
            }
        }
        self.outs = 0;
        self.bases = [false, false, false];
        self.advance_batter();
    }

    pub fn add_walk(&mut self) {
        self.message = "Ball 4! Walk!".to_string();
        self.advance_runners(0); // 0 = walk
        self.advance_batter();
    }

    pub fn add_strikeout(&mut self) {
        self.message = "Strike 3! You're out!".to_string();
        self.add_out();
    }

    pub fn advance_runners(&mut self, bases_to_advance: u8) {
        let mut runners_scored = 0;

        // Move runners backwards to avoid overwriting
        if self.bases[2] {
            // Runner on 3rd
            if bases_to_advance > 0 {
                runners_scored += 1;
                self.bases[2] = false;
            }
        }
        if self.bases[1] {
            // Runner on 2nd
            if bases_to_advance >= 2 {
                runners_scored += 1;
                self.bases[1] = false;
            } else if bases_to_advance == 1 {
                self.bases[2] = true;
                self.bases[1] = false;
            }
        }
        if self.bases[0] {
            // Runner on 1st
            match bases_to_advance {
                0 => {
                    // Walk - force advance
                    if self.bases[1] {
                        if self.bases[2] {
                            runners_scored += 1;
                        } else {
                            self.bases[2] = true;
                        }
                    }
                    self.bases[1] = true;
                }
                1 => {
                    if !self.bases[1] {
                        self.bases[1] = true;
                        self.bases[0] = false;
                    }
                }
                2 => {
                    self.bases[2] = true;
                    self.bases[0] = false;
                }
                3 | 4 => {
                    // Triple or HR
                    runners_scored += 1;
                    self.bases[0] = false;
                }
                _ => {}
            }
        }

        // Add batter to base
        match bases_to_advance {
            0 => self.bases[0] = true, // Walk
            1 => self.bases[0] = true, // Single
            2 => self.bases[1] = true, // Double
            3 => self.bases[2] = true, // Triple
            4 => runners_scored += 1,  // Home run
            _ => {}
        }

        // Update score
        match self.half {
            InningHalf::Top => self.away_score += runners_scored,
            InningHalf::Bottom => self.home_score += runners_scored,
        }
    }
}
