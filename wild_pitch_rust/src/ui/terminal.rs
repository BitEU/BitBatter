use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io::{self, Stdout};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerminalError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Terminal setup failed")]
    Setup,
}

pub type Result<T> = std::result::Result<T, TerminalError>;
pub type TerminalBackend = CrosstermBackend<Stdout>;
pub type WildPitchTerminal = Terminal<TerminalBackend>;

pub struct TerminalUI {
    terminal: WildPitchTerminal,
}

impl TerminalUI {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(Self { terminal })
    }

    pub fn size(&self) -> Rect {
        self.terminal.size().unwrap_or_default()
    }

    pub fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Frame),
    {
        self.terminal.draw(f)?;
        Ok(())
    }

    pub fn poll_event() -> Result<Option<Event>> {
        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            // Only process key press events, not key release events
            if let Event::Key(key_event) = &event {
                if key_event.kind == KeyEventKind::Press {
                    Ok(Some(event))
                } else {
                    // Skip key release events to prevent double registration
                    Ok(None)
                }
            } else {
                Ok(Some(event))
            }
        } else {
            Ok(None)
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        self.terminal.clear()?;
        Ok(())
    }
}

impl Drop for TerminalUI {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindowType {
    Scoreboard,
    LineupCards,
    PlayByPlay,
    Ballpark,
    BattingStats,
    PitchingStats,
    TeamStats,
    BoxScore,
    Scorecard,
    Menu,
    Dialog,
}

#[derive(Clone, Debug)]
pub struct WindowLayout {
    pub rect: Rect,
    pub window_type: WindowType,
    pub title: String,
    pub border_style: Style,
    pub is_active: bool,
}

impl WindowLayout {
    pub fn new(rect: Rect, window_type: WindowType, title: String) -> Self {
        Self {
            rect,
            window_type,
            title,
            border_style: Style::default().fg(Color::White),
            is_active: false,
        }
    }

    pub fn with_active(mut self, active: bool) -> Self {
        self.is_active = active;
        self.border_style = if active {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        self
    }

    pub fn block(&self) -> Block {
        Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL)
            .border_style(self.border_style)
    }
}

pub struct LayoutManager {
    size: Rect,
    windows: Vec<WindowLayout>,
    active_window: usize,
}

impl LayoutManager {
    pub fn new(size: Rect) -> Self {
        Self {
            size,
            windows: Vec::new(),
            active_window: 0,
        }
    }

    pub fn update_size(&mut self, size: Rect) {
        self.size = size;
        self.calculate_layout();
    }

    pub fn add_window(&mut self, window_type: WindowType, title: String) {
        let rect = Rect::new(0, 0, 10, 10); // Placeholder, will be recalculated
        self.windows.push(WindowLayout::new(rect, window_type, title));
        self.calculate_layout();
    }

    pub fn get_windows(&self) -> &[WindowLayout] {
        &self.windows
    }

    pub fn get_active_window(&self) -> Option<&WindowLayout> {
        self.windows.get(self.active_window)
    }

    pub fn set_active_window(&mut self, index: usize) {
        if index < self.windows.len() {
            self.active_window = index;
            self.update_active_states();
        }
    }

    pub fn next_window(&mut self) {
        if !self.windows.is_empty() {
            self.active_window = (self.active_window + 1) % self.windows.len();
            self.update_active_states();
        }
    }

    pub fn previous_window(&mut self) {
        if !self.windows.is_empty() {
            self.active_window = if self.active_window == 0 {
                self.windows.len() - 1
            } else {
                self.active_window - 1
            };
            self.update_active_states();
        }
    }

    fn update_active_states(&mut self) {
        for (i, window) in self.windows.iter_mut().enumerate() {
            *window = window.clone().with_active(i == self.active_window);
        }
    }

    fn calculate_layout(&mut self) {
        if self.windows.is_empty() {
            return;
        }

        // Create the main Wild Pitch layout matching the original
        // This is a simplified version - we'll enhance it as we add more features
        
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Menu bar
                Constraint::Min(0),     // Main content
                Constraint::Length(1),  // Status line
            ])
            .split(self.size);

        let content_area = main_layout[1];
        
        // Split content area into main windows like original Wild Pitch
        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60), // Left side (Play-by-play, Ballpark)
                Constraint::Percentage(40), // Right side (Scoreboard, Lineup)
            ])
            .split(content_area);

        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40), // Ballpark
                Constraint::Percentage(60), // Play-by-play
            ])
            .split(content_layout[0]);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40), // Scoreboard
                Constraint::Percentage(60), // Lineup Cards
            ])
            .split(content_layout[1]);

        // Assign rectangles to windows based on their type
        for window in &mut self.windows {
            window.rect = match window.window_type {
                WindowType::Ballpark => left_layout[0],
                WindowType::PlayByPlay => left_layout[1],
                WindowType::Scoreboard => right_layout[0],
                WindowType::LineupCards => right_layout[1],
                WindowType::Menu => main_layout[0],
                _ => content_area, // Other windows will overlay the main content
            };
        }

        self.update_active_states();
    }
}

pub fn create_default_layout(size: Rect) -> LayoutManager {
    let mut layout = LayoutManager::new(size);
    
    // Add the main Wild Pitch windows
    layout.add_window(WindowType::Ballpark, "Ballpark".to_string());
    layout.add_window(WindowType::PlayByPlay, "Play-by-Play".to_string());
    layout.add_window(WindowType::Scoreboard, "Scoreboard".to_string());
    layout.add_window(WindowType::LineupCards, "Lineup Cards".to_string());
    
    layout
}