#![allow(dead_code, unused_variables)]

use macroquad::prelude::*;

mod audio;
mod effects;
mod layout;
mod level;
mod level_selection;
mod matrix_match;
mod savegame;

use audio::AudioManager;
use layout::{Board, MainMenu, MenuAction};
use savegame::SaveManager;

enum GameState {
    Menu,
    LevelSelect,
    Playing,
    LevelPlaying,
    LevelComplete,
    LevelFailed,
    Paused,
    Exiting,
}

struct Game {
    state: GameState,
    menu: MainMenu,
    board: Board,
    audio: AudioManager,
    save: SaveManager,
    language: String,
    level_selection: level_selection::LevelSelection,
    fps_counter: f32,
    show_fps: bool,
}

impl Game {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Inisialisasi dengan error handling
        let save = SaveManager::new();
        let audio = AudioManager::new().await;
        let language = save.data.language.clone();

        let mut level_selection = level_selection::LevelSelection::new();
        level_selection.level_manager.max_unlocked_level = save.data.max_unlocked_level;
        level_selection.level_manager.level_stars = save.data.level_stars.clone();

        Ok(Game {
            state: GameState::Menu,
            menu: MainMenu::new(&language),
            level_selection,
            board: Board::new(),
            audio,
            save,
            language,
            fps_counter: 0.0,
            show_fps: false,
        })
    }

    fn update(&mut self, dt: f32) {
        self.fps_counter = 1.0 / dt.max(0.001);

        if is_key_pressed(KeyCode::F1) {
            self.show_fps = !self.show_fps;
        }
        match self.state {
            GameState::Menu => {
                self.update_menu(dt);
            }
            GameState::LevelSelect => {
                self.update_level_select();
            }
            GameState::Playing => {
                self.update_playing(dt);
            }
            GameState::LevelPlaying => {
                self.update_level_playing(dt);
            }
            GameState::LevelComplete => {
                self.update_level_complete();
            }
            GameState::LevelFailed => {
                self.update_level_failed();
            }
            GameState::Paused => {
                self.update_paused();
            }
            GameState::Exiting => {}
        }
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
            MenuAction::Play => {
                self.state = GameState::Playing;
                self.board = Board::new();
                self.save.data.total_games += 1;
                self.save.save();
                self.audio.play_sound("select");
            }
            MenuAction::LevelSelect => {
                self.state = GameState::LevelSelect;
                self.audio.play_sound("select");
            }
            MenuAction::SaveLoad => {
                if self.menu.show_save_load {
                    match self.menu.selected_save_option {
                        0 => { // SAVE
                            if self.save.save() {
                                self.audio.play_sound("select");
                            }
                        }
                        1 => { // LOAD
                            self.save.load_from_slot(self.save.current_slot());
                            // Update level manager
                            if let Some(level_manager) = &mut self.menu.level_manager {
                                level_manager.max_unlocked_level = self.save.data.max_unlocked_level;
                                level_manager.level_stars = self.save.data.level_stars.clone();
                            }
                            self.audio.play_sound("select");
                        }
                        2 => { // RESET
                            self.save.reset_slot(self.save.current_slot());
                            // Update level manager
                            if let Some(level_manager) = &mut self.menu.level_manager {
                                level_manager.max_unlocked_level = 1;
                                level_manager.level_stars.clear();
                            }
                            self.audio.play_sound("select");
                        }
                        3 => { // BACK
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
                self.state = GameState::Exiting;
            }
            MenuAction::Language => {
                self.language = if self.language == "en" {
                    "id".to_string()
                } else {
                    "en".to_string()
                };
                self.menu = MainMenu::new(&self.language);
                self.save.data.language = self.language.clone();
                self.save.save();
                self.audio.play_sound("select");
            }
            MenuAction::None => {}
        }
    }

    fn update_level_select(&mut self) {
        self.level_selection.update();

        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::Menu;
            self.audio.play_sound("select");
        }

        if is_key_pressed(KeyCode::Enter) || is_mouse_button_pressed(MouseButton::Left) {
            if let Some(level_id) = self.level_selection.hover_level {
                if self.level_selection.level_manager.can_play_level(level_id) {
                    if let Some(level) = self.level_selection.level_manager.get_level(level_id) {
                        self.start_level(level.clone());
                        self.audio.play_sound("select");
                    }
                }
            }
        }
    }

    fn update_playing(&mut self, dt: f32) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            self.board.handle_click(mouse_x, mouse_y, &self.audio);
        }

        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::Menu;
            self.save.update_high_score(self.board.score);
            self.audio.play_sound("select");
        }

        if is_key_pressed(KeyCode::R) {
            self.board = Board::new();
            self.audio.play_sound("select");
        }

        if is_key_pressed(KeyCode::P) {
            self.state = GameState::Paused;
            self.audio.play_sound("select");
        }

        self.board.update(dt, &self.audio);
    }

    fn update_level_playing(&mut self, dt: f32) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            self.board.handle_click(mouse_x, mouse_y, &self.audio);
        }

        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::Paused;
            self.audio.play_sound("select");
        }

        if is_key_pressed(KeyCode::R) {
            if let Some(level) = self
                .level_selection
                .level_manager
                .get_level(self.level_selection.selected_level)
            {
                self.start_level(level.clone());
                self.audio.play_sound("select");
            }
        }

        if is_key_pressed(KeyCode::P) {
            self.state = GameState::Paused;
            self.audio.play_sound("select");
        }

        self.board.update(dt, &self.audio);

        // Handle level completion/failure
        if self.board.level_complete {
            if let Some(result) = &self.board.level_result {
                // Save progress
                self.save.update_level_progress(
                    self.level_selection.selected_level,
                    result.stars,
                    result.score,
                );

                // Update level manager
                self.level_selection
                    .level_manager
                    .complete_level(self.level_selection.selected_level, result.stars);

                self.state = GameState::LevelComplete;
                self.audio.play_sound("select");
            }
        } else if self.board.level_failed {
            self.state = GameState::LevelFailed;
            self.audio.play_sound("not_match");
        }
    }

    fn update_level_complete(&mut self) {
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
            // Cek apakah masih ada level berikutnya
            if self.level_selection.selected_level < 200 {
                self.level_selection.selected_level += 1;
                if let Some(level) = self
                    .level_selection
                    .level_manager
                    .get_level(self.level_selection.selected_level)
                {
                    if self.level_selection.level_manager.can_play_level(level.id) {
                        self.start_level(level.clone());
                    } else {
                        self.state = GameState::LevelSelect;
                    }
                } else {
                    self.state = GameState::LevelSelect;
                }
            } else {
                self.state = GameState::LevelSelect;
            }
            self.audio.play_sound("select");
        }

        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::LevelSelect;
            self.audio.play_sound("select");
        }
    }

    fn update_level_failed(&mut self) {
        if is_key_pressed(KeyCode::R) {
            if let Some(level) = self
                .level_selection
                .level_manager
                .get_level(self.level_selection.selected_level)
            {
                self.start_level(level.clone());
                self.audio.play_sound("select");
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::LevelSelect;
            self.audio.play_sound("select");
        }
    }

    fn update_paused(&mut self) {
        if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
            self.state = GameState::LevelPlaying;
            self.audio.play_sound("select");
        }

        if is_key_pressed(KeyCode::Q) {
            self.state = GameState::LevelSelect;
            self.audio.play_sound("select");
        }
    }

    fn start_level(&mut self, level: level::Level) {
        self.board.start_level(level);
        self.state = GameState::LevelPlaying;
    }

    fn draw(&self) {
        match self.state {
            GameState::Menu => {
                self.menu.draw(&self.save.data, &self.audio);
            }
            GameState::LevelSelect => {
                clear_background(BLACK);
                self.level_selection.draw();
                self.draw_level_select_instructions();
            }
            GameState::Playing => {
                clear_background(BLACK);
                self.board.draw(&self.language);
                self.draw_classic_instructions();
            }
            GameState::LevelPlaying => {
                clear_background(BLACK);
                self.board.draw(&self.language);
                self.draw_level_instructions();
            }
            GameState::LevelComplete => {
                clear_background(BLACK);
                self.board.draw(&self.language);
                self.draw_level_complete_screen();
            }
            GameState::LevelFailed => {
                clear_background(BLACK);
                self.board.draw(&self.language);
                self.draw_level_failed_screen();
            }
            GameState::Paused => {
                clear_background(BLACK);
                self.board.draw(&self.language);
                self.draw_paused_screen();
            }
            GameState::Exiting => {
                self.draw_exit_screen();
            }
        }

        // Draw FPS jika diaktifkan
        if self.show_fps {
            self.draw_fps();
        }
    }

    fn draw_classic_instructions(&self) {
        let instructions = if self.language == "id" {
            "ESC: Menu | R: Ulang | M: Mute | P: Pause | F1: FPS"
        } else {
            "ESC: Menu | R: Restart | M: Mute | P: Pause | F1: FPS"
        };

        let inst_width = measure_text(instructions, None, 16, 1.0).width;
        draw_text(
            instructions,
            screen_width() / 2.0 - inst_width / 2.0,
            screen_height() - 20.0,
            16.0,
            Color::new(0.0, 0.5, 0.0, 0.7),
        );
    }

    fn draw_level_instructions(&self) {
        let instructions = if self.language == "id" {
            "ESC: Pause | R: Ulang Level | M: Mute | F1: FPS"
        } else {
            "ESC: Pause | R: Retry Level | M: Mute | F1: FPS"
        };

        let inst_width = measure_text(instructions, None, 16, 1.0).width;
        draw_text(
            instructions,
            screen_width() / 2.0 - inst_width / 2.0,
            screen_height() - 20.0,
            16.0,
            Color::new(0.0, 0.5, 0.0, 0.7),
        );
    }

    fn draw_level_select_instructions(&self) {
        let instructions = if self.language == "id" {
            "<-/-: Ganti Halaman | ESC: Kembali | Klik: Pilih Level"
        } else {
            "<-/-: Change Page | ESC: Back | Click: Select Level"
        };

        let inst_width = measure_text(instructions, None, 16, 1.0).width;
        draw_text(
            instructions,
            screen_width() / 2.0 - inst_width / 2.0,
            screen_height() - 30.0,
            16.0,
            Color::new(0.0, 0.5, 0.0, 0.7),
        );
    }

    fn draw_level_complete_screen(&self) {
        // Semi-transparent overlay
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.8),
        );

        let title = if self.language == "id" {
            "LEVEL SELESAI!"
        } else {
            "LEVEL COMPLETE!"
        };
        let title_width = measure_text(title, None, 48, 1.0).width;
        draw_text(
            title,
            screen_width() / 2.0 - title_width / 2.0,
            screen_height() / 2.0 - 100.0,
            48.0,
            GREEN,
        );

        if let Some(result) = &self.board.level_result {
            // Draw stars with ASCII
            for i in 0..3 {
                let star_x = screen_width() / 2.0 - 75.0 + i as f32 * 50.0;
                let star_color = if i < result.stars as usize {
                    YELLOW
                } else {
                    DARKGRAY
                };

                draw_text("*", star_x, screen_height() / 2.0 - 20.0, 50.0, star_color);
            }

            // Score
            let score_text = format!(
                "{}: {}",
                if self.language == "id" {
                    "Skor"
                } else {
                    "Score"
                },
                result.score
            );
            let score_width = measure_text(&score_text, None, 24, 1.0).width;
            draw_text(
                &score_text,
                screen_width() / 2.0 - score_width / 2.0,
                screen_height() / 2.0 + 40.0,
                24.0,
                WHITE,
            );

            // Next level info
            let next_text = if self.level_selection.selected_level < 200 {
                if self.language == "id" {
                    format!(
                        "Level {} berikutnya!",
                        self.level_selection.selected_level + 1
                    )
                } else {
                    format!("Next Level {}!", self.level_selection.selected_level + 1)
                }
            } else {
                if self.language == "id" {
                    "Semua level selesai!".to_string()
                } else {
                    "All levels completed!".to_string()
                }
            };
            let next_width = measure_text(&next_text, None, 20, 1.0).width;
            draw_text(
                &next_text,
                screen_width() / 2.0 - next_width / 2.0,
                screen_height() / 2.0 + 80.0,
                20.0,
                LIGHTGRAY,
            );
        }

        let continue_text = if self.language == "id" {
            "SPACE/ENTER: Lanjut | ESC: Pilih Level"
        } else {
            "SPACE/ENTER: Continue | ESC: Level Select"
        };
        let cont_width = measure_text(continue_text, None, 18, 1.0).width;
        draw_text(
            continue_text,
            screen_width() / 2.0 - cont_width / 2.0,
            screen_height() - 100.0,
            18.0,
            GREEN,
        );
    }

    fn draw_level_failed_screen(&self) {
        // Semi-transparent overlay
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.8),
        );

        let title = if self.language == "id" {
            "LEVEL GAGAL"
        } else {
            "LEVEL FAILED"
        };
        let title_width = measure_text(title, None, 48, 1.0).width;
        draw_text(
            title,
            screen_width() / 2.0 - title_width / 2.0,
            screen_height() / 2.0 - 50.0,
            48.0,
            RED,
        );

        let instruction = if self.language == "id" {
            "R: Coba Lagi | ESC: Pilih Level"
        } else {
            "R: Retry | ESC: Level Select"
        };
        let inst_width = measure_text(instruction, None, 24, 1.0).width;
        draw_text(
            instruction,
            screen_width() / 2.0 - inst_width / 2.0,
            screen_height() / 2.0 + 50.0,
            24.0,
            WHITE,
        );
    }

    fn draw_paused_screen(&self) {
        // Semi-transparent overlay
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.7),
        );

        let title = if self.language == "id" {
            "PAUSE"
        } else {
            "PAUSED"
        };
        let title_width = measure_text(title, None, 48, 1.0).width;
        draw_text(
            title,
            screen_width() / 2.0 - title_width / 2.0,
            screen_height() / 2.0 - 50.0,
            48.0,
            YELLOW,
        );

        let instruction = if self.language == "id" {
            "P: Lanjut | Q: Pilih Level | ESC: Lanjut"
        } else {
            "P: Resume | Q: Level Select | ESC: Resume"
        };
        let inst_width = measure_text(instruction, None, 24, 1.0).width;
        draw_text(
            instruction,
            screen_width() / 2.0 - inst_width / 2.0,
            screen_height() / 2.0 + 50.0,
            24.0,
            WHITE,
        );
    }

    fn draw_exit_screen(&self) {
        clear_background(BLACK);
        let text = if self.language == "id" {
            "Terima kasih telah bermain!"
        } else {
            "Thanks for playing!"
        };
        let width = measure_text(text, None, 40, 1.0).width;
        draw_text(
            text,
            screen_width() / 2.0 - width / 2.0,
            screen_height() / 2.0,
            40.0,
            GREEN,
        );
    }

    fn draw_fps(&self) {
        let fps_text = format!("FPS: {:.0}", self.fps_counter);
        draw_text(&fps_text, 10.0, 30.0, 20.0, YELLOW);
    }
}

#[macroquad::main("Matrix Crush")]
async fn main() {
    // Inisialisasi random seed
    rand::srand(miniquad::date::now() as u64);

    // Buat game dengan error handling
    let mut game = match Game::new().await {
        Ok(game) => game,
        Err(e) => {
            eprintln!("Failed to initialize game: {}", e);
            return;
        }
    };

    loop {
        let dt = get_frame_time().min(0.1);

        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            game.update(dt);
            game.draw();
        })).unwrap_or_else(|_| {
            eprintln!("Panic occurred in game loop");
            std::process::exit(1);
        });

        if let GameState::Exiting = game.state {
            game.save.save();
            break;
        }

        next_frame().await
    }
}