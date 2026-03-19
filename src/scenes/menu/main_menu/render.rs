use macroquad::prelude::*;

use crate::systems::audio::AudioManager;
use crate::systems::localization::I18nManager;
use crate::systems::save::SaveData;

use super::MainMenu;

impl MainMenu {
    pub fn draw(&self, save_data: &SaveData, _audio: &AudioManager, i18n: &I18nManager) {
        clear_background(BLACK);

        self.draw_matrix_background();

        if self.show_credits {
            self.draw_credits(i18n);
        } else if self.show_save_load {
            self.draw_save_load_menu(save_data, i18n);
        } else if self.show_mode_select {
            self.draw_mode_select_menu(i18n);
        } else if self.show_settings {
            self.draw_settings_menu(i18n);
        } else {
            self.draw_menu(save_data, i18n);
        }
    }

    fn draw_matrix_background(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let chars = ['0', '1'];
        let chars_len = chars.len();

        for i in 0..150 {
            let time = get_time() as f32;

            let speed_factor = 0.5 + (i as f32 * 0.1).sin() * 0.3;
            let x = (i as f32 * 25.0 + time * 30.0 * (i as f32 * 0.05).sin()) % screen_width;
            let y = (i as f32 * 20.0 + time * 40.0 * speed_factor) % (screen_height * 2.0)
                - screen_height;

            let raw_idx = (x as i32 + y as i32) % chars_len as i32;
            let char_idx = if raw_idx < 0 {
                (raw_idx + chars_len as i32) as usize
            } else {
                raw_idx as usize
            } % chars_len;

            let display_char = chars[char_idx];

            let brightness_factor = 0.3 + (y / screen_height).max(0.0).min(1.0) * 0.5;
            let opacity = 0.4 + ((y + screen_height) / screen_height).max(0.0).min(1.0) * 0.3;

            let color = if char_idx == 0 {
                Color::new(
                    0.0,
                    brightness_factor * 0.8,
                    brightness_factor * 0.3,
                    opacity,
                )
            } else {
                Color::new(0.0, brightness_factor, 0.0, opacity * 1.2)
            };

            let font_size = if (i % 3) == 0 { 20.0 } else { 16.0 };

            draw_text(&display_char.to_string(), x, y, font_size, color);
        }

        for col in 0..20 {
            let x = col as f32 * 50.0 + (get_time() as f32 * 10.0) % 50.0;
            let start_y =
                (get_time() as f32 * 20.0 * (col as f32 * 0.3).sin()) % screen_height;

            for offset in 0..5 {
                let y = start_y + offset as f32 * 25.0;
                if y < screen_height {
                    let raw_idx = (col + offset) % chars_len;
                    let display_char = chars[raw_idx];
                    let brightness = 0.5 - offset as f32 * 0.1;
                    let color = Color::new(0.0, brightness, 0.0, 0.3);

                    draw_text(&display_char.to_string(), x, y, 18.0, color);
                }
            }
        }
    }

    fn draw_menu(&self, save_data: &SaveData, _i18n: &I18nManager) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let alpha = self.fade_in;

        let title_font_size = 64.0;
        let title_width = measure_text(&self.title, None, title_font_size as u16, 1.0).width;
        let title_x = screen_width / 2.0 - title_width / 2.0;
        let title_y = screen_height / 4.2;

        if let Some(level_manager) = &self.level_manager {
            let total_stars = level_manager.get_total_stars();
            let stars_text = format!("Total Stars: {}", total_stars);
            let stars_width = measure_text(&stars_text, None, 20, 1.0).width;
            draw_text(
                &stars_text,
                screen_width / 2.0 - stars_width / 2.0,
                title_y + 110.0,
                20.0,
                YELLOW,
            );
        }

        for i in 0..8 {
            let glow_alpha =
                (alpha * 0.2 / (i + 1) as f32) * (get_time() as f32 * 3.0).sin().abs();
            draw_text(
                &self.title,
                title_x + (i as f32 * 1.5).sin() * 2.0,
                title_y + (i as f32 * 2.0).cos() * 2.0,
                title_font_size,
                Color::new(0.0, 0.8, 0.0, glow_alpha),
            );
        }

        draw_text(
            &self.title,
            title_x,
            title_y,
            title_font_size,
            Color::new(0.0, 1.0, 0.0, alpha),
        );

        if !self.subtitle.is_empty() {
            let subtitle_width = measure_text(&self.subtitle, None, 22, 1.0).width;
            draw_text(
                &self.subtitle,
                screen_width / 2.0 - subtitle_width / 2.0,
                title_y + 38.0,
                22.0,
                Color::new(0.0, 0.8, 0.0, alpha * 0.7),
            );
        }

        let high_score_text = if self.language == "id" {
            format!("Skor Tertinggi: {}", save_data.high_score)
        } else {
            format!("High Score: {}", save_data.high_score)
        };
        let score_width = measure_text(&high_score_text, None, 24, 1.0).width;
        draw_text(
            &high_score_text,
            screen_width / 2.0 - score_width / 2.0,
            title_y + 75.0,
            24.0,
            Color::new(0.0, 0.8, 0.0, alpha * 0.8),
        );

        let option_font_size = 36.0;
        let start_y = screen_height / 2.0;
        let spacing = 50.0;

        for (i, option) in self.options.iter().enumerate() {
            let option_width = measure_text(option, None, option_font_size as u16, 1.0).width;
            let x = screen_width / 2.0 - option_width / 2.0;
            let y = start_y + i as f32 * spacing;

            let is_selected = i == self.selected_option;

            if is_selected {
                let pulse = (get_time() as f32 * 3.0).sin() * 0.3 + 0.7;

                draw_rectangle(
                    x - 30.0,
                    y - 25.0,
                    option_width + 60.0,
                    40.0,
                    Color::new(0.0, 0.5, 0.0, pulse * 0.3),
                );

                draw_text(
                    "[",
                    x - 25.0,
                    y,
                    option_font_size,
                    Color::new(0.0, 1.0, 0.0, pulse),
                );
                draw_text(
                    "]",
                    x + option_width + 15.0,
                    y,
                    option_font_size,
                    Color::new(0.0, 1.0, 0.0, pulse),
                );
            }

            let color = if is_selected {
                Color::new(0.0, 1.0, 0.0, alpha)
            } else {
                Color::new(0.0, 0.7, 0.0, alpha * 0.7)
            };

            draw_text(option, x, y, option_font_size, color);
        }

        let stats_text = if self.language == "id" {
            format!(
                "Total Game: {} | Total Match: {}",
                save_data.total_games, save_data.total_matches
            )
        } else {
            format!(
                "Total Games: {} | Total Matches: {}",
                save_data.total_games, save_data.total_matches
            )
        };
        let stats_width = measure_text(&stats_text, None, 16, 1.0).width;
        draw_text(
            &stats_text,
            screen_width / 2.0 - stats_width / 2.0,
            screen_height - 30.0,
            16.0,
            Color::new(0.0, 0.5, 0.0, alpha * 0.5),
        );
    }

    fn draw_mode_select_menu(&self, i18n: &I18nManager) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let title = i18n.t("menu_play");
        let title_width = measure_text(&title, None, 48, 1.0).width;
        draw_text(
            &title,
            screen_width / 2.0 - title_width / 2.0,
            screen_height / 4.0,
            48.0,
            GREEN,
        );

        let option_font_size = 32.0;
        let start_y = screen_height / 2.0;
        let spacing = 55.0;

        for (i, option) in self.mode_options.iter().enumerate() {
            let option_width = measure_text(option, None, option_font_size as u16, 1.0).width;
            let x = screen_width / 2.0 - option_width / 2.0;
            let y = start_y + i as f32 * spacing;

            let is_selected = i == self.selected_mode_option;
            let color = if is_selected {
                Color::new(0.0, 1.0, 0.0, 1.0)
            } else {
                Color::new(0.0, 0.7, 0.0, 0.7)
            };

            if is_selected {
                let pulse = (get_time() as f32 * 3.0).sin() * 0.3 + 0.7;
                draw_rectangle(
                    x - 30.0,
                    y - 25.0,
                    option_width + 60.0,
                    40.0,
                    Color::new(0.0, 0.5, 0.0, pulse * 0.3),
                );
            }

            draw_text(option, x, y, option_font_size, color);
        }
    }

    fn draw_settings_menu(&self, i18n: &I18nManager) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let title = i18n.t("settings_title");
        let title_width = measure_text(&title, None, 48, 1.0).width;
        draw_text(
            &title,
            screen_width / 2.0 - title_width / 2.0,
            screen_height / 4.0,
            48.0,
            GREEN,
        );

        let lang_label = format!("{}: {}", i18n.t("settings_language"), self.language.to_uppercase());
        let music_label = format!("{}: {}%", i18n.t("settings_music"), (self.music_volume * 100.0).round());
        let sfx_label = format!("{}: {}%", i18n.t("settings_sfx"), (self.sfx_volume * 100.0).round());
        let back_label = i18n.t("menu_back");

        let items = vec![lang_label, music_label, sfx_label, back_label];

        let option_font_size = 30.0;
        let start_y = screen_height / 2.0;
        let spacing = 52.0;

        for (i, option) in items.iter().enumerate() {
            let option_width = measure_text(option, None, option_font_size as u16, 1.0).width;
            let x = screen_width / 2.0 - option_width / 2.0;
            let y = start_y + i as f32 * spacing;

            let is_selected = i == self.selected_settings_option;
            let color = if is_selected {
                Color::new(0.0, 1.0, 0.0, 1.0)
            } else {
                Color::new(0.0, 0.7, 0.0, 0.7)
            };

            if is_selected {
                let pulse = (get_time() as f32 * 3.0).sin() * 0.3 + 0.7;
                draw_rectangle(
                    x - 30.0,
                    y - 25.0,
                    option_width + 60.0,
                    40.0,
                    Color::new(0.0, 0.5, 0.0, pulse * 0.3),
                );
            }

            draw_text(option, x, y, option_font_size, color);
        }

        let hint = i18n.t("settings_hint");
        let hint_width = measure_text(&hint, None, 18, 1.0).width;
        draw_text(
            &hint,
            screen_width / 2.0 - hint_width / 2.0,
            screen_height - 50.0,
            18.0,
            DARKGREEN,
        );
    }

    fn draw_save_load_menu(&self, save_data: &SaveData, i18n: &I18nManager) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let alpha = self.fade_in;

        let title = i18n.t("save_title");
        let title_width = measure_text(&title, None, 48, 1.0).width;
        draw_text(
            &title,
            screen_width / 2.0 - title_width / 2.0,
            screen_height / 4.0,
            48.0,
            GREEN,
        );

        let mut info = i18n.t("save_info");
        info = info.replace("{high_score}", &save_data.high_score.to_string());
        info = info.replace("{level}", &save_data.max_unlocked_level.to_string());
        info = info.replace(
            "{stars}",
            &save_data.level_stars.values().sum::<u32>().to_string(),
        );

        let info_width = measure_text(&info, None, 20, 1.0).width;
        draw_text(
            &info,
            screen_width / 2.0 - info_width / 2.0,
            screen_height / 4.0 + 60.0,
            20.0,
            YELLOW,
        );

        let option_font_size = 32.0;
        let start_y = screen_height / 2.0;
        let spacing = 60.0;

        for (i, option) in self.save_load_options.iter().enumerate() {
            let option_width = measure_text(option, None, option_font_size as u16, 1.0).width;
            let x = screen_width / 2.0 - option_width / 2.0;
            let y = start_y + i as f32 * spacing;

            let is_selected = i == self.selected_save_option;

            if is_selected {
                let pulse = (get_time() as f32 * 3.0).sin() * 0.3 + 0.7;

                draw_rectangle(
                    x - 30.0,
                    y - 25.0,
                    option_width + 60.0,
                    40.0,
                    Color::new(0.0, 0.5, 0.0, pulse * 0.3),
                );

                draw_text(
                    ">",
                    x - 40.0,
                    y,
                    option_font_size,
                    Color::new(0.0, 1.0, 0.0, pulse),
                );
                draw_text(
                    "<",
                    x + option_width + 30.0,
                    y,
                    option_font_size,
                    Color::new(0.0, 1.0, 0.0, pulse),
                );
            }

            let color = if is_selected {
                Color::new(0.0, 1.0, 0.0, alpha)
            } else {
                Color::new(0.0, 0.7, 0.0, alpha * 0.7)
            };

            draw_text(option, x, y, option_font_size, color);
        }

        let hint = i18n.t("save_back");
        let hint_width = measure_text(&hint, None, 18, 1.0).width;
        draw_text(
            &hint,
            screen_width / 2.0 - hint_width / 2.0,
            screen_height - 50.0,
            18.0,
            DARKGREEN,
        );
    }

    fn draw_credits(&self, i18n: &I18nManager) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let credits = vec![
            i18n.t("credits_title"),
            String::new(),
            i18n.t("credits_dev"),
            i18n.t("credits_design"),
            i18n.t("credits_music"),
            i18n.t("credits_music_all"),
            i18n.t("credits_music_ingame"),
            i18n.t("credits_music_main"),
            i18n.t("credits_music_neural"),
            i18n.t("credits_music_lab"),
            i18n.t("credits_sfx"),
            String::new(),
            i18n.t("credits_thanks"),
            i18n.t("credits_rust"),
            i18n.t("credits_macroquad"),
            i18n.t("credits_players"),
            String::new(),
            String::new(),
            i18n.t("credits_return"),
        ];

        let start_y = screen_height / 2.0 - 150.0 - self.credits_scroll;

        for (i, line) in credits.iter().enumerate() {
            let y = start_y + i as f32 * 30.0;

            if y > 0.0 && y < screen_height {
                let width = measure_text(line, None, 24, 1.0).width;
                let x = screen_width / 2.0 - width / 2.0;

                let brightness = if line.starts_with("===") { 1.0 } else { 0.8 };
                let color = Color::new(0.0, brightness, 0.0, 1.0);

                draw_text(line, x, y, 24.0, color);
            }
        }
    }
}
