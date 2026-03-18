use super::*;

use crate::matrix_match::gem::*;

impl Board {
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

    pub(crate) fn draw_level_ui(&self, language: &str, _shake_offset: Vec2) {
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

            let dialog_width = 250.0;
            let dialog_height = 120.0;
            let mut dialog_y = panel_y + 420.0;
            if dialog_y + dialog_height > screen_height() - 10.0 {
                dialog_y = screen_height() - dialog_height - 10.0;
            }
            self.draw_anonymous_panel(
                language,
                panel_x - 10.0,
                dialog_y,
                dialog_width,
                dialog_height,
            );
        }
    }

    pub(crate) fn draw_classic_ui(&self, language: &str, _shake_offset: Vec2) {
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

        let dialog_width = 220.0;
        let dialog_height = 110.0;
        let mut dialog_y = panel_y + 360.0;
        if dialog_y + dialog_height > screen_height() - 10.0 {
            dialog_y = screen_height() - dialog_height - 10.0;
        }
        self.draw_anonymous_panel(
            language,
            panel_x - 10.0,
            dialog_y,
            dialog_width,
            dialog_height,
        );
    }

    pub(crate) fn draw_anonymous_panel(
        &self,
        language: &str,
        rect_x: f32,
        rect_y: f32,
        width: f32,
        height: f32,
    ) {
        draw_rectangle(
            rect_x,
            rect_y,
            width,
            height,
            Color::new(0.0, 0.08, 0.0, 0.85),
        );
        draw_rectangle_lines(rect_x, rect_y, width, height, 2.0, Color::new(0.2, 0.6, 0.4, 0.6));

        let padding = 10.0;
        let title_x = rect_x + padding;
        let title_y = rect_y + 22.0;

        draw_text("ANONYMOUS", title_x, title_y, 16.0, CYAN);

        let glitch = self.dialog_glitch_intensity.clamp(0.0, 1.0);
        let jitter = if glitch > 0.0 {
            vec2(
                rand::gen_range(-2.0, 2.0) * glitch * 3.0,
                rand::gen_range(-2.0, 2.0) * glitch * 3.0,
            )
        } else {
            vec2(0.0, 0.0)
        };

        let face = if glitch > 0.0 {
            let idx = ((get_time() * 12.0) as usize) % ANON_FACES.len();
            ANON_FACES[idx]
        } else {
            ANON_FACES[0]
        };

        let message = if let Some(kind) = self.dialog_kind {
            dialog_message(kind, language, self.dialog_variant)
        } else if language == "id" {
            "..."
        } else {
            "..."
        };

        let face_x = rect_x + padding + jitter.x;
        let face_y = rect_y + 55.0 + jitter.y;
        let text_x = rect_x + padding + jitter.x;
        let text_y = rect_y + 85.0 + jitter.y;

        if glitch > 0.0 {
            let ghost_offset = vec2(2.0 * glitch, -1.5 * glitch);
            let ghost_color1 = Color::new(1.0, 0.2, 0.9, 0.5 * glitch);
            let ghost_color2 = Color::new(0.2, 1.0, 0.6, 0.5 * glitch);

            draw_text_ex(
                face,
                face_x + ghost_offset.x,
                face_y + ghost_offset.y,
                TextParams {
                    font_size: 26,
                    color: ghost_color1,
                    ..Default::default()
                },
            );
            draw_text_ex(
                message,
                text_x - ghost_offset.x,
                text_y - ghost_offset.y,
                TextParams {
                    font_size: 16,
                    color: ghost_color2,
                    ..Default::default()
                },
            );
        }

        draw_text_ex(
            face,
            face_x,
            face_y,
            TextParams {
                font_size: 26,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
            message,
            text_x,
            text_y,
            TextParams {
                font_size: 16,
                color: LIGHTGRAY,
                ..Default::default()
            },
        );
    }

    pub(crate) fn draw_level_complete(&self) {
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

    pub(crate) fn draw_level_failed(&self) {
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
