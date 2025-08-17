use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Position {
    Pitcher,
    Catcher,
    FirstBase,
    SecondBase,
    ThirdBase,
    Shortstop,
    LeftField,
    CenterField,
    RightField,
    DesignatedHitter,
}

impl Position {
    pub fn is_infield(&self) -> bool {
        matches!(self, Position::FirstBase | Position::SecondBase | Position::ThirdBase | Position::Shortstop)
    }

    pub fn is_outfield(&self) -> bool {
        matches!(self, Position::LeftField | Position::CenterField | Position::RightField)
    }

    pub fn abbreviation(&self) -> &'static str {
        match self {
            Position::Pitcher => "P",
            Position::Catcher => "C",
            Position::FirstBase => "1B",
            Position::SecondBase => "2B",
            Position::ThirdBase => "3B",
            Position::Shortstop => "SS",
            Position::LeftField => "LF",
            Position::CenterField => "CF",
            Position::RightField => "RF",
            Position::DesignatedHitter => "DH",
        }
    }

    pub fn full_name(&self) -> &'static str {
        match self {
            Position::Pitcher => "Pitcher",
            Position::Catcher => "Catcher",
            Position::FirstBase => "First Base",
            Position::SecondBase => "Second Base",
            Position::ThirdBase => "Third Base",
            Position::Shortstop => "Shortstop",
            Position::LeftField => "Left Field",
            Position::CenterField => "Center Field",
            Position::RightField => "Right Field",
            Position::DesignatedHitter => "Designated Hitter",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldingStats {
    pub games_played: u32,
    pub putouts: u32,
    pub assists: u32,
    pub errors: u32,
    pub double_plays: u32,
    pub fielding_chances: u32,
    pub passed_balls: u32,      // Catcher specific
    pub stolen_bases_allowed: u32, // Catcher specific
    pub caught_stealing: u32,   // Catcher specific
}

impl FieldingStats {
    pub fn new() -> Self {
        Self {
            games_played: 0,
            putouts: 0,
            assists: 0,
            errors: 0,
            double_plays: 0,
            fielding_chances: 0,
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

    pub fn range_factor(&self) -> f64 {
        if self.games_played == 0 {
            0.0
        } else {
            (self.putouts + self.assists) as f64 / self.games_played as f64
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
pub struct FieldingAbilities {
    // Fielding ratings (0.0 to 1.0)
    pub range_rating: f64,      // Ability to get to balls
    pub hands_rating: f64,      // Sure-handedness, catching ability
    pub arm_strength: f64,      // Throwing power
    pub arm_accuracy: f64,      // Throwing precision
    pub reaction_time: f64,     // Quick first step
    pub double_play_ability: f64, // Turning two
    
    // Position-specific abilities
    pub blocking_ability: f64,  // Catcher - blocking wild pitches/passed balls
    pub framing_ability: f64,   // Catcher - pitch framing
    pub game_calling: f64,      // Catcher - calling pitches
}

impl Default for FieldingAbilities {
    fn default() -> Self {
        Self {
            range_rating: 0.5,
            hands_rating: 0.7,
            arm_strength: 0.5,
            arm_accuracy: 0.7,
            reaction_time: 0.5,
            double_play_ability: 0.5,
            blocking_ability: 0.5,
            framing_ability: 0.5,
            game_calling: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fielder {
    pub id: String,
    pub name: String,
    pub jersey_number: u8,
    pub primary_position: Position,
    pub secondary_positions: Vec<Position>,
    pub stats: FieldingStats,
    pub abilities: FieldingAbilities,
    pub is_injured: bool,
    pub fatigue_level: f64, // 0.0 = exhausted, 1.0 = fresh
}

impl Fielder {
    pub fn new(id: String, name: String, jersey_number: u8, primary_position: Position) -> Self {
        Self {
            id,
            name,
            jersey_number,
            primary_position,
            secondary_positions: Vec::new(),
            stats: FieldingStats::new(),
            abilities: FieldingAbilities::default(),
            is_injured: false,
            fatigue_level: 1.0,
        }
    }

    pub fn with_secondary_positions(mut self, positions: Vec<Position>) -> Self {
        self.secondary_positions = positions;
        self
    }

    pub fn with_abilities(mut self, abilities: FieldingAbilities) -> Self {
        self.abilities = abilities;
        self
    }

    pub fn can_play_position(&self, position: Position) -> bool {
        self.primary_position == position || self.secondary_positions.contains(&position)
    }

    pub fn position_rating(&self, position: Position) -> f64 {
        if position == self.primary_position {
            1.0
        } else if self.secondary_positions.contains(&position) {
            0.8 // 20% penalty for playing out of position
        } else {
            0.5 // 50% penalty for playing completely out of position
        }
    }

    pub fn effective_range(&self) -> f64 {
        self.abilities.range_rating * self.fatigue_level
    }

    pub fn effective_hands(&self) -> f64 {
        self.abilities.hands_rating * self.fatigue_level
    }

    pub fn effective_arm_strength(&self) -> f64 {
        self.abilities.arm_strength * self.fatigue_level
    }

    pub fn effective_reaction_time(&self) -> f64 {
        self.abilities.reaction_time * self.fatigue_level
    }

    pub fn defensive_rating(&self, position: Position) -> f64 {
        let position_modifier = self.position_rating(position);
        let base_rating = (self.effective_range() + self.effective_hands() + 
                          self.abilities.arm_accuracy + self.effective_reaction_time()) / 4.0;
        base_rating * position_modifier
    }

    pub fn is_catcher(&self) -> bool {
        self.primary_position == Position::Catcher
    }

    pub fn is_infielder(&self) -> bool {
        self.primary_position.is_infield()
    }

    pub fn is_outfielder(&self) -> bool {
        self.primary_position.is_outfield()
    }
}