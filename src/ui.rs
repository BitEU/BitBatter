use crate::game::{GameMode, GameState, InningHalf, PitchState};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render_game(frame: &mut Frame, game_state: &GameState, engine: &crate::game::GameEngine, input_state: &crate::input::InputState) {
    match &game_state.mode {
        GameMode::TeamSelection { selected_home, selected_away, input_buffer, input_mode } => {
            render_team_selection(frame, game_state, selected_home, selected_away, input_buffer, input_mode);
        }
        GameMode::Playing => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(7),  // Scoreboard
                    Constraint::Min(12),    // Field
                    Constraint::Length(5),  // Controls/Message
                ])
                .split(frame.area());

            render_scoreboard(frame, chunks[0], game_state);
            render_field(frame, chunks[1], game_state, input_state);
            render_controls(frame, chunks[2], game_state, engine);
        }
    }
}

fn render_team_selection(frame: &mut Frame, game_state: &GameState, selected_home: &Option<String>, selected_away: &Option<String>, input_buffer: &str, input_mode: &crate::game::TeamInputMode) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Title
            Constraint::Min(10),     // Team selection
            Constraint::Length(5),   // Instructions
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("⚾ Team Selection ⚾")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Team selection
    let team_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

    // Away team selection
    let away_teams: Vec<ListItem> = game_state.team_manager.get_team_list()
        .iter()
        .enumerate()
        .map(|(idx, team_abbr)| {
            let team = game_state.team_manager.get_team(team_abbr).unwrap();
            let style = if selected_away.as_ref() == Some(team_abbr) {
                Style::default().fg(Color::Black).bg(Color::Yellow)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("{}: {} - {}", idx + 1, team_abbr, team.name)).style(style)
        })
        .collect();

    let away_list = List::new(away_teams)
        .block(Block::default()
            .title("Away Team (Press A + Number)")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue)));
    frame.render_widget(away_list, team_chunks[0]);

    // Home team selection
    let home_teams: Vec<ListItem> = game_state.team_manager.get_team_list()
        .iter()
        .enumerate()
        .map(|(idx, team_abbr)| {
            let team = game_state.team_manager.get_team(team_abbr).unwrap();
            let style = if selected_home.as_ref() == Some(team_abbr) {
                Style::default().fg(Color::Black).bg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("{}: {} - {}", idx + 1, team_abbr, team.name)).style(style)
        })
        .collect();

    let home_list = List::new(home_teams)
        .block(Block::default()
            .title("Home Team (Press H + Number)")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red)));
    frame.render_widget(home_list, team_chunks[1]);

    // Instructions
    let mut instructions = vec![
        Line::from("Press A then enter team # (1-30) and ENTER | Press H then enter team # (1-30) and ENTER"),
    ];
    
    if !input_buffer.is_empty() {
        instructions.push(Line::from(Span::styled(
            format!("Current input: {} (press ENTER to confirm)", input_buffer),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        )));
    }
    
    if selected_home.is_some() && selected_away.is_some() && input_buffer.is_empty() {
        instructions.push(Line::from(Span::styled(
            "Press SPACE or ENTER to start the game!",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        )));
    }

    let instruction_paragraph = Paragraph::new(instructions)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Instructions"));
    frame.render_widget(instruction_paragraph, chunks[2]);
}

fn render_scoreboard(frame: &mut Frame, area: Rect, state: &GameState) {
    let inning_text = format!(
        "Inning: {} {}",
        state.inning,
        match state.half {
            InningHalf::Top => "▲",
            InningHalf::Bottom => "▼",
        }
    );

    let score_text = format!(
        "Away: {:2}  Home: {:2}",
        state.away_score, state.home_score
    );

    let count_text = format!(
        "Balls: {}  Strikes: {}  Outs: {}",
        state.balls, state.strikes, state.outs
    );

    let batter_info = if let Some(batter) = state.get_current_batter() {
        format!("Batter: {} ({})", batter.stats.name, batter.position.name())
    } else {
        format!("Batter #{} - {}", state.current_batter_idx + 1, state.batting_team())
    };

    let _pitcher_info = if let Some(pitcher) = state.get_current_pitcher() {
        format!("Pitcher: {}", pitcher.stats.name)
    } else {
        "Pitcher: Unknown".to_string()
    };

    let team_names = format!(
        "{} @ {}",
        state.away_team.as_ref().map(|t| state.team_manager.get_team(t).map(|team| team.name.as_str()).unwrap_or(t)).unwrap_or("Away"),
        state.home_team.as_ref().map(|t| state.team_manager.get_team(t).map(|team| team.name.as_str()).unwrap_or(t)).unwrap_or("Home")
    );

    let scoreboard = vec![
        Line::from(Span::styled(
            team_names,
            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            inning_text,
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            score_text,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            count_text,
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            batter_info,
            Style::default().fg(Color::Green),
        )),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title("⚾ Baseball Game ⚾")
        .title_alignment(Alignment::Center);

    let paragraph = Paragraph::new(scoreboard)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_field(frame: &mut Frame, area: Rect, state: &GameState, input_state: &crate::input::InputState) {
    // Split field area to show field + strike zone side by side
    let field_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),  // Field
            Constraint::Percentage(40),  // Strike zone with aiming
        ])
        .split(area);

    // Render the baseball field
    render_baseball_field(frame, field_chunks[0], state);

    // Render strike zone with aiming indicator
    render_strike_zone(frame, field_chunks[1], state, input_state);
}

fn render_baseball_field(frame: &mut Frame, area: Rect, state: &GameState) {
    // Professional ASCII baseball field
    // Credit: https://github.com/ceejay3264/ascii_baseball

    // Dynamic runner indicators - show filled circle if runner present
    let r1 = if state.bases[0] { "●" } else { " " };  // 1st base
    let r2 = if state.bases[1] { "●" } else { " " };  // 2nd base
    let r3 = if state.bases[2] { "●" } else { " " };  // 3rd base

    // Build the field with dynamic runners
    let field_art = format!(
        " __________________________                \n\
|                          \\___            \n\
|                              \\_          \n\
|          O                     \\__       \n\
|                                   \\_     \n\
|                                     \\    \n\
|                                      \\   \n\
| _ _ _ _ _ _ _ _ _            O        \\  \n\
|/                 \\_                    \\ \n\
|   O         O       \\_                  |\n\
|                       \\                 |\n\
|[{}]            [{}]      \\                |\n\
|      _______           |                |\n\
|     /       \\      O   |                |\n\
|     |    \\   \\         |                |\n\
|     \\ O      /         |       O        |\n\
|      \\______/          |                |\n\
|                    O   |                |\n\
|[{}]            [{}]      |                |\n\
|_______________________/_________________|",
        r3, r2,  // Line 12: 3rd base, then 2nd base
        r3, r1   // Line 19: duplicated for dugout view
    );

    // Color based on game state
    let style = match state.pitch_state {
        PitchState::Pitching { .. } => Style::default().fg(Color::Yellow),
        PitchState::Swinging { .. } => Style::default().fg(Color::Red),
        PitchState::BallInPlay { .. } => Style::default().fg(Color::Green),
        _ => Style::default().fg(Color::Cyan),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title("⚾ Diamond ⚾")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Green));

    let paragraph = Paragraph::new(field_art)
        .block(block)
        .style(style);

    frame.render_widget(paragraph, area);
}

fn render_strike_zone(frame: &mut Frame, area: Rect, state: &GameState, input_state: &crate::input::InputState) {
    // Determine what to show based on pitch state
    let (title, content_style) = match &state.pitch_state {
        PitchState::Aiming { .. } => ("🎯 Pitcher Aim", Style::default().fg(Color::Yellow)),
        PitchState::WaitingForBatter => ("🏏 Batter Aim", Style::default().fg(Color::Red)),
        _ => ("Strike Zone", Style::default().fg(Color::Gray)),
    };

    // Calculate aim position (9-zone grid)
    // Center = no input, arrows move from center
    let (aim_row, aim_col) = if input_state.up && input_state.left {
        (0, 0)  // Top-left
    } else if input_state.up && input_state.right {
        (0, 2)  // Top-right
    } else if input_state.up {
        (0, 1)  // Top-center
    } else if input_state.down && input_state.left {
        (2, 0)  // Bottom-left
    } else if input_state.down && input_state.right {
        (2, 2)  // Bottom-right
    } else if input_state.down {
        (2, 1)  // Bottom-center
    } else if input_state.left {
        (1, 0)  // Middle-left
    } else if input_state.right {
        (1, 2)  // Middle-right
    } else {
        (1, 1)  // Center
    };

    // Build strike zone grid
    let mut zone_lines = vec![];

    // Add title info
    zone_lines.push(Line::from(""));
    zone_lines.push(Line::from(Span::styled(
        "Strike Zone:",
        Style::default().add_modifier(Modifier::BOLD),
    )));
    zone_lines.push(Line::from(""));

    // Build 3x3 grid
    for row in 0..3 {
        let mut cells = vec![];
        for col in 0..3 {
            let symbol = if row == aim_row && col == aim_col {
                // Show crosshair at aim position
                match &state.pitch_state {
                    PitchState::Aiming { .. } => "⊕",  // Pitcher crosshair
                    PitchState::WaitingForBatter => "⊗",  // Batter crosshair
                    _ => "·",
                }
            } else {
                "·"  // Empty zone
            };

            cells.push(Span::styled(
                format!(" {} ", symbol),
                if row == aim_row && col == aim_col {
                    content_style.add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                },
            ));
        }
        zone_lines.push(Line::from(cells));
    }

    zone_lines.push(Line::from(""));

    // Add legend based on state
    if matches!(state.pitch_state, PitchState::Aiming { .. } | PitchState::WaitingForBatter) {
        zone_lines.push(Line::from(Span::styled(
            "Use arrow keys to aim",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::ITALIC),
        )));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_alignment(Alignment::Center)
        .border_style(content_style);

    let paragraph = Paragraph::new(zone_lines)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_controls(frame: &mut Frame, area: Rect, state: &GameState, engine: &crate::game::GameEngine) {
    let controls = match &state.pitch_state {
        PitchState::ChoosePitch => {
            let pitches: Vec<String> = engine
                .pitch_types
                .iter()
                .enumerate()
                .map(|(i, p)| format!("{}: {}", i + 1, p.name))
                .collect();
            format!(
                "Choose Pitch: {}  |  Press Q to quit",
                pitches.join(" | ")
            )
        }
        PitchState::Aiming { pitch_type } => {
            format!(
                "Aiming {} - Use arrow keys to aim, SPACE to pitch  |  Q: quit",
                engine.get_pitch_name(*pitch_type)
            )
        }
        PitchState::WaitingForBatter => {
            "BATTER: Use arrow keys to position, SPACE to swing  |  Q: quit".to_string()
        }
        PitchState::Pitching { .. } => "Pitching...".to_string(),
        PitchState::Swinging { .. } => "Swinging...".to_string(),
        PitchState::BallInPlay { .. } => "Ball in play!".to_string(),
        PitchState::ShowResult { .. } => "Press SPACE to continue  |  Q: quit".to_string(),
    };

    let message_line = Line::from(vec![
        Span::styled(
            "Message: ",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ),
        Span::styled(&state.message, Style::default().fg(Color::White)),
    ]);

    let text = vec![message_line, Line::from(controls)];

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Controls");

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}