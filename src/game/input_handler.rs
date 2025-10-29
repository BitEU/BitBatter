use crate::audio::AudioPlayer;
use crate::game::{constants::*, GameEngine, GameState, OutType, PitchLocation, PitchState, PlayResult, TeamInputMode, SwingTiming};
use crate::input::{GameInput, InputState};
use crate::logger::GameLogger;

pub fn handle_input(
    state: &mut GameState,
    engine: &GameEngine,
    input_state: &mut InputState,
    input: GameInput,
    audio_player: Option<&AudioPlayer>,
    logger: &GameLogger,
) {
    // Handle team selection first
    if let crate::game::GameMode::TeamSelection { .. } = &state.mode {
        handle_team_selection_input(state, input);
        return;
    }

    match &state.pitch_state {
        PitchState::ChoosePitch => {
            if let GameInput::SelectPitch(idx) = input {
                if idx < engine.pitch_types.len() {
                    state.pitch_state = PitchState::Aiming { pitch_type: idx };
                    state.message = format!(
                        "Aiming {}. Use arrows or SHIFT+(1-9) to aim, SPACE to pitch.",
                        engine.get_pitch_name(idx)
                    );
                    input_state.reset();
                }
            }
        }
        PitchState::Aiming { pitch_type } => {
            match input {
                GameInput::Up | GameInput::Down | GameInput::Left | GameInput::Right => {
                    input_state.update(&input);
                }
                GameInput::DirectPosition(num) => {
                    // Direct numpad selection - immediately lock in position and start pitch clock
                    let location = PitchLocation::from_numpad(num);
                    state.pitch_location = Some(location);
                    state.pitch_state = PitchState::PitchClock { 
                        frames_left: PITCH_CLOCK_FRAMES, 
                        pitch_type: *pitch_type 
                    };
                    state.message = "Get ready! Pitch clock started...".to_string();
                    input_state.reset();
                }
                GameInput::Action => {
                    // Lock in pitch location and start pitch clock
                    let location = PitchLocation::from_direction(
                        input_state.up,
                        input_state.down,
                        input_state.left,
                        input_state.right,
                    );
                    state.pitch_location = Some(location);
                    state.pitch_state = PitchState::PitchClock { 
                        frames_left: PITCH_CLOCK_FRAMES, 
                        pitch_type: *pitch_type 
                    };
                    state.message = "Get ready! Pitch clock started...".to_string();
                    input_state.reset();
                }
                _ => {}
            }
        }
        PitchState::BallApproaching { .. } => {
            match input {
                GameInput::Up | GameInput::Down | GameInput::Left | GameInput::Right => {
                    input_state.update(&input);
                }
                GameInput::DirectPosition(num) => {
                    // Direct numpad selection - attempt swing with timing
                    let swing_loc = PitchLocation::from_numpad(num);
                    let timing = calculate_swing_timing(state);
                    state.swing_location = Some(swing_loc);
                    state.swing_timing = timing;
                    state.pitch_state = PitchState::Swinging { 
                        frames_left: SWINGING_ANIMATION_FRAMES, 
                        swing_timing: timing
                    };
                    state.message = format!("Swing! ({})", format_timing(&timing));
                    input_state.reset();
                }
                GameInput::Action => {
                    // Batter swings with current aim
                    let swing_loc = PitchLocation::from_direction(
                        input_state.up,
                        input_state.down,
                        input_state.left,
                        input_state.right,
                    );
                    let timing = calculate_swing_timing(state);
                    state.swing_location = Some(swing_loc);
                    state.swing_timing = timing;
                    state.pitch_state = PitchState::Swinging { 
                        frames_left: SWINGING_ANIMATION_FRAMES, 
                        swing_timing: timing
                    };
                    state.message = format!("Swing! ({})", format_timing(&timing));
                    input_state.reset();
                }
                _ => {}
            }
        }
        PitchState::WaitingForBatter => {
            // Legacy state - shouldn't happen with new timing system
            match input {
                GameInput::Action => {
                    // Continue to next pitch
                    input_state.reset();
                    state.pitch_state = PitchState::ChoosePitch;
                    state.pitch_location = None;
                    state.swing_location = None;
                    state.swing_timing = SwingTiming::NoSwing;
                    state.message = "Choose your pitch!".to_string();
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
                        
                        super::update::process_play_result(state, &result, audio_player);
                        state.fielding_cursor = None;
                        state.pitch_state = PitchState::ShowResult {
                            result,
                            frames_left: RESULT_DISPLAY_FRAMES,
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
    if let crate::game::GameMode::TeamSelection { selected_home, selected_away, input_buffer, input_mode } = &mut state.mode {
        // Debug: log what input we received
        
        match input {
            GameInput::SelectAwayTeam => {
                *input_buffer = String::new();
                *input_mode = TeamInputMode::SelectingAway;
                state.message = "Enter away team number (1-30), then press ENTER:".to_string();
            }
            GameInput::SelectHomeTeam => {
                *input_buffer = String::new();
                *input_mode = TeamInputMode::SelectingHome;
                state.message = "Enter home team number (1-30), then press ENTER:".to_string();
            }
            GameInput::NumberInput(digit) => {
                if *input_mode != TeamInputMode::None && input_buffer.len() < 2 {
                    input_buffer.push(digit);
                    state.message = format!("Entered: {}", input_buffer);
                } else {
                }
            }
            GameInput::Action => {
                if !input_buffer.is_empty() {
                    if let Ok(num) = input_buffer.parse::<usize>() {
                        let teams = state.team_manager.get_team_list();
                        let idx = num.saturating_sub(1);
                        
                        if idx < teams.len() {
                            match input_mode {
                                TeamInputMode::SelectingAway => {
                                    let new_away = teams[idx].clone();
                                    // Load the team data
                                    if let Err(e) = state.team_manager.load_team(&new_away) {
                                        state.message = format!("Error loading team {}: {}", new_away, e);
                                        *input_buffer = String::new();
                                        *input_mode = TeamInputMode::None;
                                        return;
                                    }
                                    *selected_away = Some(new_away.clone());
                                    state.message = format!("Away team: {} selected", new_away);
                                }
                                TeamInputMode::SelectingHome => {
                                    let new_home = teams[idx].clone();
                                    // Load the team data
                                    if let Err(e) = state.team_manager.load_team(&new_home) {
                                        state.message = format!("Error loading team {}: {}", new_home, e);
                                        *input_buffer = String::new();
                                        *input_mode = TeamInputMode::None;
                                        return;
                                    }
                                    *selected_home = Some(new_home.clone());
                                    state.message = format!("Home team: {} selected", new_home);
                                }
                                _ => {
                                }
                            }
                        } else {
                            state.message = format!("Invalid team number: {}. Please choose 1-{}", num, teams.len());
                        }
                    } else {
                        state.message = "Invalid input. Please enter a number.".to_string();
                    }
                    input_buffer.clear();
                    *input_mode = TeamInputMode::None;
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

fn calculate_swing_timing(state: &GameState) -> SwingTiming {
    if let PitchState::BallApproaching { frames_left, can_swing, .. } = &state.pitch_state {
        if !can_swing {
            return SwingTiming::TooEarly;
        }
        
        // Calculate timing based on remaining frames
        // Perfect timing is when ball is very close to plate
        let perfect_start = PERFECT_TIMING_WINDOW_FRAMES / 2;
        let perfect_end = perfect_start + PERFECT_TIMING_WINDOW_FRAMES;
        
        let early_start = perfect_start + PERFECT_TIMING_WINDOW_FRAMES;
        let early_end = early_start + EARLY_LATE_WINDOW_FRAMES;
        
        let _late_start = 0;
        let late_end = perfect_start;
        
        match *frames_left {
            f if f <= late_end => SwingTiming::Late,
            f if f <= perfect_end => SwingTiming::Perfect,
            f if f <= early_end => SwingTiming::Early,
            _ => SwingTiming::TooEarly,
        }
    } else {
        SwingTiming::NoSwing
    }
}

fn format_timing(timing: &SwingTiming) -> &'static str {
    match timing {
        SwingTiming::TooEarly => "Too Early!",
        SwingTiming::Early => "Early",
        SwingTiming::Perfect => "Perfect!",
        SwingTiming::Late => "Late", 
        SwingTiming::TooLate => "Too Late!",
        SwingTiming::NoSwing => "No Swing",
    }
}
