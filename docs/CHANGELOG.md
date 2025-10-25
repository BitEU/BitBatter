# Changelog

All notable changes to Baseball TUI will be documented in this file.

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

### Technical
- No performance impact
- No breaking changes to gameplay
- Same binary size
- Improved code comments

### Credits
- ASCII field art: ceejay3264 (GitHub)
- User feedback that prompted this improvement

## [0.1.0] - 2025-10-25

### Initial Release

#### Added
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

#### Technical Details
- **Dependencies**
  - ratatui 0.29
  - crossterm 0.28
  - rand 0.8
  - serde 1.0
  - serde_json 1.0

- **Architecture**
  - State machine-based game flow
  - Modular design (game, input, ui)
  - Separation of rendering and logic
  - Clean error handling
  - ~1-2MB release binary

#### Documentation
- README.md with full instructions
- QUICKSTART.md for immediate play
- VISUAL_GUIDE.md with screenshots
- DEVELOPERS.md for contributors
- Build scripts for Windows and Unix

### Known Issues
- No fielding mechanics yet (automatic outs on batted balls)
- No player stats system (coming in 0.2.0)
- No team selection (default teams)
- No save/load games
- No season mode
- No two-player mode
- No statistics tracking between games

### Compatibility
- ✅ Windows 10/11 (conhost and Windows Terminal)
- ✅ Linux (most terminal emulators)
- ✅ macOS (Terminal.app, iTerm2)
- ❓ Older Windows versions (not tested)

## [Upcoming - 0.2.0]

### Planned Features
- [ ] Player statistics (power, contact, speed, control)
- [ ] Team selection screen
- [ ] Custom team creation
- [ ] Statistics tracking
- [ ] More detailed fielding
- [ ] Pitcher fatigue system
- [ ] Pinch hitters
- [ ] Defensive substitutions

## [Future Versions]

### 0.3.0 - Season Mode
- [ ] Full season play
- [ ] Standings and records
- [ ] Save/load games
- [ ] Player progression
- [ ] Injuries

### 0.4.0 - Enhanced Graphics
- [ ] Better animations
- [ ] More detailed field
- [ ] Player sprites
- [ ] Color themes
- [ ] Unicode art improvements

### 0.5.0 - Multiplayer
- [ ] Two-player mode (same keyboard)
- [ ] Network play (stretch goal)

### 1.0.0 - Full Release
- [ ] All planned features
- [ ] Comprehensive testing
- [ ] Performance optimization
- [ ] Sound effects (PC speaker beeps)
- [ ] Complete documentation

---

## Contributing

See DEVELOPERS.md for contribution guidelines.

## Version Numbering

This project follows [Semantic Versioning](https://semver.org/):
- MAJOR version: Incompatible changes
- MINOR version: New features (backwards compatible)
- PATCH version: Bug fixes

---

**Current Version: 0.1.0**  
**Status: Initial Release**  
**Last Updated: October 25, 2025**