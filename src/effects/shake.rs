// [file name]: shake.rs
use macroquad::prelude::*;

pub struct ScreenShake {
    pub amount: f32,
    pub duration: f32,
    pub max_duration: f32,
    trauma: f32,
    frequency: f32,
    seed: f32,
    last_offset: Vec2,
}

impl ScreenShake {
    pub fn new() -> Self {
        ScreenShake {
            amount: 0.0,
            duration: 0.0,
            max_duration: 0.8,
            trauma: 0.0,
            frequency: 12.0,
            seed: rand::gen_range(0.0, 100.0),
            last_offset: Vec2::ZERO,
        }
    }

    pub fn trigger(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.2); // Bisa lebih kuat
        self.amount = amount;
        self.duration = self.max_duration;
        self.seed = rand::gen_range(0.0, 100.0);
    }

    pub fn update(&mut self, dt: f32) {
        if self.duration > 0.0 {
            self.duration -= dt;
            self.trauma *= 0.98; // Decay lebih lambat untuk efek yang lebih smooth
            
            if self.duration <= 0.0 {
                self.amount = 0.0;
                self.trauma = 0.0;
                self.last_offset = Vec2::ZERO;
            }
        }
    }

    pub fn get_offset(&self) -> Vec2 {
        if self.duration <= 0.0 || self.trauma <= 0.0 {
            return Vec2::ZERO;
        }

        let shake = (self.trauma * self.trauma).min(1.0);
        let time = get_time() as f32;
        
        let x = (time * self.frequency + self.seed).sin() 
            * (time * self.frequency * 1.5 + self.seed * 2.0).cos() * 0.8
            + (time * self.frequency * 3.2 + self.seed * 3.0).sin() * 0.3;
            
        let y = (time * self.frequency * 1.3 + self.seed * 2.5).cos() 
            * (time * self.frequency * 2.1 + self.seed).sin() * 0.8
            + (time * self.frequency * 2.7 + self.seed * 4.0).cos() * 0.3;
        
        vec2(
            x * self.amount * shake * (1.0 - self.duration / self.max_duration * 0.5),
            y * self.amount * shake * (1.0 - self.duration / self.max_duration * 0.5),
        )
    }
    
    pub fn get_smooth_offset(&mut self) -> Vec2 {
        let new_offset = self.get_offset();
        self.last_offset = self.last_offset * 0.7 + new_offset * 0.3;
        self.last_offset
    }
}