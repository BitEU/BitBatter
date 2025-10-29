// Game timing constants (in frames)
pub const TARGET_FPS: u64 = 30;
pub const FRAME_TIME_MS: u64 = 1000 / TARGET_FPS;

// Animation frame durations
pub const PITCHING_ANIMATION_FRAMES: u8 = 20;
pub const SWINGING_ANIMATION_FRAMES: u8 = 10;
pub const RESULT_DISPLAY_FRAMES: u8 = 90;
pub const GAME_OVER_DELAY_SECONDS: u64 = 3;

// Timing system constants
pub const PITCH_CLOCK_FRAMES: u16 = 90; // 10 seconds at 30fps
pub const BALL_APPROACH_FRAMES: u8 = 90; // 3 seconds for ball to reach plate
pub const SWING_TIMING_WINDOW_FRAMES: u8 = 30; // 1 second timing window
pub const PERFECT_TIMING_WINDOW_FRAMES: u8 = 6; // 0.2 second perfect window
pub const EARLY_LATE_WINDOW_FRAMES: u8 = 12; // 0.4 second early/late windows each side

// Batter auto-take timing
pub const BATTER_AUTO_TAKE_FRAMES: u8 = 60; // ~2 seconds at 30fps

// Input polling
pub const INPUT_POLL_TIMEOUT_MS: u64 = 16; // ~60fps polling

// Pitcher stamina
pub const STARTING_STAMINA: f32 = 100.0;
pub const STAMINA_COST_SWING: f32 = 1.5;
pub const STAMINA_COST_TAKE: f32 = 0.8;

// Stamina fatigue thresholds and penalties
pub const STAMINA_FRESH_THRESHOLD: f32 = 70.0;
pub const STAMINA_GOOD_THRESHOLD: f32 = 50.0;
pub const STAMINA_TIRED_THRESHOLD: f32 = 30.0;
pub const STAMINA_EXHAUSTED_THRESHOLD: f32 = 15.0;

pub const FATIGUE_PENALTY_FRESH: f32 = 1.0;
pub const FATIGUE_PENALTY_GOOD: f32 = 0.95;
pub const FATIGUE_PENALTY_TIRED: f32 = 0.85;
pub const FATIGUE_PENALTY_VERY_TIRED: f32 = 0.70;
pub const FATIGUE_PENALTY_EXHAUSTED: f32 = 0.50;

// Game rules
pub const MAX_STRIKES: u8 = 3;
pub const MAX_BALLS: u8 = 4;
pub const MAX_OUTS: u8 = 3;
pub const INNINGS_PER_GAME: u8 = 9;
pub const BASES_COUNT: usize = 3;
pub const BATTING_ORDER_SIZE: usize = 9;

// Player stats thresholds
pub const MIN_PLAYER_ATTEMPTS: u32 = 50;

// Fielding timing
pub const FIELDING_TIMING_WINDOW: f32 = 15.0; // frames
pub const MAX_FIELDING_AUTO_RESOLVE_MULTIPLIER: u8 = 1; // multiplier of hang_time

// Contact quality ranges
pub const CONTACT_EXCELLENT_MIN: i32 = 85;
pub const CONTACT_GREAT_MIN: i32 = 75;
pub const CONTACT_GOOD_MIN: i32 = 55;
pub const CONTACT_WEAK_MIN: i32 = 35;

// Skill adjustments
pub const BATTER_SKILL_BONUS_MULTIPLIER: f32 = 1.5;
pub const PITCHER_SKILL_PENALTY_MULTIPLIER: f32 = 2.0;
pub const ADJACENT_BATTER_SKILL_MULTIPLIER: f32 = 1.0;
pub const ADJACENT_PITCHER_SKILL_MULTIPLIER: f32 = 1.0;

// Ball-in-play generation
pub const SPEED_EXCELLENT_MIN: f32 = 80.0;
pub const SPEED_EXCELLENT_MAX: f32 = 100.0;
pub const SPEED_GOOD_MIN: f32 = 70.0;
pub const SPEED_GOOD_MAX: f32 = 90.0;
pub const SPEED_WEAK_MIN: f32 = 40.0;
pub const SPEED_WEAK_MAX: f32 = 60.0;

pub const HANG_TIME_FLYBALL_MIN: u8 = 60;
pub const HANG_TIME_FLYBALL_MAX: u8 = 90;
pub const HANG_TIME_LINEDRIVE_MIN: u8 = 20;
pub const HANG_TIME_LINEDRIVE_MAX: u8 = 40;
pub const HANG_TIME_POPFLY_MIN: u8 = 40;
pub const HANG_TIME_POPFLY_MAX: u8 = 60;

// Fielding success base rates
pub const FIELDING_SUCCESS_POPFLY: f32 = 0.98;
pub const FIELDING_SUCCESS_FLYBALL: f32 = 0.90;
pub const FIELDING_SUCCESS_LINEDRIVE: f32 = 0.75;
pub const FIELDING_SUCCESS_GROUNDER: f32 = 0.85;

pub const FIELDING_SPEED_THRESHOLD: f32 = 95.0;
pub const FIELDING_SPEED_PENALTY_DIVISOR: f32 = 300.0;

pub const FIELDING_TIMING_GOOD_THRESHOLD: f32 = 0.6;
pub const FIELDING_TIMING_POOR_MULTIPLIER: f32 = 0.5;
pub const FIELDING_MIN_SUCCESS_RATE: f32 = 0.1;
