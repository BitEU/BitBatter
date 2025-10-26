use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerStats {
    #[serde(rename = "last_name, first_name")]
    pub name: String,
    
    #[serde(rename = "player_id")]
    pub id: String,
    
    pub attempts: u32,
    
    #[serde(rename = "avg_hit_angle")]
    pub avg_hit_angle: f32,
    
    #[serde(rename = "anglesweetspotpercent")]
    pub sweet_spot_percent: f32,
    
    #[serde(rename = "max_hit_speed")]
    pub max_hit_speed: f32,
    
    #[serde(rename = "avg_hit_speed")]
    pub avg_hit_speed: f32,
    
    pub ev50: f32,
    pub fbld: f32,
    pub gb: f32,
    
    #[serde(rename = "max_distance")]
    pub max_distance: u32,
    
    #[serde(rename = "avg_distance")]
    pub avg_distance: u32,
    
    #[serde(rename = "avg_hr_distance")]
    pub avg_hr_distance: u32,
    
    pub ev95plus: u32,
    
    #[serde(rename = "ev95percent")]
    pub ev95_percent: f32,
    
    pub barrels: u32,
    
    #[serde(rename = "brl_percent")]
    pub barrel_percent: f32,
    
    #[serde(rename = "brl_pa")]
    pub barrel_pa: f32,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub stats: PlayerStats,
    pub is_pitcher: bool,
    pub position: Position,
}

#[derive(Debug, Clone, Copy)]
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
}

impl Position {
    pub fn name(&self) -> &'static str {
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct Team {
    pub name: String,
    pub abbreviation: String,
    pub batters: Vec<Player>,
    pub pitchers: Vec<Player>,
    pub current_pitcher_idx: usize,
    pub pitcher_stamina: f32,  // 0.0 to 100.0, starts at 100
    pub pitches_thrown: u32,    // Track total pitches thrown
}

impl Team {
    pub fn new(name: String, abbreviation: String) -> Self {
        Self {
            name,
            abbreviation,
            batters: Vec::new(),
            pitchers: Vec::new(),
            current_pitcher_idx: 0,
            pitcher_stamina: crate::game::constants::STARTING_STAMINA,
            pitches_thrown: 0,
        }
    }

    pub fn get_current_pitcher(&self) -> Option<&Player> {
        self.pitchers.get(self.current_pitcher_idx)
    }

    pub fn get_batter(&self, idx: usize) -> Option<&Player> {
        if self.batters.is_empty() {
            return None;
        }
        self.batters.get(idx % self.batters.len())
    }

    pub fn batting_order_size(&self) -> usize {
        if self.batters.is_empty() {
            return crate::game::constants::BATTING_ORDER_SIZE;
        }
        self.batters.len().min(crate::game::constants::BATTING_ORDER_SIZE)
    }

    pub fn decrease_stamina(&mut self, amount: f32) {
        self.pitcher_stamina = (self.pitcher_stamina - amount).max(0.0);
        self.pitches_thrown += 1;
    }

    pub fn get_fatigue_penalty(&self) -> f32 {
        use crate::game::constants::*;
        // Returns a multiplier between 0.5 (very tired) and 1.0 (fresh)
        // Fatigue kicks in more severely below 50 stamina
        if self.pitcher_stamina >= STAMINA_FRESH_THRESHOLD {
            FATIGUE_PENALTY_FRESH
        } else if self.pitcher_stamina >= STAMINA_GOOD_THRESHOLD {
            FATIGUE_PENALTY_GOOD
        } else if self.pitcher_stamina >= STAMINA_TIRED_THRESHOLD {
            FATIGUE_PENALTY_TIRED
        } else if self.pitcher_stamina >= STAMINA_EXHAUSTED_THRESHOLD {
            FATIGUE_PENALTY_VERY_TIRED
        } else {
            FATIGUE_PENALTY_EXHAUSTED
        }
    }

    pub fn change_pitcher(&mut self) {
        if !self.pitchers.is_empty() {
            self.current_pitcher_idx = (self.current_pitcher_idx + 1) % self.pitchers.len();
            self.pitcher_stamina = crate::game::constants::STARTING_STAMINA;
            self.pitches_thrown = 0;
        }
    }
}

#[derive(Debug, Clone)]
pub struct TeamManager {
    pub teams: HashMap<String, Team>,
}

impl TeamManager {
    pub fn new() -> Self {
        Self {
            teams: HashMap::new(),
        }
    }

    /// Get list of all available team abbreviations without loading them
    pub fn get_team_list(&self) -> Vec<String> {
        vec![
            "ARI", "ATL", "BAL", "BOS", "CHC", "CIN", "CLE", "COL", "CWS", "DET",
            "HOU", "KC", "LAA", "LAD", "MIA", "MIL", "MIN", "NYM", "NYY", "OAK",
            "PHI", "PIT", "SD", "SDG", "SEA", "SF", "STL", "TB", "TEX", "THW", "TOR", "WSH"
        ].iter().map(|s| s.to_string()).collect()
    }

    /// Load a specific team's data from CSV files
    pub fn load_team(&mut self, abbr: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check if already loaded
        if self.teams.contains_key(abbr) {
            return Ok(());
        }

        let team_name = self.get_team_full_name(abbr);
        let mut team = Team::new(team_name, abbr.to_string());

        // Load batters
        let batter_path = PathBuf::from("data_down")
            .join("statcast_downloads")
            .join(format!("batter_{}_2025.csv", abbr));
        
        match Self::load_players_from_csv(&batter_path, false) {
            Ok(batters) => team.batters = batters,
            Err(e) => return Err(format!("Failed to load batters for {}: {}", abbr, e).into()),
        }

        // Load pitchers  
        let pitcher_path = PathBuf::from("data_down")
            .join("statcast_downloads")
            .join(format!("pitcher_{}_2025.csv", abbr));
        
        match Self::load_players_from_csv(&pitcher_path, true) {
            Ok(pitchers) => team.pitchers = pitchers,
            Err(e) => return Err(format!("Failed to load pitchers for {}: {}", abbr, e).into()),
        }

        // Only add teams that have players
        if team.batters.is_empty() && team.pitchers.is_empty() {
            return Err(format!("No player data found for team {}", abbr).into());
        }

        self.teams.insert(abbr.to_string(), team);
        Ok(())
    }

    pub fn get_team_full_name(&self, abbr: &str) -> String {
        let team_names = [
            ("ARI", "Arizona Diamondbacks"),
            ("ATL", "Atlanta Braves"),
            ("BAL", "Baltimore Orioles"),
            ("BOS", "Boston Red Sox"),
            ("CHC", "Chicago Cubs"),
            ("CIN", "Cincinnati Reds"),
            ("CLE", "Cleveland Guardians"),
            ("COL", "Colorado Rockies"),
            ("CWS", "Chicago White Sox"),
            ("DET", "Detroit Tigers"),
            ("HOU", "Houston Astros"),
            ("KC", "Kansas City Royals"),
            ("LAA", "Los Angeles Angels"),
            ("LAD", "Los Angeles Dodgers"),
            ("MIA", "Miami Marlins"),
            ("MIL", "Milwaukee Brewers"),
            ("MIN", "Minnesota Twins"),
            ("NYM", "New York Mets"),
            ("NYY", "New York Yankees"),
            ("OAK", "Oakland Athletics"),
            ("PHI", "Philadelphia Phillies"),
            ("PIT", "Pittsburgh Pirates"),
            ("SD", "San Diego Padres"),
            ("SDG", "Storm Dragons"),
            ("SEA", "Seattle Mariners"),
            ("SF", "San Francisco Giants"),
            ("STL", "St. Louis Cardinals"),
            ("TB", "Tampa Bay Rays"),
            ("TEX", "Texas Rangers"),
            ("THW", "Thunder Hawks"),
            ("TOR", "Toronto Blue Jays"),
            ("WSH", "Washington Nationals"),
        ];

        team_names.iter()
            .find(|(a, _)| *a == abbr)
            .map(|(_, name)| name.to_string())
            .unwrap_or_else(|| abbr.to_string())
    }

    /// Deprecated - teams are now loaded on demand
    pub fn load_teams(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // This is now a no-op - teams load on demand
        Ok(())
    }

    fn load_players_from_csv(path: &PathBuf, is_pitcher: bool) -> Result<Vec<Player>, Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path(path)?;
        let mut players = Vec::new();

        for result in rdr.deserialize() {
            let stats: PlayerStats = result?;
            
            // Only include players with reasonable number of attempts
            if stats.attempts >= crate::game::constants::MIN_PLAYER_ATTEMPTS {
                let position = if is_pitcher {
                    Position::Pitcher
                } else {
                    // For batters, we'll assign positions based on their stats
                    // This is a simple heuristic - in a real game you'd have position data
                    match players.len() % 8 {
                        0 => Position::Catcher,
                        1 => Position::FirstBase,
                        2 => Position::SecondBase,
                        3 => Position::ThirdBase,
                        4 => Position::Shortstop,
                        5 => Position::LeftField,
                        6 => Position::CenterField,
                        _ => Position::RightField,
                    }
                };

                players.push(Player {
                    stats,
                    is_pitcher,
                    position,
                });
            }
        }

        // Sort batters by barrel percentage (better hitters first)
        if !is_pitcher {
            players.sort_by(|a, b| b.stats.barrel_percent.partial_cmp(&a.stats.barrel_percent).unwrap_or(std::cmp::Ordering::Equal));
        } else {
            // Sort pitchers by barrel percentage allowed (lower is better)
            players.sort_by(|a, b| a.stats.barrel_percent.partial_cmp(&b.stats.barrel_percent).unwrap_or(std::cmp::Ordering::Equal));
        }

        Ok(players)
    }

    pub fn get_team(&self, abbr: &str) -> Option<&Team> {
        self.teams.get(abbr)
    }

    pub fn get_team_mut(&mut self, abbr: &str) -> Option<&mut Team> {
        self.teams.get_mut(abbr)
    }
}
