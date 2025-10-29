# BitBatter

A retro-style baseball game for Windows Console (conhost) written in Rust, inspired by classic NES, Atari, and MS-DOS baseball games like RBI Baseball, Baseball Stars, and HardBall! Stats are from https://baseballsavant.mlb.com/leaderboard/statcast?type=pitcher&year=2025&position=&team=&min=q&sort=barrels_per_pa&sortDir=desc

## Features

- **Realistic Timing System**: 3-second pitch clock + ball approach animation with precise swing timing
- **Interactive Fielding**: Real-time ball physics with timing-based catching mechanics
- **Classic Gameplay**: NES-style directional pitching and batting with modern timing elements
- **Professional Field**: High-quality ASCII baseball diamond with realistic perspective
- **No Flicker**: Uses Ratatui's double-buffering for smooth rendering on Windows conhost
- **Smart Controls**: Arrow keys for aiming, Space to pitch/swing, timing-based mechanics
- **Full 9-Inning Games**: Complete baseball simulation with scoring, baserunning, and outs
- **Visual Detail**: Outfield fence, infield dirt, pitcher's mound, 9 fielder positions
- **Audio Feedback**: Contextual sounds for hits, misses, catches, and crowd reactions

## Requirements

- Rust 1.67.0 or newer
- Windows Terminal, Windows Conhost, or any modern terminal emulator

## Installation

1. Install Rust from https://rustup.rs/

2. Clone or extract this project

3. Build the game:
```bash
cargo build --release
```

4. Run the game:
```bash
cargo run --release
```

Or run the compiled binary directly:
```bash
./target/release/BitBatter
```

## Controls

### Pitching Phase
- **1-4**: Select pitch type (Fastball, Curveball, Slider, Changeup)
- **Arrow Keys**: Aim pitch location (9 zones)
- **SHIFT + (1-9)**: Direct aim to specific zone (SHIFT+7=top-left, SHIFT+8=top-center, SHIFT+9=top-right, SHIFT+4=left, SHIFT+5=center, SHIFT+6=right, SHIFT+1=bottom-left, SHIFT+2=bottom-center, SHIFT+3=bottom-right)
- **Space/Enter**: Release pitch

### Batting Phase
- **Pitch Clock**: 3-second countdown to prepare for pitch
- **Ball Approach**: Watch ball travel from mound to plate
- **Timing Window**: Swing when ball enters the timing zone
  - **Perfect Timing**: ⚡ 0.2-second window for maximum contact
  - **Good Timing**: Early/Late zones for decent contact
  - **Poor Timing**: Too early/late = weak contact or swing-and-miss
- **Arrow Keys**: Position swing location during ball approach
- **SHIFT + (1-9)**: Direct aim swing to specific zone
- **Space/Enter**: Swing bat (timing matters!)
- **Don't Press Anything**: Take the pitch (ball/strike)

### General
- **Q**: Quit game
- **Esc**: Pause (future feature)

> **Note**: Direct aiming uses SHIFT + number keys (not numpad) due to terminal limitations in detecting numpad keys separately from the main number row.

## How to Play

### Pitching
1. **Choose Your Pitch**: Press 1-4 to select pitch type
2. **Aim**: Use arrow keys to aim at one of 9 locations, or use SHIFT + number (1-9) for direct selection:
   ```
   SHIFT+7 (UpLeft)    SHIFT+8 (Up)    SHIFT+9 (UpRight)
   SHIFT+4 (Left)    SHIFT+5 (Middle)    SHIFT+6 (Right)
   SHIFT+1 (DownLeft)  SHIFT+2 (Down)  SHIFT+3 (DownRight)
   ```
3. **Pitch**: Press Space to start the pitch clock

### Batting
4. **Get Ready**: 3-second pitch clock countdown begins
5. **Watch the Ball**: Ball approaches from mound to plate
6. **Time Your Swing**: 
   - Position your swing (arrows/SHIFT+numbers) during ball approach
   - Wait for ball to enter timing window
   - **Perfect Timing** = green zone for best contact
   - **Good Timing** = early/late zones still work
   - **Poor Timing** = swing and miss!
7. **Swing**: Press Space when timing feels right, or take the pitch

### Fielding
8. **React Fast**: When ball is hit, watch its trajectory
9. **Time the Catch**: Press Space at the right moment to field
10. **Perfect Timing**: Successful out vs. ball gets through for hit

### Scoring 
11. **Score Runs**: Hits advance runners and score runs
12. **3 Outs**: Each team gets 3 outs per inning  
13. **9 Innings**: Complete 9 innings to finish the game

## Game Mechanics

### Pitching
- **Fastball**: Fast but straight (90 MPH)
- **Curveball**: Slower with lots of break (75 MPH)
- **Slider**: Medium speed with some movement (82 MPH)
- **Changeup**: Off-speed pitch (78 MPH)

### Batting & Timing
- **Timing is Everything**: New realistic timing system with multiple windows
  - **Perfect Timing**: 0.2-second window = 30% contact quality bonus
  - **Good Timing** (Early/Late): Decent contact but 40% penalty
  - **Poor Timing** (Too Early/Late): 90% chance of swing-and-miss
- **Location Matching**: Swing location should match pitch location
- **Perfect Contact**: Good timing + location match + strike zone = likely hit
- **Swing-and-Miss**: Now actually happens! Poor timing leads to strikeouts
- **Power**: Perfect timing + good contact = singles, doubles, triples, home runs
- **Strike Zone**: Pitches in corners harder to hit, especially with poor timing

### Timing System Details
The game features a realistic timing system that mimics real baseball:

1. **Pitch Clock Phase** (3 seconds):
   - Visual countdown with progress bar
   - Time to get in position and prepare
   - Clock turns red in final 3 seconds

2. **Ball Approach Phase** (3 seconds):
   - Ball travels from mound to home plate
   - Position indicator shows ball location
   - Timing window opens when ball gets close

3. **Swing Timing Windows**:
   ```
   Too Early → Early → PERFECT → Late → Too Late
   (Miss)     (60%)    (130%)   (60%)   (Miss)
   ```
   - Numbers show contact quality multiplier
   - Perfect timing has best chance for hits
   - Poor timing leads to swing-and-miss

### Results
- **Strike**: Swing and miss (now actually happens!), or pitch in strike zone taken
- **Ball**: Pitch outside strike zone, no swing
- **Foul**: Weak contact (counts as strike, but won't strikeout on 2 strikes)
- **Hit**: Single, Double, Triple, or Home Run (timing affects outcome!)
- **Out**: Groundout, Flyout, Lineout, or Strikeout (timing-based strikeouts now possible)

## Technical Details

### Anti-Flicker Design
This game was specifically designed to avoid UI flickering on Windows conhost by:
- Using Ratatui's built-in double-buffering
- Single `terminal.draw()` call per frame
- Hidden cursor during gameplay
- Alternate screen buffer usage
- Proper terminal cleanup on exit
- Frame rate limiting (30 FPS)

### Architecture
```
BitBatter/
├── src/
│   ├── main.rs          # Entry point
```

### Dependencies
- **ratatui**: Terminal UI framework (fork of tui-rs)
- **crossterm**: Cross-platform terminal manipulation
- **rand**: Random number generation for game mechanics
- **serde**: Serialization (for future save game feature)

## Future Enhancements

- [ ] Save/load games
- [ ] Standings and records
- [ ] Injuries
- [ ] Network play (stretch goal)
- [ ] Comprehensive unit testing
- [ ] Performance optimization
- [ ] Complete documentation

## Keyboard Layout

```
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬─────────┐
│Esc│ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │ 8 │ 9 │ 0 │ - │ = │  Bksp   │
│Pau│Fst│Crv│Sld│Chg│   │   │   │   │   │   │   │   │         │
├───┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬───────┤
│ Tab │ Q │ W │ E │ R │ T │ Y │ U │ I │ O │ P │ [ │ ] │   \   │
│     │Qui│   │   │   │   │   │   │   │   │   │   │   │       │
├─────┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴───────┤
│ Caps │ A │ S │ D │ F │ G │ H │ J │ K │ L │ ; │ ' │  Enter   │
│      │   │   │   │   │   │   │   │   │   │   │   │  Action  │
├──────┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴──────────┤
│ Shift  │ Z │ X │ C │ V │ B │ N │ M │ , │ . │ / │   Shift    │
│        │   │   │   │   │   │   │   │   │   │   │            │
├────┬───┴┬──┴─┬─┴───┴───┴───┴───┴───┴──┬┴───┼───┴┬────┬──────┤
│Ctrl│ Win│ Alt│      Space/Action      │Alt │ Win│Menu│ Ctrl │
└────┴────┴────┴────────────────────────┴────┴────┴────┴──────┘
                      ↑ ↓ ← →
                  Arrow Keys: Aim

Direct Aiming with SHIFT + Numbers:
  Hold SHIFT, then press a number (1-9):
  
  SHIFT+7 = ↖   SHIFT+8 = ↑   SHIFT+9 = ↗
  SHIFT+4 = ←   SHIFT+5 = ·   SHIFT+6 = →
  SHIFT+1 = ↙   SHIFT+2 = ↓   SHIFT+3 = ↘
  
  * Without SHIFT, numbers 1-4 select pitch type
  * With SHIFT, numbers 1-9 instantly aim to that zone
```

## Credits

**Inspired by classic baseball games:**
- **RBI Baseball** (1987, NES)
- **Baseball Stars** (1989, NES)
- **Earl Weaver Baseball** (1987, MS-DOS)
- **HardBall!** (1985, Commodore 64)

**ASCII Field Art:**
- ceejay3264's ascii_baseball project
- GitHub: https://github.com/ceejay3264/ascii_baseball

**Audio**
- https://opengameart.org/content/3-melee-sounds
- https://opengameart.org/content/75-cc0-breaking-falling-hit-sfx
- https://freesound.org/people/moxobna/sounds/32260/
- https://freesound.org/people/SoundsExciting/sounds/365132/
- https://freesound.org/people/FoolBoyMedia/sounds/397434/

**Special Thanks:**
- User feedback that led to the v0.1.1 field improvement!

## License

MIT License - Feel free to modify and distribute!

## Contributing

Pull requests welcome! Please test on Windows conhost before submitting.