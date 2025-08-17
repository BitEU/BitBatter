use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_settings: GameSettings,
    pub simulation_settings: SimulationSettings,
    pub ui_settings: UiSettings,
    pub audio_settings: AudioSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    pub innings_per_game: u8,
    pub designated_hitter: bool,
    pub difficulty_level: DifficultyLevel,
    pub auto_save: bool,
    pub quick_play: bool,
    pub realistic_injuries: bool,
    pub fatigue_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Rookie,
    Pro,
    AllStar,
    HallOfFame,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationSettings {
    pub random_seed: Option<u64>,
    pub simulation_speed: SimulationSpeed,
    pub detailed_stats: bool,
    pub weather_effects: bool,
    pub ballpark_effects: bool,
    pub momentum_effects: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimulationSpeed {
    Slow,
    Normal,
    Fast,
    Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub terminal_mode: TerminalMode,
    pub color_scheme: ColorScheme,
    pub window_layout: WindowLayoutConfig,
    pub font_size: FontSize,
    pub animations_enabled: bool,
    pub show_tooltips: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerminalMode {
    Windowed,
    Fullscreen,
    Borderless,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorScheme {
    Classic,
    Modern,
    HighContrast,
    CustomColors(CustomColors),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomColors {
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub warning: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowLayoutConfig {
    pub scoreboard_size: WindowSize,
    pub ballpark_size: WindowSize,
    pub play_by_play_size: WindowSize,
    pub lineup_cards_size: WindowSize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowSize {
    Small,
    Medium,
    Large,
    Custom(u16, u16), // width, height percentages
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub sound_enabled: bool,
    pub music_enabled: bool,
    pub sound_volume: f32,
    pub music_volume: f32,
    pub announcer_enabled: bool,
    pub crowd_noise: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            game_settings: GameSettings {
                innings_per_game: 9,
                designated_hitter: true,
                difficulty_level: DifficultyLevel::Pro,
                auto_save: true,
                quick_play: false,
                realistic_injuries: true,
                fatigue_enabled: true,
            },
            simulation_settings: SimulationSettings {
                random_seed: None,
                simulation_speed: SimulationSpeed::Normal,
                detailed_stats: true,
                weather_effects: true,
                ballpark_effects: true,
                momentum_effects: true,
            },
            ui_settings: UiSettings {
                terminal_mode: TerminalMode::Windowed,
                color_scheme: ColorScheme::Classic,
                window_layout: WindowLayoutConfig {
                    scoreboard_size: WindowSize::Medium,
                    ballpark_size: WindowSize::Medium,
                    play_by_play_size: WindowSize::Medium,
                    lineup_cards_size: WindowSize::Medium,
                },
                font_size: FontSize::Medium,
                animations_enabled: true,
                show_tooltips: true,
            },
            audio_settings: AudioSettings {
                sound_enabled: true,
                music_enabled: true,
                sound_volume: 0.7,
                music_volume: 0.5,
                announcer_enabled: true,
                crowd_noise: true,
            },
        }
    }
}

impl GameConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: GameConfig = serde_json::from_str(&contents)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        Self::load_from_file(path).unwrap_or_default()
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.game_settings.innings_per_game < 1 {
            errors.push("Innings per game must be at least 1".to_string());
        }

        if self.audio_settings.sound_volume < 0.0 || self.audio_settings.sound_volume > 1.0 {
            errors.push("Sound volume must be between 0.0 and 1.0".to_string());
        }

        if self.audio_settings.music_volume < 0.0 || self.audio_settings.music_volume > 1.0 {
            errors.push("Music volume must be between 0.0 and 1.0".to_string());
        }

        errors
    }

    pub fn get_innings_per_game(&self) -> u8 {
        self.game_settings.innings_per_game
    }

    pub fn has_designated_hitter(&self) -> bool {
        self.game_settings.designated_hitter
    }

    pub fn get_difficulty_modifier(&self) -> f64 {
        match self.game_settings.difficulty_level {
            DifficultyLevel::Rookie => 1.2,      // 20% easier
            DifficultyLevel::Pro => 1.0,         // Normal
            DifficultyLevel::AllStar => 0.9,     // 10% harder
            DifficultyLevel::HallOfFame => 0.8,  // 20% harder
        }
    }

    pub fn get_simulation_delay_ms(&self) -> u64 {
        match self.simulation_settings.simulation_speed {
            SimulationSpeed::Slow => 2000,
            SimulationSpeed::Normal => 1000,
            SimulationSpeed::Fast => 500,
            SimulationSpeed::Instant => 0,
        }
    }

    pub fn should_show_detailed_stats(&self) -> bool {
        self.simulation_settings.detailed_stats
    }

    pub fn has_weather_effects(&self) -> bool {
        self.simulation_settings.weather_effects
    }

    pub fn has_ballpark_effects(&self) -> bool {
        self.simulation_settings.ballpark_effects
    }

    pub fn has_momentum_effects(&self) -> bool {
        self.simulation_settings.momentum_effects
    }

    pub fn is_fatigue_enabled(&self) -> bool {
        self.game_settings.fatigue_enabled
    }

    pub fn has_realistic_injuries(&self) -> bool {
        self.game_settings.realistic_injuries
    }

    pub fn should_auto_save(&self) -> bool {
        self.game_settings.auto_save
    }

    pub fn is_quick_play(&self) -> bool {
        self.game_settings.quick_play
    }
}

// Configuration paths and utilities
pub struct ConfigPaths;

impl ConfigPaths {
    pub const CONFIG_DIR: &'static str = "config";
    pub const GAME_CONFIG_FILE: &'static str = "config/game.json";
    pub const TEAMS_CONFIG_FILE: &'static str = "config/teams.json";
    pub const PLAYERS_CONFIG_FILE: &'static str = "config/players.json";
    pub const KEYBINDINGS_FILE: &'static str = "config/keybindings.json";

    pub fn ensure_config_dir() -> Result<()> {
        fs::create_dir_all(Self::CONFIG_DIR)?;
        Ok(())
    }

    pub fn config_exists() -> bool {
        Path::new(Self::GAME_CONFIG_FILE).exists()
    }

    pub fn create_default_config() -> Result<()> {
        Self::ensure_config_dir()?;
        let config = GameConfig::default();
        config.save_to_file(Self::GAME_CONFIG_FILE)?;
        Ok(())
    }
}