use bevy::log;
use bevy::prelude::*;

#[derive(Component)]
pub struct Lifes {
    lifes: u32,
    initial_lifes: u32,
    max_lifes: u32,
}

impl Lifes {
    pub fn new(initial_lifes: u32, max_lifes: u32) -> Self {
        log::info!(
            "Lifes initialized with {} lifes, max lifes: {}",
            initial_lifes,
            max_lifes
        );
        Self {
            lifes: initial_lifes,
            initial_lifes,
            max_lifes,
        }
    }

    pub fn decrease(&mut self, diff: u32) {
        self.lifes = self.lifes.saturating_sub(diff);
        log::info!("Lifes decreased to {}", self.lifes);
    }

    pub fn increase(&mut self, diff: u32) {
        self.lifes += diff;
        if self.lifes > self.max_lifes {
            self.lifes = self.max_lifes;
            log::info!("Lifes increased to max lifes: {}", self.lifes);
        }
        log::info!("Lifes increased to {}", self.lifes);
    }

    pub fn reset(&mut self) {
        self.lifes = self.initial_lifes;
        log::info!("Lifes reset to {}", self.lifes);
    }

    pub fn is_empty(&self) -> bool {
        self.lifes == 0
    }
}