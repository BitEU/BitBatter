use crate::game::state::{BallInPlay, BallType, FieldDirection, HitType, OutType, PitchLocation, PlayResult};
use crate::team::Player;
use rand::Rng;

pub struct GameEngine {
    pub pitch_types: Vec<PitchType>,
}

#[derive(Clone)]
pub struct PitchType {
    pub name: &'static str,
    pub speed: u8,    // 60-100 mph
    pub break_amount: i8, // Movement
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            pitch_types: vec![
                PitchType {
                    name: "Fastball",
                    speed: 90,
                    break_amount: 0,
                },
                PitchType {
                    name: "Curveball",
                    speed: 75,
                    break_amount: 5,
                },
                PitchType {
                    name: "Slider",
                    speed: 82,
                    break_amount: 3,
                },
                PitchType {
                    name: "Changeup",
                    speed: 78,
                    break_amount: 1,
                },
            ],
        }
    }

    pub fn calculate_pitch_result(
        &self,
        pitch_location: PitchLocation,
        swing_location: Option<PitchLocation>,
        _pitch_type_idx: usize,
        batter: Option<&Player>,
        pitcher: Option<&Player>,
        fatigue_penalty: f32,  // Multiplier from 0.5 to 1.0
    ) -> (PlayResult, Option<i32>) {  // Returns (result, contact_quality)
        let mut rng = rand::thread_rng();

        // No swing
        if swing_location.is_none() {
            return if pitch_location.is_strike() {
                (PlayResult::Strike, None)
            } else {
                (PlayResult::Ball, None)
            };
        }

        let swing_loc = swing_location.unwrap();
        
        // Calculate timing and location accuracy
        let exact_match = std::mem::discriminant(&pitch_location) == std::mem::discriminant(&swing_loc);
        let adjacent_match = !exact_match && self.locations_match(pitch_location, swing_loc);
        let is_strike_zone = pitch_location.is_strike();

        // Perfect contact - ONLY on exact match in strike zone
        if exact_match && is_strike_zone {
            let mut contact_quality = rng.gen_range(1..=100);
            
            // Adjust contact quality based on batter's skills
            if let Some(batter) = batter {
                // Better batters (higher barrel %) get bonus to contact quality
                let skill_bonus = (batter.stats.barrel_percent * 1.5) as i32;
                contact_quality = (contact_quality + skill_bonus).min(100);
            }

            // Adjust based on pitcher's ability to limit hard contact
            if let Some(pitcher) = pitcher {
                // Better pitchers (lower barrel % allowed) reduce contact quality
                // Fatigue reduces pitcher effectiveness significantly
                let pitcher_penalty = (pitcher.stats.barrel_percent * 2.0 * fatigue_penalty) as i32;
                contact_quality = (contact_quality - pitcher_penalty).max(1);
            }

            // Even with perfect timing, outcomes are heavily weighted toward outs
            // This reflects real baseball: ~.300 batting average means 70% outs!
            let result = match contact_quality {
                90..=100 => {
                    // Exceptional contact - home run or extra bases
                    let hr_chance = if let Some(batter) = batter {
                        (batter.stats.max_distance as f32 / 500.0 * 100.0) as u32
                    } else { 25 };
                    
                    if rng.gen_range(1..=100) <= hr_chance.min(25) {
                        PlayResult::Hit(HitType::HomeRun)
                    } else if rng.gen_bool(0.6) {
                        PlayResult::Hit(HitType::Triple)
                    } else {
                        PlayResult::Hit(HitType::Double)
                    }
                }
                75..=89 => {
                    // Great contact - mostly doubles/singles, some outs
                    let roll = rng.gen_range(1..=10);
                    match roll {
                        1 => PlayResult::Hit(HitType::Triple),
                        2..=4 => PlayResult::Hit(HitType::Double),
                        5..=7 => PlayResult::Hit(HitType::Single),
                        _ => {
                            // Even great contact can be caught
                            if rng.gen_bool(0.6) {
                                PlayResult::Out(OutType::Flyout)
                            } else {
                                PlayResult::Out(OutType::LineOut)
                            }
                        }
                    }
                }
                55..=74 => {
                    // Good contact - mix of hits and outs (realistic .300 avg)
                    let roll = rng.gen_range(1..=10);
                    match roll {
                        1..=3 => PlayResult::Hit(HitType::Single),
                        4 => PlayResult::Hit(HitType::Double),
                        5..=6 => PlayResult::Foul,
                        _ => {
                            // Most outcomes are outs
                            let gb_tendency = batter.map(|b| b.stats.gb).unwrap_or(50.0);
                            if rng.gen_range(0.0..100.0) < gb_tendency {
                                PlayResult::Out(OutType::Groundout)
                            } else {
                                PlayResult::Out(OutType::Flyout)
                            }
                        }
                    }
                }
                35..=54 => {
                    // Weak contact - mostly outs and fouls
                    let roll = rng.gen_range(1..=10);
                    match roll {
                        1..=2 => PlayResult::Foul,
                        3 => PlayResult::Hit(HitType::Single),
                        _ => {
                            let gb_tendency = batter.map(|b| b.stats.gb).unwrap_or(50.0);
                            if rng.gen_range(0.0..100.0) < gb_tendency {
                                PlayResult::Out(OutType::Groundout)
                            } else {
                                PlayResult::Out(OutType::Flyout)
                            }
                        }
                    }
                }
                _ => {
                    // Poor contact - mostly outs
                    if rng.gen_bool(0.2) {
                        PlayResult::Foul
                    } else {
                        let gb_tendency = batter.map(|b| b.stats.gb).unwrap_or(50.0);
                        if rng.gen_range(0.0..100.0) < gb_tendency {
                            PlayResult::Out(OutType::Groundout)
                        } else {
                            PlayResult::Out(OutType::Flyout)
                        }
                    }
                }
            };
            return (result, Some(contact_quality));
        }

        // Good contact - adjacent match in strike zone (weaker than perfect)
        if adjacent_match && is_strike_zone {
            let mut contact_quality = rand::thread_rng().gen_range(1..=100);
            
            // Adjust based on batter skill
            if let Some(batter) = batter {
                let skill_bonus = (batter.stats.barrel_percent * 1.0) as i32;
                contact_quality = (contact_quality + skill_bonus).min(100);
            }
            
            // Adjust based on pitcher ability
            if let Some(pitcher) = pitcher {
                let pitcher_penalty = (pitcher.stats.barrel_percent * 1.0 * fatigue_penalty) as i32;
                contact_quality = (contact_quality - pitcher_penalty).max(1);
            }

            let result = match contact_quality {
                75..=100 => PlayResult::Hit(HitType::Single),
                50..=74 => {
                    if rand::thread_rng().gen_bool(0.5) {
                        PlayResult::Hit(HitType::Single)
                    } else {
                        PlayResult::Foul
                    }
                }
                30..=49 => PlayResult::Foul,
                _ => {
                    let gb_tendency = batter.map(|b| b.stats.gb).unwrap_or(50.0);
                    if rand::thread_rng().gen_range(0.0..100.0) < gb_tendency {
                        PlayResult::Out(OutType::Groundout)
                    } else {
                        PlayResult::Out(OutType::Flyout)
                    }
                }
            };
            return (result, Some(contact_quality));
        }

        // Close contact - exact or adjacent match outside strike zone
        if (exact_match || adjacent_match) && !is_strike_zone {
            return if rng.gen_bool(0.7) {
                (PlayResult::Foul, Some(20))
            } else {
                (PlayResult::Out(OutType::Flyout), Some(15))
            };
        }

        // Swing and miss or weak contact
        (if is_strike_zone {
            if rng.gen_bool(0.6) {
                PlayResult::Strike // Swing and miss
            } else {
                PlayResult::Foul // Weak contact
            }
        } else {
            if rng.gen_bool(0.8) {
                PlayResult::Strike // Bad swing
            } else {
                PlayResult::Foul
            }
        }, Some(10))
    }

    fn locations_match(&self, loc1: PitchLocation, loc2: PitchLocation) -> bool {
        // Check if locations are adjacent (NOT exact match - that's checked separately)
        // This should only be used for weak contact, not perfect hits
        use PitchLocation::*;
        matches!(
            (loc1, loc2),
            (Up, UpInside) | (UpInside, Up) |
            (Up, UpOutside) | (UpOutside, Up) |
            (Inside, UpInside) | (UpInside, Inside) |
            (Inside, Middle) | (Middle, Inside) |
            (Inside, DownInside) | (DownInside, Inside) |
            (Outside, UpOutside) | (UpOutside, Outside) |
            (Outside, Middle) | (Middle, Outside) |
            (Outside, DownOutside) | (DownOutside, Outside) |
            (Down, DownInside) | (DownInside, Down) |
            (Down, DownOutside) | (DownOutside, Down) |
            (Middle, Up) | (Up, Middle) |
            (Middle, Down) | (Down, Middle)
        )
    }

    pub fn get_pitch_name(&self, idx: usize) -> &str {
        self.pitch_types.get(idx).map(|p| p.name).unwrap_or("Unknown")
    }

    /// Generate ball-in-play data from contact quality
    pub fn generate_ball_in_play(
        &self,
        contact_quality: i32,
        batter: Option<&Player>,
        _pitcher: Option<&Player>,
    ) -> Option<BallInPlay> {
        let mut rng = rand::thread_rng();
        
        // Determine ball type based on contact quality
        let (ball_type, speed, hang_time) = match contact_quality {
            85..=100 => {
                // Excellent contact - likely fly ball or line drive
                if rng.gen_bool(0.6) {
                    (BallType::FlyBall, rng.gen_range(80.0..100.0), rng.gen_range(60..90))
                } else {
                    (BallType::LineDrive, rng.gen_range(90.0..110.0), rng.gen_range(20..40))
                }
            }
            60..=84 => {
                // Good contact - mix of outcomes
                let roll = rng.gen_range(1..=10);
                match roll {
                    1..=3 => (BallType::FlyBall, rng.gen_range(70.0..90.0), rng.gen_range(50..70)),
                    4..=6 => (BallType::LineDrive, rng.gen_range(80.0..100.0), rng.gen_range(25..45)),
                    _ => (BallType::Grounder, rng.gen_range(60.0..90.0), 0),
                }
            }
            40..=59 => {
                // Weak contact - mostly grounders
                if rng.gen_bool(0.7) {
                    (BallType::Grounder, rng.gen_range(50.0..75.0), 0)
                } else {
                    (BallType::PopFly, rng.gen_range(40.0..60.0), rng.gen_range(40..60))
                }
            }
            _ => {
                // Very weak contact - grounders and pop flies
                let gb_tendency = batter.map(|b| b.stats.gb).unwrap_or(50.0);
                if rng.gen_range(0.0..100.0) < gb_tendency {
                    (BallType::Grounder, rng.gen_range(40.0..65.0), 0)
                } else {
                    (BallType::PopFly, rng.gen_range(30.0..50.0), rng.gen_range(30..50))
                }
            }
        };

        // Determine field direction based on swing and random variation
        let direction = self.generate_field_direction(&ball_type);

        Some(BallInPlay {
            ball_type,
            direction,
            speed,
            hang_time,
            initial_contact_quality: contact_quality,
        })
    }

    fn generate_field_direction(&self, ball_type: &BallType) -> FieldDirection {
        let mut rng = rand::thread_rng();
        
        // Different ball types have different distribution
        match ball_type {
            BallType::Grounder => {
                let roll = rng.gen_range(1..=9);
                match roll {
                    1 => FieldDirection::ThirdBase,
                    2..=3 => FieldDirection::Shortstop,
                    4..=6 => FieldDirection::SecondBase,
                    7..=8 => FieldDirection::FirstBase,
                    _ => FieldDirection::Shortstop,
                }
            }
            BallType::LineDrive => {
                let roll = rng.gen_range(1..=9);
                match roll {
                    1 => FieldDirection::LeftField,
                    2 => FieldDirection::LeftCenter,
                    3..=4 => FieldDirection::CenterField,
                    5 => FieldDirection::RightCenter,
                    6 => FieldDirection::RightField,
                    7 => FieldDirection::ThirdBase,
                    8 => FieldDirection::Shortstop,
                    _ => FieldDirection::FirstBase,
                }
            }
            BallType::FlyBall | BallType::PopFly => {
                let roll = rng.gen_range(1..=7);
                match roll {
                    1 => FieldDirection::LeftField,
                    2 => FieldDirection::LeftCenter,
                    3..=4 => FieldDirection::CenterField,
                    5 => FieldDirection::RightCenter,
                    _ => FieldDirection::RightField,
                }
            }
        }
    }

    /// Calculate fielding outcome based on user timing and ball characteristics
    pub fn calculate_fielding_result(
        &self,
        ball: &BallInPlay,
        catch_timing: u8,  // How many frames it took to position
        perfect_timing: u8, // Optimal timing window
    ) -> (PlayResult, f32) {  // Returns (result, success_chance)
        let mut rng = rand::thread_rng();
        
        // Calculate timing accuracy (closer to perfect = higher accuracy)
        let timing_diff = (catch_timing as i32 - perfect_timing as i32).abs() as f32;
        // Much more forgiving timing window - within 15 frames is good
        let timing_accuracy = 1.0 - (timing_diff / 15.0).min(1.0);

        // Base catch success rate - fielders catch MOST balls
        // Since we only field hits now, success = preventing the hit (catching it for an out)
        let base_success = match ball.ball_type {
            BallType::PopFly => 0.98,     // Almost always caught
            BallType::FlyBall => 0.90,    // Usually caught
            BallType::LineDrive => 0.75,  // Harder but still mostly caught
            BallType::Grounder => 0.85,   // Most are fielded
        };

        // Speed only slightly affects difficulty for very fast balls
        let speed_penalty = if ball.speed > 95.0 {
            (ball.speed - 95.0) / 300.0  // Minimal penalty
        } else {
            0.0
        };
        
        // Calculate final success chance
        // Good timing (>0.6) gives nearly full success rate
        // Bad timing still gives decent chance
        let success_chance = if timing_accuracy > 0.6 {
            (base_success - speed_penalty).max(0.1)
        } else {
            // Poor timing - reduced but still possible
            ((base_success - speed_penalty) * (0.5 + timing_accuracy * 0.5)).max(0.1)
        };

        // Determine outcome
        let result = if rng.gen_range(0.0..1.0) < success_chance {
            // Successful catch/field
            match ball.ball_type {
                BallType::FlyBall | BallType::PopFly | BallType::LineDrive => {
                    PlayResult::Out(OutType::Flyout)
                }
                BallType::Grounder => {
                    PlayResult::Out(OutType::Groundout)
                }
            }
        } else {
            // Ball gets through - determine hit type
            self.ball_gets_through(ball)
        };
        
        (result, success_chance)
    }

    pub fn ball_gets_through(&self, ball: &BallInPlay) -> PlayResult {
        let mut rng = rand::thread_rng();
        
        // Use original contact quality to determine hit
        match ball.initial_contact_quality {
            85..=100 => {
                // Great contact that got through
                if ball.speed > 95.0 {
                    if rng.gen_bool(0.4) {
                        PlayResult::Hit(HitType::HomeRun)
                    } else {
                        PlayResult::Hit(HitType::Triple)
                    }
                } else {
                    let roll = rng.gen_range(1..=3);
                    match roll {
                        1 => PlayResult::Hit(HitType::Triple),
                        _ => PlayResult::Hit(HitType::Double),
                    }
                }
            }
            60..=84 => {
                let roll = rng.gen_range(1..=10);
                match roll {
                    1..=2 => PlayResult::Hit(HitType::Triple),
                    3..=5 => PlayResult::Hit(HitType::Double),
                    _ => PlayResult::Hit(HitType::Single),
                }
            }
            _ => PlayResult::Hit(HitType::Single),
        }
    }
}
