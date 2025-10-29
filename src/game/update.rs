use crate::audio::AudioPlayer;
use crate::game::{constants::*, GameEngine, GameState, HitType, InningHalf, OutType, PitchState, PlayResult, SwingTiming};
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
        PitchState::PitchClock { frames_left, pitch_type } => {
            *frames_left -= 1;
            let seconds_left = (*frames_left as f32 / TARGET_FPS as f32).ceil() as u16;
            
            if seconds_left <= 3 {
                state.message = format!("GET READY! {}...", seconds_left);
            } else {
                state.message = format!("Pitch clock: {}s - Get in position!", seconds_left);
            }
            
            if *frames_left == 0 {
                // Clock expires - start ball approach
                state.pitch_state = PitchState::BallApproaching {
                    frames_left: BALL_APPROACH_FRAMES,
                    ball_position: 0.0,
                    pitch_type: *pitch_type,
                    can_swing: false,
                };
                state.message = "Here comes the pitch! Watch the ball!".to_string();
            }
        }
        PitchState::Pitching { frames_left } => {
            *frames_left -= 1;
            if *frames_left == 0 {
                // Legacy - transition to ball approaching
                state.pitch_state = PitchState::BallApproaching {
                    frames_left: BALL_APPROACH_FRAMES,
                    ball_position: 0.0,
                    pitch_type: 0, // Default pitch type
                    can_swing: false,
                };
                state.message = "Here comes the pitch!".to_string();
                input_state.reset();
            }
        }
        PitchState::BallApproaching { frames_left, ball_position, can_swing, .. } => {
            *frames_left -= 1;
            
            // Update ball position (0.0 = mound, 1.0 = plate)
            *ball_position = 1.0 - (*frames_left as f32 / BALL_APPROACH_FRAMES as f32);
            
            // Enable swinging when ball enters timing window
            let timing_window_start = SWING_TIMING_WINDOW_FRAMES;
            if *frames_left <= timing_window_start && !*can_swing {
                *can_swing = true;
                state.message = "SWING NOW! Time your swing!".to_string();
            }
            
            // Update message with timing cues
            if *can_swing {
                if *frames_left <= PERFECT_TIMING_WINDOW_FRAMES {
                    state.message = "PERFECT TIMING!".to_string();
                } else if *frames_left <= (PERFECT_TIMING_WINDOW_FRAMES + EARLY_LATE_WINDOW_FRAMES) {
                    state.message = "Good timing zone...".to_string();
                }
            }
            
            if *frames_left == 0 {
                // Ball reaches plate - no swing means take
                state.swing_timing = SwingTiming::NoSwing;
                let pitch_loc = state.pitch_location.unwrap();
                
                let result = if pitch_loc.is_strike() {
                    PlayResult::Strike
                } else {
                    PlayResult::Ball
                };
                
                state.pitch_state = PitchState::ShowResult {
                    result,
                    frames_left: RESULT_DISPLAY_FRAMES,
                };
                state.message = "Taken!".to_string();
            }
        }
        PitchState::WaitingForBatter => {
            // Auto-take after configured frames (~2 seconds)
            // This allows batter to choose not to swing
        }
        PitchState::Swinging { frames_left, swing_timing } => {
            *frames_left -= 1;
            if *frames_left == 0 {
                // Collect all data needed for calculation
                let pitch_loc = state.pitch_location.unwrap();
                let swing_loc = state.swing_location;
                let swing_timing_copy = *swing_timing;
                let fatigue_penalty = state.get_current_pitching_team()
                    .map(|t| t.get_fatigue_penalty())
                    .unwrap_or(FATIGUE_PENALTY_FRESH);
                let batter = state.get_current_batter().cloned();
                let pitcher = state.get_current_pitcher().cloned();
                
                // Now modify state - decrease pitcher stamina
                if let Some(team) = state.get_current_pitching_team_mut() {
                    let stamina_cost = if swing_loc.is_some() { STAMINA_COST_SWING } else { STAMINA_COST_TAKE };
                    team.decrease_stamina(stamina_cost);
                }
                
                // Calculate result with timing consideration
                let (result, contact_quality) = engine.calculate_pitch_result_with_timing(
                    pitch_loc,
                    swing_loc,
                    0,
                    batter.as_ref(),
                    pitcher.as_ref(),
                    fatigue_penalty,
                    &swing_timing_copy,
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
                state.swing_timing = SwingTiming::NoSwing;
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
