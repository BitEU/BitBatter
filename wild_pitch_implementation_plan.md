# Wild Pitch Baseball Simulation - Rust Implementation Plan

## Overview
This document outlines the comprehensive plan to recreate Wild Pitch (1993) in Rust for Windows 10 terminal, incorporating ALL features from the original game. Wild Pitch was a sophisticated baseball simulation with realistic statistical gameplay, comprehensive management options, and detailed reporting.

## Project Structure

```
wild_pitch_rust/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── game/
│   │   ├── mod.rs
│   │   ├── engine.rs          # Core game simulation engine
│   │   ├── events.rs          # Play outcomes and events
│   │   └── state.rs           # Game state management
│   ├── teams/
│   │   ├── mod.rs
│   │   ├── roster.rs          # Player rosters and team data
│   │   ├── lineup.rs          # Lineup management
│   │   └── stats.rs           # Team statistics
│   ├── players/
│   │   ├── mod.rs
│   │   ├── batter.rs          # Batting statistics and tendencies
│   │   ├── pitcher.rs         # Pitching statistics and tendencies
│   │   └── fielder.rs         # Fielding statistics
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── terminal.rs        # Terminal interface management
│   │   ├── windows.rs         # Window system for scoreboard, lineup, etc.
│   │   ├── menus.rs           # Menu system
│   │   └── dialogs.rs         # Dialog boxes
│   ├── data/
│   │   ├── mod.rs
│   │   ├── loader.rs          # Data file loading
│   │   └── serialization.rs   # Save/load game state
│   └── utils/
│       ├── mod.rs
│       ├── config.rs          # Configuration management
│       └── random.rs          # Random number generation
├── data/
│   ├── teams/                 # Team data files
│   ├── players/              # Player statistics files
│   └── leagues/              # League configuration
├── saves/                    # Saved games
└── reports/                  # Generated reports and scorecards
```

## Core Features Implementation

### 1. Game Engine (src/game/)

#### 1.1 Core Simulation Engine
- **Statistical-based outcomes**: Each play result based on actual player statistics
- **Realistic baseball physics**: Factor in ballpark dimensions, weather conditions
- **Situational awareness**: Different outcomes based on runners on base, outs, inning
- **Pitcher fatigue system**: Performance degradation over innings pitched
- **Momentum factors**: Hot/cold streaks affecting performance

#### 1.2 Play Resolution System
- **Batter vs Pitcher matchups**: Consider L/R handedness, historical performance
- **Defensive positioning effects**: Infield in/out, shifts, guard lines
- **Base running logic**: Automatic advancement, stolen base attempts
- **Error simulation**: Fielding errors based on player defensive ratings
- **Special situations**: Hit & run, sacrifice attempts, squeeze plays

### 2. User Interface System (src/ui/)

#### 2.1 Terminal-based Windows (Mimicking Original GUI)
- **Scoreboard Window**: Live scoring by inning, runs/hits/errors
- **Lineup Cards Window**: Current lineups for both teams with positions
- **Play-by-Play Window**: Real-time game events and outcomes
- **Ballpark Window**: ASCII art field representation with player positions
- **Statistics Windows**: Batting, pitching, and team stats with pagination

#### 2.2 Menu System
- **Game Menu**: New game, save, load, exit
- **Manager Menus**: Separate offensive/defensive management options
- **Stats Menu**: Access to all statistical categories
- **Options Menu**: Game settings, league rules, display options
- **Window Menu**: Window management (tile, cascade, zoom)

#### 2.3 Dialog System
- **Team Selection**: Choose visiting and home teams from leagues
- **Lineup Management**: Drag-and-drop style lineup construction
- **Game Options**: Human vs computer, league vs exhibition
- **League Options**: DH rule, scoring rules (sacrifice fly counts)
- **Defensive Positioning**: Infield/outfield alignment choices

### 3. Team and Player Management (src/teams/, src/players/)

#### 3.1 Player Statistics System
- **Comprehensive batting stats**: AVG, OBP, SLG, HR, RBI, SB, plus situational stats
- **Advanced pitching stats**: ERA, WHIP, K/9, BB/9, saves, holds
- **Fielding statistics**: Fielding percentage, range factor, assists
- **Situational splits**: vs LHP/RHP, with runners in scoring position
- **Historical tendencies**: Clutch performance, platoon splits

#### 3.2 Lineup Management
- **Flexible positioning**: Players can play multiple positions with different ratings
- **Automatic lineup generation**: AI-suggested optimal lineups
- **Substitution tracking**: Pinch hitters, pinch runners, defensive replacements
- **Pitcher usage tracking**: Days rest, pitch counts, starter vs reliever roles

### 4. Managerial Strategy Options

#### 4.1 Offensive Management
- **Pinch Hitting**: Replace current batter with bench player
- **Pinch Running**: Substitute faster runner for base runner
- **Base Stealing**: Attempt stolen base with success based on runner speed vs catcher arm
- **Hit & Run**: Runner goes on pitch, batter must make contact
- **Sacrifice Bunting**: Advance runner with sacrifice attempt
- **Squeeze Play**: Runner on 3rd breaks for home on bunt attempt

#### 4.2 Defensive Management
- **Pitching Changes**: Bring in relievers from bullpen
- **Defensive Substitutions**: Replace fielders for defensive improvement
- **Fielding Position Swaps**: Move players to different positions
- **Defensive Positioning**: 
  - Infield: Normal depth, drawn in, shifted left/right, guard lines
  - Outfield: Normal depth, shallow, shifted left/right
- **Intentional Walks**: Strategic free passes

### 5. Statistical Engine

#### 5.1 Real-time Statistics Tracking
- **Game statistics**: Updated after each play
- **Season statistics**: Cumulative stats across multiple games
- **Multiple categories**: Standard stats plus new-age analytics
- **Situational breakdowns**: Performance in different game situations
- **Streak tracking**: Hot/cold streaks, hitting/pitching streaks

#### 5.2 Advanced Statistical Categories
- **Batting**: AVG, OBP, SLG, OPS, HR, RBI, SB, CS, BB, K, GIDP
- **Pitching**: ERA, WHIP, W-L, SV, HLD, IP, H, R, ER, BB, K
- **Fielding**: PCT, A, PO, E, DP turned, Range Factor
- **Team Stats**: Runs scored/allowed, team batting/pitching averages

### 6. Reporting System

#### 6.1 Box Score Generation
- **USA Today style**: Modern expanded box score format
- **Batting lines**: AB, R, H, RBI, BB, K for each player
- **Pitching lines**: IP, H, R, ER, BB, K, ERA for each pitcher
- **Game summary**: WP, LP, SV, attendance, time of game

#### 6.2 Scorecard System
- **Professional format**: USA Today Baseball Weekly style
- **Play-by-play notation**: Standard baseball scoring symbols
- **Base runner tracking**: Complete movement notation
- **Pitching changes**: Clear indication of relief appearances
- **Comprehensive codes**: All possible play outcomes documented

#### 6.3 Statistical Reports
- **Printable formats**: Text-based reports suitable for file output
- **Multiple categories**: Sortable by various statistical measures
- **Season summaries**: Cumulative performance reports
- **Comparison reports**: Head-to-head player/team comparisons

### 7. Game Modes and Options

#### 7.1 Game Types
- **Exhibition Games**: One-off games with no stat tracking
- **League Games**: Games that count toward season statistics
- **Tournament Mode**: Playoff-style elimination games
- **Season Simulation**: Full season with schedule and standings

#### 7.2 Management Options
- **Human vs Human**: Both teams controlled by players
- **Human vs Computer**: One team managed by AI
- **Computer vs Computer**: Full simulation mode
- **Mixed Management**: Switch control during game

#### 7.3 Customization Options
- **League Rules**: DH usage, sacrifice fly scoring rules
- **Game Factors**: Ballpark effects, weather conditions
- **Difficulty Settings**: AI intelligence levels
- **Display Options**: Color schemes, window arrangements

### 8. Data Management

#### 8.1 Team Data Files
- **Roster files**: Complete player listings with positions
- **Statistical databases**: Historical and current season stats
- **League definitions**: Team groupings and rule sets
- **Ballpark data**: Dimensions and environmental factors

#### 8.2 Save System
- **Game state preservation**: Complete game situation saves
- **Multiple save slots**: Store different games/seasons
- **Auto-save functionality**: Prevent data loss
- **Export capabilities**: Text-based report generation

### 9. AI Management System

#### 9.1 Computer Manager AI
- **Strategic decision making**: Situational awareness for substitutions
- **Lineup optimization**: AI-generated lineups based on matchups
- **In-game tactics**: Appropriate use of steal attempts, bunts, etc.
- **Pitching management**: Realistic starter/reliever usage patterns
- **Defensive positioning**: Smart positioning based on batter tendencies

#### 9.2 Difficulty Levels
- **Rookie**: Basic AI decisions, obvious strategies
- **Veteran**: Competitive AI with good strategic sense
- **All-Star**: Advanced AI using complex situational analysis
- **Hall of Fame**: Maximum difficulty with perfect information usage

## Technical Implementation Details

### 10. Rust-Specific Considerations

#### 10.1 Terminal UI Libraries
- **crossterm**: Cross-platform terminal manipulation
- **tui-rs/ratatui**: Terminal UI framework for complex layouts
- **console**: Enhanced console operations for Windows

#### 10.2 Data Structures
- **Player structs**: Comprehensive stat tracking with borrowing rules
- **Game state management**: Efficient state representation
- **Event system**: Clean event handling for game actions
- **Error handling**: Robust error management with Result types

#### 10.3 Performance Optimizations
- **Efficient statistics calculations**: Fast stat updates during gameplay
- **Memory management**: Optimal use of Rust's ownership system
- **Concurrent processing**: Background AI thinking while waiting for input
- **File I/O optimization**: Efficient data loading and saving

### 11. Windows Console Optimization

#### 11.1 Console Features
- **ANSI color support**: Enhanced visual presentation
- **Unicode character support**: Box drawing characters for UI
- **Console buffer management**: Smooth screen updates
- **Keyboard input handling**: Responsive user interaction

#### 11.2 Display Considerations
- **80x25 minimum compatibility**: Works on standard console sizes
- **Scalable layouts**: Adapt to larger console windows
- **Clear visual hierarchy**: Easy navigation between windows
- **Status line indicators**: Always-visible game state information

## Implementation Phases

### Phase 1: Core Foundation (Weeks 1-3)
1. Set up Rust project structure
2. Implement basic terminal UI framework
3. Create player and team data structures
4. Build fundamental game state management
5. Develop basic menu system

### Phase 2: Game Engine (Weeks 4-6)
1. Implement statistical play resolution engine
2. Create comprehensive event system
3. Build AI decision-making framework
4. Develop pitcher fatigue and performance systems
5. Add ballpark and weather effects

### Phase 3: User Interface (Weeks 7-9)
1. Complete all window types (scoreboard, lineup, etc.)
2. Implement lineup management dialogs
3. Create statistical display systems
4. Build comprehensive menu system
5. Add game options and configuration

### Phase 4: Management Features (Weeks 10-12)
1. Implement all offensive management options
2. Add defensive management capabilities
3. Create substitution and position swap systems
4. Build defensive positioning options
5. Add strategic play calling (steal, bunt, etc.)

### Phase 5: Statistics and Reporting (Weeks 13-15)
1. Complete statistical tracking system
2. Implement box score generation
3. Create scorecard system with proper notation
4. Build comprehensive reporting system
5. Add export and printing capabilities

### Phase 6: Polish and Testing (Weeks 16-18)
1. Comprehensive testing of all features
2. Performance optimization
3. Bug fixes and stability improvements
4. User experience refinements
5. Documentation completion

## Success Criteria

The implementation will be considered complete when it includes:

1. **All original features**: Every feature from Wild Pitch (1993) is implemented
2. **Statistical accuracy**: Realistic baseball simulation based on player stats
3. **Complete management options**: All strategic decisions available to human managers
4. **Professional reporting**: USA Today-style box scores and scorecards
5. **Smooth user experience**: Intuitive terminal-based interface
6. **Robust save system**: Reliable game state preservation
7. **AI competency**: Challenging computer opponents at multiple difficulty levels
8. **Performance**: Responsive gameplay even with complex calculations

## Conclusion

This implementation plan provides a roadmap for creating a faithful and enhanced recreation of Wild Pitch in Rust. By focusing on the core statistical simulation engine first, then building the user interface and management features, we can ensure that the essential gameplay experience matches the original while taking advantage of modern development practices and the safety guarantees of Rust.

The modular architecture will allow for easy testing and future enhancements, while the terminal-based UI will provide the authentic feel of classic computer baseball simulations updated for modern systems.