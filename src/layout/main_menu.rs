use crate::audio::AudioManager;
use crate::savegame::SaveData;
use macroquad::prelude::*;

pub enum MenuAction {
    Play,
    Credits,
    Exit,
    Language,
    LevelSelect,
    SaveLoad,
    None,
}

pub struct MainMenu {
    pub selected_option: usize,
    pub options: Vec<String>,
    pub title: String,
    pub fade_in: f32,
    pub show_credits: bool,
    pub credits_scroll: f32,
    pub language: String,
    pub level_manager: Option<crate::level::LevelManager>,
    pub show_save_load: bool,
    pub save_load_options: Vec<String>,
    pub selected_save_option: usize,
}

impl MainMenu {
    pub fn new(language: &str) -> Self {
        let options = if language == "id" {
            vec![
                "MAIN".to_string(),
                "PILIH LEVEL".to_string(),
                "SIMPAN/MUAT".to_string(),
                "KREDIT".to_string(),
                "KELUAR".to_string(),
                "BAHASA: ID".to_string(),
            ]
        } else {
            vec![
                "PLAY".to_string(),
                "LEVEL SELECT".to_string(),
                "SAVE/LOAD".to_string(),
                "CREDITS".to_string(),
                "EXIT".to_string(),
                "LANGUAGE: EN".to_string(),
            ]
        };

        let save_load_options = if language == "id" {
            vec![
                "SIMPAN PROGRESS".to_string(),
                "MUAT PROGRESS".to_string(),
                "HAPUS PROGRESS".to_string(),
                "KEMBALI".to_string(),
            ]
        } else {
            vec![
                "SAVE PROGRESS".to_string(),
                "LOAD PROGRESS".to_string(),
                "RESET PROGRESS".to_string(),
                "BACK".to_string(),
            ]
        };

        MainMenu {
            selected_option: 0,
            options,
            title: "MATRIX CRUSH".to_string(),
            fade_in: 0.0,
            show_credits: false,
            credits_scroll: 0.0,
            language: language.to_string(),
            level_manager: None,
            show_save_load: false,
            save_load_options,
            selected_save_option: 0,
        }
    }

    pub fn update(&mut self, dt: f32, _audio: &AudioManager) {
        if self.fade_in < 1.0 {
            self.fade_in += dt * 2.0;
            if self.fade_in > 1.0 {
                self.fade_in = 1.0;
            }
        }

        if self.show_credits {
            self.credits_scroll += dt * 30.0;
        }

        if self.show_save_load {
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                if self.selected_save_option > 0 {
                    self.selected_save_option -= 1;
                } else {
                    self.selected_save_option = self.save_load_options.len() - 1;
                }
            }
            if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                self.selected_save_option = (self.selected_save_option + 1) % self.save_load_options.len();
            }
        } else if !self.show_credits {
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                if self.selected_option > 0 {
                    self.selected_option -= 1;
                } else {
                    self.selected_option = self.options.len() - 1;
                }
            }
            if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                self.selected_option = (self.selected_option + 1) % self.options.len();
            }
        }
    }

    pub fn handle_input(&mut self) -> MenuAction {
        if self.show_credits {
            if is_key_pressed(KeyCode::Escape) {
                self.show_credits = false;
            }
            return MenuAction::None;
        }

        if self.show_save_load {
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                return MenuAction::SaveLoad;
            }
            if is_key_pressed(KeyCode::Escape) {
                self.show_save_load = false;
            }
            return MenuAction::None;
        }

        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            match self.selected_option {
                0 => return MenuAction::Play,
                1 => return MenuAction::LevelSelect,
                2 => {
                    self.show_save_load = true;
                    self.selected_save_option = 0;
                    return MenuAction::SaveLoad;
                }
                3 => {
                    self.show_credits = true;
                    self.credits_scroll = 0.0;
                    return MenuAction::Credits;
                }
                4 => return MenuAction::Exit,
                5 => return MenuAction::Language,
                _ => return MenuAction::None,
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            return MenuAction::Exit;
        }

        MenuAction::None
    }

    pub fn draw(&self, save_data: &SaveData, audio: &AudioManager) {
        clear_background(BLACK);

        self.draw_matrix_background();

        if self.show_credits {
            self.draw_credits();
        } else if self.show_save_load {
            self.draw_save_load_menu(save_data);
        } else {
            self.draw_menu(save_data, audio);
        }
    }

    // PERBAIKAN: Matrix background dengan safe indexing
    fn draw_matrix_background(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let chars = ['0', '1']; // Gunakan array langsung, bukan string
        let chars_len = chars.len();

        for i in 0..150 {
            let time = get_time() as f32;

            let speed_factor = 0.5 + (i as f32 * 0.1).sin() * 0.3;
            let x = (i as f32 * 25.0 + time * 30.0 * (i as f32 * 0.05).sin()) % screen_width;
            let y = (i as f32 * 20.0 + time * 40.0 * speed_factor) % (screen_height * 2.0)
                - screen_height;

            // PERBAIKAN: Safe index calculation
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
            let start_y = (get_time() as f32 * 20.0 * (col as f32 * 0.3).sin()) % screen_height;

            for offset in 0..5 {
                let y = start_y + offset as f32 * 25.0;
                if y < screen_height {
                    // PERBAIKAN: Safe index calculation
                    let raw_idx = (col + offset) % chars_len;
                    let display_char = chars[raw_idx];
                    let brightness = 0.5 - offset as f32 * 0.1;
                    let color = Color::new(0.0, brightness, 0.0, 0.3);

                    draw_text(&display_char.to_string(), x, y, 18.0, color);
                }
            }
        }
    }

    fn draw_menu(&self, save_data: &SaveData, audio: &AudioManager) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let alpha = self.fade_in;

        let title_font_size = 64.0;
        let title_width = measure_text(&self.title, None, title_font_size as u16, 1.0).width;
        let title_x = screen_width / 2.0 - title_width / 2.0;
        let title_y = screen_height / 4.0;

        if let Some(level_manager) = &self.level_manager {
            let total_stars = level_manager.get_total_stars();
            let stars_text = format!("Total Stars: {}", total_stars);
            let stars_width = measure_text(&stars_text, None, 20, 1.0).width;
            draw_text(
                &stars_text,
                screen_width / 2.0 - stars_width / 2.0,
                title_y + 100.0,
                20.0,
                YELLOW,
            );
        }

        for i in 0..8 {
            let glow_alpha = (alpha * 0.2 / (i + 1) as f32) * (get_time() as f32 * 3.0).sin().abs();
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

        let high_score_text = if self.language == "id" {
            format!("Skor Tertinggi: {}", save_data.high_score)
        } else {
            format!("High Score: {}", save_data.high_score)
        };
        let score_width = measure_text(&high_score_text, None, 24, 1.0).width;
        draw_text(
            &high_score_text,
            screen_width / 2.0 - score_width / 2.0,
            title_y + 50.0,
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

    fn draw_save_load_menu(&self, save_data: &SaveData) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let alpha = self.fade_in;

        // Title
        let title = if self.language == "id" {
            "SIMPAN / MUAT"
        } else {
            "SAVE / LOAD"
        };
        let title_width = measure_text(title, None, 48, 1.0).width;
        draw_text(
            title,
            screen_width / 2.0 - title_width / 2.0,
            screen_height / 4.0,
            48.0,
            GREEN,
        );

        // Info save data
        let save_info = format!(
            "{}: {} | {}: {} | {}: {}",
            if self.language == "id" {
                "Skor Tertinggi"
            } else {
                "High Score"
            },
            save_data.high_score,
            if self.language == "id" {
                "Level"
            } else {
                "Level"
            },
            save_data.max_unlocked_level,
            if self.language == "id" {
                "Bintang"
            } else {
                "Stars"
            },
            save_data.level_stars.values().sum::<u32>()
        );
        let info_width = measure_text(&save_info, None, 20, 1.0).width;
        draw_text(
            &save_info,
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

        let hint = if self.language == "id" {
            "ESC: Kembali"
        } else {
            "ESC: Back"
        };
        let hint_width = measure_text(hint, None, 18, 1.0).width;
        draw_text(
            hint,
            screen_width / 2.0 - hint_width / 2.0,
            screen_height - 50.0,
            18.0,
            DARKGREEN,
        );
    }

    fn draw_credits(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let credits = if self.language == "id" {
            vec![
                "=== KREDIT ===",
                "",
                "Pengembang: Rizky Adjie Raya Saputra",
                "Desain: Rizky Adjie Raya Saputra dan EI AI",
                "Musik: -",
                "Sound FX: JSFXR",
                "",
                "Terima kasih kepada:",
                "- Komunitas Rust",
                "- Macroquad Framework",
                "- Para Player",
                "",
                "",
                "Tekan ESC untuk kembali",
            ]
        } else {
            vec![
                "=== CREDITS ===",
                "",
                "Developer: Rizky Adjie Raya Saputra",
                "Design: Rizky Adjie Raya Saputra & EI AI",
                "Music: -",
                "Sound FX: JSFXR",
                "",
                "Special Thanks to:",
                "- Rust Community",
                "- Macroquad Framework",
                "- All Players",
                "",
                "",
                "Press ESC to return",
            ]
        };

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