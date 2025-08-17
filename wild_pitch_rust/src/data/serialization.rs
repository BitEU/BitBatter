use crate::game::GameState;
use crate::teams::Team;
use crate::utils::GameConfig;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedGame {
    pub game_id: String,
    pub save_timestamp: String,
    pub game_state: GameState,
    pub description: String,
    pub inning_display: String,
    pub score_display: String,
}

impl SavedGame {
    pub fn new(game_state: GameState, description: String) -> Self {
        let save_timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
        let inning_display = format!(
            "{} {}",
            if game_state.is_top_inning() { "Top" } else { "Bottom" },
            game_state.inning()
        );
        let score_display = format!(
            "{} {} - {} {}",
            game_state.visitor_team.abbreviation,
            game_state.visitor_score(),
            game_state.home_score(),
            game_state.home_team.abbreviation
        );

        Self {
            game_id: game_state.game_id.clone(),
            save_timestamp,
            game_state,
            description,
            inning_display,
            score_display,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedGameList {
    pub saves: Vec<SavedGame>,
    pub last_modified: String,
}

impl SavedGameList {
    pub fn new() -> Self {
        Self {
            saves: Vec::new(),
            last_modified: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        }
    }

    pub fn add_save(&mut self, saved_game: SavedGame) {
        self.saves.push(saved_game);
        self.update_timestamp();
    }

    pub fn remove_save(&mut self, game_id: &str) -> bool {
        let initial_len = self.saves.len();
        self.saves.retain(|save| save.game_id != game_id);
        let removed = self.saves.len() != initial_len;
        if removed {
            self.update_timestamp();
        }
        removed
    }

    pub fn get_save(&self, game_id: &str) -> Option<&SavedGame> {
        self.saves.iter().find(|save| save.game_id == game_id)
    }

    pub fn list_saves(&self) -> Vec<String> {
        self.saves
            .iter()
            .map(|save| format!("{}: {} - {}", save.game_id, save.inning_display, save.score_display))
            .collect()
    }

    fn update_timestamp(&mut self) {
        self.last_modified = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    }
}

pub struct GameSerializer;

impl GameSerializer {
    pub const SAVES_DIR: &'static str = "saves";
    pub const SAVES_INDEX_FILE: &'static str = "saves/index.json";
    pub const CONFIG_FILE: &'static str = "config/game.json";

    pub fn ensure_saves_dir() -> Result<()> {
        fs::create_dir_all(Self::SAVES_DIR)?;
        Ok(())
    }

    pub fn save_game(game_state: &GameState, description: Option<String>) -> Result<String> {
        Self::ensure_saves_dir()?;

        let saved_game = SavedGame::new(
            game_state.clone(),
            description.unwrap_or_else(|| "Quick Save".to_string()),
        );

        let save_file = format!("saves/{}.json", saved_game.game_id);
        let json = serde_json::to_string_pretty(&saved_game)?;
        fs::write(&save_file, json)?;

        // Update saves index
        let mut saves_list = Self::load_saves_index()?;
        saves_list.add_save(saved_game);
        Self::save_saves_index(&saves_list)?;

        Ok(save_file)
    }

    pub fn load_game(game_id: &str) -> Result<GameState> {
        let save_file = format!("saves/{}.json", game_id);
        let contents = fs::read_to_string(&save_file)?;
        let saved_game: SavedGame = serde_json::from_str(&contents)?;
        Ok(saved_game.game_state)
    }

    pub fn delete_save(game_id: &str) -> Result<bool> {
        let save_file = format!("saves/{}.json", game_id);
        
        // Remove from filesystem
        let file_existed = Path::new(&save_file).exists();
        if file_existed {
            fs::remove_file(&save_file)?;
        }

        // Update saves index
        let mut saves_list = Self::load_saves_index()?;
        let removed_from_index = saves_list.remove_save(game_id);
        Self::save_saves_index(&saves_list)?;

        Ok(file_existed || removed_from_index)
    }

    pub fn load_saves_index() -> Result<SavedGameList> {
        if Path::new(Self::SAVES_INDEX_FILE).exists() {
            let contents = fs::read_to_string(Self::SAVES_INDEX_FILE)?;
            let saves_list: SavedGameList = serde_json::from_str(&contents)?;
            Ok(saves_list)
        } else {
            Ok(SavedGameList::new())
        }
    }

    pub fn save_saves_index(saves_list: &SavedGameList) -> Result<()> {
        Self::ensure_saves_dir()?;
        let json = serde_json::to_string_pretty(saves_list)?;
        fs::write(Self::SAVES_INDEX_FILE, json)?;
        Ok(())
    }

    pub fn list_saves() -> Result<Vec<SavedGame>> {
        let saves_list = Self::load_saves_index()?;
        Ok(saves_list.saves)
    }

    pub fn auto_save(game_state: &GameState) -> Result<String> {
        let description = format!(
            "Auto-save: {} {}, {} {} - {} {}",
            if game_state.is_top_inning() { "Top" } else { "Bottom" },
            game_state.inning(),
            game_state.visitor_team.abbreviation,
            game_state.visitor_score(),
            game_state.home_score(),
            game_state.home_team.abbreviation
        );

        Self::save_game(game_state, Some(description))
    }

    pub fn export_game_stats(game_state: &GameState, path: &str) -> Result<()> {
        #[derive(Serialize)]
        struct GameExport {
            game_info: GameInfo,
            box_score: BoxScore,
            play_by_play: Vec<String>,
        }

        #[derive(Serialize)]
        struct GameInfo {
            game_id: String,
            visitor_team: String,
            home_team: String,
            final_score: String,
            innings: u8,
            game_time: String,
            attendance: u32,
            weather: String,
        }

        #[derive(Serialize)]
        struct BoxScore {
            visitor: TeamBoxScore,
            home: TeamBoxScore,
        }

        #[derive(Serialize)]
        struct TeamBoxScore {
            team_name: String,
            runs: u32,
            hits: u32,
            errors: u32,
            runs_by_inning: Vec<u32>,
        }

        let export = GameExport {
            game_info: GameInfo {
                game_id: game_state.game_id.clone(),
                visitor_team: game_state.visitor_team.full_name(),
                home_team: game_state.home_team.full_name(),
                final_score: format!(
                    "{} {} - {} {}",
                    game_state.visitor_team.abbreviation,
                    game_state.visitor_score(),
                    game_state.home_score(),
                    game_state.home_team.abbreviation
                ),
                innings: game_state.inning(),
                game_time: game_state.game_start_time.clone(),
                attendance: game_state.attendance,
                weather: game_state.weather.clone(),
            },
            box_score: BoxScore {
                visitor: TeamBoxScore {
                    team_name: game_state.visitor_team.full_name(),
                    runs: game_state.score.visitor,
                    hits: game_state.visitor_team.stats.batting.hits,
                    errors: game_state.visitor_team.stats.fielding.errors,
                    runs_by_inning: game_state.score.visitor_runs_by_inning.clone(),
                },
                home: TeamBoxScore {
                    team_name: game_state.home_team.full_name(),
                    runs: game_state.score.home,
                    hits: game_state.home_team.stats.batting.hits,
                    errors: game_state.home_team.stats.fielding.errors,
                    runs_by_inning: game_state.score.home_runs_by_inning.clone(),
                },
            },
            play_by_play: game_state.play_by_play.clone(),
        };

        let json = serde_json::to_string_pretty(&export)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn save_config(config: &GameConfig) -> Result<()> {
        crate::utils::ConfigPaths::ensure_config_dir()?;
        config.save_to_file(Self::CONFIG_FILE)
    }

    pub fn load_config() -> Result<GameConfig> {
        if Path::new(Self::CONFIG_FILE).exists() {
            GameConfig::load_from_file(Self::CONFIG_FILE)
        } else {
            let config = GameConfig::default();
            Self::save_config(&config)?;
            Ok(config)
        }
    }

    pub fn backup_saves(backup_path: &str) -> Result<()> {
        let saves_list = Self::load_saves_index()?;
        let backup_data = serde_json::to_string_pretty(&saves_list)?;
        fs::write(backup_path, backup_data)?;
        Ok(())
    }

    pub fn restore_saves_from_backup(backup_path: &str) -> Result<()> {
        let contents = fs::read_to_string(backup_path)?;
        let saves_list: SavedGameList = serde_json::from_str(&contents)?;
        
        Self::ensure_saves_dir()?;
        Self::save_saves_index(&saves_list)?;
        
        Ok(())
    }
}