use crate::frame::{Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};
use rusty_time::Timer;
use std::time::Duration;

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
    flash: bool,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < 9)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                {
                    army.push(Invader { x, y });
                }
            }
        }
        Self {
            army,
            move_timer: Timer::new(Duration::from_millis(2000)),
            direction: 1,
            flash: false,
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.tick(delta);
        if self.move_timer.just_finished() {
            self.move_timer.reset();
            self.flash = !self.flash;
            let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards {
                let new_duration = self
                    .move_timer
                    .duration()
                    .as_millis()
                    .saturating_sub(250)
                    .max(250);
                self.move_timer = Timer::new(Duration::from_millis(new_duration as u64));
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            self.speed_up();
            return true;
        }
        false
    }
    pub fn speed_up(&mut self) {
        let new_duration =
            ((self.move_timer.duration().as_millis() as f32) * 0.95).max(180.0) as u64;
        self.move_timer = Timer::new(Duration::from_millis(new_duration));
    }
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }
    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            self.army.remove(idx);
            self.speed_up();
            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if self.flash { "M" } else { "W" };
        }
    }
}

impl Default for Invaders {
    fn default() -> Self {
        Self::new()
    }
}
