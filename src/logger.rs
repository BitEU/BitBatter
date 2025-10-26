use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
use crate::game::state::{PlayResult, PitchLocation, BallInPlay};
use crate::team::Player;

pub struct GameLogger {
    log_path: String,
}

impl GameLogger {
    pub fn new() -> Self {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let log_path = format!("game_log_{}.txt", timestamp);
        
        // Create initial log file with header
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&log_path)
        {
            let _ = writeln!(file, "{}", "=".repeat(80));
            let _ = writeln!(file, "TERMINAL BASEBALL - GAME LOG");
            let _ = writeln!(file, "Started: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
            let _ = writeln!(file, "{}", "=".repeat(80));
            let _ = writeln!(file, "");
        }
        
        Self { log_path }
    }
    
    pub fn log_pitch_result(
        &self,
        pitch_num: u32,
        inning: u8,
        half: &str,
        batter: Option<&Player>,
        pitcher: Option<&Player>,
        pitch_location: PitchLocation,
        swing_location: Option<PitchLocation>,
        contact_quality: Option<i32>,
        result: &PlayResult,
        fatigue_penalty: f32,
    ) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
        {
            let _ = writeln!(file, "\n{}", "-".repeat(80));
            let _ = writeln!(file, "PITCH #{} - Inning {} {}", pitch_num, inning, half);
            let _ = writeln!(file, "{}", "-".repeat(80));
            
            // Batter info
            if let Some(b) = batter {
                let _ = writeln!(file, "BATTER: {}", b.stats.name);
                let _ = writeln!(file, "  Barrel%: {:.1}%", b.stats.barrel_percent);
                let _ = writeln!(file, "  GB%: {:.1}%", b.stats.gb);
                let _ = writeln!(file, "  Max Distance: {} ft", b.stats.max_distance);
            } else {
                let _ = writeln!(file, "BATTER: Unknown");
            }
            
            // Pitcher info
            if let Some(p) = pitcher {
                let _ = writeln!(file, "PITCHER: {}", p.stats.name);
                let _ = writeln!(file, "  Barrel% Allowed: {:.1}%", p.stats.barrel_percent);
                let _ = writeln!(file, "  Fatigue Penalty: {:.2}x", fatigue_penalty);
            } else {
                let _ = writeln!(file, "PITCHER: Unknown");
            }
            
            // Pitch details
            let _ = writeln!(file, "\nPITCH LOCATION: {:?}", pitch_location);
            let _ = writeln!(file, "STRIKE ZONE: {}", pitch_location.is_strike());
            
            if let Some(swing) = swing_location {
                let _ = writeln!(file, "SWING LOCATION: {:?}", swing);
                let exact_match = std::mem::discriminant(&pitch_location) == std::mem::discriminant(&swing);
                let _ = writeln!(file, "EXACT MATCH: {}", exact_match);
                
                if let Some(cq) = contact_quality {
                    let _ = writeln!(file, "CONTACT QUALITY: {}/100", cq);
                }
            } else {
                let _ = writeln!(file, "SWING: No swing (take)");
            }
            
            // Result
            let _ = writeln!(file, "\nRESULT: {}", match result {
                PlayResult::Strike => "STRIKE".to_string(),
                PlayResult::Ball => "BALL".to_string(),
                PlayResult::Foul => "FOUL".to_string(),
                PlayResult::Hit(hit_type) => format!("HIT - {:?}", hit_type),
                PlayResult::Out(out_type) => format!("OUT - {:?}", out_type),
            });
        }
    }
    
    pub fn log_fielding_attempt(
        &self,
        ball: &BallInPlay,
        catch_timing: u8,
        perfect_timing: u8,
        success_chance: f32,
        result: &PlayResult,
    ) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
        {
            let _ = writeln!(file, "\n  FIELDING ATTEMPT:");
            let _ = writeln!(file, "    Ball Type: {:?}", ball.ball_type);
            let _ = writeln!(file, "    Direction: {:?}", ball.direction);
            let _ = writeln!(file, "    Speed: {:.1} mph", ball.speed);
            let _ = writeln!(file, "    Hang Time: {} frames", ball.hang_time);
            let _ = writeln!(file, "    Contact Quality: {}/100", ball.initial_contact_quality);
            let _ = writeln!(file, "    Catch Timing: {} frames (perfect: {})", catch_timing, perfect_timing);
            let _ = writeln!(file, "    Timing Diff: {} frames", (catch_timing as i32 - perfect_timing as i32).abs());
            let _ = writeln!(file, "    Success Chance: {:.1}%", success_chance * 100.0);
            let _ = writeln!(file, "    FIELDING RESULT: {}", match result {
                PlayResult::Out(out_type) => format!("OUT - {:?}", out_type),
                PlayResult::Hit(hit_type) => format!("HIT - {:?}", hit_type),
                _ => "Unknown".to_string(),
            });
        }
    }
    
    pub fn log_inning_summary(
        &self,
        inning: u8,
        half: &str,
        runs: u8,
        hits: u8,
        outs: u8,
    ) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
        {
            let _ = writeln!(file, "\n{}", "=".repeat(80));
            let _ = writeln!(file, "INNING {} {} SUMMARY", inning, half);
            let _ = writeln!(file, "  Runs: {}", runs);
            let _ = writeln!(file, "  Hits: {}", hits);
            let _ = writeln!(file, "  Outs: {}", outs);
            let _ = writeln!(file, "{}", "=".repeat(80));
        }
    }
    
    pub fn log_game_summary(
        &self,
        away_team: &str,
        home_team: &str,
        away_score: u8,
        home_score: u8,
    ) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
        {
            let _ = writeln!(file, "\n\n{}", "=".repeat(80));
            let _ = writeln!(file, "FINAL SCORE");
            let _ = writeln!(file, "{}", "=".repeat(80));
            let _ = writeln!(file, "{}: {}", away_team, away_score);
            let _ = writeln!(file, "{}: {}", home_team, home_score);
            let _ = writeln!(file, "{}", "=".repeat(80));
            let _ = writeln!(file, "Log saved to: {}", self.log_path);
        }
    }
}
