use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use wild_pitch::{
    data::{DataLoader, GameSerializer, MLBTestData},
    game::{GameEngine, GameState},
    teams::Team,
    ui::{
        create_default_layout, Dialog, DialogManager, DialogResult, MenuAction, MenuManager, TerminalUI,
        WindowManager,
    },
    utils::{GameConfig, ConfigPaths},
};

struct WildPitchApp {
    terminal_ui: TerminalUI,
    menu_manager: MenuManager,
    dialog_manager: DialogManager,
    window_manager: WindowManager,
    game_engine: GameEngine,
    current_game: Option<GameState>,
    config: GameConfig,
    is_running: bool,
    show_menu: bool,
}

impl WildPitchApp {
    fn new() -> Result<Self> {
        let terminal_ui = TerminalUI::new()?;
        let menu_manager = MenuManager::new();
        let dialog_manager = DialogManager::new();
        let window_manager = WindowManager::new();
        let game_engine = GameEngine::new();
        let config = GameSerializer::load_config().unwrap_or_default();

        Ok(Self {
            terminal_ui,
            menu_manager,
            dialog_manager,
            window_manager,
            game_engine,
            current_game: None,
            config,
            is_running: true,
            show_menu: true,
        })
    }

    fn run(&mut self) -> Result<()> {
        while self.is_running {
            self.update()?;
            self.render()?;
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        if let Some(event) = TerminalUI::poll_event()? {
            if let Event::Key(key_event) = event {
                if self.dialog_manager.has_dialog() {
                    self.handle_dialog_input(key_event)?;
                } else if self.show_menu {
                    self.handle_menu_input(key_event)?;
                } else {
                    self.handle_game_input(key_event)?;
                }
            }
        }
        Ok(())
    }

    fn handle_dialog_input(&mut self, key_event: KeyEvent) -> Result<()> {
        if let Some(result) = self.dialog_manager.handle_key_event(key_event) {
            match result {
                DialogResult::Yes => {
                    // Handle confirmation dialogs
                    self.is_running = false;
                },
                DialogResult::Custom(value) => {
                    // Handle input dialogs
                    println!("Got input: {}", value);
                },
                _ => {
                    // Dialog was cancelled or closed
                },
            }
        }
        Ok(())
    }

    fn handle_menu_input(&mut self, key_event: KeyEvent) -> Result<()> {
        if let Some(action) = self.menu_manager.handle_key_event(key_event) {
            if let Some(action) = self.menu_manager.process_action(action) {
                self.handle_menu_action(action)?;
            }
        }
        Ok(())
    }

    fn handle_menu_action(&mut self, action: MenuAction) -> Result<()> {
        match action {
            MenuAction::NewGame => {
                self.start_new_game()?;
            },
            MenuAction::LoadGame => {
                self.show_load_game_dialog();
            },
            MenuAction::SaveGame => {
                if let Some(ref game_state) = self.current_game {
                    GameSerializer::save_game(game_state, None)?;
                }
            },
            MenuAction::Settings => {
                // Settings are handled by submenu navigation
            },
            MenuAction::Resume => {
                self.show_menu = false;
            },
            MenuAction::Quit => {
                self.handle_quit();
            },
            MenuAction::Custom(custom_action) => {
                self.handle_custom_action(custom_action)?;
            },
            _ => {
                // Other actions handled by menu manager
            },
        }
        Ok(())
    }

    fn handle_game_input(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                self.show_menu = true;
            },
            KeyCode::Char(' ') => {
                // Simulate next at-bat
                if let Some(ref mut game_state) = self.current_game {
                    let _event = self.game_engine.simulate_at_bat(game_state)?;
                }
            },
            KeyCode::Char('s') => {
                // Quick save
                if let Some(ref game_state) = self.current_game {
                    GameSerializer::auto_save(game_state)?;
                }
            },
            _ => {},
        }
        Ok(())
    }

    fn start_new_game(&mut self) -> Result<()> {
        // Create MLB teams using real Baseball Savant data
        match MLBTestData::create_mlb_teams() {
            Ok((yankees, dodgers)) => {
                let game_id = format!("game_{}", chrono::Utc::now().timestamp());
                let game_state = GameState::new(game_id, yankees, dodgers);
                
                self.current_game = Some(game_state);
                self.show_menu = false;
                
                // Show a dialog with information about the loaded teams
                let dialog = Dialog::information(
                    "MLB Game Started".to_string(),
                    "Yankees vs Dodgers game loaded with real Baseball Savant player data!".to_string(),
                );
                self.dialog_manager.show_dialog(dialog);
            },
            Err(e) => {
                // Fallback to sample data if MLB data fails
                let league_data = DataLoader::create_sample_league();
                if league_data.teams.len() >= 2 {
                    let visitor_team = DataLoader::create_team_from_data(&league_data.teams[0])?;
                    let home_team = DataLoader::create_team_from_data(&league_data.teams[1])?;
                    
                    let game_id = format!("game_{}", chrono::Utc::now().timestamp());
                    let game_state = GameState::new(game_id, visitor_team, home_team);
                    
                    self.current_game = Some(game_state);
                    self.show_menu = false;
                    
                    let dialog = Dialog::warning(
                        "Fallback Data".to_string(),
                        format!("MLB data failed to load ({}), using sample teams instead.", e),
                    );
                    self.dialog_manager.show_dialog(dialog);
                }
            }
        }
        Ok(())
    }

    fn show_load_game_dialog(&mut self) {
        // For now, just show an info dialog
        let dialog = Dialog::information(
            "Load Game".to_string(),
            "Load game functionality not yet implemented".to_string(),
        );
        self.dialog_manager.show_dialog(dialog);
    }

    fn handle_quit(&mut self) {
        if self.current_game.is_some() {
            let dialog = Dialog::confirmation(
                "Quit Game".to_string(),
                "Are you sure you want to quit? Unsaved progress will be lost.".to_string(),
            );
            self.dialog_manager.show_dialog(dialog);
        } else {
            self.is_running = false;
        }
    }

    fn handle_custom_action(&mut self, action: String) -> Result<()> {
        match action.as_str() {
            "difficulty" => {
                let dialog = Dialog::information(
                    "Difficulty".to_string(),
                    "Difficulty settings not yet implemented".to_string(),
                );
                self.dialog_manager.show_dialog(dialog);
            },
            "options" => {
                let dialog = Dialog::information(
                    "Game Options".to_string(),
                    "Game options not yet implemented".to_string(),
                );
                self.dialog_manager.show_dialog(dialog);
            },
            "mlb_analysis" => {
                let analysis = MLBTestData::analyze_player_conversion();
                let dialog = Dialog::information(
                    "MLB Data Analysis".to_string(),
                    analysis,
                );
                self.dialog_manager.show_dialog(dialog);
            },
            _ => {
                let dialog = Dialog::information(
                    "Not Implemented".to_string(),
                    format!("Feature '{}' not yet implemented", action),
                );
                self.dialog_manager.show_dialog(dialog);
            },
        }
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        self.terminal_ui.draw(|frame| {
            let size = frame.size();

            if self.show_menu {
                // Show menu overlay
                if let Some(ref game_state) = self.current_game {
                    // Show game in background
                    let layout = create_default_layout(size);
                    for window in layout.get_windows() {
                        self.window_manager.render_window(frame, window, game_state);
                    }
                }
                
                // Show menu on top
                self.menu_manager.show_menu_overlay(frame, size);
            } else if let Some(ref game_state) = self.current_game {
                // Show game
                let layout = create_default_layout(size);
                for window in layout.get_windows() {
                    self.window_manager.render_window(frame, window, game_state);
                }
            } else {
                // Show main menu
                self.menu_manager.render(frame, size);
            }

            // Always render dialogs on top
            self.dialog_manager.render(frame, size);
        })?;

        Ok(())
    }
}

fn main() -> Result<()> {
    // Initialize configuration directory
    ConfigPaths::ensure_config_dir()?;
    
    // Create and run the application
    let mut app = WildPitchApp::new()?;
    app.run()?;

    Ok(())
}
