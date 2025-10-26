use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum GameInput {
    Up,
    Down,
    Left,
    Right,
    Action,  // Space or Enter - context dependent (pitch/swing)
    SelectPitch(usize),
    SelectAwayTeam,
    SelectHomeTeam,
    NumberInput(char),
    Pause,
    Quit,
    DirectPosition(u8), // Numpad 1-9 for direct strike zone selection
}

/// Input mode state for team selection
#[derive(Debug, Clone, PartialEq)]
pub enum TeamSelectionInputMode {
    None,
    AwaitingAwayNumber,
    AwaitingHomeNumber,
}

pub struct InputPoller {
    team_selection_mode: TeamSelectionInputMode,
}

impl InputPoller {
    pub fn new() -> Self {
        Self {
            team_selection_mode: TeamSelectionInputMode::None,
        }
    }

    pub fn poll_input(&mut self, poll_timeout_ms: u64) -> Result<Option<GameInput>, std::io::Error> {
        if event::poll(Duration::from_millis(poll_timeout_ms))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == crossterm::event::KeyEventKind::Press {
                    return Ok(self.parse_key_input(key_event));
                }
            }
        }
        Ok(None)
    }

    fn parse_key_input(&mut self, key_event: KeyEvent) -> Option<GameInput> {
        // Check if we're waiting for a number after A or H
        match &self.team_selection_mode {
            TeamSelectionInputMode::AwaitingAwayNumber | TeamSelectionInputMode::AwaitingHomeNumber => {
                if let KeyCode::Char(c) = key_event.code {
                    if c.is_ascii_digit() {
                        return Some(GameInput::NumberInput(c));
                    } else if c == '\r' || c == '\n' {
                        self.team_selection_mode = TeamSelectionInputMode::None;
                        return Some(GameInput::Action);
                    }
                } else if key_event.code == KeyCode::Enter {
                    self.team_selection_mode = TeamSelectionInputMode::None;
                    return Some(GameInput::Action);
                } else if key_event.code == KeyCode::Esc {
                    self.team_selection_mode = TeamSelectionInputMode::None;
                    return None;
                }
                return None;
            }
            TeamSelectionInputMode::None => {
                // Normal input processing
            }
        }

        match key_event.code {
            KeyCode::Up => Some(GameInput::Up),
            KeyCode::Down => Some(GameInput::Down),
            KeyCode::Left => Some(GameInput::Left),
            KeyCode::Right => Some(GameInput::Right),
            KeyCode::Char(' ') | KeyCode::Enter => Some(GameInput::Action),
            KeyCode::Char('q') | KeyCode::Char('Q') => Some(GameInput::Quit),
            KeyCode::Esc => Some(GameInput::Pause),
            
            // Regular number keys (1-4) for pitch selection
            KeyCode::Char(c) if c >= '1' && c <= '4' && !key_event.modifiers.contains(KeyModifiers::SHIFT) => {
                let num = c.to_digit(10).unwrap() as usize;
                Some(GameInput::SelectPitch(num - 1))
            }
            
            // SHIFT + number keys (1-9) for direct aiming (simulates numpad)
            KeyCode::Char(c) if c >= '1' && c <= '9' && key_event.modifiers.contains(KeyModifiers::SHIFT) => {
                let num = c.to_digit(10).unwrap() as u8;
                Some(GameInput::DirectPosition(num))
            }
            
            // Handle A for away team selection
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.team_selection_mode = TeamSelectionInputMode::AwaitingAwayNumber;
                Some(GameInput::SelectAwayTeam)
            }
            
            // Handle H for home team selection
            KeyCode::Char('h') | KeyCode::Char('H') => {
                self.team_selection_mode = TeamSelectionInputMode::AwaitingHomeNumber;
                Some(GameInput::SelectHomeTeam)
            }
            
            _ => None,
        }
    }
}

pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn update(&mut self, input: &GameInput) {
        match input {
            GameInput::Up => self.up = true,
            GameInput::Down => self.down = true,
            GameInput::Left => self.left = true,
            GameInput::Right => self.right = true,
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.up = false;
        self.down = false;
        self.left = false;
        self.right = false;
    }
}

