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
}

pub fn poll_input() -> Result<Option<GameInput>, std::io::Error> {
    // Non-blocking poll with 16ms timeout (~60fps)
    if event::poll(Duration::from_millis(16))? {
        if let Event::Key(key_event) = event::read()? {
            // Only process key press events (not release)
            if key_event.kind == crossterm::event::KeyEventKind::Press {
                return Ok(parse_key_input(key_event));
            }
        }
    }
    Ok(None)
}

fn parse_key_input(key_event: KeyEvent) -> Option<GameInput> {
    match key_event.code {
        KeyCode::Up => Some(GameInput::Up),
        KeyCode::Down => Some(GameInput::Down),
        KeyCode::Left => Some(GameInput::Left),
        KeyCode::Right => Some(GameInput::Right),
        KeyCode::Char(' ') | KeyCode::Enter => Some(GameInput::Action),
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(GameInput::Quit),
        KeyCode::Esc => Some(GameInput::Pause),
        
        // Team selection - A + number for away team
        KeyCode::Char(c) if c >= '1' && c <= '9' => {
            if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                // Shift + number for away team (A is shift+a)
                None // We'll handle this differently
            } else {
                let num = c.to_digit(10).unwrap() as usize;
                Some(GameInput::SelectPitch(num - 1))
            }
        }
        
        // Handle A + numbers for away team selection
        KeyCode::Char('a') | KeyCode::Char('A') => None, // Modifier key, wait for number
        KeyCode::Char('h') | KeyCode::Char('H') => None, // Modifier key, wait for number
        
        _ => None,
    }
}

pub fn poll_input_with_modifiers() -> Result<Option<GameInput>, std::io::Error> {
    static mut AWAITING_AWAY_NUM: bool = false;
    static mut AWAITING_HOME_NUM: bool = false;
    
    if event::poll(Duration::from_millis(16))? {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == crossterm::event::KeyEventKind::Press {
                unsafe {
                    // Check if we're waiting for a number after A or H
                    if AWAITING_AWAY_NUM {
                        if let KeyCode::Char(c) = key_event.code {
                            if c.is_ascii_digit() {
                                return Ok(Some(GameInput::NumberInput(c)));
                            } else if c == '\r' || c == '\n' {
                                AWAITING_AWAY_NUM = false;
                                return Ok(Some(GameInput::Action));
                            }
                        } else if key_event.code == KeyCode::Enter {
                            AWAITING_AWAY_NUM = false;
                            return Ok(Some(GameInput::Action));
                        } else if key_event.code == KeyCode::Esc {
                            AWAITING_AWAY_NUM = false;
                            return Ok(None);
                        }
                        return Ok(None);
                    }
                    
                    if AWAITING_HOME_NUM {
                        if let KeyCode::Char(c) = key_event.code {
                            if c.is_ascii_digit() {
                                return Ok(Some(GameInput::NumberInput(c)));
                            } else if c == '\r' || c == '\n' {
                                AWAITING_HOME_NUM = false;
                                return Ok(Some(GameInput::Action));
                            }
                        } else if key_event.code == KeyCode::Enter {
                            AWAITING_HOME_NUM = false;
                            return Ok(Some(GameInput::Action));
                        } else if key_event.code == KeyCode::Esc {
                            AWAITING_HOME_NUM = false;
                            return Ok(None);
                        }
                        return Ok(None);
                    }
                    
                    // Check for A or H key press
                    match key_event.code {
                        KeyCode::Char('a') | KeyCode::Char('A') => {
                            AWAITING_AWAY_NUM = true;
                            return Ok(Some(GameInput::SelectAwayTeam));
                        }
                        KeyCode::Char('h') | KeyCode::Char('H') => {
                            AWAITING_HOME_NUM = true;
                            return Ok(Some(GameInput::SelectHomeTeam));
                        }
                        _ => return Ok(parse_key_input(key_event)),
                    }
                }
            }
        }
    }
    Ok(None)
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
