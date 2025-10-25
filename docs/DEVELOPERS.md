# Developer Guide - Baseball TUI

## Architecture Overview

### Module Structure

```
terminalbball/
├── src/
│   ├── main.rs           # Entry point, game loop, terminal setup
│   ├── game/
│   │   ├── mod.rs        # Module exports
│   │   ├── state.rs      # Game state and rules
│   │   └── engine.rs     # Game mechanics (pitching, batting)
│   ├── input.rs          # Input handling and polling
│   └── ui.rs             # Rendering logic
```

### Data Flow

```
Input (keyboard)
    ↓
input::poll_input() → GameInput enum
    ↓
main::handle_input() → Updates GameState
    ↓
main::update_game_state() → State machine progression
    ↓
ui::render_game() → Ratatui widgets
    ↓
Terminal (display)
```

## Key Design Patterns

### 1. State Machine (game/state.rs)

The game uses an enum-based state machine:

```rust
pub enum PitchState {
    ChoosePitch,           // Pitcher selects pitch type
    Aiming { pitch_type },  // Pitcher aims location
    Pitching { frames },    // Animation playing
    WaitingForBatter,       // Batter's turn
    Swinging { frames },    // Swing animation
    BallInPlay { frames },  // Ball hit, in play
    ShowResult { result },  // Display outcome
}
```

Each frame, `update_game_state()` advances the state based on timers and conditions.

### 2. Double Buffering (ui.rs)

Ratatui handles double buffering automatically:

```rust
terminal.draw(|frame| {
    ui::render_game(frame, &game_state, &engine);
})?;
```

**Critical**: All rendering happens in ONE closure call. Never write to stdout directly!

### 3. Non-Blocking Input (input.rs)

```rust
if event::poll(Duration::from_millis(16))? {
    // Process input
}
```

16ms poll = ~60fps input responsiveness without blocking the render loop.

## Adding Features

### Adding a New Pitch Type

**1. Update engine.rs:**
```rust
impl GameEngine {
    pub fn new() -> Self {
        Self {
            pitch_types: vec![
                // ... existing pitches
                PitchType {
                    name: "Knuckleball",
                    speed: 65,
                    break_amount: 8,
                },
            ],
        }
    }
}
```

**2. Update controls in ui.rs:**
```rust
// Add "5: Knuckleball" to the controls display
```

**3. Update input.rs:**
```rust
KeyCode::Char('5') => Some(GameInput::SelectPitch(4)),
```

### Adding Player Stats

**1. Create player.rs:**
```rust
pub struct Player {
    pub name: String,
    pub power: u8,      // 0-100
    pub contact: u8,
    pub speed: u8,
}
```

**2. Update GameState:**
```rust
pub struct GameState {
    // ... existing fields
    pub home_roster: Vec<Player>,
    pub away_roster: Vec<Player>,
}
```

**3. Modify calculate_pitch_result():**
```rust
pub fn calculate_pitch_result(
    &self,
    pitch_location: PitchLocation,
    swing_location: Option<PitchLocation>,
    batter: &Player,  // Add this
) -> PlayResult {
    // Use batter.contact and batter.power in calculations
}
```

### Adding Fielding Mechanics

**1. Create new state:**
```rust
pub enum FieldingState {
    BallInFlight { target_base: Base },
    Throwing { from: Position, to: Base },
    Safe,
    Out,
}
```

**2. Add to PitchState:**
```rust
pub enum PitchState {
    // ... existing variants
    Fielding { state: FieldingState, frames_left: u8 },
}
```

**3. Update render_field():**
```rust
fn render_field(frame: &mut Frame, area: Rect, state: &GameState) {
    // Show ball position
    // Show fielder positions
    // Animate throw
}
```

### Adding Save/Load

**1. Make GameState serializable:**
```rust
#[derive(Serialize, Deserialize)]
pub struct GameState {
    // ... fields
}
```

**2. Create save.rs:**
```rust
pub fn save_game(state: &GameState, path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(state)?;
    std::fs::write(path, json)?;
    Ok(())
}

pub fn load_game(path: &str) -> Result<GameState> {
    let json = std::fs::read_to_string(path)?;
    let state = serde_json::from_str(&json)?;
    Ok(state)
}
```

**3. Add menu state:**
```rust
pub enum GameMode {
    MainMenu,
    Playing,
    LoadGame,
}
```

## Performance Optimization

### Current Frame Budget
- Target: 30 FPS (33ms per frame)
- Input polling: ~0.1ms
- State update: ~0.1ms
- Rendering: ~1-2ms
- Sleep: remaining time

### Optimization Tips

1. **Avoid String Allocation**
```rust
// Bad
let msg = format!("Score: {}", score);

// Better (if in hot path)
use std::fmt::Write;
let mut msg = String::with_capacity(20);
write!(&mut msg, "Score: {}", score).unwrap();
```

2. **Reuse Buffers**
```rust
pub struct GameState {
    message_buffer: String,  // Reuse instead of allocating
}
```

3. **Profile Before Optimizing**
```bash
cargo install flamegraph
cargo flamegraph --bin terminalbball
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strike_zone() {
        assert!(PitchLocation::Middle.is_strike());
        assert!(!PitchLocation::UpInside.is_strike());
    }

    #[test]
    fn test_runner_advancement() {
        let mut state = GameState::new();
        state.bases = [true, false, false];
        state.advance_runners(1); // Single
        assert_eq!(state.bases, [true, true, false]);
    }
}
```

Run tests:
```bash
cargo test
```

### Integration Tests

Create `tests/game_flow.rs`:
```rust
use baseball_tui::game::*;

#[test]
fn test_full_inning() {
    let mut state = GameState::new();
    // Simulate 3 outs
    state.add_out();
    state.add_out();
    state.add_out();
    assert_eq!(state.outs, 0); // Reset
    assert_eq!(state.half, InningHalf::Bottom);
}
```

## Debugging

### Enable Logging

Add to Cargo.toml:
```toml
[dependencies]
log = "0.4"
env_logger = "0.11"
```

In main.rs:
```rust
fn main() {
    env_logger::init();
    // ... rest of code
}
```

In code:
```rust
log::debug!("Pitch result: {:?}", result);
```

Run with logs:
```bash
RUST_LOG=debug cargo run
```

### Debug Overlay

Add to ui.rs:
```rust
fn render_debug_overlay(frame: &mut Frame, area: Rect, state: &GameState) {
    let debug_text = format!(
        "State: {:?}\nPitch: {:?}\nSwing: {:?}",
        state.pitch_state,
        state.pitch_location,
        state.swing_location
    );
    // Render in corner
}
```

Enable with feature flag:
```toml
[features]
debug-overlay = []
```

```rust
#[cfg(feature = "debug-overlay")]
render_debug_overlay(frame, debug_area, state);
```

## Common Pitfalls

### 1. Writing to stdout Directly ❌
```rust
// NEVER DO THIS:
println!("Score: {}", score);
```
This bypasses Ratatui's buffer and causes flicker!

### 2. Forgetting Terminal Cleanup ❌
```rust
// ALWAYS restore terminal, even on error:
let res = run_game(&mut terminal);
disable_raw_mode()?;
execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
```

### 3. Blocking on Input ❌
```rust
// DON'T:
let event = event::read()?; // Blocks!

// DO:
if event::poll(Duration::from_millis(16))? {
    let event = event::read()?;
}
```

### 4. Expensive Calculations in Render ❌
```rust
// DON'T:
fn render_field(frame: &mut Frame, ...) {
    let complex_calculation = /* heavy work */;
    // ...
}

// DO:
fn update_game_state(...) {
    state.cached_field_data = /* heavy work */;
}

fn render_field(frame: &mut Frame, state: &GameState) {
    // Use state.cached_field_data
}
```

## Code Style

### Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Documentation
```rust
/// Calculates the result of a pitch/swing interaction.
///
/// # Arguments
/// * `pitch_location` - Where the pitch crossed the plate
/// * `swing_location` - Where the batter aimed (None if no swing)
/// * `pitch_type_idx` - Index into pitch_types array
///
/// # Returns
/// The outcome of the at-bat action
pub fn calculate_pitch_result(...) -> PlayResult {
```

Generate docs:
```bash
cargo doc --open
```

## Building for Release

### Optimized Binary
```bash
cargo build --release --target x86_64-pc-windows-msvc
```

### Cross-Compilation
```bash
# For Windows from Linux:
cargo install cross
cross build --release --target x86_64-pc-windows-gnu
```

### Reducing Binary Size

Add to Cargo.toml:
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Remove debug symbols
```

Result: ~1-2MB binary

## Contributing Guidelines

1. **Fork and Branch**
   - Create feature branch: `git checkout -b feature/team-selection`

2. **Code Quality**
   - Run `cargo fmt` before committing
   - Run `cargo clippy` and fix warnings
   - Add tests for new features

3. **Testing**
   - Test on Windows conhost specifically
   - Test on Windows Terminal
   - Verify no flicker occurs

4. **Documentation**
   - Update README if adding features
   - Add code comments for complex logic
   - Update VISUAL_GUIDE.md for UI changes

5. **Pull Request**
   - Describe changes clearly
   - Link to any related issues
   - Include screenshots if UI changed

## Resources

- [Ratatui Documentation](https://docs.rs/ratatui)
- [Crossterm Documentation](https://docs.rs/crossterm)
- [Rust Book](https://doc.rust-lang.org/book/)
- [NES Baseball Games](https://www.mobygames.com/game/nes/rbi-baseball)

## Support

Questions? Issues?
- Check existing GitHub issues
- Read the docs
- Test on Windows conhost first

---

Happy coding! ⚾💻
