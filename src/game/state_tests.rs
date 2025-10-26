#[cfg(test)]
mod tests {
    use crate::game::{constants::*, GameState, InningHalf};

    #[test]
    fn test_new_game_state() {
        let state = GameState::new();
        assert_eq!(state.inning, 1);
        assert_eq!(state.half, InningHalf::Top);
        assert_eq!(state.outs, 0);
        assert_eq!(state.balls, 0);
        assert_eq!(state.strikes, 0);
        assert_eq!(state.home_score, 0);
        assert_eq!(state.away_score, 0);
        assert_eq!(state.bases, [false, false, false]);
        assert_eq!(state.current_batter_idx, 0);
        assert_eq!(state.game_over, false);
        assert_eq!(state.quit_requested, false);
    }

    #[test]
    fn test_add_out() {
        let mut state = GameState::new();
        state.home_team = Some("NYY".to_string());
        state.away_team = Some("BOS".to_string());
        
        assert_eq!(state.outs, 0);
        state.add_out();
        assert_eq!(state.outs, 1);
        state.add_out();
        assert_eq!(state.outs, 2);
    }

    #[test]
    fn test_three_outs_ends_half_inning() {
        let mut state = GameState::new();
        state.home_team = Some("NYY".to_string());
        state.away_team = Some("BOS".to_string());
        
        assert_eq!(state.half, InningHalf::Top);
        assert_eq!(state.outs, 0);
        
        state.add_out();
        state.add_out();
        state.add_out(); // Third out should trigger half-inning change
        
        assert_eq!(state.half, InningHalf::Bottom);
        assert_eq!(state.outs, 0); // Outs reset
    }

    #[test]
    fn test_advance_runners_single() {
        let mut state = GameState::new();
        state.home_team = Some("NYY".to_string());
        state.away_team = Some("BOS".to_string());
        state.half = InningHalf::Top; // Away team batting
        
        // Single with nobody on
        state.advance_runners(1);
        assert_eq!(state.bases, [true, false, false]);
        assert_eq!(state.away_score, 0);
        
        // Another single - runner advances to 2nd
        state.advance_runners(1);
        assert_eq!(state.bases, [true, true, false]);
        assert_eq!(state.away_score, 0);
    }

    #[test]
    fn test_advance_runners_home_run() {
        let mut state = GameState::new();
        state.home_team = Some("NYY".to_string());
        state.away_team = Some("BOS".to_string());
        state.half = InningHalf::Top; // Away team batting
        
        // Load the bases
        state.bases = [true, true, true];
        
        // Home run clears bases and scores 4
        state.advance_runners(4);
        assert_eq!(state.bases, [false, false, false]);
        assert_eq!(state.away_score, 4); // 3 runners + batter
    }

    #[test]
    fn test_advance_runners_walk() {
        let mut state = GameState::new();
        state.home_team = Some("NYY".to_string());
        state.away_team = Some("BOS".to_string());
        state.half = InningHalf::Bottom; // Home team batting
        
        // Walk with nobody on
        state.advance_runners(0);
        assert_eq!(state.bases, [true, false, false]);
        assert_eq!(state.home_score, 0);
    }

    #[test]
    fn test_end_half_inning_resets_bases() {
        let mut state = GameState::new();
        state.home_team = Some("NYY".to_string());
        state.away_team = Some("BOS".to_string());
        state.bases = [true, true, true];
        state.outs = 2;
        
        state.add_out(); // Third out
        
        // Bases should be cleared
        assert_eq!(state.bases, [false, false, false]);
        assert_eq!(state.outs, 0);
    }

    #[test]
    fn test_game_ends_after_9_innings() {
        let mut state = GameState::new();
        state.home_team = Some("NYY".to_string());
        state.away_team = Some("BOS".to_string());
        state.inning = INNINGS_PER_GAME;
        state.half = InningHalf::Bottom;
        state.away_score = 5;
        state.home_score = 3;
        state.outs = 2;
        
        assert_eq!(state.game_over, false);
        state.add_out(); // End bottom of 9th
        
        assert_eq!(state.game_over, true);
    }

    #[test]
    fn test_extra_innings_if_tied() {
        let mut state = GameState::new();
        state.home_team = Some("NYY".to_string());
        state.away_team = Some("BOS".to_string());
        state.inning = INNINGS_PER_GAME;
        state.half = InningHalf::Bottom;
        state.away_score = 3;
        state.home_score = 3; // Tied!
        state.outs = 2;
        
        state.add_out(); // End bottom of 9th
        
        // Game should continue to extra innings
        assert_eq!(state.game_over, false);
        assert_eq!(state.inning, INNINGS_PER_GAME + 1);
        assert_eq!(state.half, InningHalf::Top);
    }

    #[test]
    fn test_balls_and_strikes() {
        let mut state = GameState::new();
        state.home_team = Some("NYY".to_string());
        state.away_team = Some("BOS".to_string());
        
        assert_eq!(state.balls, 0);
        assert_eq!(state.strikes, 0);
        
        // Add strikes
        state.strikes = 1;
        assert_eq!(state.strikes, 1);
        
        state.strikes = 2;
        assert_eq!(state.strikes, 2);
        
        // Add balls
        state.balls = 3;
        assert_eq!(state.balls, 3);
    }
}
