use super::*;
use macroquad::prelude::rand;

use crate::game::entities::gem::*;
use crate::systems::audio::AudioManager;
use crate::systems::effects::{BombEffect, Particle, SweepEffect};

impl Board {
    pub(crate) fn create_match_particles(&mut self, matches: &[(usize, usize)], is_combo: bool) {
        for &(x, y) in matches {
            let gem = self.grid[y][x];
            let color = get_gem_color(gem);

            let particle_count = if is_combo { 8 } else { 5 };

            for _ in 0..particle_count {
                let screen_x = x as f32 * cell_size() + board_offset_x() + cell_size() / 2.0;
                let screen_y = y as f32 * cell_size() + board_offset_y() + cell_size() / 2.0;

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

    pub(crate) fn trigger_bomb_visual(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut affected = Vec::new();
        let bomb_color = get_gem_color(BOMB_GEM);

        let screen_x = x as f32 * cell_size() + board_offset_x() + cell_size() / 2.0;
        let screen_y = y as f32 * cell_size() + board_offset_y() + cell_size() / 2.0;
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
                        let screen_x = nx as f32 * cell_size() + board_offset_x() + cell_size() / 2.0;
                        let screen_y = ny as f32 * cell_size() + board_offset_y() + cell_size() / 2.0;

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

    pub(crate) fn trigger_sweep_visual(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut affected = Vec::new();
        let sweep_color = get_gem_color(SWEEP_GEM);

        let direction = rand::gen_range(0.0, std::f32::consts::PI * 2.0);

        let screen_x = x as f32 * cell_size() + board_offset_x() + cell_size() / 2.0;
        let screen_y = y as f32 * cell_size() + board_offset_y() + cell_size() / 2.0;
        let travel = (GRID_WIDTH.max(GRID_HEIGHT) as f32) * cell_size() * 1.4;
        self.sweep_effects.push(SweepEffect::new(
            screen_x,
            screen_y,
            direction,
            sweep_color,
            travel,
        ));

        for i in 0..GRID_WIDTH {
            if self.grid[y][i] != ' ' && i != x {
                affected.push((i, y));

                let gem = self.grid[y][i];
                let color = get_gem_color(gem);
                let screen_x = i as f32 * cell_size() + board_offset_x() + cell_size() / 2.0;
                let screen_y = y as f32 * cell_size() + board_offset_y() + cell_size() / 2.0;

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
                let screen_x = x as f32 * cell_size() + board_offset_x() + cell_size() / 2.0;
                let screen_y = i as f32 * cell_size() + board_offset_y() + cell_size() / 2.0;

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

    pub(crate) fn trigger_bomb_with_audio(
        &mut self,
        x: usize,
        y: usize,
        audio: &AudioManager,
    ) -> Vec<(usize, usize)> {
        let affected = self.trigger_bomb_visual(x, y);
        audio.play_sound("x_bomb");
        affected
    }

    pub(crate) fn trigger_sweep_with_audio(
        &mut self,
        x: usize,
        y: usize,
        audio: &AudioManager,
    ) -> Vec<(usize, usize)> {
        let affected = self.trigger_sweep_visual(x, y);
        audio.play_sound("v_sweep");
        affected
    }

    pub(crate) fn trigger_glitch_visual(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut affected = Vec::new();
        let glitch_color = get_gem_color(GLITCH_GEM);

        for _ in 0..8 {
            let rx = rand::gen_range(0, GRID_WIDTH);
            let ry = rand::gen_range(0, GRID_HEIGHT);
            if self.grid[ry][rx] != ' ' {
                affected.push((rx, ry));
                let screen_x = rx as f32 * cell_size() + board_offset_x() + cell_size() / 2.0;
                let screen_y = ry as f32 * cell_size() + board_offset_y() + cell_size() / 2.0;
                for _ in 0..5 {
                    self.particles.push(Particle::new(screen_x, screen_y, self.grid[ry][rx], glitch_color));
                }
            }
        }
        self.screen_shake.trigger(10.0);
        self.earned_entropy += 2;
        affected
    }

    pub(crate) fn trigger_antimatter_visual(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut affected = Vec::new();
        let color = get_gem_color(ANTIMATTER_GEM);

        for dy in -2..=2 {
            for dx in -2..=2 {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < GRID_WIDTH as i32 && ny >= 0 && ny < GRID_HEIGHT as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if self.grid[ny][nx] != ' ' {
                        affected.push((nx, ny));
                        let screen_x = nx as f32 * cell_size() + board_offset_x() + cell_size() / 2.0;
                        let screen_y = ny as f32 * cell_size() + board_offset_y() + cell_size() / 2.0;
                        for _ in 0..5 {
                            self.particles.push(Particle::new_bomb_effect(screen_x, screen_y, self.grid[ny][nx], color));
                        }
                    }
                }
            }
        }
        self.screen_shake.trigger(25.0);
        affected
    }

    pub(crate) fn trigger_void_visual(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut affected = Vec::new();

        for dy in -(GRID_HEIGHT as i32)..GRID_HEIGHT as i32 {
            for dx in -(GRID_WIDTH as i32)..GRID_WIDTH as i32 {
                if dx == 0 || dy == 0 || dx.abs() == dy.abs() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < GRID_WIDTH as i32 && ny >= 0 && ny < GRID_HEIGHT as i32 {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if self.grid[ny][nx] != ' ' {
                            affected.push((nx, ny));
                        }
                    }
                }
            }
        }
        self.screen_shake.trigger(20.0);
        affected
    }

    pub(crate) fn trigger_glitch_with_audio(&mut self, x: usize, y: usize, audio: &AudioManager) -> Vec<(usize, usize)> {
        let affected = self.trigger_glitch_visual(x, y);
        audio.play_sound("combo");
        affected
    }

    pub(crate) fn trigger_antimatter_with_audio(&mut self, x: usize, y: usize, audio: &AudioManager) -> Vec<(usize, usize)> {
        let affected = self.trigger_antimatter_visual(x, y);
        audio.play_sound("explosion");
        affected
    }

    pub(crate) fn trigger_void_with_audio(&mut self, x: usize, y: usize, audio: &AudioManager) -> Vec<(usize, usize)> {
        let affected = self.trigger_void_visual(x, y);
        audio.play_sound("explosion");
        affected
    }

    pub(crate) fn create_error_effect(&mut self, pos1: (usize, usize), pos2: (usize, usize)) {
        self.error_positions = vec![pos1, pos2];
        self.swap_error_timer = 0.5;
        self.screen_shake.trigger(8.0);
        self.combo = 0;

        for &pos in &[pos1, pos2] {
            let gem = self.grid[pos.1][pos.0];
            let color = get_gem_color(gem);

            for _ in 0..4 {
                let screen_x = pos.0 as f32 * cell_size() + board_offset_x() + cell_size() / 2.0;
                let screen_y = pos.1 as f32 * cell_size() + board_offset_y() + cell_size() / 2.0;

                self.particles
                    .push(Particle::new_error(screen_x, screen_y, gem, color));
            }
        }
    }
}
