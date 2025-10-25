use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum GameInput {
    Up,
    Down,
    Left,
    Right,
    Action,  // Space or Enter - context dependent (pitch/swing)
    SelectPitch(usize),
    Pause,
    Quit,
}

pub fn poll_input() -> Result<Option<GameInput>, std::io::Error> {
    // Non-blocking poll with 16ms timeout (~60fps)
    if event::poll(Duration::from_millis(16))? {
        if let Event::Key(key_event) = event::read()? {
            // Only process key press events (not release)
            if key_event.kind == crossterm::event::KeyEventKind::Press {
                return Ok(match key_event.code {
                    KeyCode::Up => Some(GameInput::Up),
                    KeyCode::Down => Some(GameInput::Down),
                    KeyCode::Left => Some(GameInput::Left),
                    KeyCode::Right => Some(GameInput::Right),
                    KeyCode::Char(' ') | KeyCode::Enter => Some(GameInput::Action),
                    KeyCode::Char('1') => Some(GameInput::SelectPitch(0)),
                    KeyCode::Char('2') => Some(GameInput::SelectPitch(1)),
                    KeyCode::Char('3') => Some(GameInput::SelectPitch(2)),
                    KeyCode::Char('4') => Some(GameInput::SelectPitch(3)),
                    KeyCode::Char('q') | KeyCode::Char('Q') => Some(GameInput::Quit),
                    KeyCode::Esc => Some(GameInput::Pause),
                    _ => None,
                });
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
