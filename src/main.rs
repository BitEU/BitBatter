mod game;
mod input;
mod ui;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::{GameEngine, GameState, HitType, OutType, PitchLocation, PitchState, PlayResult};
use input::{poll_input, GameInput, InputState};
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

    let target_fps = 30;
    let frame_time = Duration::from_millis(1000 / target_fps);

    loop {
        let frame_start = Instant::now();

        // Handle input
        if let Some(input) = poll_input()? {
            if input == GameInput::Quit {
                break;
            }
            handle_input(&mut game_state, &engine, &mut input_state, input);
        }

        // Update game logic (animations, etc.)
        update_game_state(&mut game_state, &engine, &mut input_state);

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
) {
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
        PitchState::Aiming { pitch_type } => {
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

fn update_game_state(state: &mut GameState, engine: &GameEngine, input_state: &mut InputState) {
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
                // Calculate result
                let pitch_loc = state.pitch_location.unwrap();
                let swing_loc = state.swing_location;
                
                // For now, use pitch type 0 (could track the actual type)
                let result = engine.calculate_pitch_result(pitch_loc, swing_loc, 0);
                
                process_play_result(state, &result);
                
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

fn process_play_result(state: &mut GameState, result: &PlayResult) {
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
