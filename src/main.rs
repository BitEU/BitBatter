mod game;
mod input;
mod ui;
mod team;
mod audio;
mod logger;

use audio::AudioPlayer;
use logger::GameLogger;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::{GameEngine, GameMode, GameState, HitType, OutType, PitchLocation, PitchState, PlayResult};
use input::{GameInput, InputState};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Hide cursor to prevent flicker
    terminal.hide_cursor()?;

    // Run game with proper error handling
    let res = run_game(&mut terminal);

    // ALWAYS restore terminal - even on panic
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_game(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut game_state = GameState::new();
    let engine = GameEngine::new();
    let mut input_state = InputState::new();
    let audio_player = AudioPlayer::new();
    let logger = GameLogger::new();
    
    let mut pitch_count = 0u32;
    let mut inning_hits = 0u8;

    let target_fps = 30;
    let frame_time = Duration::from_millis(1000 / target_fps);

    loop {
        let frame_start = Instant::now();

        // Handle input
        if let Some(input) = input::poll_input_with_modifiers()? {
            if input == GameInput::Quit {
                break;
            }
            handle_input(&mut game_state, &engine, &mut input_state, input, audio_player.as_ref(), &logger);
        }

        // Update game logic (animations, etc.)
        update_game_state(&mut game_state, &engine, &mut input_state, audio_player.as_ref(), &logger, &mut pitch_count, &mut inning_hits);

        // Render ONCE per frame - critical for no flicker!
        terminal.draw(|frame| {
            ui::render_game(frame, &game_state, &engine, &input_state);
        })?;

        // Frame rate limiting to prevent CPU spam
        let elapsed = frame_start.elapsed();
        if elapsed < frame_time {
            thread::sleep(frame_time - elapsed);
        }

        // Exit if game is over
        if game_state.game_over && matches!(game_state.pitch_state, PitchState::ShowResult { .. }) {
            thread::sleep(Duration::from_secs(3));
            break;
        }
    }

    Ok(())
}

fn handle_input(
    state: &mut GameState,
    engine: &GameEngine,
    input_state: &mut InputState,
    input: GameInput,
    audio_player: Option<&AudioPlayer>,
    logger: &GameLogger,
) {
    // Handle team selection first
    if let GameMode::TeamSelection { .. } = &state.mode {
        handle_team_selection_input(state, input);
        return;
    }

    match &state.pitch_state {
        PitchState::ChoosePitch => {
            if let GameInput::SelectPitch(idx) = input {
                if idx < engine.pitch_types.len() {
                    state.pitch_state = PitchState::Aiming { pitch_type: idx };
                    state.message = format!(
                        "Aiming {}. Use arrows to aim, SPACE to pitch.",
                        engine.get_pitch_name(idx)
                    );
                    input_state.reset();
                }
            }
        }
        PitchState::Aiming { pitch_type: _ } => {
            match input {
                GameInput::Up | GameInput::Down | GameInput::Left | GameInput::Right => {
                    input_state.update(&input);
                }
                GameInput::Action => {
                    // Lock in pitch location
                    let location = PitchLocation::from_direction(
                        input_state.up,
                        input_state.down,
                        input_state.left,
                        input_state.right,
                    );
                    state.pitch_location = Some(location);
                    state.pitch_state = PitchState::Pitching { frames_left: 20 };
                    state.message = "Pitch released!".to_string();
                    input_state.reset();
                }
                _ => {}
            }
        }
        PitchState::WaitingForBatter => {
            match input {
                GameInput::Up | GameInput::Down | GameInput::Left | GameInput::Right => {
                    input_state.update(&input);
                }
                GameInput::Action => {
                    // Batter swings
                    let swing_loc = PitchLocation::from_direction(
                        input_state.up,
                        input_state.down,
                        input_state.left,
                        input_state.right,
                    );
                    state.swing_location = Some(swing_loc);
                    state.pitch_state = PitchState::Swinging { frames_left: 10 };
                    state.message = "Swing!".to_string();
                    input_state.reset();
                }
                _ => {}
            }
        }
        PitchState::Fielding { .. } => {
            // Handle fielding input - move fielder and attempt catch
            match input {
                GameInput::Action => {
                    // Attempt to catch/field the ball
                    if let PitchState::Fielding { ball_in_play, frames_elapsed } = &state.pitch_state {
                        let perfect_timing = ball_in_play.hang_time / 2;
                        let (result, success_chance) = engine.calculate_fielding_result(
                            ball_in_play,
                            *frames_elapsed,
                            perfect_timing,
                        );
                        
                        // Log fielding attempt
                        logger.log_fielding_attempt(
                            ball_in_play,
                            *frames_elapsed,
                            perfect_timing,
                            success_chance,
                            &result,
                        );
                        
                        // Play appropriate sound
                        if let Some(player) = audio_player.as_ref() {
                            match &result {
                                PlayResult::Out(OutType::Flyout) | PlayResult::Out(OutType::LineOut) => {
                                    player.play_catch();
                                }
                                PlayResult::Out(OutType::Groundout) => {
                                    player.play_ground_ball();
                                }
                                PlayResult::Hit(_) => {
                                    match ball_in_play.initial_contact_quality {
                                        85..=100 => player.play_cheer_triple_and_homer(),
                                        60..=84 => player.play_cheer_double(),
                                        _ => player.play_cheer_single(),
                                    }
                                }
                                _ => {}
                            }
                        }
                        
                        process_play_result(state, &result, audio_player);
                        state.fielding_cursor = None;
                        state.pitch_state = PitchState::ShowResult {
                            result,
                            frames_left: 90,
                        };
                    }
                }
                _ => {}
            }
        }
        PitchState::ShowResult { .. } => {
            if input == GameInput::Action {
                // Continue to next pitch
                input_state.reset();
                state.pitch_state = PitchState::ChoosePitch;
                state.pitch_location = None;
                state.swing_location = None;
                state.message = "Choose your pitch!".to_string();
            }
        }
        _ => {}
    }
}

fn handle_team_selection_input(state: &mut GameState, input: GameInput) {
    if let GameMode::TeamSelection { selected_home, selected_away, input_buffer, input_mode } = &mut state.mode {
        match input {
            GameInput::SelectAwayTeam => {
                *input_buffer = String::new();
                *input_mode = game::TeamInputMode::SelectingAway;
                state.message = "Enter away team number (1-30), then press ENTER:".to_string();
            }
            GameInput::SelectHomeTeam => {
                *input_buffer = String::new();
                *input_mode = game::TeamInputMode::SelectingHome;
                state.message = "Enter home team number (1-30), then press ENTER:".to_string();
            }
            GameInput::NumberInput(digit) => {
                if *input_mode != game::TeamInputMode::None && input_buffer.len() < 2 {
                    input_buffer.push(digit);
                    state.message = format!("Entered: {}", input_buffer);
                }
            }
            GameInput::Action => {
                if !input_buffer.is_empty() {
                    if let Ok(num) = input_buffer.parse::<usize>() {
                        let teams = state.team_manager.get_team_list();
                        let idx = num.saturating_sub(1);
                        
                        if idx < teams.len() {
                            match input_mode {
                                game::TeamInputMode::SelectingAway => {
                                    let new_away = teams[idx].clone();
                                    *selected_away = Some(new_away.clone());
                                    state.message = format!("Away team: {} selected", new_away);
                                }
                                game::TeamInputMode::SelectingHome => {
                                    let new_home = teams[idx].clone();
                                    *selected_home = Some(new_home.clone());
                                    state.message = format!("Home team: {} selected", new_home);
                                }
                                _ => {}
                            }
                        } else {
                            state.message = format!("Invalid team number: {}", num);
                        }
                    }
                    input_buffer.clear();
                    *input_mode = game::TeamInputMode::None;
                } else if selected_home.is_some() && selected_away.is_some() {
                    // Start game if both teams selected and buffer is empty
                    let home = selected_home.clone().unwrap();
                    let away = selected_away.clone().unwrap();
                    state.start_game(home, away);
                }
            }
            _ => {}
        }
    }
}

fn update_game_state(state: &mut GameState, engine: &GameEngine, input_state: &mut InputState, audio_player: Option<&AudioPlayer>, logger: &GameLogger, pitch_count: &mut u32, inning_hits: &mut u8) {
    match &mut state.pitch_state {
        PitchState::Pitching { frames_left } => {
            *frames_left -= 1;
            if *frames_left == 0 {
                // Pitch arrives - switch to batter
                state.pitch_state = PitchState::WaitingForBatter;
                state.message = "Batter up! Aim and press SPACE to swing, or let it go.".to_string();
                input_state.reset();
            }
        }
        PitchState::WaitingForBatter => {
            // Auto-take after 60 frames (~2 seconds)
            // This allows batter to choose not to swing
        }
        PitchState::Swinging { frames_left } => {
            *frames_left -= 1;
            if *frames_left == 0 {
                // Calculate result with player stats
                let pitch_loc = state.pitch_location.unwrap();
                let swing_loc = state.swing_location;
                
                // Get fatigue penalty from current pitching team
                let fatigue_penalty = state.get_current_pitching_team()
                    .map(|t| t.get_fatigue_penalty())
                    .unwrap_or(1.0);
                
                // Clone player references to avoid borrow issues
                let batter = state.get_current_batter().cloned();
                let pitcher = state.get_current_pitcher().cloned();
                
                // Decrease pitcher stamina after pitch (more for swings)
                if let Some(team) = state.get_current_pitching_team_mut() {
                    // Decrease stamina: more for swings (1.5), less for takes (0.8)
                    let stamina_cost = if swing_loc.is_some() { 1.5 } else { 0.8 };
                    team.decrease_stamina(stamina_cost);
                }
                
                // For now, use pitch type 0 (could track the actual type)
                let (result, contact_quality) = engine.calculate_pitch_result(pitch_loc, swing_loc, 0, batter.as_ref(), pitcher.as_ref(), fatigue_penalty);
                
                // Log pitch result
                *pitch_count += 1;
                let half_str = match state.half {
                    game::InningHalf::Top => "Top",
                    game::InningHalf::Bottom => "Bottom",
                };
                logger.log_pitch_result(
                    *pitch_count,
                    state.inning,
                    half_str,
                    batter.as_ref(),
                    pitcher.as_ref(),
                    pitch_loc,
                    swing_loc,
                    contact_quality,
                    &result,
                    fatigue_penalty,
                );
                
                //Track hits for inning summary
                if matches!(&result, PlayResult::Hit(_)) {
                    *inning_hits += 1;
                }
                
                // Play sound based on result
                if let Some(player) = audio_player {
                    match &result {
                        PlayResult::Hit(_) | PlayResult::Out(_) => {
                            // Ball in play - check if we should trigger fielding
                            player.play_bat_contact();
                        }
                        PlayResult::Foul => player.play_bat_contact(),
                        PlayResult::Strike => player.play_miss(),
                        _ => {}
                    }
                }
                
                // Check if result should trigger fielding gameplay
                // ONLY trigger fielding for hits - outs are automatic
                match &result {
                    PlayResult::Hit(_) => {
                        // Generate ball-in-play with contact quality estimation
                        let contact_quality = estimate_contact_quality(&result);
                        if let Some(ball_in_play) = engine.generate_ball_in_play(contact_quality, batter.as_ref(), pitcher.as_ref()) {
                            // Switch to fielding mode
                            state.fielding_cursor = Some(ball_in_play.direction);
                            state.message = format!("{:?} to {:?}! Press SPACE to field!", ball_in_play.ball_type, ball_in_play.direction);
                            state.pitch_state = PitchState::Fielding {
                                ball_in_play,
                                frames_elapsed: 0,
                            };
                        } else {
                            // Fallback to immediate result
                            process_play_result(state, &result, audio_player);
                            state.pitch_state = PitchState::ShowResult {
                                result,
                                frames_left: 90,
                            };
                        }
                    }
                    _ => {
                        // Immediate result (strike, ball, foul)
                        process_play_result(state, &result, audio_player);
                        state.pitch_state = PitchState::ShowResult {
                            result,
                            frames_left: 90,
                        };
                    }
                }
            }
        }
        PitchState::Fielding { ball_in_play, frames_elapsed } => {
            *frames_elapsed += 1;
            
            // Auto-resolve if player doesn't act in time
            let max_time = ball_in_play.hang_time.max(45);
            if *frames_elapsed >= max_time {
                // Too slow - ball gets through
                let result = engine.ball_gets_through(ball_in_play);
                
                if let Some(player) = audio_player {
                    match &result {
                        PlayResult::Hit(_) => player.play_cheer_single(),
                        _ => {}
                    }
                }
                
                process_play_result(state, &result, audio_player);
                state.fielding_cursor = None;
                state.pitch_state = PitchState::ShowResult {
                    result,
                    frames_left: 90,
                };
            }
        }
        PitchState::BallInPlay { frames_left } => {
            *frames_left -= 1;
            if *frames_left == 0 {
                // Ball play resolved - continue
                state.pitch_state = PitchState::ChoosePitch;
            }
        }
        PitchState::ShowResult { frames_left, .. } => {
            *frames_left -= 1;
            if *frames_left == 0 {
                // Auto-continue after timeout
                input_state.reset();
                state.pitch_state = PitchState::ChoosePitch;
                state.pitch_location = None;
                state.swing_location = None;
                state.message = "Choose your pitch!".to_string();
            }
        }
        _ => {}
    }
}

fn process_play_result(state: &mut GameState, result: &PlayResult, audio_player: Option<&AudioPlayer>) {
    match result {
        PlayResult::Strike => {
            state.strikes += 1;
            state.message = format!("Strike {}!", state.strikes);
            if state.strikes >= 3 {
                state.add_strikeout();
            }
        }
        PlayResult::Ball => {
            state.balls += 1;
            state.message = format!("Ball {}!", state.balls);
            if state.balls >= 4 {
                state.add_walk();
            }
        }
        PlayResult::Foul => {
            if state.strikes < 2 {
                state.strikes += 1;
            }
            state.message = "Foul ball!".to_string();
        }
        PlayResult::Hit(hit_type) => {
            // Play cheer sound based on hit type
            if let Some(player) = audio_player {
                match hit_type {
                    HitType::Single => player.play_cheer_single(),
                    HitType::Double => player.play_cheer_double(),
                    HitType::Triple | HitType::HomeRun => player.play_cheer_triple_and_homer(),
                }
            }
            
            let bases = match hit_type {
                HitType::Single => 1,
                HitType::Double => 2,
                HitType::Triple => 3,
                HitType::HomeRun => 4,
            };
            state.message = match hit_type {
                HitType::Single => "Single!".to_string(),
                HitType::Double => "Double!".to_string(),
                HitType::Triple => "Triple!".to_string(),
                HitType::HomeRun => "HOME RUN!".to_string(),
            };
            state.advance_runners(bases);
            state.advance_batter();
        }
        PlayResult::Out(out_type) => {
            state.message = match out_type {
                OutType::Strikeout => "Strikeout!".to_string(),
                OutType::Groundout => "Groundout!".to_string(),
                OutType::Flyout => "Fly out!".to_string(),
                OutType::LineOut => "Line out!".to_string(),
            };
            state.add_out();
        }
    }
}

// Helper function to estimate contact quality from play result
fn estimate_contact_quality(result: &PlayResult) -> i32 {
    match result {
        PlayResult::Hit(HitType::HomeRun) | PlayResult::Hit(HitType::Triple) => 95,
        PlayResult::Hit(HitType::Double) => 75,
        PlayResult::Hit(HitType::Single) => 55,
        PlayResult::Out(OutType::Flyout) | PlayResult::Out(OutType::LineOut) => 65,
        PlayResult::Out(OutType::Groundout) => 35,
        _ => 20,
    }
}
