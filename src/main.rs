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
use game::{constants::*, GameEngine, GameState};
use input::InputPoller;
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
    let mut input_state = input::InputState::new();
    let mut input_poller = InputPoller::new();
    let audio_player = AudioPlayer::new();
    let logger = GameLogger::new();
    
    let mut pitch_count = 0u32;
    let mut inning_hits = 0u8;

    let frame_time = Duration::from_millis(FRAME_TIME_MS);

    loop {
        let frame_start = Instant::now();

        // Handle input
        if let Some(input) = input_poller.poll_input(INPUT_POLL_TIMEOUT_MS)? {
            if input == input::GameInput::Quit {
                // Handle quit confirmation
                if game_state.quit_requested {
                    break; // Confirmed quit
                } else {
                    game_state.quit_requested = true;
                    game_state.message = "Press Q again to quit, or any other key to continue".to_string();
                    // Don't process any other input this frame
                    continue;
                }
            } else {
                // Any other input cancels quit request
                if game_state.quit_requested {
                    game_state.quit_requested = false;
                    game_state.message = "Quit cancelled. Continue playing!".to_string();
                }
                
                game::input_handler::handle_input(
                    &mut game_state,
                    &engine,
                    &mut input_state,
                    input,
                    audio_player.as_ref(),
                    &logger,
                );
            }
        }

        // Update game logic (animations, etc.)
        game::update::update_game_state(
            &mut game_state,
            &engine,
            &mut input_state,
            audio_player.as_ref(),
            &logger,
            &mut pitch_count,
            &mut inning_hits,
        );

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
        if game_state.game_over && matches!(game_state.pitch_state, game::PitchState::ShowResult { .. }) {
            thread::sleep(Duration::from_secs(GAME_OVER_DELAY_SECONDS));
            break;
        }
    }

    Ok(())
}
