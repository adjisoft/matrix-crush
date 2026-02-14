use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub high_score: u32,
    pub total_games: u32,
    pub total_matches: u32,
    pub language: String,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for SaveData {
    fn default() -> Self {
        SaveData {
            high_score: 0,
            total_games: 0,
            total_matches: 0,
            language: "en".to_string(),
            music_volume: 0.5,
            sfx_volume: 0.7,
        }
    }
}

pub struct SaveManager {
    save_path: PathBuf,
    pub data: SaveData,
}

impl SaveManager {
    pub fn new() -> Self {
        let save_path = if let Some(proj_dirs) = ProjectDirs::from("com", "matrix_match", "ascii-match") {
            proj_dirs.data_dir().to_path_buf()
        } else {
            PathBuf::from("./save")
        };

        fs::create_dir_all(&save_path).unwrap_or_default();
        let save_file = save_path.join("save.ron");

        let data = if save_file.exists() {
            if let Ok(content) = fs::read_to_string(&save_file) {
                ron::from_str(&content).unwrap_or_default()
            } else {
                SaveData::default()
            }
        } else {
            SaveData::default()
        };

        SaveManager {
            save_path: save_file,
            data,
        }
    }

    pub fn save(&self) {
        if let Ok(content) = ron::to_string(&self.data) {
            let _ = fs::write(&self.save_path, content);
        }
    }

    pub fn update_high_score(&mut self, score: u32) {
        if score > self.data.high_score {
            self.data.high_score = score;
            self.save();
        }
    }
}
