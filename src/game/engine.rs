use crate::game::state::{HitType, OutType, PitchLocation, PlayResult};
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
    ) -> PlayResult {
        let mut rng = rand::thread_rng();

        // No swing
        if swing_location.is_none() {
            return if pitch_location.is_strike() {
                PlayResult::Strike
            } else {
                PlayResult::Ball
            };
        }

        let swing_loc = swing_location.unwrap();
        
        // Calculate timing and location accuracy
        let location_match = self.locations_match(pitch_location, swing_loc);
        let is_strike_zone = pitch_location.is_strike();

        // Perfect contact - use player stats to influence outcomes
        if location_match && is_strike_zone {
            let mut contact_quality = rng.gen_range(1..=100);
            
            // Adjust contact quality based on batter's skills
            if let Some(batter) = batter {
                // Better batters (higher barrel %) get bonus to contact quality
                let skill_bonus = (batter.stats.barrel_percent * 2.0) as i32;
                contact_quality = (contact_quality + skill_bonus).min(100);
            }

            // Adjust based on pitcher's ability to limit hard contact
            if let Some(pitcher) = pitcher {
                // Better pitchers (lower barrel % allowed) reduce contact quality
                let pitcher_penalty = (pitcher.stats.barrel_percent * 1.5) as i32;
                contact_quality = (contact_quality - pitcher_penalty).max(1);
            }

            return match contact_quality {
                85..=100 => {
                    // Home run chance influenced by batter's power stats
                    let hr_chance = if let Some(batter) = batter {
                        (batter.stats.max_distance as f32 / 450.0 * 100.0) as u32
                    } else { 50 };
                    
                    if rng.gen_range(1..=100) <= hr_chance.min(40) {
                        PlayResult::Hit(HitType::HomeRun)
                    } else {
                        PlayResult::Hit(HitType::Triple)
                    }
                }
                60..=84 => {
                    let hit_roll = rng.gen_range(1..=10);
                    match hit_roll {
                        1..=2 => PlayResult::Hit(HitType::Triple),
                        3..=5 => PlayResult::Hit(HitType::Double),
                        _ => PlayResult::Hit(HitType::Single),
                    }
                }
                40..=59 => PlayResult::Hit(HitType::Single),
                25..=39 => PlayResult::Foul,
                _ => {
                    // Ground ball vs fly ball tendency based on batter stats
                    let gb_tendency = batter.map(|b| b.stats.gb).unwrap_or(50.0);
                    if rng.gen_range(0.0..100.0) < gb_tendency {
                        PlayResult::Out(OutType::Groundout)
                    } else {
                        let out_roll = rng.gen_range(1..=2);
                        match out_roll {
                            1 => PlayResult::Out(OutType::Flyout),
                            _ => PlayResult::Out(OutType::LineOut),
                        }
                    }
                }
            };
        }

        // Close contact
        if location_match && !is_strike_zone {
            return if rng.gen_bool(0.7) {
                PlayResult::Foul
            } else {
                PlayResult::Out(OutType::Flyout)
            };
        }

        // Swing and miss or weak contact
        if is_strike_zone {
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
        }
    }

    fn locations_match(&self, loc1: PitchLocation, loc2: PitchLocation) -> bool {
        // Exact match
        if std::mem::discriminant(&loc1) == std::mem::discriminant(&loc2) {
            return true;
        }

        // Adjacent locations also count as match
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
}
