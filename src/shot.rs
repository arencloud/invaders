use crate::frame::{Drawable, Frame};
use rusty_time::Timer;
use std::time::Duration;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
    anim_flip: bool,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::new(Duration::from_millis(50)),
            anim_flip: false,
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.just_finished() && !self.exploding {
            self.anim_flip = !self.anim_flip;
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::new(Duration::from_millis(250));
    }
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.finished()) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding {
            "*"
        } else if self.anim_flip {
            "!"
        } else {
            "|"
        };
    }
}
