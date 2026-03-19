use super::*;
use std::collections::VecDeque;

use macroquad::prelude::rand;

use crate::game::entities::gem::*;
use crate::systems::audio::AudioManager;
use crate::systems::effects::FallingGem;

impl Board {
    pub(crate) fn apply_gravity_with_animation(&mut self) {
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

                    let start_y = board_offset_y() - 80.0 - (rand::gen_range(0.0, 50.0));
                    let color = get_gem_color(new_gem);

                    falling_gems_temp.push(FallingGem::new(
                        new_gem,
                        color,
                        x,
                        y,
                        start_y,
                        cell_size(),
                        board_offset_x(),
                        board_offset_y(),
                    ));
                }
            }
        }

        self.grid = new_grid;
        self.falling_gems.extend(falling_gems_temp);
        self.is_animating = !self.falling_gems.is_empty();
    }

    pub(crate) fn update_falling_gems(&mut self, dt: f32) {
        for gem in &mut self.falling_gems {
            gem.update(dt);
        }

        self.falling_gems.retain(|gem| !gem.is_finished());
        self.is_animating = !self.falling_gems.is_empty();
    }

    pub(crate) fn check_for_matches(&self) -> bool {
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

    pub(crate) fn check_horizontal_match(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
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

    pub(crate) fn check_vertical_match(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
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

    pub(crate) fn clear_matches(&mut self) -> u32 {
        if self.is_animating {
            return 0;
        }

        let mut matches_to_clear = Vec::new();

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if is_special_gem(self.grid[y][x]) {
                    // Do not auto-trigger special gems on grid validation
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

            self.earned_data_core += match_count;
            if self.combo >= 5 {
                self.earned_entropy += 1;
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

    pub(crate) fn process_special_gems(&mut self, audio: &AudioManager) -> u32 {
        if self.special_gems_to_process.is_empty() {
            return 0;
        }

        let mut pending: VecDeque<(usize, usize)> =
            self.special_gems_to_process.drain(..).collect();
        let mut queued = [[false; GRID_WIDTH]; GRID_HEIGHT];
        let mut processed = [[false; GRID_WIDTH]; GRID_HEIGHT];
        let mut affected_all: Vec<(usize, usize)> = Vec::new();

        for &(x, y) in &pending {
            queued[y][x] = true;
        }

        while let Some((x, y)) = pending.pop_front() {
            if processed[y][x] {
                continue;
            }
            processed[y][x] = true;

            let gem = self.grid[y][x];
            if gem == ' ' {
                continue;
            }

            let mut affected = if gem == BOMB_GEM {
                self.trigger_bomb_with_audio(x, y, audio)
            } else if gem == SWEEP_GEM {
                self.trigger_sweep_with_audio(x, y, audio)
            } else if gem == GLITCH_GEM {
                self.trigger_glitch_with_audio(x, y, audio)
            } else if gem == ANTIMATTER_GEM {
                self.trigger_antimatter_with_audio(x, y, audio)
            } else if gem == VOID_GEM {
                self.trigger_void_with_audio(x, y, audio)
            } else {
                Vec::new()
            };

            affected.push((x, y));

            for (ax, ay) in affected {
                if self.grid[ay][ax] == ' ' {
                    continue;
                }

                affected_all.push((ax, ay));

                if is_special_gem(self.grid[ay][ax])
                    && !processed[ay][ax]
                    && !queued[ay][ax]
                    && (ax != x || ay != y)
                {
                    pending.push_back((ax, ay));
                    queued[ay][ax] = true;
                }
            }
        }

        affected_all.sort();
        affected_all.dedup();

        let cleared_count = affected_all.len() as u32;
        if cleared_count > 0 {
            let base_score = cleared_count * 10;
            let combo_multiplier = 1.0 + (self.combo as f32 * 0.2).min(2.0);
            let total_score = (base_score as f32 * combo_multiplier) as u32;

            self.score += total_score;
            if let Some(session) = &mut self.level_session {
                session.add_score(total_score);
            }

            self.earned_data_core += cleared_count;
            if self.combo >= 5 {
                self.earned_entropy += 1;
            }
        }

        for (x, y) in &affected_all {
            if self.grid[*y][*x] != ' ' {
                self.grid[*y][*x] = ' ';
            }
        }

        affected_all.len() as u32
    }

    fn trigger_dialog(&mut self, kind: DialogKind) {
        let count = dialog_variant_count(kind);
        let variant = if count > 0 {
            rand::gen_range(0, count as i32) as usize
        } else {
            0
        };

        self.dialog_kind = Some(kind);
        self.dialog_variant = variant;
        self.dialog_timer = 2.4;
        self.dialog_glitch_intensity = 1.0;
    }

    fn update_dialog(&mut self, dt: f32) {
        if self.dialog_timer > 0.0 {
            self.dialog_timer -= dt;
            if self.dialog_timer <= 0.0 {
                self.dialog_timer = 0.0;
                self.dialog_kind = None;
            }
        }

        self.dialog_glitch_intensity = (self.dialog_glitch_intensity - dt * 3.0).max(0.0);
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
                self.last_combo_shown = 0;
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

            self.trigger_dialog(DialogKind::Powerup);
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

                if self.combo >= 3 && self.combo > self.last_combo_shown {
                    self.last_combo_shown = self.combo;
                    self.trigger_dialog(DialogKind::Combo);
                }

                if self.score >= self.next_score_milestone {
                    while self.score >= self.next_score_milestone {
                        self.next_score_milestone += 1000;
                    }
                    self.trigger_dialog(DialogKind::Score);
                }

                self.apply_gravity_with_animation();
            }
        }

        self.update_dialog(dt);
    }
}
