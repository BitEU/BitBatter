use crate::audio::AudioPlayer;
use crate::game::{constants::*, GameEngine, GameState, HitType, InningHalf, OutType, PitchState, PlayResult};
use crate::input::InputState;
use crate::logger::GameLogger;

pub fn update_game_state(
    state: &mut GameState,
    engine: &GameEngine,
    input_state: &mut InputState,
    audio_player: Option<&AudioPlayer>,
    logger: &GameLogger,
    pitch_count: &mut u32,
    inning_hits: &mut u8,
) {
    match &mut state.pitch_state {
        PitchState::Pitching { frames_left } => {
            *frames_left -= 1;
            if *frames_left == 0 {
                // Pitch arrives - switch to batter
                state.pitch_state = PitchState::WaitingForBatter;
                state.message = "Batter up! Aim with arrows or SHIFT+(1-9), SPACE to swing, or let it go.".to_string();
                input_state.reset();
            }
        }
        PitchState::WaitingForBatter => {
            // Auto-take after configured frames (~2 seconds)
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
                    .unwrap_or(FATIGUE_PENALTY_FRESH);
                
                // Clone player references to avoid borrow issues
                let batter = state.get_current_batter().cloned();
                let pitcher = state.get_current_pitcher().cloned();
                
                // Decrease pitcher stamina after pitch (more for swings)
                if let Some(team) = state.get_current_pitching_team_mut() {
                    // Decrease stamina: more for swings, less for takes
                    let stamina_cost = if swing_loc.is_some() { STAMINA_COST_SWING } else { STAMINA_COST_TAKE };
                    team.decrease_stamina(stamina_cost);
                }
                
                // For now, use pitch type 0 (could track the actual type)
                let (result, contact_quality) = engine.calculate_pitch_result(
                    pitch_loc,
                    swing_loc,
                    0,
                    batter.as_ref(),
                    pitcher.as_ref(),
                    fatigue_penalty,
                );
                
                // Log pitch result
                *pitch_count += 1;
                let half_str = match state.half {
                    InningHalf::Top => "Top",
                    InningHalf::Bottom => "Bottom",
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
                
                // Track hits for inning summary
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
                        // Generate ball-in-play with contact quality
                        if let Some(contact_quality) = contact_quality {
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
                                    frames_left: RESULT_DISPLAY_FRAMES,
                                };
                            }
                        } else {
                            // No contact quality - immediate result
                            process_play_result(state, &result, audio_player);
                            state.pitch_state = PitchState::ShowResult {
                                result,
                                frames_left: RESULT_DISPLAY_FRAMES,
                            };
                        }
                    }
                    _ => {
                        // Immediate result (strike, ball, foul)
                        process_play_result(state, &result, audio_player);
                        state.pitch_state = PitchState::ShowResult {
                            result,
                            frames_left: RESULT_DISPLAY_FRAMES,
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
                    frames_left: RESULT_DISPLAY_FRAMES,
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

pub fn process_play_result(state: &mut GameState, result: &PlayResult, audio_player: Option<&AudioPlayer>) {
    match result {
        PlayResult::Strike => {
            state.strikes += 1;
            state.message = format!("Strike {}!", state.strikes);
            if state.strikes >= MAX_STRIKES {
                state.add_strikeout();
            }
        }
        PlayResult::Ball => {
            state.balls += 1;
            state.message = format!("Ball {}!", state.balls);
            if state.balls >= MAX_BALLS {
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
