#[cfg(test)]
mod tests {
    use crate::game::{GameEngine, PitchLocation};

    #[test]
    fn test_pitch_location_from_numpad() {
        assert!(matches!(PitchLocation::from_numpad(7), PitchLocation::UpInside));
        assert!(matches!(PitchLocation::from_numpad(8), PitchLocation::Up));
        assert!(matches!(PitchLocation::from_numpad(9), PitchLocation::UpOutside));
        assert!(matches!(PitchLocation::from_numpad(4), PitchLocation::Inside));
        assert!(matches!(PitchLocation::from_numpad(5), PitchLocation::Middle));
        assert!(matches!(PitchLocation::from_numpad(6), PitchLocation::Outside));
        assert!(matches!(PitchLocation::from_numpad(1), PitchLocation::DownInside));
        assert!(matches!(PitchLocation::from_numpad(2), PitchLocation::Down));
        assert!(matches!(PitchLocation::from_numpad(3), PitchLocation::DownOutside));
    }

    #[test]
    fn test_pitch_location_is_strike() {
        // Strike zone locations
        assert!(PitchLocation::Up.is_strike());
        assert!(PitchLocation::Middle.is_strike());
        assert!(PitchLocation::Down.is_strike());
        assert!(PitchLocation::Inside.is_strike());
        assert!(PitchLocation::Outside.is_strike());
        
        // Ball locations (corners)
        assert!(!PitchLocation::UpInside.is_strike());
        assert!(!PitchLocation::UpOutside.is_strike());
        assert!(!PitchLocation::DownInside.is_strike());
        assert!(!PitchLocation::DownOutside.is_strike());
    }

    #[test]
    fn test_pitch_result_no_swing_strike() {
        let engine = GameEngine::new();
        let pitch_loc = PitchLocation::Middle; // Strike zone
        let swing_loc = None; // No swing
        
        let (result, _) = engine.calculate_pitch_result(pitch_loc, swing_loc, 0, None, None, 1.0);
        
        assert!(matches!(result, crate::game::PlayResult::Strike));
    }

    #[test]
    fn test_pitch_result_no_swing_ball() {
        let engine = GameEngine::new();
        let pitch_loc = PitchLocation::UpInside; // Outside strike zone
        let swing_loc = None; // No swing
        
        let (result, _) = engine.calculate_pitch_result(pitch_loc, swing_loc, 0, None, None, 1.0);
        
        assert!(matches!(result, crate::game::PlayResult::Ball));
    }

    #[test]
    fn test_engine_has_pitch_types() {
        let engine = GameEngine::new();
        assert_eq!(engine.pitch_types.len(), 4);
        assert_eq!(engine.get_pitch_name(0), "Fastball");
        assert_eq!(engine.get_pitch_name(1), "Curveball");
        assert_eq!(engine.get_pitch_name(2), "Slider");
        assert_eq!(engine.get_pitch_name(3), "Changeup");
    }
}
