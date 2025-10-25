# Baseball TUI - Retro Terminal Baseball Game

A retro-style baseball game for Windows Console (conhost) written in Rust, inspired by classic NES, Atari, and MS-DOS baseball games like RBI Baseball, Baseball Stars, and HardBall! Stats are from https://baseballsavant.mlb.com/leaderboard/statcast?type=pitcher&year=2025&position=&team=&min=q&sort=barrels_per_pa&sortDir=desc

**NEW in v0.1.1:** Professional ASCII field rendering! (User feedback implemented)

## Features

- **Classic Gameplay**: NES-style directional pitching and batting
- **Professional Field**: High-quality ASCII baseball diamond with realistic perspective
- **No Flicker**: Uses Ratatui's double-buffering for smooth rendering on Windows conhost
- **Simple Controls**: Arrow keys for aiming, Space to pitch/swing
- **Full 9-Inning Games**: Complete baseball simulation with scoring, baserunning, and outs
- **Visual Detail**: Outfield fence, infield dirt, pitcher's mound, 9 fielder positions

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
./target/release/baseball-tui
```

## Controls

### Pitching Phase
- **1-4**: Select pitch type (Fastball, Curveball, Slider, Changeup)
- **Arrow Keys**: Aim pitch location (9 zones)
- **Space/Enter**: Release pitch

### Batting Phase
- **Arrow Keys**: Position swing location
- **Space/Enter**: Swing bat
- **Don't Press Anything**: Take the pitch (ball/strike)

### General
- **Q**: Quit game
- **Esc**: Pause (future feature)

## How to Play

1. **Choose Your Pitch**: Press 1-4 to select pitch type
2. **Aim**: Use arrow keys to aim at one of 9 locations:
   ```
   UpLeft    Up    UpRight
   Left    Middle    Right
   DownLeft  Down  DownRight
   ```
3. **Pitch**: Press Space to throw
4. **Bat**: As the batter, aim your swing and press Space
5. **Score Runs**: Hits advance runners and score runs
6. **3 Outs**: Each team gets 3 outs per inning
7. **9 Innings**: Complete 9 innings to finish the game

## Game Mechanics

### Pitching
- **Fastball**: Fast but straight (90 MPH)
- **Curveball**: Slower with lots of break (75 MPH)
- **Slider**: Medium speed with some movement (82 MPH)
- **Changeup**: Off-speed pitch (78 MPH)

### Batting
- **Perfect Contact**: Swing location matches pitch + in strike zone = likely hit
- **Timing Matters**: Early/late swings result in fouls or weak contact
- **Power**: Good contact can result in singles, doubles, triples, or home runs
- **Strike Zone**: Pitches in the corners are harder to hit

### Results
- **Strike**: Swing and miss, or pitch in strike zone taken
- **Ball**: Pitch outside strike zone, no swing
- **Foul**: Weak contact (counts as strike, but won't strikeout on 2 strikes)
- **Hit**: Single, Double, Triple, or Home Run
- **Out**: Groundout, Flyout, Lineout, or Strikeout

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
baseball-tui/
├── src/
│   ├── main.rs          # Entry point and game loop
│   ├── game/
│   │   ├── mod.rs       # Game module exports
│   │   ├── state.rs     # Game state management
│   │   └── engine.rs    # Game logic and physics
│   ├── input.rs         # Input handling
│   └── ui.rs            # Terminal UI rendering
└── Cargo.toml           # Dependencies
```

### Dependencies
- **ratatui**: Terminal UI framework (fork of tui-rs)
- **crossterm**: Cross-platform terminal manipulation
- **rand**: Random number generation for game mechanics
- **serde**: Serialization (for future save game feature)

## Future Enhancements

- [ ] Save/load games
- [ ] More detailed fielding mechanics
- [ ] Pitch animations
- [ ] Sound effects
- [ ] Statistics tracking
- [ ] Two-player mode

## Troubleshooting

### UI appears mangled or flickering
- Make sure you're running on a modern terminal (Windows Terminal recommended)
- Update Windows if using older conhost
- Try running with `--release` flag for better performance

### Characters don't display correctly
- Ensure your terminal font supports Unicode box drawing characters
- Try a different font (Cascadia Code, Consolas recommended)

### Game runs too slow
- Build with `--release` flag
- Close other applications
- Try reducing terminal window size

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

---

**Play Ball!** ⚾