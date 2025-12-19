# Invaders

Retro terminal shooter with sound, score tracking, and a bit of polish (menus, pause, starfield, colorized render).

## Quick start
- Play with sound: `cargo run`
- Mute at runtime: `cargo run -- --mute`
- Build/run without sound dependencies: `cargo run --no-default-features`
- Start menu: press Space/Enter to begin; Esc/Q to exit. Retry after win/lose with Space/Enter.

## Controls
- Move: Left/Right arrows, `A`/`D`, or vim-style `H`/`L`
- Shoot: `Space`, `Enter`, `W`, or `K`
- Pause: `P`
- Quit: `Esc` or `Q`

## Gameplay notes
- Difficulty ramps as invaders move and as you score hits; movement and shots have small animations.
- Score and persistent high-score are shown on the top row; high-score persists in `high_score.txt`.
- HUD also shows remaining invaders; background is a moving starfield.

## Audio
- Sound depends on ALSA dev libraries on Linux. The build script uses `pkg-config` to probe `alsa` and emits a warning with install instructions (e.g., `libasound2-dev`).
- If you cannot install ALSA, use `--no-default-features` to compile without sound or `--mute` to silence playback at runtime.

## Development
- Format: `cargo fmt --all -- --check`
- Lint: `cargo clippy --no-default-features --all-targets -- -D warnings`
- Tests: `cargo test --no-default-features`

## Portability
- WebAssembly/HTML5: current `rodio/cpal` backend is not WASM-ready; a WebAudio-specific backend would be needed for browser builds.
