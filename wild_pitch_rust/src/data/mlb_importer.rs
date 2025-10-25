use crate::players::{Batter, BatterTendencies, Player, Position, Handedness};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseballSavantBatter {
    pub last_name: String,
    pub first_name: String,
    pub player_id: String,
    pub attempts: f64,
    pub avg_hit_angle: f64,
    pub anglesweetspotpercent: f64,
    pub max_hit_speed: f64,
    pub avg_hit_speed: f64,
    pub ev50: f64,
    pub fbld: f64,
    pub gb: f64,
    pub max_distance: f64,
    pub avg_distance: f64,
    pub avg_hr_distance: f64,
    pub ev95plus: f64,
    pub ev95percent: f64,
    pub barrels: f64,
    pub brl_percent: f64,
    pub brl_pa: f64,
}

impl BaseballSavantBatter {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Convert Baseball Savant metrics to our game's batter tendencies
    pub fn to_batter_tendencies(&self) -> BatterTendencies {
        // Convert Statcast metrics to 0.0-1.0 scale for our game
        
        // Contact rate based on sweet spot percentage and average hit speed
        let contact_rate = (self.anglesweetspotpercent / 100.0 * 0.4 + 
                           (self.avg_hit_speed - 70.0) / 50.0 * 0.6)
                           .clamp(0.3, 0.95);

        // Power rating based on barrel rate, max exit velocity, and average distance
        let power_rating = (self.brl_percent / 30.0 * 0.4 +
                           (self.max_hit_speed - 90.0) / 30.0 * 0.3 +
                           (self.avg_distance - 150.0) / 100.0 * 0.3)
                           .clamp(0.2, 1.0);

        // Speed rating - we'll estimate based on ground ball rate (faster players might hit more ground balls)
        let speed_rating = if self.gb > 45.0 {
            0.7 // Higher ground ball rate might indicate speed
        } else {
            0.5 // Default speed
        };

        // Patience rating based on barrel rate (patient hitters get better pitches)
        let patience_rating = (self.brl_percent / 20.0).clamp(0.3, 0.8);

        // Clutch rating - we'll use a default since this data doesn't include situational stats
        let clutch_rating = 0.5;

        // Platoon splits - defaults for now, would need additional data
        let vs_lefty_modifier = 1.0;
        let vs_righty_modifier = 1.0;
        let with_runners_modifier = 1.0;

        BatterTendencies {
            contact_rate,
            power_rating,
            speed_rating,
            patience_rating,
            clutch_rating,
            vs_lefty_modifier,
            vs_righty_modifier,
            with_runners_modifier,
        }
    }

    // Create a basic batter with estimated stats based on Statcast data
    pub fn to_player(&self, jersey_number: u8, position: Position) -> Player {
        let player_id = format!("mlb_{}", self.player_id);
        let name = self.full_name();
        
        let mut player = Player::position_player(
            player_id,
            name,
            jersey_number,
            position,
            Handedness::Right, // Default, would need additional data
            Handedness::Right, // Default, would need additional data
        );

        // Set batter tendencies based on Statcast data
        if let Some(ref mut batter) = player.batter {
            batter.tendencies = self.to_batter_tendencies();
            
            // Estimate basic stats based on attempts and performance metrics
            let estimated_at_bats = (self.attempts * 0.85) as u32; // Estimate ABs from attempts
            let estimated_hits = (estimated_at_bats as f64 * self.contact_rate_estimate()) as u32;
            let estimated_home_runs = (self.barrels * 0.8) as u32; // Rough barrel-to-HR conversion
            
            batter.stats.at_bats = estimated_at_bats;
            batter.stats.hits = estimated_hits;
            batter.stats.home_runs = estimated_home_runs;
            batter.stats.doubles = (estimated_hits as f64 * 0.2) as u32;
            batter.stats.triples = (estimated_hits as f64 * 0.02) as u32;
            batter.stats.runs_batted_in = estimated_home_runs * 2 + estimated_hits / 4;
            batter.stats.walks = (self.attempts * 0.1) as u32; // Estimate walks
            batter.stats.strikeouts = (self.attempts * 0.2) as u32; // Estimate strikeouts
        }

        player
    }

    fn contact_rate_estimate(&self) -> f64 {
        // Estimate contact rate from Statcast metrics
        (self.anglesweetspotpercent / 100.0 * 0.6 + 0.15).clamp(0.15, 0.35)
    }
}

#[derive(Debug, Clone)]
pub struct MLBTeamData {
    pub team_name: String,
    pub team_id: String,
    pub players: Vec<BaseballSavantBatter>,
}

pub struct MLBDataImporter;

impl MLBDataImporter {
    pub fn parse_baseball_savant_csv(csv_data: &str) -> Result<Vec<BaseballSavantBatter>> {
        let mut players = Vec::new();
        let lines: Vec<&str> = csv_data.lines().collect();
        
        if lines.is_empty() {
            return Ok(players);
        }

        // Skip header line
        for line in lines.iter().skip(1) {
            if line.trim().is_empty() {
                continue;
            }

            let player = Self::parse_csv_line(line)?;
            players.push(player);
        }

        Ok(players)
    }

    fn parse_csv_line(line: &str) -> Result<BaseballSavantBatter> {
        let fields: Vec<&str> = line.split(',').map(|s| s.trim_matches('"').trim()).collect();
        
        if fields.len() < 18 {
            return Err(anyhow::anyhow!("Invalid CSV line: not enough fields"));
        }

        // Parse name (format: "Last, First")
        let name_parts: Vec<&str> = fields[0].split(", ").collect();
        let (last_name, first_name) = if name_parts.len() >= 2 {
            (name_parts[0].to_string(), name_parts[1].to_string())
        } else {
            // If no comma, assume it's last name only
            let parts: Vec<&str> = fields[0].split_whitespace().collect();
            if parts.len() >= 2 {
                (parts.last().unwrap().to_string(), parts[0].to_string())
            } else {
                (fields[0].to_string(), "".to_string())
            }
        };

        Ok(BaseballSavantBatter {
            last_name,
            first_name,
            player_id: fields[1].to_string(),
            attempts: fields[2].parse().unwrap_or(0.0),
            avg_hit_angle: fields[3].parse().unwrap_or(0.0),
            anglesweetspotpercent: fields[4].parse().unwrap_or(0.0),
            max_hit_speed: fields[5].parse().unwrap_or(0.0),
            avg_hit_speed: fields[6].parse().unwrap_or(0.0),
            ev50: fields[7].parse().unwrap_or(0.0),
            fbld: fields[8].parse().unwrap_or(0.0),
            gb: fields[9].parse().unwrap_or(0.0),
            max_distance: fields[10].parse().unwrap_or(0.0),
            avg_distance: fields[11].parse().unwrap_or(0.0),
            avg_hr_distance: fields[12].parse().unwrap_or(0.0),
            ev95plus: fields[13].parse().unwrap_or(0.0),
            ev95percent: fields[14].parse().unwrap_or(0.0),
            barrels: fields[15].parse().unwrap_or(0.0),
            brl_percent: fields[16].parse().unwrap_or(0.0),
            brl_pa: fields[17].parse().unwrap_or(0.0),
        })
    }

    pub async fn fetch_team_data(team_id: &str, year: u16) -> Result<MLBTeamData> {
        let url = format!(
            "https://baseballsavant.mlb.com/leaderboard/statcast?type=batter&year={}&position=&team={}&min=q&sort=barrels_per_pa&sortDir=desc&csv=true",
            year, team_id
        );

        // For now, we'll return mock data since we can't make HTTP requests directly
        // In a real implementation, you'd use reqwest or similar to fetch the data
        let team_name = match team_id {
            "147" => "New York Yankees",
            "119" => "Los Angeles Dodgers",
            _ => "Unknown Team",
        };

        Ok(MLBTeamData {
            team_name: team_name.to_string(),
            team_id: team_id.to_string(),
            players: Vec::new(), // Would be populated from HTTP request
        })
    }

    pub fn create_team_from_savant_data(
        team_data: &MLBTeamData,
        starting_positions: &[(Position, usize)], // (Position, player_index)
    ) -> Result<crate::teams::Team> {
        let mut team = crate::teams::Team::new(
            format!("mlb_{}", team_data.team_id),
            Self::get_team_name(&team_data.team_id),
            Self::get_team_city(&team_data.team_id),
            Self::get_team_abbreviation(&team_data.team_id),
        );

        // Set team colors
        let (primary, secondary) = Self::get_team_colors(&team_data.team_id);
        team = team.with_colors(primary, secondary);

        // Set ballpark
        team = team.with_ballpark(Self::get_ballpark_name(&team_data.team_id));

        // Add players to team
        for (i, savant_player) in team_data.players.iter().enumerate() {
            let jersey_number = (i + 1) as u8; // Simple jersey numbering
            
            // Determine position - use provided mapping or default to outfield
            let position = starting_positions
                .iter()
                .find(|(_, idx)| *idx == i)
                .map(|(pos, _)| *pos)
                .unwrap_or(Position::RightField);

            let player = savant_player.to_player(jersey_number, position);
            team.add_player(player).map_err(|e| anyhow::anyhow!(e))?;
        }

        Ok(team)
    }

    fn get_team_name(team_id: &str) -> String {
        match team_id {
            "147" => "Yankees".to_string(),
            "119" => "Dodgers".to_string(),
            _ => "Team".to_string(),
        }
    }

    fn get_team_city(team_id: &str) -> String {
        match team_id {
            "147" => "New York".to_string(),
            "119" => "Los Angeles".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    fn get_team_abbreviation(team_id: &str) -> String {
        match team_id {
            "147" => "NYY".to_string(),
            "119" => "LAD".to_string(),
            _ => "UNK".to_string(),
        }
    }

    fn get_team_colors(team_id: &str) -> (String, String) {
        match team_id {
            "147" => ("Navy Blue".to_string(), "Gray".to_string()),
            "119" => ("Dodger Blue".to_string(), "White".to_string()),
            _ => ("Blue".to_string(), "White".to_string()),
        }
    }

    fn get_ballpark_name(team_id: &str) -> String {
        match team_id {
            "147" => "Yankee Stadium".to_string(),
            "119" => "Dodger Stadium".to_string(),
            _ => "Stadium".to_string(),
        }
    }

    // Helper function to create a standard MLB lineup from imported players
    pub fn create_standard_lineup(players: &[BaseballSavantBatter]) -> Vec<(Position, usize)> {
        // This is a simple mapping - in reality you'd want more sophisticated position assignment
        let positions = vec![
            Position::Catcher,
            Position::FirstBase,
            Position::SecondBase,
            Position::ThirdBase,
            Position::Shortstop,
            Position::LeftField,
            Position::CenterField,
            Position::RightField,
            Position::DesignatedHitter,
        ];

        positions
            .into_iter()
            .enumerate()
            .take(players.len().min(9))
            .map(|(i, pos)| (pos, i))
            .collect()
    }
}