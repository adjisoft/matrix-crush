#![windows_subsystem = "windows"]

use macroquad::prelude::*;

mod audio;
mod effects;
mod layout;
mod matrix_match;
mod savegame;

use audio::AudioManager;
use layout::{Board, MainMenu, MenuAction};
use savegame::SaveManager;

enum GameState {
    Menu,
    Playing,
    Exiting,
}

struct Game {
    state: GameState,
    menu: MainMenu,
    board: Board,
    audio: AudioManager,
    save: SaveManager,
    language: String,
}

impl Game {
    async fn new() -> Self {
        let save = SaveManager::new();
        let audio = AudioManager::new().await;
        let language = save.data.language.clone();

        Game {
            state: GameState::Menu,
            menu: MainMenu::new(&language),
            board: Board::new(),
            audio,
            save,
            language,
        }
    }

    fn update(&mut self, dt: f32) {
        match self.state {
            GameState::Menu => {
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
            GameState::Playing => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    self.board.handle_click(mouse_x, mouse_y, &self.audio);
                }

                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Menu;
                    self.save.update_high_score(self.board.score);
                }

                if is_key_pressed(KeyCode::R) {
                    self.board = Board::new();
                    self.audio.play_sound("select");
                }

                if is_key_pressed(KeyCode::M) {
                    self.audio.toggle_mute();
                }

                self.board.update(dt, &self.audio);
            }
            GameState::Exiting => {}
        }
    }

    fn draw(&self) {
        match self.state {
            GameState::Menu => {
                self.menu.draw(&self.save.data, &self.audio);
            }
            GameState::Playing => {
                clear_background(BLACK);
                self.board.draw(&self.language);

                let instructions = if self.language == "id" {
                    "ESC: Menu | R: Ulang | M: Mute"
                } else {
                    "ESC: Menu | R: Restart | M: Mute"
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
            GameState::Exiting => {
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
        }
    }
}

#[macroquad::main("Matrix Crush")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut game = Game::new().await;

    loop {
        let dt = get_frame_time();

        game.update(dt);
        game.draw();

        if let GameState::Exiting = game.state {
            next_frame().await;
            break;
        }

        next_frame().await
    }
}
