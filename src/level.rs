use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

use crate::matrix_match::gem::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LevelObjective {
    Score(u32),
    CollectGems { gem_type: char, count: u32 },
    ClearGems(u32),
    Combo(u32),
    SpecialGems(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub moves: u32,
    pub objectives: Vec<LevelObjective>,
    pub grid_layout: Option<[[char; 8]; 8]>,
    pub special_gems_allowed: bool,
    pub time_limit: Option<f32>,
    pub unlock_score: u32,
    pub stars_required: [u32; 3],
}

impl Level {
    pub fn new(id: u32, moves: u32, objectives: Vec<LevelObjective>) -> Self {
        let stars = Self::calculate_stars(&objectives);

        Level {
            id,
            name: format!("Level {}", id),
            description: String::new(),
            moves,
            objectives,
            grid_layout: None,
            special_gems_allowed: true,
            time_limit: None,
            unlock_score: stars[0],
            stars_required: stars,
        }
    }

    fn calculate_stars(objectives: &[LevelObjective]) -> [u32; 3] {
        let total = objectives
            .iter()
            .map(|obj| match obj {
                LevelObjective::Score(s) => *s,
                LevelObjective::CollectGems { count, .. } => *count * 10,
                LevelObjective::ClearGems(c) => *c * 5,
                LevelObjective::Combo(c) => *c * 50,
                LevelObjective::SpecialGems(s) => *s * 100,
            })
            .sum::<u32>();

        [
            total / 2,     //1 bintang: 50%
            total * 3 / 4, //2 bintang: 75%
            total,         //3 bintang: 100%
        ]
    }

    pub fn check_completion(
        &self,
        score: u32,
        collected_gems: &std::collections::HashMap<char, u32>,
        max_combo: u32,
        special_created: u32,
    ) -> LevelResult {
        let mut completed = true;
        let mut progress = Vec::new();

        for objective in &self.objectives {
            let (completed_obj, current, target) = match objective {
                LevelObjective::Score(target) => (score >= *target, score, *target),
                LevelObjective::CollectGems { gem_type, count } => {
                    let current = *collected_gems.get(gem_type).unwrap_or(&0);
                    (current >= *count, current, *count)
                }
                LevelObjective::ClearGems(target) => (score / 10 >= *target, score / 10, *target),
                LevelObjective::Combo(target) => (max_combo >= *target, max_combo, *target),
                LevelObjective::SpecialGems(target) => {
                    (special_created >= *target, special_created, *target)
                }
            };

            progress.push((current, target));
            if !completed_obj {
                completed = false;
            }
        }

        let stars = if completed {
            if score >= self.stars_required[2] {
                3
            } else if score >= self.stars_required[1] {
                2
            } else if score >= self.stars_required[0] {
                1
            } else {
                0
            }
        } else {
            0
        };

        LevelResult {
            completed,
            stars,
            progress,
            score,
            moves_used: 0,
        }
    }
}

pub struct LevelResult {
    pub completed: bool,
    pub stars: u32,
    pub progress: Vec<(u32, u32)>,
    pub score: u32,
    pub moves_used: u32,
}

pub struct LevelManager {
    pub levels: Vec<Level>,
    pub current_level: u32,
    pub max_unlocked_level: u32,
    pub level_stars: std::collections::HashMap<u32, u32>,
}

impl LevelManager {
    pub fn new() -> Self {
        let mut manager = LevelManager {
            levels: Vec::with_capacity(200),
            current_level: 1,
            max_unlocked_level: 1,
            level_stars: std::collections::HashMap::new(),
        };

        manager.generate_levels();
        manager
    }

    pub fn generate_levels(&mut self) {
        for i in 1..=10 {
            let objectives = vec![LevelObjective::Score(100 * i)];
            let mut level = Level::new(i, 15 + i * 2, objectives);
            level.name = format!("Tutorial {}", i);
            level.description = "Pelajari dasar permainan".to_string();
            self.levels.push(level);
        }

        for i in 11..=30 {
            let objectives = vec![
                LevelObjective::Score(200 * (i - 10)),
                LevelObjective::Combo(3 + (i - 11) / 5),
            ];
            let mut level = Level::new(i, 20 + (i - 10) * 2, objectives);
            level.name = format!("Combo Challenge {}", i - 10);
            level.description = "Raih kombo tinggi".to_string();
            self.levels.push(level);
        }

        let gem_types = [GEM_1, GEM_2, GEM_3, GEM_4, GEM_5];
        for i in 31..=60 {
            let gem_idx = ((i - 31) % 5) as usize;
            let objectives = vec![
                LevelObjective::Score(300 + (i - 30) * 20),
                LevelObjective::CollectGems {
                    gem_type: gem_types[gem_idx],
                    count: 5 + (i - 31) / 2,
                },
            ];
            let mut level = Level::new(i, 25 + (i - 30) / 2, objectives);
            level.name = format!("Collector {}", i - 30);
            level.description = format!("Kumpulkan {} sebanyak mungkin", gem_types[gem_idx]);
            self.levels.push(level);
        }

        for i in 61..=90 {
            let objectives = vec![
                LevelObjective::Score(500 + (i - 60) * 30),
                LevelObjective::SpecialGems(2 + (i - 61) / 5),
            ];
            let mut level = Level::new(i, 20 + (i - 60) / 2, objectives);
            level.name = format!("Specialist {}", i - 60);
            level.description = "Ciptakan gem spesial".to_string();
            self.levels.push(level);
        }

        for i in 91..=120 {
            let objectives = vec![
                LevelObjective::Score(800 + (i - 90) * 40),
                LevelObjective::ClearGems(50 + (i - 91) * 3),
            ];
            let mut level = Level::new(i, 30 + (i - 90) / 3, objectives);
            level.name = format!("Clearing {}", i - 90);
            level.description = "Bersihkan papan".to_string();
            self.levels.push(level);
        }

        for i in 121..=150 {
            let objectives = vec![
                LevelObjective::Score(1000 + (i - 120) * 50),
                LevelObjective::Combo(5 + (i - 121) / 5),
                LevelObjective::SpecialGems(3 + (i - 121) / 8),
            ];
            let mut level = Level::new(i, 25 + (i - 120) / 4, objectives);
            level.name = format!("Master {}", i - 120);
            level.description = "Kombinasi objectives".to_string();
            self.levels.push(level);
        }

        for i in 151..=180 {
            let objectives = vec![
                LevelObjective::Score(1500 + (i - 150) * 60),
                LevelObjective::CollectGems {
                    gem_type: GEM_5,
                    count: 8 + (i - 151) / 3,
                },
            ];
            let mut level = Level::new(i, 15 + (i - 150) / 5, objectives);
            level.name = format!("Speed Run {}", i - 150);
            level.description = "Selesaikan dengan moves terbatas".to_string();
            self.levels.push(level);
        }

        for i in 181..=200 {
            let objectives = vec![
                LevelObjective::Score(2000 + (i - 180) * 80),
                LevelObjective::Combo(8 + (i - 181) / 3),
                LevelObjective::SpecialGems(5 + (i - 181) / 4),
                LevelObjective::ClearGems(80 + (i - 181) * 2),
            ];
            let mut level = Level::new(i, 20 + (i - 180) / 2, objectives);
            level.name = format!("Expert {}", i - 180);
            level.description = "Ujian terakhir".to_string();
            self.levels.push(level);
        }
    }

    pub fn get_current_level(&self) -> Option<&Level> {
        self.levels.get(self.current_level as usize - 1)
    }

    pub fn get_level(&self, level_id: u32) -> Option<&Level> {
        if level_id >= 1 && level_id <= self.levels.len() as u32 {
            self.levels.get(level_id as usize - 1)
        } else {
            None
        }
    }

    pub fn complete_level(&mut self, level_id: u32, stars: u32) {
        let current_stars = self.level_stars.entry(level_id).or_insert(0);
        if stars > *current_stars {
            *current_stars = stars;
        }

        if level_id == self.max_unlocked_level && level_id < 200 {
            self.max_unlocked_level += 1;
        }
    }

    pub fn can_play_level(&self, level_id: u32) -> bool {
        level_id <= self.max_unlocked_level
    }

    pub fn get_level_stars(&self, level_id: u32) -> u32 {
        *self.level_stars.get(&level_id).unwrap_or(&0)
    }

    pub fn get_total_stars(&self) -> u32 {
        self.level_stars.values().sum()
    }
}

#[derive(Clone)]
pub struct LevelSession {
    pub level: Level,
    pub moves_left: u32,
    pub initial_moves: u32,
    pub score: u32,
    pub collected_gems: std::collections::HashMap<char, u32>,
    pub special_created: u32,
    pub max_combo: u32,
    pub time_left: Option<f32>,
    pub objectives_completed: Vec<bool>,
}

impl LevelSession {
    pub fn new(level: Level) -> Self {
        LevelSession {
            moves_left: level.moves,
            initial_moves: level.moves,
            score: 0,
            collected_gems: std::collections::HashMap::new(),
            special_created: 0,
            max_combo: 0,
            time_left: level.time_limit,
            objectives_completed: vec![false; level.objectives.len()],
            level,
        }
    }

    pub fn use_move(&mut self) -> bool {
        if self.moves_left > 0 {
            self.moves_left -= 1;
            true
        } else {
            false
        }
    }

    pub fn add_score(&mut self, points: u32) {
        self.score += points;
    }

    pub fn collect_gem(&mut self, gem: char) {
        *self.collected_gems.entry(gem).or_insert(0) += 1;
    }

    pub fn add_special(&mut self) {
        self.special_created += 1;
    }

    pub fn update_combo(&mut self, combo: u32) {
        if combo > self.max_combo {
            self.max_combo = combo;
        }
    }

    pub fn update_time(&mut self, dt: f32) {
        if let Some(time) = &mut self.time_left {
            *time -= dt;
            if *time < 0.0 {
                *time = 0.0;
            }
        }
    }

    pub fn check_completion(&self) -> LevelResult {
        self.level.check_completion(
            self.score,
            &self.collected_gems,
            self.max_combo,
            self.special_created,
        )
    }

    pub fn is_out_of_moves(&self) -> bool {
        self.moves_left == 0
    }

    pub fn is_time_out(&self) -> bool {
        if let Some(time) = self.time_left {
            time <= 0.0
        } else {
            false
        }
    }

    pub fn get_progress_percentage(&self) -> f32 {
        let result = self.check_completion();
        if result.completed {
            1.0
        } else {
            let total_progress: f32 = result
                .progress
                .iter()
                .map(|(current, target)| {
                    if *target == 0 {
                        1.0
                    } else {
                        *current as f32 / *target as f32
                    }
                })
                .sum();
            total_progress / result.progress.len() as f32
        }
    }
}
