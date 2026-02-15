use macroquad::audio::*;
use macroquad::prelude::*;
use std::collections::HashMap;

#[warn(dead_code)]
pub struct AudioManager {
    sounds: HashMap<String, Sound>,
}

impl AudioManager {
    pub async fn new() -> Self {
        let mut sounds = HashMap::new();

        if let Ok(sound) = load_sound("assets/sounds/select.wav").await {
            sounds.insert("select".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/match.wav").await {
            sounds.insert("match".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/combo.wav").await {
            sounds.insert("combo".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/not_match.wav").await {
            sounds.insert("not_match".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/explosion.wav").await {
            sounds.insert("explosion".to_string(), sound);
        }

        AudioManager {
            sounds,
        }
    }

    pub fn play_sound(&self, name: &str) {
        if let Some(sound) = self.sounds.get(name) {
            play_sound_once(sound);
        }
    }
}
