use macroquad::prelude::*;
use std::collections::VecDeque;

use crate::audio::AudioManager;
use crate::effects::{BombEffect, FallingGem, Particle, ScreenShake, SweepEffect};
use crate::level::LevelSession;
use crate::matrix_match::gem::*;

pub const GRID_WIDTH: usize = 8;
pub const GRID_HEIGHT: usize = 8;
pub const CELL_SIZE: f32 = 45.0;
pub const BOARD_OFFSET_X: f32 = 150.0;
pub const BOARD_OFFSET_Y: f32 = 80.0;

const CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);

pub struct Board {
    pub grid: [[char; GRID_WIDTH]; GRID_HEIGHT],
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
    pub level_result: Option<crate::level::LevelResult>,
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

    fn create_match_particles(&mut self, matches: &[(usize, usize)], is_combo: bool) {
        for &(x, y) in matches {
            let gem = self.grid[y][x];
            let color = get_gem_color(gem);

            let particle_count = if is_combo { 8 } else { 5 };

            for _ in 0..particle_count {
                let screen_x = x as f32 * CELL_SIZE + BOARD_OFFSET_X + CELL_SIZE / 2.0;
                let screen_y = y as f32 * CELL_SIZE + BOARD_OFFSET_Y + CELL_SIZE / 2.0;

                if is_combo {
                    self.particles
                        .push(Particle::new_explosion(screen_x, screen_y, gem, color));
                } else {
                    self.particles
                        .push(Particle::new(screen_x, screen_y, gem, color));
                }
            }
        }
    }

    fn trigger_bomb_visual(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut affected = Vec::new();
        let bomb_color = get_gem_color(BOMB_GEM);

        let screen_x = x as f32 * CELL_SIZE + BOARD_OFFSET_X + CELL_SIZE / 2.0;
        let screen_y = y as f32 * CELL_SIZE + BOARD_OFFSET_Y + CELL_SIZE / 2.0;
        self.bomb_effects
            .push(BombEffect::new(screen_x, screen_y, bomb_color));

        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < GRID_WIDTH as i32 && ny >= 0 && ny < GRID_HEIGHT as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if self.grid[ny][nx] != ' ' {
                        affected.push((nx, ny));

                        let gem = self.grid[ny][nx];
                        let color = get_gem_color(gem);
                        let screen_x = nx as f32 * CELL_SIZE + BOARD_OFFSET_X + CELL_SIZE / 2.0;
                        let screen_y = ny as f32 * CELL_SIZE + BOARD_OFFSET_Y + CELL_SIZE / 2.0;

                        for _ in 0..10 {
                            self.particles
                                .push(Particle::new_bomb_effect(screen_x, screen_y, gem, color));
                        }
                    }
                }
            }
        }

        self.screen_shake.trigger(15.0);

        affected
    }

    fn trigger_sweep_visual(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut affected = Vec::new();
        let sweep_color = get_gem_color(SWEEP_GEM);

        let direction = rand::gen_range(0.0, std::f32::consts::PI * 2.0);

        let screen_x = x as f32 * CELL_SIZE + BOARD_OFFSET_X + CELL_SIZE / 2.0;
        let screen_y = y as f32 * CELL_SIZE + BOARD_OFFSET_Y + CELL_SIZE / 2.0;
        self.sweep_effects
            .push(SweepEffect::new(screen_x, screen_y, direction, sweep_color));

        for i in 0..GRID_WIDTH {
            if self.grid[y][i] != ' ' && i != x {
                affected.push((i, y));

                let gem = self.grid[y][i];
                let color = get_gem_color(gem);
                let screen_x = i as f32 * CELL_SIZE + BOARD_OFFSET_X + CELL_SIZE / 2.0;
                let screen_y = y as f32 * CELL_SIZE + BOARD_OFFSET_Y + CELL_SIZE / 2.0;

                for _ in 0..6 {
                    self.particles.push(Particle::new_sweep_effect(
                        screen_x, screen_y, gem, color, direction,
                    ));
                }
            }
        }

        for i in 0..GRID_HEIGHT {
            if self.grid[i][x] != ' ' && i != y {
                affected.push((x, i));

                let gem = self.grid[i][x];
                let color = get_gem_color(gem);
                let screen_x = x as f32 * CELL_SIZE + BOARD_OFFSET_X + CELL_SIZE / 2.0;
                let screen_y = i as f32 * CELL_SIZE + BOARD_OFFSET_Y + CELL_SIZE / 2.0;

                for _ in 0..6 {
                    self.particles.push(Particle::new_sweep_effect(
                        screen_x, screen_y, gem, color, direction,
                    ));
                }
            }
        }

        self.screen_shake.trigger(8.0);

        affected
    }

    fn trigger_bomb_with_audio(
        &mut self,
        x: usize,
        y: usize,
        audio: &AudioManager,
    ) -> Vec<(usize, usize)> {
        let affected = self.trigger_bomb_visual(x, y);
        audio.play_sound("explosion");
        affected
    }

    fn trigger_sweep_with_audio(
        &mut self,
        x: usize,
        y: usize,
        audio: &AudioManager,
    ) -> Vec<(usize, usize)> {
        let affected = self.trigger_sweep_visual(x, y);
        audio.play_sound("combo");
        affected
    }

    fn create_error_effect(&mut self, pos1: (usize, usize), pos2: (usize, usize)) {
        self.error_positions = vec![pos1, pos2];
        self.swap_error_timer = 0.5;
        self.screen_shake.trigger(8.0);
        self.combo = 0;

        for &pos in &[pos1, pos2] {
            let gem = self.grid[pos.1][pos.0];
            let color = get_gem_color(gem);

            for _ in 0..4 {
                let screen_x = pos.0 as f32 * CELL_SIZE + BOARD_OFFSET_X + CELL_SIZE / 2.0;
                let screen_y = pos.1 as f32 * CELL_SIZE + BOARD_OFFSET_Y + CELL_SIZE / 2.0;

                self.particles
                    .push(Particle::new_error(screen_x, screen_y, gem, color));
            }
        }
    }

    fn apply_gravity_with_animation(&mut self) {
        let mut new_grid = [[GEM_1; GRID_WIDTH]; GRID_HEIGHT];
        let mut falling_gems_temp = Vec::new();

        for x in 0..GRID_WIDTH {
            let mut column: VecDeque<char> = VecDeque::new();

            for y in (0..GRID_HEIGHT).rev() {
                if self.grid[y][x] != ' ' {
                    column.push_front(self.grid[y][x]);
                }
            }

            for y in (0..GRID_HEIGHT).rev() {
                if let Some(gem) = column.pop_back() {
                    new_grid[y][x] = gem;
                } else {
                    let new_gem = GEM_CHARS[rand::gen_range(0, 5)];
                    new_grid[y][x] = new_gem;

                    let start_y = BOARD_OFFSET_Y - 80.0 - (rand::gen_range(0.0, 50.0));
                    let color = get_gem_color(new_gem);

                    falling_gems_temp.push(FallingGem::new(new_gem, color, x, y, start_y));
                }
            }
        }

        self.grid = new_grid;
        self.falling_gems.extend(falling_gems_temp);
        self.is_animating = !self.falling_gems.is_empty();
    }

    fn update_falling_gems(&mut self, dt: f32) {
        for gem in &mut self.falling_gems {
            gem.update(dt);
        }

        self.falling_gems.retain(|gem| !gem.is_finished());
        self.is_animating = !self.falling_gems.is_empty();
    }

    pub fn swap_gems(
        &mut self,
        pos1: (usize, usize),
        pos2: (usize, usize),
        audio: &AudioManager,
    ) -> bool {
        if self.is_animating || self.level_complete || self.level_failed {
            return false;
        }

        if self.level_mode {
            if let Some(session) = &mut self.level_session {
                if !session.use_move() {
                    self.level_failed = true;
                    return false;
                }
            }
        }

        let temp = self.grid[pos1.1][pos1.0];
        self.grid[pos1.1][pos1.0] = self.grid[pos2.1][pos2.0];
        self.grid[pos2.1][pos2.0] = temp;

        if self.check_for_matches() {
            audio.play_sound("select");
            true
        } else {
            let temp = self.grid[pos1.1][pos1.0];
            self.grid[pos1.1][pos1.0] = self.grid[pos2.1][pos2.0];
            self.grid[pos2.1][pos2.0] = temp;

            self.create_error_effect(pos1, pos2);
            audio.play_sound("not_match");

            if self.level_mode {
                if let Some(session) = &mut self.level_session {
                    session.moves_left += 1;
                }
            }

            false
        }
    }

    fn check_for_matches(&self) -> bool {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if self.check_horizontal_match(x, y).len() >= 3
                    || self.check_vertical_match(x, y).len() >= 3
                {
                    return true;
                }
            }
        }
        false
    }

    fn check_horizontal_match(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        let gem = self.grid[y][x];
        if gem == ' ' || is_special_gem(gem) {
            return matches;
        }

        let mut count = 1;
        for dx in 1..GRID_WIDTH - x {
            if self.grid[y][x + dx] == gem {
                count += 1;
            } else {
                break;
            }
        }

        if count >= 3 {
            for dx in 0..count {
                matches.push((x + dx, y));
            }
        }
        matches
    }

    fn check_vertical_match(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        let gem = self.grid[y][x];
        if gem == ' ' || is_special_gem(gem) {
            return matches;
        }

        let mut count = 1;
        for dy in 1..GRID_HEIGHT - y {
            if self.grid[y + dy][x] == gem {
                count += 1;
            } else {
                break;
            }
        }

        if count >= 3 {
            for dy in 0..count {
                matches.push((x, y + dy));
            }
        }
        matches
    }

    fn clear_matches(&mut self) -> u32 {
        if self.is_animating {
            return 0;
        }

        let mut matches_to_clear = Vec::new();

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if is_special_gem(self.grid[y][x]) {
                    self.special_gems_to_process.push((x, y));
                    continue;
                }

                let horizontal = self.check_horizontal_match(x, y);
                let vertical = self.check_vertical_match(x, y);

                if horizontal.len() >= 3 {
                    matches_to_clear.extend(horizontal);
                }
                if vertical.len() >= 3 {
                    matches_to_clear.extend(vertical);
                }
            }
        }

        matches_to_clear.sort();
        matches_to_clear.dedup();

        let match_count = matches_to_clear.len() as u32;

        if match_count > 0 {
            self.combo += 1;
            if self.combo > self.max_combo {
                self.max_combo = self.combo;
            }
            self.combo_timer = 2.0;

            if self.level_mode {
                if let Some(session) = &mut self.level_session {
                    for &(x, y) in &matches_to_clear {
                        session.collect_gem(self.grid[y][x]);
                    }
                }
            }

            let base_score = match_count * 10;
            let combo_multiplier = 1.0 + (self.combo as f32 * 0.2).min(2.0);
            let total_score = (base_score as f32 * combo_multiplier) as u32;

            self.score += total_score;

            if let Some(session) = &mut self.level_session {
                session.add_score(total_score);
            }

            self.last_match_count = match_count;
            self.match_effect_timer = 0.5;

            self.screen_shake.trigger(3.0 + self.combo as f32);

            let is_combo = self.combo >= 3;

            if match_count >= 3 {
                if let Some(&(x, y)) = matches_to_clear.first() {
                    if let Some(special_gem) = create_special_gem(match_count as usize) {
                        self.grid[y][x] = special_gem;
                        matches_to_clear.retain(|&pos| pos != (x, y));
                    }
                }
            }

            self.create_match_particles(&matches_to_clear, is_combo);

            for (x, y) in matches_to_clear {
                self.grid[y][x] = ' ';
            }

            if self.level_mode {
                if let Some(session) = &self.level_session {
                    let result = session.check_completion();
                    if result.completed {
                        self.level_complete = true;
                        self.level_result = Some(result);
                    }
                }
            }
        }

        match_count
    }

    fn process_special_gems(&mut self, audio: &AudioManager) -> u32 {
        if self.special_gems_to_process.is_empty() {
            return 0;
        }

        let gems_to_process = self.special_gems_to_process.clone();
        self.special_gems_to_process.clear();

        let mut affected = Vec::new();

        for &(x, y) in &gems_to_process {
            if self.grid[y][x] == BOMB_GEM {
                affected.extend(self.trigger_bomb_with_audio(x, y, audio));
            } else if self.grid[y][x] == SWEEP_GEM {
                affected.extend(self.trigger_sweep_with_audio(x, y, audio));
            }
        }

        affected.extend(gems_to_process);

        for (x, y) in &affected {
            if self.grid[*y][*x] != ' ' {
                self.grid[*y][*x] = ' ';
            }
        }

        affected.len() as u32
    }

    pub fn update(&mut self, dt: f32, audio: &AudioManager) {
        if self.level_mode {
            if let Some(session) = &mut self.level_session {
                session.update_time(dt);

                if session.is_out_of_moves() || session.is_time_out() {
                    self.level_failed = true;
                }
            }
        }

        self.particles.retain_mut(|p| {
            p.update(dt);
            p.is_alive()
        });

        self.sweep_effects.retain_mut(|e| {
            e.update(dt);
            e.is_alive()
        });

        self.bomb_effects.retain_mut(|e| {
            e.update(dt);
            e.is_alive()
        });

        self.update_falling_gems(dt);
        self.screen_shake.update(dt);

        if self.combo_timer > 0.0 {
            self.combo_timer -= dt;
            if self.combo_timer <= 0.0 {
                self.combo = 0;
            }
        }

        if self.match_effect_timer > 0.0 {
            self.match_effect_timer -= dt;
        }

        if self.swap_error_timer > 0.0 {
            self.swap_error_timer -= dt;
            if self.swap_error_timer <= 0.0 {
                self.error_positions.clear();
            }
        }

        let special_count = self.process_special_gems(audio);
        if special_count > 0 {
            self.apply_gravity_with_animation();

            if let Some(session) = &mut self.level_session {
                session.add_special();
            }
        }

        if !self.is_animating {
            let matches_cleared = self.clear_matches();
            if matches_cleared > 0 {
                audio.play_sound("match");
                if self.combo >= 3 {
                    audio.play_sound("combo");
                }
                if self.combo >= 5 {
                    audio.play_sound("explosion");
                }

                if let Some(session) = &mut self.level_session {
                    session.update_combo(self.combo);
                }

                self.apply_gravity_with_animation();
            }
        }
    }

    pub fn draw(&self, language: &str) {
        let shake_offset = self.screen_shake.get_offset();

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let rect = Rect::new(
                    x as f32 * CELL_SIZE + BOARD_OFFSET_X + shake_offset.x,
                    y as f32 * CELL_SIZE + BOARD_OFFSET_Y + shake_offset.y,
                    CELL_SIZE,
                    CELL_SIZE,
                );

                let bg_intensity = if (x + y) % 2 == 0 { 0.15 } else { 0.2 };
                let bg_color = Color::new(0.0, bg_intensity, 0.0, 1.0);
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg_color);

                let border_color = self.get_border_color(x, y);
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, border_color);
            }
        }

        // Draw effects
        for effect in &self.sweep_effects {
            effect.draw(shake_offset);
        }

        for effect in &self.bomb_effects {
            effect.draw(shake_offset);
        }

        for gem in &self.falling_gems {
            gem.draw(shake_offset);
        }

        // Draw gems
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let has_falling = self
                    .falling_gems
                    .iter()
                    .any(|g| g.target_x == x && g.target_y == y);

                if !has_falling {
                    let gem = self.grid[y][x];
                    if gem != ' ' {
                        let color = get_gem_color(gem);
                        let rect = Rect::new(
                            x as f32 * CELL_SIZE + BOARD_OFFSET_X + shake_offset.x,
                            y as f32 * CELL_SIZE + BOARD_OFFSET_Y + shake_offset.y,
                            CELL_SIZE,
                            CELL_SIZE,
                        );

                        let mut final_color = color;
                        if self.match_effect_timer > 0.0 && self.grid[y][x] != ' ' {
                            let pulse = (self.match_effect_timer * 10.0).sin().abs() * 0.5 + 0.5;
                            final_color = Color::new(color.r, color.g * pulse, color.b, color.a);
                        }

                        if is_special_gem(gem) {
                            let pulse = (get_time() as f32 * 3.0).sin().abs() * 0.3 + 0.7;
                            final_color = Color::new(
                                final_color.r * pulse,
                                final_color.g * pulse,
                                final_color.b * pulse,
                                final_color.a,
                            );
                        }

                        // PERBAIKAN: Ganti emoji dengan karakter ASCII
                        let gem_char = match gem {
                            '💎' => 'O', // Ruby
                            '💠' => '#', // Diamond/Sapphire
                            '💚' => '@', // Emerald
                            '💛' => '$', // Topaz/Gold
                            '💜' => '%', // Amethyst
                            '💣' => 'X', // Bomb (ganti dari emoji bom)
                            '🌀' => '+', // Sweep (ganti dari emoji tornado)
                            _ => gem,
                        };

                        draw_text_ex(
                            &gem_char.to_string(),
                            rect.x + CELL_SIZE / 2.0 - 8.0,
                            rect.y + CELL_SIZE / 2.0 + 8.0,
                            TextParams {
                                font_size: if is_special_gem(gem) { 32 } else { 28 },
                                color: final_color,
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }

        for particle in &self.particles {
            particle.draw(shake_offset);
        }

        // Draw UI Panel (berbeda untuk level mode)
        if self.level_mode {
            self.draw_level_ui(language, shake_offset);
        } else {
            self.draw_classic_ui(language, shake_offset);
        }

        // Draw level complete/failed overlay
        if self.level_complete {
            self.draw_level_complete();
        } else if self.level_failed {
            self.draw_level_failed();
        }
    }

    fn draw_level_ui(&self, language: &str, _shake_offset: Vec2) {
        if let Some(session) = &self.level_session {
            let panel_x = BOARD_OFFSET_X + GRID_WIDTH as f32 * CELL_SIZE + 30.0;
            let panel_y = BOARD_OFFSET_Y;

            // Panel background
            draw_rectangle(
                panel_x - 10.0,
                panel_y - 10.0,
                250.0,
                400.0,
                Color::new(0.0, 0.1, 0.0, 0.9),
            );

            // Level info
            draw_text(
                &format!("Level {}", session.level.id),
                panel_x,
                panel_y + 30.0,
                24.0,
                GREEN,
            );

            // Moves left
            let moves_text = if language == "id" { "GERAKAN" } else { "MOVES" };
            draw_text(moves_text, panel_x, panel_y + 70.0, 20.0, YELLOW);

            let moves_color = if session.moves_left <= 3 {
                RED
            } else if session.moves_left <= 5 {
                ORANGE
            } else {
                WHITE
            };

            draw_text(
                &format!("{} / {}", session.moves_left, session.initial_moves),
                panel_x,
                panel_y + 100.0,
                28.0,
                moves_color,
            );

            // Score
            draw_text("SCORE", panel_x, panel_y + 140.0, 20.0, LIGHTGRAY);
            draw_text(
                &format!("{}", self.score),
                panel_x,
                panel_y + 170.0,
                24.0,
                WHITE,
            );

            // Objectives
            draw_text("OBJECTIVES:", panel_x, panel_y + 210.0, 18.0, CYAN);

            let result = session.check_completion();
            for (i, ((current, target), obj)) in result
                .progress
                .iter()
                .zip(&session.level.objectives)
                .enumerate()
            {
                let y = panel_y + 240.0 + i as f32 * 25.0;

                let obj_text = match obj {
                    crate::level::LevelObjective::Score(_) => {
                        format!("Score: {}/{}", current, target)
                    }
                    crate::level::LevelObjective::CollectGems { gem_type, .. } => {
                        // PERBAIKAN: Ganti emoji dengan karakter ASCII di UI
                        let gem_display = match gem_type {
                            '💎' => "O", // Ruby
                            '💠' => "#", // Sapphire
                            '💚' => "@", // Emerald
                            '💛' => "$", // Topaz
                            '💜' => "%", // Amethyst
                            _ => "?",
                        };
                        format!(
                            "{} {}: {}/{}",
                            if language == "id" {
                                "Kumpulkan"
                            } else {
                                "Collect"
                            },
                            gem_display,
                            current,
                            target
                        )
                    }
                    crate::level::LevelObjective::ClearGems(_) => {
                        format!("Clear: {}/{}", current, target)
                    }
                    crate::level::LevelObjective::Combo(_) => {
                        format!("Combo: {}/{}", current, target)
                    }
                    crate::level::LevelObjective::SpecialGems(_) => {
                        format!("Special: {}/{}", current, target)
                    }
                };

                let color = if current >= target { GREEN } else { WHITE };
                draw_text(&obj_text, panel_x + 10.0, y, 14.0, color);
            }

            // Progress bar
            let progress = session.get_progress_percentage();
            let bar_width = 220.0;
            let bar_x = panel_x;
            let bar_y = panel_y + 350.0;

            draw_rectangle(bar_x, bar_y, bar_width, 15.0, DARKGRAY);
            draw_rectangle(bar_x, bar_y, bar_width * progress, 15.0, GREEN);

            draw_text(
                &format!("{:.0}%", progress * 100.0),
                bar_x + bar_width / 2.0 - 20.0,
                bar_y - 5.0,
                14.0,
                WHITE,
            );
        }
    }

    fn draw_classic_ui(&self, language: &str, _shake_offset: Vec2) {
        let panel_x = BOARD_OFFSET_X + GRID_WIDTH as f32 * CELL_SIZE + 30.0;
        let panel_y = BOARD_OFFSET_Y;

        draw_rectangle(
            panel_x - 10.0,
            panel_y - 10.0,
            220.0,
            350.0,
            Color::new(0.0, 0.1, 0.0, 0.8),
        );

        let score_text = if language == "id" { "SKOR" } else { "SCORE" };
        draw_text(score_text, panel_x, panel_y + 30.0, 24.0, GREEN);
        draw_text(
            &format!("{}", self.score),
            panel_x,
            panel_y + 60.0,
            32.0,
            WHITE,
        );

        if self.combo > 0 {
            let combo_text = if language == "id" { "KOMBO" } else { "COMBO" };
            draw_text(combo_text, panel_x, panel_y + 110.0, 24.0, YELLOW);

            let combo_color = if self.combo >= 5 {
                Color::new(1.0, 0.0, 0.0, 1.0)
            } else if self.combo >= 3 {
                Color::new(1.0, 0.5, 0.0, 1.0)
            } else {
                YELLOW
            };

            let combo_display = format!("x{}", self.combo);
            let pulse = if self.combo_timer > 0.0 {
                (self.combo_timer * 5.0).sin().abs() * 0.5 + 0.5
            } else {
                1.0
            };

            draw_text_ex(
                &combo_display,
                panel_x,
                panel_y + 140.0,
                TextParams {
                    font_size: 40,
                    color: Color::new(combo_color.r, combo_color.g, combo_color.b, pulse),
                    ..Default::default()
                },
            );
        }

        let multiplier = 1.0 + (self.combo as f32 * 0.2).min(2.0);
        if multiplier > 1.0 {
            draw_text(
                &format!("{:.1}x", multiplier),
                panel_x,
                panel_y + 180.0,
                20.0,
                Color::new(0.5, 1.0, 0.5, 0.8),
            );
        }

        // PERBAIKAN: Ganti emoji dengan karakter ASCII di penjelasan special
        draw_text("SPECIAL:", panel_x, panel_y + 220.0, 18.0, CYAN);
        draw_text(
            "X Bomb: 3x3",
            panel_x,
            panel_y + 245.0,
            14.0,
            Color::new(1.0, 0.5, 0.5, 1.0),
        );
        draw_text(
            "+ Sweep: Row+Col",
            panel_x,
            panel_y + 265.0,
            14.0,
            Color::new(0.5, 1.0, 1.0, 1.0),
        );

        if self.max_combo > 0 {
            let max_text = if language == "id" {
                format!("Kombo Maks: {}", self.max_combo)
            } else {
                format!("Max Combo: {}", self.max_combo)
            };
            draw_text(&max_text, panel_x, panel_y + 300.0, 16.0, GRAY);
        }

        if self.is_animating {
            let anim_text = if language == "id" {
                "> BAGUS <"
            } else {
                "> GOOD <"
            };
            let text_width = measure_text(anim_text, None, 20, 1.0).width;
            draw_text(
                anim_text,
                BOARD_OFFSET_X + (GRID_WIDTH as f32 * CELL_SIZE) / 2.0 - text_width / 2.0,
                BOARD_OFFSET_Y - 30.0,
                20.0,
                Color::new(0.0, 1.0, 0.0, 0.7),
            );
        }
    }

    fn draw_level_complete(&self) {
        // Semi-transparent overlay
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.7),
        );

        draw_text(
            "LEVEL COMPLETE!",
            screen_width() / 2.0 - 150.0,
            screen_height() / 2.0 - 100.0,
            48.0,
            GREEN,
        );

        if let Some(result) = &self.level_result {
            // PERBAIKAN: Ganti emoji bintang dengan karakter ASCII
            for i in 0..result.stars {
                draw_text(
                    "*", // Ganti dari "★" ke "*"
                    screen_width() / 2.0 - 50.0 + i as f32 * 60.0,
                    screen_height() / 2.0,
                    50.0,
                    YELLOW,
                );
            }

            draw_text(
                &format!("Score: {}", result.score),
                screen_width() / 2.0 - 70.0,
                screen_height() / 2.0 + 70.0,
                24.0,
                WHITE,
            );

            draw_text(
                "Press SPACE to continue",
                screen_width() / 2.0 - 130.0,
                screen_height() / 2.0 + 120.0,
                20.0,
                LIGHTGRAY,
            );
        }
    }

    fn draw_level_failed(&self) {
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.7),
        );

        draw_text(
            "LEVEL FAILED",
            screen_width() / 2.0 - 130.0,
            screen_height() / 2.0 - 50.0,
            48.0,
            RED,
        );

        draw_text(
            "Press R to retry | ESC for menu",
            screen_width() / 2.0 - 180.0,
            screen_height() / 2.0 + 50.0,
            24.0,
            WHITE,
        );
    }

    pub fn start_level(&mut self, level: crate::level::Level) {
        self.level_mode = true;
        self.level_session = Some(crate::level::LevelSession::new(level));
        self.level_complete = false;
        self.level_failed = false;
        self.show_level_results = false;
        self.score = 0;
        self.combo = 0;

        self.initialize_grid();
    }

    fn get_border_color(&self, x: usize, y: usize) -> Color {
        if let Some(selected) = self.selected {
            if selected == (x, y) {
                return CYAN;
            }
        }

        if self.error_positions.contains(&(x, y)) && self.swap_error_timer > 0.0 {
            return RED;
        }

        Color::new(0.3, 0.3, 0.3, 1.0)
    }

    pub fn handle_click(&mut self, mouse_x: f32, mouse_y: f32, audio: &AudioManager) -> bool {
        if self.is_animating {
            return false;
        }

        let grid_x = ((mouse_x - BOARD_OFFSET_X) / CELL_SIZE) as usize;
        let grid_y = ((mouse_y - BOARD_OFFSET_Y) / CELL_SIZE) as usize;

        if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
            if let Some(selected) = self.selected {
                let dx = (grid_x as i32 - selected.0 as i32).abs();
                let dy = (grid_y as i32 - selected.1 as i32).abs();

                if (dx == 1 && dy == 0) || (dx == 0 && dy == 1) {
                    let result = self.swap_gems(selected, (grid_x, grid_y), audio);
                    self.selected = None;
                    return result;
                } else {
                    self.selected = Some((grid_x, grid_y));
                    audio.play_sound("select");
                }
            } else {
                self.selected = Some((grid_x, grid_y));
                audio.play_sound("select");
            }
        }
        false
    }
}
