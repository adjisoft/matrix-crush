use macroquad::prelude::*;

use super::*;

impl Board {
    pub(crate) fn draw_level_ui(&self, language: &str, _shake_offset: Vec2) {
        if let Some(session) = &self.level_session {
            let is_portrait = screen_width() <= screen_height();
            let panel_x = if is_portrait {
                board_offset_x()
            } else {
                board_offset_x() + GRID_WIDTH as f32 * cell_size() + 30.0
            };
            let panel_y = if is_portrait {
                board_offset_y() + GRID_HEIGHT as f32 * cell_size() + 20.0
            } else {
                board_offset_y()
            };

            draw_rectangle(
                panel_x - 10.0,
                panel_y - 10.0,
                250.0,
                400.0,
                Color::new(0.0, 0.1, 0.0, 0.9),
            );

            draw_text(
                &format!("Level {}", session.level.id),
                panel_x,
                panel_y + 30.0,
                28.0,
                GREEN,
            );

            let moves_text = if language == "id" { "GERAKAN" } else { "MOVES" };
            draw_text(moves_text, panel_x, panel_y + 70.0, 24.0, YELLOW);

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
                32.0,
                moves_color,
            );

            draw_text("SCORE", panel_x, panel_y + 140.0, 24.0, LIGHTGRAY);
            draw_text(
                &format!("{}", self.score),
                panel_x,
                panel_y + 170.0,
                28.0,
                WHITE,
            );

            draw_text("OBJECTIVES:", panel_x, panel_y + 210.0, 22.0, CYAN);

            let result = session.check_completion();
            for (i, ((current, target), obj)) in result
                .progress
                .iter()
                .zip(&session.level.objectives)
                .enumerate()
            {
                let y = panel_y + 240.0 + i as f32 * 25.0;

                let obj_text = match obj {
                    crate::game::progression::LevelObjective::Score(_) => {
                        format!("Score: {}/{}", current, target)
                    }
                    crate::game::progression::LevelObjective::CollectGems { gem_type, .. } => {
                        let gem_display = match gem_type {
                            '💎' => "O",
                            '💠' => "#",
                            '💚' => "@",
                            '💛' => "$",
                            '💜' => "%",
                            _ => "?",
                        };
                        format!(
                            "{} {}: {}/{}",
                            if language == "id" { "Kumpulkan" } else { "Collect" },
                            gem_display,
                            current,
                            target
                        )
                    }
                    crate::game::progression::LevelObjective::ClearGems(_) => {
                        format!("Clear: {}/{}", current, target)
                    }
                    crate::game::progression::LevelObjective::Combo(_) => {
                        format!("Combo: {}/{}", current, target)
                    }
                    crate::game::progression::LevelObjective::SpecialGems(_) => {
                        format!("Special: {}/{}", current, target)
                    }
                };

                let color = if current >= target { GREEN } else { WHITE };
                draw_text(&obj_text, panel_x + 10.0, y, 18.0, color);
            }

            let progress = session.get_progress_percentage();
            let bar_width = 220.0;
            let bar_x = panel_x;
            let bar_y = panel_y + 350.0;

            draw_rectangle(bar_x, bar_y, bar_width, 15.0, DARKGRAY);
            draw_rectangle(bar_x, bar_y, bar_width * progress, 15.0, GREEN);

            draw_text(
                &format!("{:.0}%", progress * 100.0),
                bar_x + bar_width / 2.0 - 20.0,
                bar_y - 6.0,
                18.0,
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
        let is_portrait = screen_width() <= screen_height();
        let panel_x = if is_portrait {
            board_offset_x()
        } else {
            board_offset_x() + GRID_WIDTH as f32 * cell_size() + 30.0
        };
        let panel_y = if is_portrait {
            board_offset_y() + GRID_HEIGHT as f32 * cell_size() + 20.0
        } else {
            board_offset_y()
        };

        draw_rectangle(
            panel_x - 10.0,
            panel_y - 10.0,
            220.0,
            350.0,
            Color::new(0.0, 0.1, 0.0, 0.8),
        );

        let score_text = if language == "id" { "SKOR" } else { "SCORE" };
        draw_text(score_text, panel_x, panel_y + 30.0, 28.0, GREEN);
        draw_text(&format!("{}", self.score), panel_x, panel_y + 60.0, 36.0, WHITE);

        if self.combo > 0 {
            let combo_text = if language == "id" { "KOMBO" } else { "COMBO" };
            draw_text(combo_text, panel_x, panel_y + 110.0, 28.0, YELLOW);

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
                    font_size: 46,
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
                24.0,
                Color::new(0.5, 1.0, 0.5, 0.8),
            );
        }

        draw_text("SPECIAL:", panel_x, panel_y + 220.0, 22.0, CYAN);
        draw_text(
            "X Bomb: 3x3",
            panel_x,
            panel_y + 245.0,
            18.0,
            Color::new(1.0, 0.5, 0.5, 1.0),
        );
        draw_text(
            "+ Sweep: Row+Col",
            panel_x,
            panel_y + 265.0,
            18.0,
            Color::new(0.5, 1.0, 1.0, 1.0),
        );

        if self.max_combo > 0 {
            let max_text = if language == "id" {
                format!("Kombo Maks: {}", self.max_combo)
            } else {
                format!("Max Combo: {}", self.max_combo)
            };
            draw_text(&max_text, panel_x, panel_y + 300.0, 18.0, GRAY);
        }

        if self.is_animating {
            let anim_text = if language == "id" { "> BAGUS <" } else { "> GOOD <" };
            let text_width = measure_text(anim_text, None, 24, 1.0).width;
            draw_text(
                anim_text,
                board_offset_x() + (GRID_WIDTH as f32 * cell_size()) / 2.0 - text_width / 2.0,
                board_offset_y() - 30.0,
                24.0,
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

        draw_text("ANONYMOUS", title_x, title_y, 20.0, CYAN);

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
                    font_size: 30,
                    color: ghost_color1,
                    ..Default::default()
                },
            );
            draw_text_ex(
                message,
                text_x - ghost_offset.x,
                text_y - ghost_offset.y,
                TextParams {
                    font_size: 20,
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
                font_size: 30,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
            message,
            text_x,
            text_y,
            TextParams {
                font_size: 20,
                color: LIGHTGRAY,
                ..Default::default()
            },
        );
    }

    pub(crate) fn draw_level_complete(&self) {
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
            for i in 0..result.stars {
                draw_text(
                    "*",
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
                28.0,
                WHITE,
            );

            draw_text(
                "Press SPACE to continue",
                screen_width() / 2.0 - 130.0,
                screen_height() / 2.0 + 120.0,
                24.0,
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
            28.0,
            WHITE,
        );
    }
}
