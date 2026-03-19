use macroquad::prelude::*;

use crate::game::progression::LevelManager;
use crate::systems::audio::AudioManager;
use crate::systems::localization::I18nManager;

pub enum MenuAction {
    PlayClassic,
    PlayStory,
    Credits,
    Exit,
    Language,
    SaveLoad,
    None,
}

pub struct MainMenu {
    pub selected_option: usize,
    pub options: Vec<String>,
    pub title: String,
    pub subtitle: String,
    pub fade_in: f32,
    pub show_credits: bool,
    pub credits_scroll: f32,
    pub language: String,
    pub level_manager: Option<LevelManager>,
    pub show_save_load: bool,
    pub save_load_options: Vec<String>,
    pub selected_save_option: usize,
    pub show_mode_select: bool,
    pub mode_options: Vec<String>,
    pub selected_mode_option: usize,
    pub show_settings: bool,
    pub selected_settings_option: usize,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl MainMenu {
    pub fn new(i18n: &I18nManager) -> Self {
        let options = vec![
            i18n.t("menu_play"),
            i18n.t("menu_settings"),
            i18n.t("menu_save_load"),
            i18n.t("menu_credits"),
            i18n.t("menu_exit"),
        ];

        let save_load_options = vec![
            i18n.t("save_save"),
            i18n.t("save_load"),
            i18n.t("save_reset"),
            i18n.t("menu_back"),
        ];

        let mode_options = vec![
            i18n.t("menu_story"),
            i18n.t("menu_classic"),
            i18n.t("menu_back"),
        ];

        MainMenu {
            selected_option: 0,
            options,
            title: i18n.t("game_title"),
            subtitle: i18n.t("game_subtitle"),
            fade_in: 0.0,
            show_credits: false,
            credits_scroll: 0.0,
            language: i18n.current_lang.clone(),
            level_manager: None,
            show_save_load: false,
            save_load_options,
            selected_save_option: 0,
            show_mode_select: false,
            mode_options,
            selected_mode_option: 0,
            show_settings: false,
            selected_settings_option: 0,
            music_volume: 0.5,
            sfx_volume: 0.7,
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
                self.selected_save_option =
                    (self.selected_save_option + 1) % self.save_load_options.len();
            }
        } else if self.show_mode_select {
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                if self.selected_mode_option > 0 {
                    self.selected_mode_option -= 1;
                } else {
                    self.selected_mode_option = self.mode_options.len() - 1;
                }
            }
            if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                self.selected_mode_option =
                    (self.selected_mode_option + 1) % self.mode_options.len();
            }
        } else if self.show_settings {
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                if self.selected_settings_option > 0 {
                    self.selected_settings_option -= 1;
                } else {
                    self.selected_settings_option = 3;
                }
            }
            if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                self.selected_settings_option = (self.selected_settings_option + 1) % 4;
            }

            if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                self.adjust_setting(-0.05);
            }
            if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                self.adjust_setting(0.05);
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

        if self.show_mode_select {
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                match self.selected_mode_option {
                    0 => return MenuAction::PlayStory,
                    1 => return MenuAction::PlayClassic,
                    2 => {
                        self.show_mode_select = false;
                        return MenuAction::None;
                    }
                    _ => return MenuAction::None,
                }
            }
            if is_key_pressed(KeyCode::Escape) {
                self.show_mode_select = false;
            }
            return MenuAction::None;
        }

        if self.show_settings {
            if is_key_pressed(KeyCode::Escape) {
                self.show_settings = false;
                return MenuAction::None;
            }

            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                match self.selected_settings_option {
                    0 => return MenuAction::Language,
                    3 => {
                        self.show_settings = false;
                        return MenuAction::None;
                    }
                    _ => {}
                }
            }

            return MenuAction::None;
        }

        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            match self.selected_option {
                0 => {
                    self.show_mode_select = true;
                    self.selected_mode_option = 0;
                    return MenuAction::None;
                }
                1 => {
                    self.show_settings = true;
                    self.selected_settings_option = 0;
                    return MenuAction::None;
                }
                2 => {
                    self.show_save_load = true;
                    self.selected_save_option = 0;
                    return MenuAction::None;
                }
                3 => {
                    self.show_credits = true;
                    self.credits_scroll = 0.0;
                    return MenuAction::Credits;
                }
                4 => return MenuAction::Exit,
                _ => return MenuAction::None,
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            return MenuAction::Exit;
        }

        MenuAction::None
    }

    fn adjust_setting(&mut self, delta: f32) {
        match self.selected_settings_option {
            1 => {
                self.music_volume = (self.music_volume + delta).clamp(0.0, 1.0);
            }
            2 => {
                self.sfx_volume = (self.sfx_volume + delta).clamp(0.0, 1.0);
            }
            _ => {}
        }
    }
}
