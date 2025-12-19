use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use invaders::frame::{new_frame, Drawable};
use invaders::invaders::Invaders;
use invaders::player::Player;
use invaders::{frame, render};
use std::error::Error;
use std::fmt;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{env, io, thread};

#[cfg(feature = "sound")]
use rusty_audio::Audio;
#[cfg(feature = "sound")]
use std::panic;

const HIGH_SCORE_FILE: &str = "high_score.txt";

struct ScoreBoard {
    score: u32,
    high_score: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameState {
    Menu,
    Playing,
    Won,
    Lost,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            GameState::Menu => "Start",
            GameState::Playing => "Playing",
            GameState::Won => "You Win!",
            GameState::Lost => "Game Over",
        };
        write!(f, "{label}")
    }
}

impl ScoreBoard {
    fn load() -> Self {
        let high_score = std::fs::read_to_string(HIGH_SCORE_FILE)
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
            .unwrap_or(0);
        Self {
            score: 0,
            high_score,
        }
    }

    fn add_points(&mut self, points: u32) {
        self.score = self.score.saturating_add(points);
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }

    fn persist(&self) {
        if self.score >= self.high_score {
            let _ = std::fs::write(HIGH_SCORE_FILE, self.high_score.to_string());
        }
    }
}

#[cfg(feature = "sound")]
struct Sound {
    audio: Option<Audio>,
}

#[cfg(feature = "sound")]
impl Sound {
    fn new(enabled: bool) -> Self {
        if !enabled {
            return Self { audio: None };
        }
        let audio = panic::catch_unwind(Audio::new)
            .ok()
            .and_then(|a| (!a.disabled()).then_some(a));

        if let Some(mut audio) = audio {
            audio.add("explode", "sound/explode.wav");
            audio.add("lose", "sound/lose.wav");
            audio.add("move", "sound/move.wav");
            audio.add("pew", "sound/pew.wav");
            audio.add("startup", "sound/startup.wav");
            audio.add("win", "sound/win.wav");
            audio.play("startup");
            Self { audio: Some(audio) }
        } else {
            eprintln!("Audio initialization failed; continuing without sound. Pass --mute to silence this message.");
            Self { audio: None }
        }
    }

    fn play(&mut self, key: &str) {
        if let Some(audio) = self.audio.as_mut() {
            audio.play(key);
        }
    }

    fn wait(self) {
        if let Some(audio) = self.audio {
            audio.wait();
        }
    }
}

#[cfg(not(feature = "sound"))]
struct Sound;

#[cfg(not(feature = "sound"))]
impl Sound {
    fn new(_: bool) -> Self {
        Self
    }
    fn play(&mut self, _: &str) {}
    fn wait(self) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let mute = env::args().any(|a| a == "--mute");
    let mut sound = Sound::new(!mute);
    let mut score = ScoreBoard::load();
    let mut paused = false;
    let input = InputConfig::default();
    let mut state = GameState::Menu;
    let mut frame_tick = 0_u64;

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        while let Ok(curr_frame) = render_rx.recv() {
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();
        frame::fill_starry_background(&mut curr_frame, frame_tick);

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                let code = key_event.code;
                if input.is_pause(code) && state == GameState::Playing {
                    paused = !paused;
                    continue;
                }
                if input.is_quit(code) {
                    sound.play("lose");
                    break 'gameloop;
                }
                if state != GameState::Playing {
                    match code {
                        code if input.is_fire(code) => {
                            state = GameState::Playing;
                            score.score = 0;
                            player = Player::new();
                            invaders = Invaders::new();
                        }
                        code if input.is_left(code) || input.is_right(code) => {
                            state = GameState::Playing;
                            score.score = 0;
                            player = Player::new();
                            invaders = Invaders::new();
                        }
                        KeyCode::Esc | KeyCode::Char('q') => break 'gameloop,
                        _ => {}
                    }
                    continue;
                }
                match code {
                    code if input.is_left(code) => player.move_left(),
                    code if input.is_right(code) => player.move_right(),
                    code if input.is_fire(code) => {
                        if player.shoot() {
                            sound.play("pew");
                        }
                    }
                    _ => {}
                }
            }
        }

        if state == GameState::Menu {
            frame::write_text(&mut curr_frame, 8, 8, "INVADERS");
            frame::write_text(&mut curr_frame, 4, 10, "+----------------------+");
            frame::write_text(&mut curr_frame, 4, 11, "|  Press SPACE/ENTER  |");
            frame::write_text(&mut curr_frame, 4, 12, "|        to start      |");
            frame::write_text(&mut curr_frame, 4, 13, "|   ESC or Q to exit   |");
            frame::write_text(&mut curr_frame, 4, 14, "+----------------------+");
            let _ = render_tx.send(curr_frame);
            thread::sleep(Duration::from_millis(16));
            frame_tick = frame_tick.wrapping_add(1);
            continue;
        }

        if paused {
            frame::write_text(&mut curr_frame, 12, 9, "[ PAUSED ]");
            frame::write_text(&mut curr_frame, 5, 10, "Press P to resume");
            let _ = render_tx.send(curr_frame);
            thread::sleep(Duration::from_millis(16));
            frame_tick = frame_tick.wrapping_add(1);
            continue;
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            sound.play("move");
        }
        let hits = player.detect_hits(&mut invaders);
        if hits > 0 {
            sound.play("explode");
            score.add_points(hits as u32 * 50);
        }

        // Draw & render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        frame::write_text(
            &mut curr_frame,
            1,
            0,
            &format!("Score: {}  High: {}", score.score, score.high_score),
        );
        frame::write_text(
            &mut curr_frame,
            26,
            0,
            &format!("Invaders: {:02}", invaders.army.len()),
        );
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
        frame_tick = frame_tick.wrapping_add(1);

        //Win or lose
        if invaders.all_killed() {
            sound.play("win");
            state = GameState::Won;
        }
        if invaders.reached_bottom() {
            sound.play("lose");
            state = GameState::Lost;
        }

        if state == GameState::Won || state == GameState::Lost {
            let mut end_frame = new_frame();
            frame::fill_starry_background(&mut end_frame, frame_tick);
            let msg = if state == GameState::Won {
                "YOU WIN!"
            } else {
                "GAME OVER"
            };
            frame::write_text(&mut end_frame, 10, 8, msg);
            frame::write_text(&mut end_frame, 6, 10, "Press SPACE to retry, ESC to exit");
            let _ = render_tx.send(end_frame);
            loop {
                if let Event::Key(key_event) = event::read()? {
                    let code = key_event.code;
                    if input.is_fire(code) {
                        state = GameState::Playing;
                        score.score = 0;
                        player = Player::new();
                        invaders = Invaders::new();
                        instant = Instant::now();
                        break;
                    }
                    if input.is_quit(code) {
                        break 'gameloop;
                    }
                }
            }
        }
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    sound.wait();
    score.persist();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

struct InputConfig {
    left: Vec<KeyCode>,
    right: Vec<KeyCode>,
    fire: Vec<KeyCode>,
    quit: Vec<KeyCode>,
    pause: Vec<KeyCode>,
}

impl InputConfig {
    fn is_left(&self, code: KeyCode) -> bool {
        self.left.contains(&code)
    }
    fn is_right(&self, code: KeyCode) -> bool {
        self.right.contains(&code)
    }
    fn is_fire(&self, code: KeyCode) -> bool {
        self.fire.contains(&code)
    }
    fn is_quit(&self, code: KeyCode) -> bool {
        self.quit.contains(&code)
    }
    fn is_pause(&self, code: KeyCode) -> bool {
        self.pause.contains(&code)
    }
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            left: vec![KeyCode::Left, KeyCode::Char('a'), KeyCode::Char('h')],
            right: vec![KeyCode::Right, KeyCode::Char('d'), KeyCode::Char('l')],
            fire: vec![
                KeyCode::Char(' '),
                KeyCode::Enter,
                KeyCode::Char('k'),
                KeyCode::Char('w'),
            ],
            quit: vec![KeyCode::Esc, KeyCode::Char('q')],
            pause: vec![KeyCode::Char('p')],
        }
    }
}
