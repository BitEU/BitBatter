use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::game::GameState;
use super::{WindowLayout, WindowType};

pub trait WindowRenderer {
    fn render(&self, frame: &mut Frame, layout: &WindowLayout, game_state: &GameState);
}

pub struct ScoreboardWindow;

impl WindowRenderer for ScoreboardWindow {
    fn render(&self, frame: &mut Frame, layout: &WindowLayout, game_state: &GameState) {
        let block = layout.block();
        
        // Create a simple scoreboard display
        let scoreboard_text = vec![
            Line::from(vec![
                Span::styled("Inning: ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("{} {}", 
                        if game_state.is_top_inning() { "T" } else { "B" },
                        game_state.inning()
                    ),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(vec![
                Span::styled("Outs: ", Style::default().fg(Color::White)),
                Span::styled(
                    game_state.outs().to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Visitors: ", Style::default().fg(Color::White)),
                Span::styled(
                    game_state.visitor_score().to_string(),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(vec![
                Span::styled("Home: ", Style::default().fg(Color::White)),
                Span::styled(
                    game_state.home_score().to_string(),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                ),
            ]),
        ];

        let paragraph = Paragraph::new(scoreboard_text)
            .block(block)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, layout.rect);
    }
}

pub struct LineupCardsWindow;

impl WindowRenderer for LineupCardsWindow {
    fn render(&self, frame: &mut Frame, layout: &WindowLayout, game_state: &GameState) {
        let block = layout.block();
        
        let lineup_text = vec![
            Line::from(vec![
                Span::styled("LINEUP CARDS", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from("Visitors:"),
            Line::from("1. Player 1 - 2B"),
            Line::from("2. Player 2 - SS"),
            Line::from("3. Player 3 - CF"),
            Line::from("4. Player 4 - 1B"),
            Line::from("5. Player 5 - LF"),
            Line::from("6. Player 6 - RF"),
            Line::from("7. Player 7 - 3B"),
            Line::from("8. Player 8 - C"),
            Line::from("9. Player 9 - P"),
            Line::from(""),
            Line::from("Home:"),
            Line::from("1. Player A - CF"),
            Line::from("2. Player B - 2B"),
            Line::from("3. Player C - RF"),
            Line::from("4. Player D - 1B"),
            Line::from("5. Player E - 3B"),
            Line::from("6. Player F - LF"),
            Line::from("7. Player G - SS"),
            Line::from("8. Player H - C"),
            Line::from("9. Player I - P"),
        ];

        let paragraph = Paragraph::new(lineup_text)
            .block(block)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, layout.rect);
    }
}

pub struct PlayByPlayWindow;

impl WindowRenderer for PlayByPlayWindow {
    fn render(&self, frame: &mut Frame, layout: &WindowLayout, game_state: &GameState) {
        let block = layout.block();
        
        let play_items: Vec<ListItem> = game_state.play_by_play
            .iter()
            .map(|play| ListItem::new(play.clone()))
            .collect();

        let play_list = List::new(play_items)
            .block(block)
            .highlight_style(Style::default().fg(Color::Yellow));

        frame.render_widget(play_list, layout.rect);
    }
}

pub struct BallparkWindow;

impl WindowRenderer for BallparkWindow {
    fn render(&self, frame: &mut Frame, layout: &WindowLayout, game_state: &GameState) {
        let block = layout.block();
        
        // ASCII art representation of baseball field
        let field_art = vec![
            Line::from("                 ⚾ BALLPARK ⚾"),
            Line::from(""),
            Line::from("                    🏟️"),
            Line::from("              CF    👤    RF"),
            Line::from("          👤              👤"),
            Line::from("                    LF"),
            Line::from("              👤        "),
            Line::from("                    "),
            Line::from("      3B    👤    2B    👤    1B"),
            Line::from("        👤         SS         👤"),
            Line::from("                 👤"),
            Line::from("                    "),
            Line::from("                 👤  P"),
            Line::from("                    "),
            Line::from("                 C"),
            Line::from("                👤"),
            Line::from("              🏠 HOME 🏠"),
        ];

        let paragraph = Paragraph::new(field_art)
            .block(block)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, layout.rect);
    }
}

pub struct WindowManager {
    scoreboard: ScoreboardWindow,
    lineup_cards: LineupCardsWindow,
    play_by_play: PlayByPlayWindow,
    ballpark: BallparkWindow,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            scoreboard: ScoreboardWindow,
            lineup_cards: LineupCardsWindow,
            play_by_play: PlayByPlayWindow,
            ballpark: BallparkWindow,
        }
    }

    pub fn render_window(
        &self,
        frame: &mut Frame,
        layout: &WindowLayout,
        game_state: &GameState,
    ) {
        match layout.window_type {
            WindowType::Scoreboard => self.scoreboard.render(frame, layout, game_state),
            WindowType::LineupCards => self.lineup_cards.render(frame, layout, game_state),
            WindowType::PlayByPlay => self.play_by_play.render(frame, layout, game_state),
            WindowType::Ballpark => self.ballpark.render(frame, layout, game_state),
            _ => {
                // Placeholder for other window types
                let block = layout.block();
                let placeholder = Paragraph::new(format!("Window: {}", layout.title))
                    .block(block)
                    .alignment(Alignment::Center);
                frame.render_widget(placeholder, layout.rect);
            }
        }
    }
}