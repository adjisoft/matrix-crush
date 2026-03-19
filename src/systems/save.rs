use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
use directories::ProjectDirs;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub high_score: u32,
    pub total_games: u32,
    pub total_matches: u32,
    pub language: String,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub sound_enabled: bool,
    pub last_played: String,

    pub max_unlocked_level: u32,
    pub level_stars: HashMap<u32, u32>,
    pub level_scores: HashMap<u32, u32>,
    pub save_slot: u32,
    // Currency
    pub data_core: u32,
    pub entropy: u32,
    pub glitch_matter: u32,
    // Story Mode
    pub story_choices: HashMap<u32, u8>,  // level_id -> chosen route (0=default,1=trust,2=break)
    pub story_unlocked: bool,             // completed story mode
}

impl Default for SaveData {
    fn default() -> Self {
        let last_played = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                chrono::Local::now().format("%Y-%m-%d").to_string()
            }
            #[cfg(target_arch = "wasm32")]
            {
                "1970-01-01".to_string()
            }
        };

        SaveData {
            high_score: 0,
            total_games: 0,
            total_matches: 0,
            language: "en".to_string(),
            music_volume: 0.5,
            sfx_volume: 0.7,
            sound_enabled: true,
            last_played,
            max_unlocked_level: 1,
            level_stars: HashMap::new(),
            level_scores: HashMap::new(),
            save_slot: 1,
            data_core: 0,
            entropy: 0,
            glitch_matter: 0,
            story_choices: HashMap::new(),
            story_unlocked: false,
        }
    }
}

impl SaveData {
    pub fn new(slot: u32) -> Self {
        let mut data = SaveData::default();
        data.save_slot = slot;
        data
    }
}

pub struct SaveManager {
    #[cfg(not(target_arch = "wasm32"))]
    save_dir: PathBuf,
    #[cfg(target_arch = "wasm32")]
    slot_cache: RefCell<HashMap<u32, SaveData>>,
    pub data: SaveData, // Ubah jadi public
    current_slot: u32,  // Tetap private
}

impl SaveManager {
    pub fn new() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let save_dir =
            if let Some(proj_dirs) = ProjectDirs::from("com", "matrix_match", "ascii-match") {
                proj_dirs.data_dir().to_path_buf()
            } else {
                PathBuf::from("./save")
            };

        #[cfg(not(target_arch = "wasm32"))]
        fs::create_dir_all(&save_dir).unwrap_or_default();

        let mut manager = SaveManager {
            #[cfg(not(target_arch = "wasm32"))]
            save_dir,
            #[cfg(target_arch = "wasm32")]
            slot_cache: RefCell::new(HashMap::new()),
            data: SaveData::default(),
            current_slot: 1,
        };

        manager.load_from_slot(1);
        manager
    }

    // Public getter untuk current_slot
    pub fn current_slot(&self) -> u32 {
        self.current_slot
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn get_slot_path(&self, slot: u32) -> PathBuf {
        self.save_dir.join(format!("save_slot_{}.ron", slot))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_from_slot(&mut self, slot: u32) -> bool {
        let save_file = self.get_slot_path(slot);

        if save_file.exists() {
            if let Ok(content) = fs::read_to_string(&save_file) {
                if let Ok(data) = ron::from_str(&content) {
                    self.data = data;
                    self.current_slot = slot;
                    return true;
                }
            }
        }

        self.data = SaveData::new(slot);
        self.current_slot = slot;
        false
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load_from_slot(&mut self, slot: u32) -> bool {
        if let Some(data) = self.slot_cache.borrow().get(&slot).cloned() {
            self.data = data;
            self.current_slot = slot;
            return true;
        }

        self.data = SaveData::new(slot);
        self.current_slot = slot;
        false
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_to_slot(&self, slot: u32) -> bool {
        let save_file = self.get_slot_path(slot);
        if let Ok(content) = ron::to_string(&self.data) {
            fs::write(save_file, content).is_ok()
        } else {
            false
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save_to_slot(&self, slot: u32) -> bool {
        self.slot_cache
            .borrow_mut()
            .insert(slot, self.data.clone());
        true
    }

    // Ganti nama dari save_current ke save
    pub fn save(&self) -> bool {
        self.save_to_slot(self.current_slot)
    }

    pub fn reset_slot(&mut self, slot: u32) {
        self.data = SaveData::new(slot);
        self.current_slot = slot;
        self.save();
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_slot_info(&self, slot: u32) -> Option<(u32, u32, u32)> {
        let save_file = self.get_slot_path(slot);
        if save_file.exists() {
            if let Ok(content) = fs::read_to_string(&save_file) {
                if let Ok(data) = ron::from_str::<SaveData>(&content) {
                    return Some((
                        data.high_score,
                        data.max_unlocked_level,
                        data.level_stars.values().sum(),
                    ));
                }
            }
        }
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_slot_info(&self, slot: u32) -> Option<(u32, u32, u32)> {
        self.slot_cache.borrow().get(&slot).map(|data| {
            (
                data.high_score,
                data.max_unlocked_level,
                data.level_stars.values().sum(),
            )
        })
    }

    pub fn update_high_score(&mut self, score: u32) {
        if score > self.data.high_score {
            self.data.high_score = score;
            self.save();
        }
    }

    pub fn add_game(&mut self) {
        self.data.total_games += 1;
        self.save();
    }

    pub fn add_match(&mut self) {
        self.data.total_matches += 1;
        self.save();
    }

    pub fn update_level_progress(&mut self, level_id: u32, stars: u32, score: u32) {
        let current_stars = self.data.level_stars.entry(level_id).or_insert(0);
        if stars > *current_stars {
            *current_stars = stars;
        }

        let current_score = self.data.level_scores.entry(level_id).or_insert(0);
        if score > *current_score {
            *current_score = score;
        }

        if level_id == self.data.max_unlocked_level && level_id < 200 {
            self.data.max_unlocked_level += 1;
        }

        self.save();
    }

    pub fn get_level_stars(&self, level_id: u32) -> u32 {
        *self.data.level_stars.get(&level_id).unwrap_or(&0)
    }

    pub fn get_level_score(&self, level_id: u32) -> u32 {
        *self.data.level_scores.get(&level_id).unwrap_or(&0)
    }

    pub fn get_total_stars(&self) -> u32 {
        self.data.level_stars.values().sum()
    }

    pub fn reset_progress(&mut self) {
        self.data.high_score = 0;
        self.data.total_games = 0;
        self.data.total_matches = 0;
        self.data.max_unlocked_level = 1;
        self.data.level_stars.clear();
        self.data.level_scores.clear();
        self.save();
    }

    pub fn copy_to_slot(&mut self, target_slot: u32) -> bool {
        self.save_to_slot(target_slot)
    }
}
