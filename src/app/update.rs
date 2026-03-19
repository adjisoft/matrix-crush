use macroquad::prelude::*;

use crate::game::story::dialogue::{ScenePhase, get_scene};
use crate::scenes::menu::level_select::LevelSelectEvent;
use crate::scenes::menu::main_menu::MenuAction;

use super::{App, AppState};

impl App {
    pub fn update(&mut self, dt: f32) {
        self.fps_counter = 1.0 / dt.max(0.001);

        if is_key_pressed(KeyCode::F1) {
            self.show_fps = !self.show_fps;
        }
        match self.state {
            AppState::Menu => self.update_menu(dt),
            AppState::LevelSelect => self.update_level_select(dt),
            AppState::Playing => self.update_playing(dt),
            AppState::StoryIntro => self.update_story_intro(dt),
            AppState::LevelPlaying => self.update_level_playing(dt),
            AppState::StoryOutro => self.update_story_outro(dt),
            AppState::LevelComplete => self.update_level_complete(),
            AppState::LevelFailed => self.update_level_failed(),
            AppState::Paused => self.update_paused(),
            AppState::ResearchCenter => self.update_research_center(dt),
            AppState::Exiting => {}
        }

        self.sync_music();
    }

    fn update_menu(&mut self, dt: f32) {
        self.menu.update(dt, &self.audio);

        if is_key_pressed(KeyCode::Up)
            || is_key_pressed(KeyCode::Down)
            || is_key_pressed(KeyCode::W)
            || is_key_pressed(KeyCode::S)
        {
            self.audio.play_sound("select");
        }

        match self.menu.handle_input() {
            MenuAction::PlayClassic => {
                self.state = AppState::Playing;
                self.board = crate::scenes::gameplay::Board::new();
                self.save.data.total_games += 1;
                self.save.save();
                self.audio.play_sound("select");
            }
            MenuAction::PlayStory => {
                self.state = AppState::LevelSelect;
                self.audio.play_sound("select");
            }
            MenuAction::SaveLoad => {
                if self.menu.show_save_load {
                    match self.menu.selected_save_option {
                        0 => {
                            if self.save.save() {
                                self.audio.play_sound("select");
                            }
                        }
                        1 => {
                            self.save.load_from_slot(self.save.current_slot());
                            if let Some(lm) = &mut self.menu.level_manager {
                                lm.max_unlocked_level = self.save.data.max_unlocked_level;
                                lm.level_stars = self.save.data.level_stars.clone();
                            }
                            self.menu.music_volume = self.save.data.music_volume;
                            self.menu.sfx_volume = self.save.data.sfx_volume;
                            self.audio.set_music_volume(self.menu.music_volume);
                            self.audio.set_sfx_volume(self.menu.sfx_volume);
                            self.audio.play_sound("select");
                        }
                        2 => {
                            self.save.reset_slot(self.save.current_slot());
                            if let Some(lm) = &mut self.menu.level_manager {
                                lm.max_unlocked_level = 1;
                                lm.level_stars.clear();
                            }
                            self.menu.music_volume = self.save.data.music_volume;
                            self.menu.sfx_volume = self.save.data.sfx_volume;
                            self.audio.set_music_volume(self.menu.music_volume);
                            self.audio.set_sfx_volume(self.menu.sfx_volume);
                            self.audio.play_sound("select");
                        }
                        3 => {
                            self.menu.show_save_load = false;
                        }
                        _ => {}
                    }
                }
            }
            MenuAction::Credits => {
                self.audio.play_sound("select");
            }
            MenuAction::Exit => {
                self.state = AppState::Exiting;
            }
            MenuAction::Language => {
                let new_lang = if self.i18n.current_lang == "en" {
                    "id".to_string()
                } else {
                    "en".to_string()
                };
                self.i18n.set_language(&new_lang);
                let music_volume = self.menu.music_volume;
                let sfx_volume = self.menu.sfx_volume;
                let keep_settings = self.menu.show_settings;
                let keep_selected = self.menu.selected_settings_option;
                self.menu = crate::scenes::menu::main_menu::MainMenu::new(&self.i18n);
                self.menu.music_volume = music_volume;
                self.menu.sfx_volume = sfx_volume;
                if keep_settings {
                    self.menu.show_settings = true;
                    self.menu.selected_settings_option = keep_selected;
                }
                self.save.data.language = new_lang;
                self.save.save();
                self.audio.play_sound("select");
            }
            MenuAction::None => {}
        }

        self.apply_menu_settings();
    }

    fn apply_menu_settings(&mut self) {
        let mut changed = false;
        if (self.save.data.music_volume - self.menu.music_volume).abs() > f32::EPSILON {
            self.save.data.music_volume = self.menu.music_volume;
            changed = true;
        }
        if (self.save.data.sfx_volume - self.menu.sfx_volume).abs() > f32::EPSILON {
            self.save.data.sfx_volume = self.menu.sfx_volume;
            changed = true;
        }
        if changed {
            self.save.save();
        }
        self.audio.set_music_volume(self.menu.music_volume);
        self.audio.set_sfx_volume(self.menu.sfx_volume);
    }

    fn update_level_select(&mut self, dt: f32) {
        self.level_selection.update(dt, Some(&self.save.data));

        if is_key_pressed(KeyCode::Escape) {
            self.state = AppState::Menu;
            self.audio.play_sound("select");
            return;
        }

        if let Some(event) = self.level_selection.take_event() {
            match event {
                LevelSelectEvent::Playable(level_id) => {
                    self.start_story_intro(level_id);
                    self.audio.play_sound("select");
                }
                LevelSelectEvent::Locked(_) => {
                    self.audio.play_sound("not_match");
                }
                LevelSelectEvent::ComingSoon(_) => {
                    self.audio.play_sound("not_match");
                }
                LevelSelectEvent::Portal => {
                    self.state = AppState::ResearchCenter;
                    self.audio.play_sound("select");
                }
            }
        }
    }

    fn start_story_intro(&mut self, level_id: u32) {
        self.pending_level_id = Some(level_id);
        if let Some(scene) = get_scene(level_id, ScenePhase::Intro) {
            self.story_runner = Some(crate::scenes::story::runner::StoryRunner::new(scene));
            self.state = AppState::StoryIntro;
        } else {
            self.launch_pending_level();
        }
    }

    fn update_story_intro(&mut self, dt: f32) {
        if is_key_pressed(KeyCode::Escape) {
            self.story_runner = None;
            self.launch_pending_level();
            return;
        }

        let done = if let Some(runner) = &mut self.story_runner {
            runner.update(dt);
            if runner.take_beep() {
                self.audio.play_sound("beep");
            }
            runner.is_done()
        } else {
            true
        };

        if done {
            if let Some(runner) = &self.story_runner {
                if let (Some(lid), Some(route)) = (self.pending_level_id, runner.chosen_route()) {
                    self.save.data.story_choices.insert(lid, route);
                    self.save.save();
                }
            }
            self.story_runner = None;
            self.launch_pending_level();
        }
    }

    fn launch_pending_level(&mut self) {
        if let Some(level_id) = self.pending_level_id.take() {
            if let Some(level) = self.level_selection.level_manager.get_level(level_id) {
                let lev = level.clone();
                self.level_selection.selected_level = level_id;
                self.board.start_level(lev);
                self.state = AppState::LevelPlaying;
                self.save.data.total_games += 1;
                self.save.save();
            }
        }
    }

    fn update_playing(&mut self, dt: f32) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            self.board.handle_click(mx, my, &self.audio);
        }
        if is_key_pressed(KeyCode::Escape) {
            self.state = AppState::Menu;
            self.save.update_high_score(self.board.score);
            self.save.data.data_core += self.board.earned_data_core;
            self.save.data.glitch_matter += self.board.earned_entropy;
            self.save.data.entropy += self.board.earned_entropy;
            self.save.save();
            self.audio.play_sound("select");
        }
        if is_key_pressed(KeyCode::R) {
            self.board = crate::scenes::gameplay::Board::new();
            self.audio.play_sound("select");
        }
        if is_key_pressed(KeyCode::P) {
            self.state = AppState::Paused;
            self.audio.play_sound("select");
        }
        self.board.update(dt, &self.audio);
    }

    fn update_level_playing(&mut self, dt: f32) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            self.board.handle_click(mx, my, &self.audio);
        }
        if is_key_pressed(KeyCode::Escape) {
            self.state = AppState::Paused;
            self.audio.play_sound("select");
        }
        if is_key_pressed(KeyCode::R) {
            if let Some(level) = self
                .level_selection
                .level_manager
                .get_level(self.level_selection.selected_level)
            {
                let lev = level.clone();
                self.board.start_level(lev);
                self.audio.play_sound("select");
            }
        }
        if is_key_pressed(KeyCode::P) {
            self.state = AppState::Paused;
            self.audio.play_sound("select");
        }

        self.board.update(dt, &self.audio);

        if self.board.level_complete {
            if let Some(result) = &self.board.level_result {
                self.save.update_level_progress(
                    self.level_selection.selected_level,
                    result.stars,
                    result.score,
                );
                self.level_selection
                    .level_manager
                    .complete_level(self.level_selection.selected_level, result.stars);

                self.save.data.data_core += self.board.earned_data_core;
                self.save.data.glitch_matter += self.board.earned_entropy;
                self.save.data.entropy += self.board.earned_entropy;
                self.save.save();

                let lid = self.level_selection.selected_level;
                if let Some(scene) = get_scene(lid, ScenePhase::Outro) {
                    self.story_runner = Some(crate::scenes::story::runner::StoryRunner::new(scene));
                    self.state = AppState::StoryOutro;
                } else {
                    self.state = AppState::LevelComplete;
                }
                self.audio.play_sound("select");
            }
        } else if self.board.level_failed {
            self.save.data.data_core += self.board.earned_data_core;
            self.save.data.glitch_matter += self.board.earned_entropy;
            self.save.data.entropy += self.board.earned_entropy;
            self.save.save();
            self.state = AppState::LevelFailed;
            self.audio.play_sound("not_match");
        }
    }

    fn update_story_outro(&mut self, dt: f32) {
        if is_key_pressed(KeyCode::Escape) {
            self.story_runner = None;
            self.state = AppState::LevelComplete;
            return;
        }

        let done = if let Some(runner) = &mut self.story_runner {
            runner.update(dt);
            if runner.take_beep() {
                self.audio.play_sound("beep");
            }
            runner.is_done()
        } else {
            true
        };

        if done {
            self.story_runner = None;
            self.state = AppState::LevelComplete;
        }
    }

    fn update_level_complete(&mut self) {
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
            let next = self.level_selection.selected_level + 1;
            if next <= 5 && self.level_selection.level_manager.can_play_level(next) {
                self.start_story_intro(next);
            } else {
                self.state = AppState::LevelSelect;
            }
            self.audio.play_sound("select");
        }
        if is_key_pressed(KeyCode::Escape) {
            self.state = AppState::LevelSelect;
            self.audio.play_sound("select");
        }
    }

    fn update_level_failed(&mut self) {
        if is_key_pressed(KeyCode::R) {
            let lid = self.level_selection.selected_level;
            self.start_story_intro(lid);
            self.audio.play_sound("select");
        }
        if is_key_pressed(KeyCode::Escape) {
            self.state = AppState::LevelSelect;
            self.audio.play_sound("select");
        }
    }

    fn update_paused(&mut self) {
        if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
            self.state = AppState::LevelPlaying;
            self.audio.play_sound("select");
        }
        if is_key_pressed(KeyCode::Q) {
            self.state = AppState::LevelSelect;
            self.audio.play_sound("select");
        }
    }

    fn update_research_center(&mut self, dt: f32) {
        if self.research_center.update(dt, &self.audio) {
            self.state = AppState::LevelSelect;
            self.audio.play_sound("select");
        }
    }

    fn sync_music(&mut self) {
        let track = match self.state {
            AppState::Menu => Some("bg_main_menu"),
            AppState::LevelSelect => Some("bg_neural"),
            AppState::ResearchCenter => Some("bg_research"),
            AppState::Playing
            | AppState::LevelPlaying
            | AppState::StoryIntro
            | AppState::StoryOutro
            | AppState::LevelComplete
            | AppState::LevelFailed
            | AppState::Paused => Some("bg_in_game"),
            AppState::Exiting => None,
        };

        if let Some(key) = track {
            self.audio.play_music(key);
        } else {
            self.audio.stop_music();
        }
    }
}
