# Changelog

All notable changes to Baseball TUI will be documented in this file.

## [0.2.1] - 2025-10-28

### Added - MAJOR FEATURE: Realistic Batting Timing System!

**Revolutionary timing mechanics replace instant swinging!**

- **3-Second Pitch Clock**: Visual countdown before each pitch for realistic pacing
- **Ball Approach Animation**: Watch the ball travel from mound to plate with real-time position tracking
- **Timing Windows**: Multiple swing timing zones with different outcomes
  - **Perfect Timing**: 0.2-second window for maximum contact quality
  - **Good Timing**: Early/Late zones provide decent contact
  - **Poor Timing**: Too early/late results in weak contact or swing-and-miss
- **Visual Timing Display**: 
  - Pitch clock countdown with progress bar
  - Ball position indicator showing travel to home plate
  - Real-time timing feedback (Perfect!, Early, Late, etc.)
  - Emoji indicators for timing quality
- **Swing-and-Miss Mechanics**: Finally implemented realistic strikeouts!
  - Poor timing = high chance of missing completely
  - Bad pitch location + poor timing = almost guaranteed miss
  - Timing affects contact quality multiplier (0.1x to 1.3x)
- **Enhanced Audio Cues**: Miss sounds now properly trigger on swing-and-miss
- **Realistic Baseball Pacing**: 3-second between-pitch timer mimics real game flow

### Changed
- Batting phase completely redesigned around timing mechanics
- `PitchState` expanded with `PitchClock` and `BallApproaching` states
- New `SwingTiming` enum tracks timing quality (TooEarly, Early, Perfect, Late, TooLate, NoSwing)
- `GameEngine` enhanced with timing-aware pitch calculation
- UI layout reorganized to accommodate timing display
- Swing-and-miss now actually occurs (was previously missing from game)
- Contact quality formula now includes timing multipliers

### Technical
- New timing constants for pitch clock, ball approach, and swing windows
- Enhanced input handler with timing calculation functions
- Timing-aware game state updates and animation frames
- Improved game engine with `calculate_pitch_result_with_timing()` method
- Visual timing display with progress bars and ball position tracking
- Proper borrow checker handling for complex game state updates

### Game Balance
- Timing is now crucial for successful hitting
- Perfect timing provides 30% contact quality bonus
- Early/Late timing reduces contact quality to 60%
- Very poor timing almost guarantees swing-and-miss
- Realistic strikeout rates now possible through timing mechanics

## [0.2.0] - 2025-10-26

### Added - MAJOR FEATURE: Interactive Fielding Gameplay! 🎮⚾

**Revolutionary fielding mechanics replace automatic outs!**

- **Real-Time Fielding**: Players now field balls in real-time instead of automatic outcomes
- **Ball Physics**: Realistic ball trajectories based on contact quality
  - Grounders: Fast-moving ground balls to infielders
  - Line Drives: Hard-hit balls with medium hang time
  - Fly Balls: High arcing balls to outfielders
  - Pop Flies: Easy high flies with long hang time
- **Timing-Based Catching**: Press SPACE at the right moment to field/catch
  - Perfect timing = successful out
  - Poor timing = ball gets through for a hit
- **Field Directions**: Balls hit to 9 different field positions
  - Infield: 1B, 2B, 3B, SS
  - Outfield: LF, LC, CF, RC, RF
- **Dynamic Difficulty**: Ball speed and type affect catch difficulty
  - Faster balls = harder to field
  - Line drives harder than pop flies
  - Contact quality influences ball characteristics
- **Audio Feedback**: Appropriate sounds for catches, grounders, and hits
- **Visual Indicators**: Real-time display of ball type, direction, and time remaining

### Changed
- Ball-in-play results now trigger interactive fielding minigame
- Game state expanded to support fielding cursor and ball tracking
- Enhanced UI to show fielding information and countdown timer
- Improved play result processing to handle fielding outcomes

### Technical
- New `BallInPlay` struct tracks ball physics (type, direction, speed, hang time)
- New `BallType` enum (Grounder, LineDrive, FlyBall, PopFly)
- New `FieldDirection` enum for 9 field positions
- New `PitchState::Fielding` game state for fielding mode
- Enhanced `GameEngine` with ball trajectory generation
- Timing-based outcome calculation in `calculate_fielding_result()`
- Player stats influence ball physics and fielding difficulty

### Game Balance
- Contact quality determines ball type probability
- Better contact = harder/faster balls
- Weaker contact = easier grounders and pop flies
- Auto-resolve if player doesn't react in time (ball gets through)
- Optimal timing window varies by ball type

## [0.1.1] - 2025-10-25

### Changed
- **MAJOR IMPROVEMENT**: Replaced simplistic diamond with professional ASCII baseball field
- Field now uses high-quality art from https://github.com/ceejay3264/ascii_baseball
- Added realistic outfield fence curve
- Added proper infield dirt boundaries
- Added detailed pitcher's mound visualization
- Added 9 fielder position markers
- Improved visual clarity of runner positions
- Enhanced border styling with colors
- Updated documentation to reflect new field layout

### Added
- `ui_enhanced.rs` - Optional enhanced renderer with ball position indicators
- `UPDATE_NOTES.md` - Detailed update documentation
- Proper attribution for ASCII art source

## [0.1.0] - 2025-10-25

### Initial Release

### Added
- **Core Gameplay**
  - Full 9-inning baseball simulation
  - Pitching mechanics with 4 pitch types (Fastball, Curveball, Slider, Changeup)
  - Batting mechanics with 9-zone strike zone
  - Automatic baserunning and scoring
  - Balls, strikes, and outs tracking
  - Inning progression (top/bottom)

- **User Interface**
  - ASCII diamond display with runner indicators
  - Scoreboard showing inning, score, count, and batter
  - Real-time control instructions
  - Color-coded game states (pitching, swinging, ball in play)
  - Message system for play results

- **Technical Features**
  - Ratatui-based TUI with double-buffering (no flicker!)
  - Crossterm for cross-platform terminal support
  - 30 FPS frame rate with smooth animations
  - Non-blocking input polling
  - Proper terminal setup/cleanup
  - Windows conhost compatibility

- **Game Mechanics**
  - Directional pitching (9 zones)
  - Directional batting (9 zones)
  - Contact quality based on location matching
  - Random outcomes for realistic variation
  - Hit types: Single, Double, Triple, Home Run
  - Out types: Strikeout, Groundout, Flyout, Lineout
  - Automatic runner advancement
  - Force plays on walks
  - Scoring system