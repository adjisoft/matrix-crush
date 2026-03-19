use macroquad::prelude::*;

use super::{App, AppState};

impl App {
    pub fn draw(&self) {
        match self.state {
            AppState::Menu => {
                self.menu.draw(&self.save.data, &self.audio, &self.i18n);
            }
            AppState::LevelSelect => {
                clear_background(BLACK);
                self.level_selection.draw(Some(&self.save.data), &self.i18n);
            }
            AppState::Playing => {
                clear_background(BLACK);
                self.board.draw(&self.i18n.current_lang);
                self.draw_classic_instructions();
                self.draw_currency_hud();
            }
            AppState::StoryIntro => {
                clear_background(BLACK);
                self.draw_story_bg();
                if let Some(runner) = &self.story_runner {
                    runner.draw();
                }
                self.draw_skip_hint();
            }
            AppState::LevelPlaying => {
                clear_background(BLACK);
                self.board.draw(&self.i18n.current_lang);
                self.draw_level_instructions();
                self.draw_currency_hud();
            }
            AppState::StoryOutro => {
                clear_background(BLACK);
                self.board.draw(&self.i18n.current_lang);
                self.draw_story_overlay();
                if let Some(runner) = &self.story_runner {
                    runner.draw();
                }
                self.draw_skip_hint();
            }
            AppState::LevelComplete => {
                clear_background(BLACK);
                self.board.draw(&self.i18n.current_lang);
                self.draw_level_complete_screen();
            }
            AppState::LevelFailed => {
                clear_background(BLACK);
                self.board.draw(&self.i18n.current_lang);
                self.draw_level_failed_screen();
            }
            AppState::Paused => {
                clear_background(BLACK);
                self.board.draw(&self.i18n.current_lang);
                self.draw_paused_screen();
            }
            AppState::ResearchCenter => {
                clear_background(BLACK);
                self.research_center.draw();
            }
            AppState::Exiting => {
                self.draw_exit_screen();
            }
        }

        if self.show_fps {
            self.draw_fps();
        }
    }

    fn draw_story_bg(&self) {
        let sw = screen_width();
        let sh = screen_height();
        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.8));
        draw_text(
            "STORY MODE",
            sw / 2.0 - 80.0,
            80.0,
            36.0,
            Color::new(0.0, 1.0, 0.0, 0.8),
        );
    }

    fn draw_story_overlay(&self) {
        let sw = screen_width();
        let sh = screen_height();
        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.5));
    }

    fn draw_skip_hint(&self) {
        let sw = screen_width();
        let hint = "ESC: Skip Story";
        let w = measure_text(hint, None, 20, 1.0).width;
        draw_text(hint, sw - w - 20.0, 30.0, 20.0, GRAY);
    }

    fn draw_currency_hud(&self) {
        let sw = screen_width();
        let sh = screen_height();
        let text = format!(
            "DC: {}   GM: {}",
            self.save.data.data_core,
            self.save.data.glitch_matter
        );
        let w = measure_text(&text, None, 20, 1.0).width;
        draw_text(&text, sw - w - 20.0, sh - 20.0, 20.0, GREEN);
    }

    fn draw_classic_instructions(&self) {
        let text = if self.i18n.current_lang == "id" {
            "Klik: tukar | R: reset | P: pause | ESC: menu"
        } else {
            "Click: swap | R: reset | P: pause | ESC: menu"
        };
        draw_text(
            text,
            20.0,
            screen_height() - 20.0,
            20.0,
            Color::new(0.0, 0.8, 0.0, 0.8),
        );
    }

    fn draw_level_instructions(&self) {
        let text = if self.i18n.current_lang == "id" {
            "Klik: tukar | P: pause | ESC: pause"
        } else {
            "Click: swap | P: pause | ESC: pause"
        };
        draw_text(
            text,
            20.0,
            screen_height() - 20.0,
            20.0,
            Color::new(0.0, 0.8, 0.0, 0.8),
        );
    }

    fn draw_level_complete_screen(&self) {
        let sw = screen_width();
        let sh = screen_height();
        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.82));

        let title = if self.i18n.current_lang == "id" {
            "LEVEL SELESAI"
        } else {
            "LEVEL COMPLETE"
        };
        let tw = measure_text(title, None, 48, 1.0).width;
        draw_text(title, sw / 2.0 - tw / 2.0, sh / 2.0 - 120.0, 48.0, GREEN);

        if let Some(result) = &self.board.level_result {
            let stars = result.stars;
            let star_str = "*".repeat(stars as usize);
            let swt = measure_text(&star_str, None, 48, 1.0).width;
            draw_text(&star_str, sw / 2.0 - swt / 2.0, sh / 2.0 - 40.0, 48.0, YELLOW);

            let score_t = format!("Score: {}", result.score);
            let scw = measure_text(&score_t, None, 24, 1.0).width;
            draw_text(&score_t, sw / 2.0 - scw / 2.0, sh / 2.0 + 30.0, 24.0, WHITE);

            let dc_earn = format!(
                "+{} DC  +{} GM",
                self.board.earned_data_core, self.board.earned_entropy
            );
            let dew = measure_text(&dc_earn, None, 22, 1.0).width;
            draw_text(
                &dc_earn,
                sw / 2.0 - dew / 2.0,
                sh / 2.0 + 60.0,
                22.0,
                Color::new(0.0, 1.0, 0.5, 0.9),
            );
        }

        let cont = "SPACE/ENTER: Next Level  |  ESC: Level Select";
        let cw = measure_text(cont, None, 20, 1.0).width;
        draw_text(cont, sw / 2.0 - cw / 2.0, sh - 80.0, 20.0, GREEN);
    }

    fn draw_level_failed_screen(&self) {
        let sw = screen_width();
        let sh = screen_height();
        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.82));

        let title = if self.i18n.current_lang == "id" {
            "LEVEL GAGAL"
        } else {
            "LEVEL FAILED"
        };
        let tw = measure_text(title, None, 48, 1.0).width;
        draw_text(title, sw / 2.0 - tw / 2.0, sh / 2.0 - 60.0, 48.0, RED);

        let inst = if self.i18n.current_lang == "id" {
            "R: Coba Lagi  |  ESC: Pilih Level"
        } else {
            "R: Retry  |  ESC: Level Select"
        };
        let iw = measure_text(inst, None, 26, 1.0).width;
        draw_text(inst, sw / 2.0 - iw / 2.0, sh / 2.0 + 40.0, 26.0, WHITE);
    }

    fn draw_paused_screen(&self) {
        let sw = screen_width();
        let sh = screen_height();
        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.72));

        let title = "PAUSED";
        let tw = measure_text(title, None, 48, 1.0).width;
        draw_text(title, sw / 2.0 - tw / 2.0, sh / 2.0 - 60.0, 48.0, YELLOW);

        let inst = "P / ESC: Resume  |  Q: Level Select";
        let iw = measure_text(inst, None, 26, 1.0).width;
        draw_text(inst, sw / 2.0 - iw / 2.0, sh / 2.0 + 40.0, 26.0, WHITE);
    }

    fn draw_exit_screen(&self) {
        clear_background(BLACK);
        let text = if self.i18n.current_lang == "id" {
            "Terima kasih telah bermain!"
        } else {
            "Thanks for playing!"
        };
        let w = measure_text(text, None, 40, 1.0).width;
        draw_text(
            text,
            screen_width() / 2.0 - w / 2.0,
            screen_height() / 2.0,
            40.0,
            GREEN,
        );
    }

    fn draw_fps(&self) {
        draw_text(
            &format!("FPS: {:.0}", self.fps_counter),
            10.0,
            30.0,
            18.0,
            YELLOW,
        );
    }
}
