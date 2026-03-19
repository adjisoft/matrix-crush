use macroquad::prelude::*;

use super::*;
use crate::game::entities::gem::*;

impl Board {
    pub fn draw(&self, language: &str) {
        let shake_offset = self.screen_shake.get_offset();

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let rect = Rect::new(
                    x as f32 * cell_size() + board_offset_x() + shake_offset.x,
                    y as f32 * cell_size() + board_offset_y() + shake_offset.y,
                    cell_size(),
                    cell_size(),
                );

                let bg_intensity = if (x + y) % 2 == 0 { 0.15 } else { 0.2 };
                let bg_color = Color::new(0.0, bg_intensity, 0.0, 1.0);
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg_color);

                let border_color = self.get_border_color(x, y);
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, border_color);
            }
        }

        for effect in &self.sweep_effects {
            effect.draw(shake_offset);
        }

        for effect in &self.bomb_effects {
            effect.draw(shake_offset);
        }

        for gem in &self.falling_gems {
            gem.draw(shake_offset);
        }

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
                            x as f32 * cell_size() + board_offset_x() + shake_offset.x,
                            y as f32 * cell_size() + board_offset_y() + shake_offset.y,
                            cell_size(),
                            cell_size(),
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

                        let gem_char = match gem {
                            '💎' => 'O',
                            '💠' => '#',
                            '💚' => '@',
                            '💛' => '$',
                            '💜' => '%',
                            '💣' => 'X',
                            '🌀' => '+',
                            _ => gem,
                        };

                        draw_text_ex(
                            &gem_char.to_string(),
                            rect.x + cell_size() / 2.0 - 8.0,
                            rect.y + cell_size() / 2.0 + 8.0,
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

        if self.level_mode {
            self.draw_level_ui(language, shake_offset);
        } else {
            self.draw_classic_ui(language, shake_offset);
        }

        if self.level_complete {
            self.draw_level_complete();
        } else if self.level_failed {
            self.draw_level_failed();
        }
    }

    pub(crate) fn get_border_color(&self, x: usize, y: usize) -> Color {
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
}
