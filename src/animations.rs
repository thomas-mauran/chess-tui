//! Tachyonfx-based animation state.

use crate::constants::Pages;
use ratatui::prelude::Rect;
use std::time::Instant;
use tachyonfx::Effect;

pub struct AnimationState {
    pub effects: Vec<(Effect, Rect)>,
    pub last_frame: Instant,
    pub startup_done: bool,
    pub last_page: Option<Pages>,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            effects: Vec::new(),
            last_frame: Instant::now(),
            startup_done: false,
            last_page: None,
        }
    }
}

impl AnimationState {
    pub fn is_active(&self) -> bool {
        !self.effects.is_empty()
    }

    pub fn push(&mut self, effect: Effect, area: Rect) {
        self.effects.push((effect, area));
    }

    pub fn tick(&mut self) -> tachyonfx::Duration {
        let elapsed = self.last_frame.elapsed();
        self.last_frame = Instant::now();
        elapsed.into()
    }
}
