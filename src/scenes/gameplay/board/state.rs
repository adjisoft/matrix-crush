use macroquad::prelude::*;

use crate::game::board::{Grid, GRID_HEIGHT, GRID_WIDTH};
use crate::game::entities::gem::*;
use crate::game::progression::LevelSession;
use crate::systems::effects::{BombEffect, FallingGem, Particle, ScreenShake, SweepEffect};

pub const CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);

pub fn cell_size() -> f32 {
    let max_w = screen_width() * 0.9;
    let max_h = screen_height() * 0.7;
    let w_size = max_w / GRID_WIDTH as f32;
    let h_size = max_h / GRID_HEIGHT as f32;
    w_size.min(h_size).min(50.0)
}

pub fn board_offset_x() -> f32 {
    let total_w = GRID_WIDTH as f32 * cell_size();
    if screen_width() > screen_height() {
        let leftover = screen_width() - total_w - 200.0;
        if leftover > 0.0 {
            leftover / 2.0
        } else {
            10.0
        }
    } else {
        (screen_width() - total_w) / 2.0
    }
}

pub fn board_offset_y() -> f32 {
    let total_h = GRID_HEIGHT as f32 * cell_size();
    let top_margin: f32 = if screen_width() > screen_height() { 80.0 } else { 120.0 };
    top_margin.max((screen_height() - total_h) / 2.0)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DialogKind {
    Combo,
    Powerup,
    Score,
}

pub const ANON_FACES: [&str; 5] = ["(0_0)", "(O_O)", "(0_0)", "(o_o)", "(@_@)"];

const DIALOG_COMBO_ID: [&str; 4] = [
    "Combo mantap!",
    "Teruskan ritmenya.",
    "Laju kombo naik.",
    "Nyala!",
];
const DIALOG_COMBO_EN: [&str; 4] = [
    "Nice combo!",
    "Keep the rhythm.",
    "Combo rising.",
    "On fire!",
];

const DIALOG_POWERUP_ID: [&str; 4] = [
    "Powerup aktif.",
    "Ledakan terpicu.",
    "Energi melonjak.",
    "Grid terguncang.",
];
const DIALOG_POWERUP_EN: [&str; 4] = [
    "Power up triggered.",
    "Blast engaged.",
    "Energy spike.",
    "Grid disrupted.",
];

const DIALOG_SCORE_ID: [&str; 4] = [
    "Skor naik.",
    "Progres bagus.",
    "Terus gas.",
    "Milestone tercapai.",
];
const DIALOG_SCORE_EN: [&str; 4] = [
    "Score climbing.",
    "Good progress.",
    "Keep pushing.",
    "Milestone reached.",
];

pub fn dialog_variant_count(kind: DialogKind) -> usize {
    match kind {
        DialogKind::Combo => DIALOG_COMBO_ID.len(),
        DialogKind::Powerup => DIALOG_POWERUP_ID.len(),
        DialogKind::Score => DIALOG_SCORE_ID.len(),
    }
}

pub fn dialog_message(kind: DialogKind, language: &str, variant: usize) -> &'static str {
    let use_id = language == "id";
    match kind {
        DialogKind::Combo => {
            let list = if use_id { &DIALOG_COMBO_ID } else { &DIALOG_COMBO_EN };
            list[variant % list.len()]
        }
        DialogKind::Powerup => {
            let list = if use_id { &DIALOG_POWERUP_ID } else { &DIALOG_POWERUP_EN };
            list[variant % list.len()]
        }
        DialogKind::Score => {
            let list = if use_id { &DIALOG_SCORE_ID } else { &DIALOG_SCORE_EN };
            list[variant % list.len()]
        }
    }
}

pub struct Board {
    pub grid: Grid,
    pub selected: Option<(usize, usize)>,
    pub score: u32,
    pub combo: u32,
    pub max_combo: u32,
    pub combo_timer: f32,
    pub particles: Vec<Particle>,
    pub falling_gems: Vec<FallingGem>,
    pub sweep_effects: Vec<SweepEffect>,
    pub bomb_effects: Vec<BombEffect>,
    pub screen_shake: ScreenShake,
    pub swap_error_timer: f32,
    pub error_positions: Vec<(usize, usize)>,
    pub is_animating: bool,
    pub last_match_count: u32,
    pub match_effect_timer: f32,
    pub special_gems_to_process: Vec<(usize, usize)>,
    pub level_session: Option<LevelSession>,
    pub level_mode: bool,
    pub level_complete: bool,
    pub level_failed: bool,
    pub show_level_results: bool,
    pub level_result: Option<crate::game::progression::LevelResult>,
    pub earned_data_core: u32,
    pub earned_entropy: u32,
    pub dialog_kind: Option<DialogKind>,
    pub dialog_variant: usize,
    pub dialog_timer: f32,
    pub dialog_glitch_intensity: f32,
    pub next_score_milestone: u32,
    pub last_combo_shown: u32,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            grid: [[GEM_1; GRID_WIDTH]; GRID_HEIGHT],
            selected: None,
            score: 0,
            combo: 0,
            max_combo: 0,
            combo_timer: 0.0,
            particles: Vec::new(),
            falling_gems: Vec::new(),
            sweep_effects: Vec::new(),
            bomb_effects: Vec::new(),
            screen_shake: ScreenShake::new(),
            swap_error_timer: 0.0,
            error_positions: Vec::new(),
            is_animating: false,
            last_match_count: 0,
            match_effect_timer: 0.0,
            special_gems_to_process: Vec::new(),
            level_session: None,
            level_mode: false,
            level_complete: false,
            level_failed: false,
            show_level_results: false,
            level_result: None,
            earned_data_core: 0,
            earned_entropy: 0,
            dialog_kind: None,
            dialog_variant: 0,
            dialog_timer: 0.0,
            dialog_glitch_intensity: 0.0,
            next_score_milestone: 1000,
            last_combo_shown: 0,
        };
        board.initialize_grid();
        board
    }

    pub fn initialize_grid(&mut self) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                self.grid[y][x] = GEM_CHARS[rand::gen_range(0, 5)];
            }
        }

        while self.clear_matches() > 0 {
            self.apply_gravity_with_animation();
        }
    }

    pub fn start_level(&mut self, level: crate::game::progression::Level) {
        self.level_mode = true;
        self.level_session = Some(crate::game::progression::LevelSession::new(level));
        self.level_complete = false;
        self.level_failed = false;
        self.show_level_results = false;
        self.score = 0;
        self.combo = 0;
        self.special_gems_to_process.clear();
        self.dialog_kind = None;
        self.dialog_variant = 0;
        self.dialog_timer = 0.0;
        self.dialog_glitch_intensity = 0.0;
        self.next_score_milestone = 1000;
        self.last_combo_shown = 0;

        self.initialize_grid();
    }
}
