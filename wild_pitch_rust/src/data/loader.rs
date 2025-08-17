use crate::players::{Player, Position, Handedness, PitcherRole};
use crate::teams::Team;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub id: String,
    pub name: String,
    pub jersey_number: u8,
    pub age: u8,
    pub height: String,
    pub weight: u16,
    pub position: Position,
    pub throws: Handedness,
    pub bats: Handedness,
    pub salary: u32,
    pub years_pro: u8,
    pub pitcher_role: Option<PitcherRole>,
    pub secondary_positions: Vec<Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamData {
    pub id: String,
    pub name: String,
    pub city: String,
    pub abbreviation: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub ballpark_name: String,
    pub players: Vec<PlayerData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeagueData {
    pub name: String,
    pub teams: Vec<TeamData>,
    pub season_year: u16,
}

pub struct DataLoader;

impl DataLoader {
    pub fn load_league_from_file<P: AsRef<Path>>(path: P) -> Result<LeagueData> {
        let contents = fs::read_to_string(path)?;
        let league_data: LeagueData = serde_json::from_str(&contents)?;
        Ok(league_data)
    }

    pub fn save_league_to_file<P: AsRef<Path>>(league: &LeagueData, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(league)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn create_team_from_data(team_data: &TeamData) -> Result<Team> {
        let mut team = Team::new(
            team_data.id.clone(),
            team_data.name.clone(),
            team_data.city.clone(),
            team_data.abbreviation.clone(),
        )
        .with_colors(team_data.primary_color.clone(), team_data.secondary_color.clone())
        .with_ballpark(team_data.ballpark_name.clone());

        // Add players to team
        for player_data in &team_data.players {
            let player = Self::create_player_from_data(player_data)?;
            team.add_player(player).map_err(|e| anyhow::anyhow!(e))?;
        }

        Ok(team)
    }

    pub fn create_player_from_data(player_data: &PlayerData) -> Result<Player> {
        let mut player = if player_data.position == Position::Pitcher {
            Player::pitcher(
                player_data.id.clone(),
                player_data.name.clone(),
                player_data.jersey_number,
                player_data.throws,
                player_data.pitcher_role.unwrap_or(PitcherRole::Starter),
            )
        } else {
            Player::position_player(
                player_data.id.clone(),
                player_data.name.clone(),
                player_data.jersey_number,
                player_data.position,
                player_data.throws,
                player_data.bats,
            )
        };

        // Set additional player attributes
        player.age = player_data.age;
        player.height = player_data.height.clone();
        player.weight = player_data.weight;
        player.salary = player_data.salary;
        player.years_pro = player_data.years_pro;
        
        // Set secondary positions
        player.fielder.secondary_positions = player_data.secondary_positions.clone();

        Ok(player)
    }

    pub fn create_sample_league() -> LeagueData {
        let team1_players = vec![
            // Starting lineup
            PlayerData {
                id: "player_001".to_string(),
                name: "Mike Rodriguez".to_string(),
                jersey_number: 12,
                age: 28,
                height: "5'11\"".to_string(),
                weight: 180,
                position: Position::SecondBase,
                throws: Handedness::Right,
                bats: Handedness::Right,
                salary: 750000,
                years_pro: 5,
                pitcher_role: None,
                secondary_positions: vec![Position::Shortstop],
            },
            PlayerData {
                id: "player_002".to_string(),
                name: "Carlos Martinez".to_string(),
                jersey_number: 27,
                age: 24,
                height: "6'2\"".to_string(),
                weight: 195,
                position: Position::Shortstop,
                throws: Handedness::Right,
                bats: Handedness::Left,
                salary: 650000,
                years_pro: 3,
                pitcher_role: None,
                secondary_positions: vec![Position::SecondBase],
            },
            PlayerData {
                id: "player_003".to_string(),
                name: "David Thompson".to_string(),
                jersey_number: 34,
                age: 26,
                height: "6'0\"".to_string(),
                weight: 185,
                position: Position::CenterField,
                throws: Handedness::Right,
                bats: Handedness::Right,
                salary: 1200000,
                years_pro: 4,
                pitcher_role: None,
                secondary_positions: vec![Position::RightField, Position::LeftField],
            },
            PlayerData {
                id: "player_004".to_string(),
                name: "Josh Wilson".to_string(),
                jersey_number: 15,
                age: 29,
                height: "6'3\"".to_string(),
                weight: 220,
                position: Position::FirstBase,
                throws: Handedness::Left,
                bats: Handedness::Left,
                salary: 2100000,
                years_pro: 7,
                pitcher_role: None,
                secondary_positions: vec![],
            },
            PlayerData {
                id: "player_005".to_string(),
                name: "Tony Garcia".to_string(),
                jersey_number: 9,
                age: 25,
                height: "5'10\"".to_string(),
                weight: 175,
                position: Position::LeftField,
                throws: Handedness::Right,
                bats: Handedness::Switch,
                salary: 850000,
                years_pro: 3,
                pitcher_role: None,
                secondary_positions: vec![Position::RightField],
            },
            PlayerData {
                id: "player_006".to_string(),
                name: "Alex Johnson".to_string(),
                jersey_number: 22,
                age: 27,
                height: "6'1\"".to_string(),
                weight: 190,
                position: Position::RightField,
                throws: Handedness::Right,
                bats: Handedness::Right,
                salary: 950000,
                years_pro: 5,
                pitcher_role: None,
                secondary_positions: vec![Position::CenterField],
            },
            PlayerData {
                id: "player_007".to_string(),
                name: "Ryan Anderson".to_string(),
                jersey_number: 5,
                age: 30,
                height: "6'0\"".to_string(),
                weight: 185,
                position: Position::ThirdBase,
                throws: Handedness::Right,
                bats: Handedness::Right,
                salary: 1500000,
                years_pro: 8,
                pitcher_role: None,
                secondary_positions: vec![],
            },
            PlayerData {
                id: "player_008".to_string(),
                name: "Steve Davis".to_string(),
                jersey_number: 8,
                age: 26,
                height: "6'2\"".to_string(),
                weight: 200,
                position: Position::Catcher,
                throws: Handedness::Right,
                bats: Handedness::Right,
                salary: 800000,
                years_pro: 4,
                pitcher_role: None,
                secondary_positions: vec![],
            },
            // Starting pitcher
            PlayerData {
                id: "player_009".to_string(),
                name: "Jake Smith".to_string(),
                jersey_number: 21,
                age: 28,
                height: "6'4\"".to_string(),
                weight: 210,
                position: Position::Pitcher,
                throws: Handedness::Right,
                bats: Handedness::Right,
                salary: 3200000,
                years_pro: 6,
                pitcher_role: Some(PitcherRole::Starter),
                secondary_positions: vec![],
            },
        ];

        let team2_players = vec![
            // Similar structure for team 2 - abbreviated for brevity
            PlayerData {
                id: "player_101".to_string(),
                name: "Frank Williams".to_string(),
                jersey_number: 3,
                age: 27,
                height: "5'9\"".to_string(),
                weight: 170,
                position: Position::SecondBase,
                throws: Handedness::Right,
                bats: Handedness::Left,
                salary: 700000,
                years_pro: 4,
                pitcher_role: None,
                secondary_positions: vec![],
            },
            // ... more players would be added here
        ];

        let teams = vec![
            TeamData {
                id: "team_001".to_string(),
                name: "Eagles".to_string(),
                city: "Metro".to_string(),
                abbreviation: "MET".to_string(),
                primary_color: "Blue".to_string(),
                secondary_color: "White".to_string(),
                ballpark_name: "Eagle Stadium".to_string(),
                players: team1_players,
            },
            TeamData {
                id: "team_002".to_string(),
                name: "Tigers".to_string(),
                city: "Capital".to_string(),
                abbreviation: "CAP".to_string(),
                primary_color: "Orange".to_string(),
                secondary_color: "Black".to_string(),
                ballpark_name: "Tiger Field".to_string(),
                players: team2_players,
            },
        ];

        LeagueData {
            name: "Wild Pitch League".to_string(),
            teams,
            season_year: 2024,
        }
    }

    pub fn save_sample_league_to_file<P: AsRef<Path>>(path: P) -> Result<()> {
        let league = Self::create_sample_league();
        Self::save_league_to_file(&league, path)
    }

    pub fn load_or_create_sample_league<P: AsRef<Path>>(path: P) -> Result<LeagueData> {
        if path.as_ref().exists() {
            Self::load_league_from_file(path)
        } else {
            let league = Self::create_sample_league();
            Self::save_league_to_file(&league, &path)?;
            Ok(league)
        }
    }
}