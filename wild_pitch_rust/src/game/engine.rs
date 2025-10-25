use crate::game::{GameEvent, GameState, PlayResult, HitType, InningHalf};
use crate::players::{Player, Position};
use crate::utils::WildPitchRng;
use anyhow::Result;

pub struct GameEngine {
    rng: WildPitchRng,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            rng: WildPitchRng::new(),
        }
    }

    pub fn simulate_at_bat(&mut self, game_state: &mut GameState) -> Result<GameEvent> {
        let batter_id = game_state.situation.current_batter_id.clone();
        let pitcher_id = game_state.situation.current_pitcher_id.clone();
        
        // Get player data for simulation (without borrowing game_state mutably)
        let play_result = {
            let batter = self.get_current_batter(game_state)?;
            let pitcher = self.get_current_pitcher(game_state)?;
            self.determine_play_result(batter, pitcher)
        };
        
        let mut event = GameEvent::new(
            game_state.situation.inning,
            game_state.situation.inning_half,
            game_state.situation.outs,
            batter_id,
            pitcher_id,
            play_result,
        );

        // Process the result and update game state
        self.process_play_result(&mut event, game_state)?;

        Ok(event)
    }

    fn get_current_batter<'a>(&self, game_state: &'a GameState) -> Result<&'a Player> {
        let batting_team = game_state.current_batting_team();
        batting_team.get_player(&game_state.situation.current_batter_id)
            .ok_or_else(|| anyhow::anyhow!("Current batter not found"))
    }

    fn get_current_pitcher<'a>(&self, game_state: &'a GameState) -> Result<&'a Player> {
        let pitching_team = game_state.current_pitching_team();
        pitching_team.get_player(&game_state.situation.current_pitcher_id)
            .ok_or_else(|| anyhow::anyhow!("Current pitcher not found"))
    }

    fn determine_play_result(&mut self, batter: &Player, pitcher: &Player) -> PlayResult {
        // Get effective ratings based on fatigue and situational factors
        let batter_stats = batter.batter.as_ref().unwrap();
        let pitcher_stats = pitcher.pitcher.as_ref().unwrap();
        
        let contact_chance = batter_stats.effective_contact_rate();
        let pitcher_control = pitcher_stats.effective_control();
        
        // Base probabilities - these would be much more sophisticated in the full game
        let walk_chance = 0.08 * (1.0 - pitcher_control) * (1.0 + batter_stats.tendencies.patience_rating);
        let strikeout_chance = 0.20 * pitcher_control * (1.0 - contact_chance);
        let hit_chance = 0.25 * contact_chance * (1.0 - pitcher_control);
        
        let roll = self.rng.gen_range(0.0..1.0);
        
        if roll < walk_chance {
            PlayResult::Walk
        } else if roll < walk_chance + strikeout_chance {
            PlayResult::Strikeout
        } else if roll < walk_chance + strikeout_chance + hit_chance {
            self.determine_hit_type(batter, pitcher)
        } else {
            self.determine_out_type()
        }
    }

    fn determine_hit_type(&mut self, batter: &Player, _pitcher: &Player) -> PlayResult {
        let batter_stats = batter.batter.as_ref().unwrap();
        let power = batter_stats.effective_power_rating();
        
        let roll = self.rng.gen_range(0.0..1.0);
        
        if roll < power * 0.05 { // Home run chance
            PlayResult::Hit(HitType::HomeRun)
        } else if roll < power * 0.15 { // Double chance
            PlayResult::Hit(HitType::Double(None))
        } else if roll < power * 0.18 { // Triple chance (rare)
            PlayResult::Hit(HitType::Triple(None))
        } else {
            PlayResult::Hit(HitType::Single(None))
        }
    }

    fn determine_out_type(&mut self) -> PlayResult {
        let roll = self.rng.gen_range(0.0..1.0);
        
        if roll < 0.4 {
            // Ground out
            let fielder = self.random_infield_position();
            PlayResult::Hit(HitType::GroundOut(fielder))
        } else if roll < 0.8 {
            // Fly out
            let fielder = self.random_outfield_position();
            PlayResult::Hit(HitType::FlyOut(fielder))
        } else {
            // Line out
            let fielder = self.random_defensive_position();
            PlayResult::Hit(HitType::LineOut(fielder))
        }
    }

    fn random_infield_position(&mut self) -> Position {
        let positions = [Position::FirstBase, Position::SecondBase, Position::ThirdBase, Position::Shortstop];
        positions[self.rng.gen_range(0..positions.len())]
    }

    fn random_outfield_position(&mut self) -> Position {
        let positions = [Position::LeftField, Position::CenterField, Position::RightField];
        positions[self.rng.gen_range(0..positions.len())]
    }

    fn random_defensive_position(&mut self) -> Position {
        let positions = [
            Position::FirstBase, Position::SecondBase, Position::ThirdBase, Position::Shortstop,
            Position::LeftField, Position::CenterField, Position::RightField
        ];
        positions[self.rng.gen_range(0..positions.len())]
    }

    fn process_play_result(&mut self, event: &mut GameEvent, game_state: &mut GameState) -> Result<()> {
        let batter_name = {
            let batter = self.get_current_batter(game_state)?;
            batter.name.clone()
        };
        
        match &event.result {
            PlayResult::Walk => {
                self.process_walk(game_state);
                event.description = format!("{} walks", batter_name);
            },
            PlayResult::Strikeout => {
                game_state.situation.add_out();
                event.description = format!("{} strikes out", batter_name);
            },
            PlayResult::Hit(hit_type) => {
                event.runs_scored = self.process_hit(hit_type, game_state);
                event.description = event.format_play_description(&batter_name);
            },
            PlayResult::HitByPitch => {
                self.process_walk(game_state); // Similar to walk
                event.description = format!("{} hit by pitch", batter_name);
            },
            _ => {
                // Handle other play results
                if event.is_out() {
                    game_state.situation.add_out();
                }
                event.description = event.format_play_description(&batter_name);
            }
        }

        // Add runs to score
        if event.runs_scored > 0 {
            let is_home_team = matches!(game_state.situation.inning_half, InningHalf::Bottom);
            for _ in 0..event.runs_scored {
                game_state.score.add_run(is_home_team, game_state.situation.inning);
            }
        }

        // Add to play-by-play
        game_state.add_play(event.description.clone());

        // Advance to next batter if the current at-bat is over
        if self.is_at_bat_over(&event.result) {
            game_state.advance_to_next_batter();
        }

        // Check if inning is over
        if game_state.situation.is_inning_over() {
            game_state.end_inning();
        }

        // Check if game is over
        game_state.check_game_end();

        Ok(())
    }

    fn process_walk(&self, game_state: &mut GameState) {
        let batter_id = game_state.situation.current_batter_id.clone();
        
        // Move runners if bases are loaded
        if game_state.situation.runners.is_bases_loaded() {
            // Runner on third scores
            game_state.situation.runners.set_runner(crate::game::state::Base::Third, None);
            // TODO: Add run to score and update event
        }
        
        // Advance runners
        if let Some(runner_on_second) = game_state.situation.runners.second.take() {
            game_state.situation.runners.set_runner(crate::game::state::Base::Third, Some(runner_on_second));
        }
        
        if let Some(runner_on_first) = game_state.situation.runners.first.take() {
            game_state.situation.runners.set_runner(crate::game::state::Base::Second, Some(runner_on_first));
        }
        
        // Batter goes to first
        game_state.situation.runners.set_runner(crate::game::state::Base::First, Some(batter_id));
    }

    fn process_hit(&self, hit_type: &HitType, game_state: &mut GameState) -> u8 {
        let batter_id = game_state.situation.current_batter_id.clone();
        let mut runs_scored = 0;

        match hit_type {
            HitType::Single(_) => {
                // Runners advance one base
                if let Some(runner_on_third) = game_state.situation.runners.third.take() {
                    runs_scored += 1;
                    // Runner scores
                }
                
                if let Some(runner_on_second) = game_state.situation.runners.second.take() {
                    game_state.situation.runners.set_runner(crate::game::state::Base::Third, Some(runner_on_second));
                }
                
                if let Some(runner_on_first) = game_state.situation.runners.first.take() {
                    game_state.situation.runners.set_runner(crate::game::state::Base::Second, Some(runner_on_first));
                }
                
                game_state.situation.runners.set_runner(crate::game::state::Base::First, Some(batter_id));
            },
            HitType::Double(_) => {
                // Runners advance two bases
                if let Some(_runner_on_third) = game_state.situation.runners.third.take() {
                    runs_scored += 1;
                }
                
                if let Some(_runner_on_second) = game_state.situation.runners.second.take() {
                    runs_scored += 1;
                }
                
                if let Some(runner_on_first) = game_state.situation.runners.first.take() {
                    game_state.situation.runners.set_runner(crate::game::state::Base::Third, Some(runner_on_first));
                }
                
                game_state.situation.runners.set_runner(crate::game::state::Base::Second, Some(batter_id));
            },
            HitType::Triple(_) => {
                // All runners score, batter to third
                if game_state.situation.runners.third.is_some() {
                    runs_scored += 1;
                }
                if game_state.situation.runners.second.is_some() {
                    runs_scored += 1;
                }
                if game_state.situation.runners.first.is_some() {
                    runs_scored += 1;
                }
                
                game_state.situation.runners.clear();
                game_state.situation.runners.set_runner(crate::game::state::Base::Third, Some(batter_id));
            },
            HitType::HomeRun => {
                // Everyone scores
                runs_scored += 1; // Batter
                if game_state.situation.runners.first.is_some() {
                    runs_scored += 1;
                }
                if game_state.situation.runners.second.is_some() {
                    runs_scored += 1;
                }
                if game_state.situation.runners.third.is_some() {
                    runs_scored += 1;
                }
                
                game_state.situation.runners.clear();
            },
            HitType::GroundOut(_) | HitType::FlyOut(_) | HitType::LineOut(_) | HitType::PopOut(_) => {
                game_state.situation.add_out();
                // TODO: Handle potential runner advancement on sacrifice flies
            },
        }

        runs_scored
    }

    fn is_at_bat_over(&self, result: &PlayResult) -> bool {
        !matches!(result, PlayResult::Ball | PlayResult::Strike | PlayResult::FoulBall)
    }

    pub fn simulate_inning(&mut self, game_state: &mut GameState) -> Result<Vec<GameEvent>> {
        let mut events = Vec::new();
        
        while !game_state.situation.is_inning_over() && !game_state.is_game_over() {
            let event = self.simulate_at_bat(game_state)?;
            events.push(event);
        }
        
        Ok(events)
    }

    pub fn simulate_game(&mut self, game_state: &mut GameState) -> Result<Vec<GameEvent>> {
        game_state.start_game();
        let mut all_events = Vec::new();
        
        while !game_state.is_game_over() {
            let inning_events = self.simulate_inning(game_state)?;
            all_events.extend(inning_events);
        }
        
        Ok(all_events)
    }
}