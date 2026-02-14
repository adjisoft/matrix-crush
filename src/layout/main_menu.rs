use crate::audio::AudioManager;
use crate::savegame::SaveData;
use macroquad::prelude::*;

pub enum MenuAction {
    Play,
    Credits,
    Exit,
    Language,
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
}

impl MainMenu {
    pub fn new(language: &str) -> Self {
        let options = if language == "id" {
            vec![
                "MAIN".to_string(),
                "KREDIT".to_string(),
                "KELUAR".to_string(),
                "BAHASA: ID".to_string(),
            ]
        } else {
            vec![
                "PLAY".to_string(),
                "CREDITS".to_string(),
                "EXIT".to_string(),
                "LANGUAGE: EN".to_string(),
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

        if !self.show_credits {
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                self.selected_option =
                    (self.selected_option + self.options.len() - 1) % self.options.len();
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
            return MenuAction::Credits;
        }
        return MenuAction::None;
    }

    if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
        match self.selected_option {
            0 => return MenuAction::Play,
            1 => {
                self.show_credits = true;
                self.credits_scroll = 0.0;
                return MenuAction::Credits;
            }
            2 => return MenuAction::Exit,
            3 => return MenuAction::Language,
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
        } else {
            self.draw_menu(save_data, audio);
        }
    }

    fn draw_matrix_background(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let chars = "01アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン";
        let chars: Vec<char> = chars.chars().collect();

        for i in 0..50 {
            let x = (i as f32 * 30.0 + get_time() as f32 * 20.0) % screen_width;
            let y = (i as f32 * 25.0 + get_time() as f32 * 15.0 * (i as f32 * 0.1)) % screen_height;

            let idx = (get_time() as usize + i) % chars.len();
            let brightness = 0.1 + ((y / screen_height) * 0.3);
            let color = Color::new(0.0, brightness, 0.0, 0.3);

            draw_text(&chars[idx].to_string(), x, y, 16.0, color);
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

        let sound_text = if audio.is_muted { "🔇" } else { "🔊" };
        draw_text(sound_text, screen_width - 50.0, 50.0, 30.0, GREEN);

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
