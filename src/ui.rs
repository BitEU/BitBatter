use crate::game::{GameMode, GameState, InningHalf, PitchState, SwingTiming};
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
                    Constraint::Length(8),  // Scoreboard (increased from 7 to 8)
                    Constraint::Length(4),  // Timing display
                    Constraint::Min(8),     // Field (reduced to make room for timing)
                    Constraint::Length(5),  // Controls/Message
                ])
                .split(frame.area());

            render_scoreboard(frame, chunks[0], game_state);
            render_timing_display(frame, chunks[1], game_state);
            render_field(frame, chunks[2], game_state, input_state);
            render_controls(frame, chunks[3], game_state, engine);
        }
    }
}

fn render_team_selection(frame: &mut Frame, game_state: &GameState, selected_home: &Option<String>, selected_away: &Option<String>, input_buffer: &str, _input_mode: &crate::game::TeamInputMode) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Title
            Constraint::Min(10),     // Team selection
            Constraint::Length(5),   // Instructions
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("Team Selection")
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
            let team_name = game_state.team_manager.get_team_full_name(team_abbr);
            let style = if selected_away.as_ref() == Some(team_abbr) {
                Style::default().fg(Color::Black).bg(Color::Yellow)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("{}: {} - {}", idx + 1, team_abbr, team_name)).style(style)
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
            let team_name = game_state.team_manager.get_team_full_name(team_abbr);
            let style = if selected_home.as_ref() == Some(team_abbr) {
                Style::default().fg(Color::Black).bg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("{}: {} - {}", idx + 1, team_abbr, team_name)).style(style)
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
            InningHalf::Top => "^",
            InningHalf::Bottom => "v",
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

    let pitcher_info = if let Some(pitcher) = state.get_current_pitcher() {
        let pitching_team = state.get_current_pitching_team();
        let stamina = pitching_team.map(|t| t.pitcher_stamina).unwrap_or(100.0);
        let pitches = pitching_team.map(|t| t.pitches_thrown).unwrap_or(0);
        format!("Pitcher: {} | Stamina: {:.0}% | Pitches: {}", 
                pitcher.stats.name, stamina, pitches)
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
        Line::from(Span::styled(
            pitcher_info,
            Style::default().fg(Color::LightBlue),
        )),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Baseball Game")
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
    let r1 = if state.bases[0] { "*" } else { " " };  // 1st base
    let r2 = if state.bases[1] { "*" } else { " " };  // 2nd base
    let r3 = if state.bases[2] { "*" } else { " " };  // 3rd base

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

    // Calculate vertical centering
    let field_lines = 20; // Number of lines in the field art
    let available_height = area.height.saturating_sub(2) as usize; // Subtract 2 for borders
    let padding = if available_height > field_lines {
        (available_height - field_lines) / 2
    } else {
        0
    };

    // Add vertical padding for centering
    let centered_field = if padding > 0 {
        let mut lines = vec![String::new(); padding];
        lines.extend(field_art.lines().map(|s| s.to_string()));
        lines.join("\n")
    } else {
        field_art
    };

    // Color based on game state
    let style = match state.pitch_state {
        PitchState::Pitching { .. } => Style::default().fg(Color::Yellow),
        PitchState::Swinging { .. } => Style::default().fg(Color::Red),
        PitchState::BallInPlay { .. } | PitchState::Fielding { .. } => Style::default().fg(Color::Green),
        _ => Style::default().fg(Color::Cyan),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Diamond")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Green));

    let paragraph = Paragraph::new(centered_field)
        .block(block)
        .alignment(Alignment::Center)
        .style(style);

    frame.render_widget(paragraph, area);
}

fn render_strike_zone(frame: &mut Frame, area: Rect, state: &GameState, input_state: &crate::input::InputState) {
    // Determine what to show based on pitch state
    let (title, content_style) = match &state.pitch_state {
        PitchState::Aiming { .. } => ("[P] Pitcher Aim", Style::default().fg(Color::Yellow)),
        PitchState::WaitingForBatter => ("[B] Batter Aim", Style::default().fg(Color::Red)),
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
                    PitchState::Aiming { .. } => "+",  // Pitcher crosshair
                    PitchState::WaitingForBatter => "X",  // Batter crosshair
                    _ => ".",
                }
            } else {
                "."  // Empty zone
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
        PitchState::PitchClock { .. } => {
            "GET READY! Position yourself for the incoming pitch...  |  Q: quit".to_string()
        }
        PitchState::BallApproaching { can_swing, .. } => {
            if *can_swing {
                "⚡ SWING NOW! Use arrow keys + SPACE or SHIFT+(1-9) to swing!  |  Q: quit".to_string()
            } else {
                "⏳ Ball approaching... Get ready to swing!  |  Q: quit".to_string()
            }
        }
        PitchState::WaitingForBatter => {
            "BATTER: Use arrow keys to position, SPACE to swing  |  Q: quit".to_string()
        }
        PitchState::Pitching { .. } => "Pitching...".to_string(),
        PitchState::Swinging { .. } => "Swinging...".to_string(),
        PitchState::BallInPlay { .. } => "Ball in play!".to_string(),
        PitchState::Fielding { ball_in_play, frames_elapsed } => {
            let time_left = ball_in_play.hang_time.saturating_sub(*frames_elapsed);
            format!(
                "FIELDING: {:?} to {:?}! Time: {} frames - Press SPACE to field!  |  Q: quit",
                ball_in_play.ball_type, ball_in_play.direction, time_left
            )
        }
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

fn render_timing_display(frame: &mut Frame, area: Rect, state: &GameState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Timing");

    match &state.pitch_state {
        PitchState::PitchClock { frames_left, .. } => {
            let seconds_left = (*frames_left as f32 / 30.0).ceil() as u16;
            let clock_text = format!("PITCH CLOCK: {}s", seconds_left);
            
            // Create countdown bar
            let progress = 1.0 - (*frames_left as f32 / crate::game::constants::PITCH_CLOCK_FRAMES as f32);
            let bar_width = (area.width.saturating_sub(4)) as f32 * progress;
            let filled_chars = (bar_width as usize).min(area.width.saturating_sub(4) as usize);
            let empty_chars = (area.width.saturating_sub(4) as usize).saturating_sub(filled_chars);
            
            let clock_bar = format!("[{}{}]", 
                "=".repeat(filled_chars),
                "-".repeat(empty_chars)
            );
            
            let text = vec![
                Line::from(Span::styled(
                    clock_text,
                    Style::default().fg(if seconds_left <= 3 { Color::Red } else { Color::Yellow })
                        .add_modifier(Modifier::BOLD)
                )),
                Line::from(clock_bar),
            ];
            
            let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }
        PitchState::BallApproaching { frames_left, ball_position, can_swing, .. } => {
            // Ball approach visualization
            let ball_width = area.width.saturating_sub(4) as f32;
            let ball_pos = (*ball_position * ball_width) as usize;
            
            // Create ball position display
            let mut ball_display = vec![' '; ball_width as usize];
            if ball_pos < ball_display.len() {
                ball_display[ball_pos] = 'O';
            }
            
            // Timing window indicator
            let _timing_window_start = crate::game::constants::SWING_TIMING_WINDOW_FRAMES;
            let perfect_window = crate::game::constants::PERFECT_TIMING_WINDOW_FRAMES;
            
            let timing_info = if *can_swing {
                if *frames_left <= perfect_window {
                    "⚡ PERFECT TIMING! ⚡"
                } else {
                    "🎯 Swing Zone Active"
                }
            } else {
                "⏳ Ball Approaching..."
            };
            
            let ball_track = ball_display.iter().collect::<String>();
            
            let text = vec![
                Line::from(Span::styled(
                    timing_info,
                    Style::default().fg(if *can_swing { Color::Green } else { Color::Cyan })
                        .add_modifier(Modifier::BOLD)
                )),
                Line::from(format!("Mound [{}] Plate", ball_track)),
            ];
            
            let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }
        PitchState::Swinging { swing_timing, .. } => {
            let timing_text = match swing_timing {
                SwingTiming::TooEarly => "❌ TOO EARLY!",
                SwingTiming::Early => "⚠️  EARLY",
                SwingTiming::Perfect => "⚡ PERFECT! ⚡",
                SwingTiming::Late => "⚠️  LATE",
                SwingTiming::TooLate => "❌ TOO LATE!",
                SwingTiming::NoSwing => "👀 NO SWING",
            };
            
            let color = match swing_timing {
                SwingTiming::Perfect => Color::Green,
                SwingTiming::Early | SwingTiming::Late => Color::Yellow,
                SwingTiming::TooEarly | SwingTiming::TooLate => Color::Red,
                SwingTiming::NoSwing => Color::Blue,
            };
            
            let text = vec![
                Line::from(Span::styled(
                    timing_text,
                    Style::default().fg(color).add_modifier(Modifier::BOLD)
                )),
            ];
            
            let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }
        _ => {
            // Default display for other states
            let text = vec![
                Line::from("Ready to pitch..."),
            ];
            
            let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }
    }
}