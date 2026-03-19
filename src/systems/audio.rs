use macroquad::audio::*;
use macroquad::prelude::*;
use std::collections::HashMap;

#[warn(dead_code)]
pub struct AudioManager {
    sounds: HashMap<String, Sound>,
    music: HashMap<String, Sound>,
    current_music: Option<String>,
    music_volume: f32,
    sfx_volume: f32,
}

impl AudioManager {
    pub async fn new() -> Self {
        let mut sounds = HashMap::new();
        let mut music = HashMap::new();

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
        if let Ok(sound) = load_sound("assets/sounds/beep.wav").await {
            sounds.insert("beep".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/x_bomb.wav").await {
            sounds.insert("x_bomb".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/v_sweep.wav").await {
            sounds.insert("v_sweep".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/distort_glitch.wav").await {
            sounds.insert("glitch_distort".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/glitchy_portal.wav").await {
            sounds.insert("glitch_portal".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/glitchy_shoots.wav").await {
            sounds.insert("glitch_shoots".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/reverse_glitch.wav").await {
            sounds.insert("glitch_reverse".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/reverse_glitch2.wav").await {
            sounds.insert("glitch_reverse2".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/super_glitchy.wav").await {
            sounds.insert("glitch_super".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/void_glitchy.wav").await {
            sounds.insert("glitch_void".to_string(), sound);
        }
        if let Ok(sound) = load_sound("assets/sounds/atom_collider.wav").await {
            sounds.insert("collider".to_string(), sound);
        }

        if let Ok(track) = load_sound("assets/sounds/main_menu.ogg").await {
            music.insert("bg_main_menu".to_string(), track);
        } else if let Ok(track) = load_sound("assets/sounds/main_menu.wav").await {
            music.insert("bg_main_menu".to_string(), track);
        }
        if let Ok(track) = load_sound("assets/sounds/in_game.ogg").await {
            music.insert("bg_in_game".to_string(), track);
        } else if let Ok(track) = load_sound("assets/sounds/in_game.wav").await {
            music.insert("bg_in_game".to_string(), track);
        }
        if let Ok(track) = load_sound("assets/sounds/neural_sectors.ogg").await {
            music.insert("bg_neural".to_string(), track);
        } else if let Ok(track) = load_sound("assets/sounds/neural_sectors.wav").await {
            music.insert("bg_neural".to_string(), track);
        }
        if let Ok(track) = load_sound("assets/sounds/abonded_labs.ogg").await {
            music.insert("bg_research".to_string(), track);
        } else if let Ok(track) = load_sound("assets/sounds/abonded_labs.wav").await {
            music.insert("bg_research".to_string(), track);
        }

        AudioManager {
            sounds,
            music,
            current_music: None,
            music_volume: 0.5,
            sfx_volume: 0.7,
        }
    }

    pub fn play_sound(&self, name: &str) {
        if let Some(sound) = self.sounds.get(name) {
            play_sound(sound, PlaySoundParams { looped: false, volume: self.sfx_volume });
        }
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
        if let Some(key) = &self.current_music {
            if let Some(sound) = self.music.get(key) {
                set_sound_volume(sound, self.music_volume);
            }
        }
    }

    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }

    pub fn play_music(&mut self, name: &str) {
        if self.current_music.as_deref() == Some(name) {
            return;
        }

        if let Some(current) = &self.current_music {
            if let Some(sound) = self.music.get(current) {
                stop_sound(sound);
            }
        }

        if let Some(sound) = self.music.get(name) {
            play_sound(sound, PlaySoundParams { looped: true, volume: self.music_volume });
            self.current_music = Some(name.to_string());
        }
    }

    pub fn stop_music(&mut self) {
        if let Some(current) = &self.current_music {
            if let Some(sound) = self.music.get(current) {
                stop_sound(sound);
            }
        }
        self.current_music = None;
    }
}
