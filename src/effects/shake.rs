use macroquad::prelude::*;

pub struct ScreenShake {
    pub amount: f32,
    pub duration: f32,
    pub max_duration: f32,
    trauma: f32,
}

impl ScreenShake {
    pub fn new() -> Self {
        ScreenShake {
            amount: 0.0,
            duration: 0.0,
            max_duration: 0.4,
            trauma: 0.0,
        }
    }

    pub fn trigger(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount * 0.1).min(1.0);
        self.amount = amount;
        self.duration = self.max_duration;
    }

    pub fn update(&mut self, dt: f32) {
        if self.duration > 0.0 {
            self.duration -= dt;
            self.trauma *= 0.95;
            if self.duration <= 0.0 {
                self.amount = 0.0;
                self.trauma = 0.0;
            }
        }
    }

    pub fn get_offset(&self) -> Vec2 {
        if self.duration <= 0.0 || self.trauma <= 0.0 {
            return vec2(0.0, 0.0);
        }

        let shake = self.trauma * self.trauma;
        vec2(
            rand::gen_range(-1.0, 1.0) * self.amount * shake, // Hapus parentheses
            rand::gen_range(-1.0, 1.0) * self.amount * shake, // Hapus parentheses
        )
    }
}
