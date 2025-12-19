# Invaders (v0.2.0)

Retro terminal shooter with sound, score tracking, and a festive holiday skin (snowfield, tree-like invaders, candy-cane shots).

## Play
- With sound: `cargo run`
- Mute at runtime: `cargo run -- --mute`
- No sound dependencies: `cargo run --no-default-features`
- Menu: Space/Enter to start, Esc/Q to exit. Retry after win/lose with Space/Enter.

## Controls
- Move: arrows, `A`/`D`, or `H`/`L`
- Shoot: `Space`, `Enter`, `W`, or `K`
- Pause: `P`
- Quit: `Esc` or `Q`

## Gameplay
- Snowy background, festive menus, tree-like invaders, candy-cane shots, and a snowy ground line.
- Difficulty ramps as invaders move and when you score hits; invaders and shots animate.
- Score + persistent high score on the HUD (`high_score.txt` is used to store it).
- HUD shows remaining invaders to track progress.

## Audio
- Linux needs ALSA dev libs. The build script probes `alsa` via `pkg-config` and emits a friendly warning; install `libasound2-dev` (or equivalent) for sound.
- If ALSA is not available, build/run without sound (`--no-default-features`) or just mute (`--mute`).
- The `sound/` folder (explode/lose/move/pew/startup/win WAVs) ships with release artifacts. These sounds were AI-generated using the author's voice as a source.

## Development
- Format: `cargo fmt --all -- --check`
- Lint: `cargo clippy --no-default-features --all-targets -- -D warnings`
- Tests: `cargo test --no-default-features`

## CI and releases
- CI: `.github/workflows/ci.yml` runs fmt, clippy, and tests (no-sound) on pushes/PRs.
- Releases: `.github/workflows/release.yml` builds release binaries on Linux/macOS/Windows, bundles the `sound/` assets and README, and uploads them as artifacts.

## Portability
- WebAssembly/HTML5: current `rodio/cpal` backend is not WASM-ready; a WebAudio backend would be needed for browser builds.
