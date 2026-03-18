use crate::level::LevelManager;
use macroquad::prelude::*;

pub struct LevelSelection {
    pub level_manager: LevelManager,
    pub scroll_offset: f32,
    pub selected_level: u32,
    pub hover_level: Option<u32>,
    pub page: u32,
    pub levels_per_page: u32,
}

impl LevelSelection {
    pub fn new() -> Self {
        LevelSelection {
            level_manager: LevelManager::new(),
            scroll_offset: 0.0,
            selected_level: 1,
            hover_level: None,
            page: 0,
            levels_per_page: 20,
        }
    }

    pub fn update(&mut self) {
        let _rows = 4;
        let cols = 5;
        let cell_size = 80.0;
        let start_x = screen_width() / 2.0 - (cols as f32 * cell_size) / 2.0;
        let start_y = 150.0;

        let (mouse_x, mouse_y) = mouse_position();

        self.hover_level = None;

        for i in 0..self.levels_per_page {
            let level_id = self.page * self.levels_per_page + i + 1;
            if level_id > 200 {
                break;
            }

            let _row = (i / cols) as usize;
            let col = (i % cols) as usize;

            let x = start_x + col as f32 * cell_size;
            let y = start_y + (i / cols) as f32 * cell_size + self.scroll_offset;

            let rect = Rect::new(x, y, cell_size - 10.0, cell_size - 10.0);

            if rect.contains(vec2(mouse_x, mouse_y)) {
                self.hover_level = Some(level_id);

                if is_mouse_button_pressed(MouseButton::Left) {
                    if self.level_manager.can_play_level(level_id) {
                        self.selected_level = level_id;
                    }
                }
                break;
            }
        }

        let scroll = mouse_wheel();
        self.scroll_offset += scroll.1 * 20.0;
        self.scroll_offset = self.scroll_offset.clamp(-300.0, 0.0);

        if is_key_pressed(KeyCode::Left) && self.page > 0 {
            self.page -= 1;
            self.scroll_offset = 0.0;
        }
        if is_key_pressed(KeyCode::Right) && self.page < 9 {
            self.page += 1;
            self.scroll_offset = 0.0;
        }
    }

    pub fn draw(&self) {
        let _rows = 4;
        let cols = 5;
        let cell_size = 80.0;
        let start_x = screen_width() / 2.0 - (cols as f32 * cell_size) / 2.0;
        let start_y = 150.0;

        // Draw title
        draw_text(
            &format!("SELECT LEVEL - Page {}/10", self.page + 1),
            screen_width() / 2.0 - 150.0,
            80.0,
            40.0,
            GREEN,
        );

        // Draw level grid
        for i in 0..self.levels_per_page {
            let level_id = self.page * self.levels_per_page + i + 1;
            if level_id > 200 {
                break;
            }

            let _row = (i / cols) as usize;
            let col = (i % cols) as usize;

            let x = start_x + col as f32 * cell_size;
            let y = start_y + (i / cols) as f32 * cell_size + self.scroll_offset;

            let rect = Rect::new(x, y, cell_size - 10.0, cell_size - 10.0);

            // Background color based on unlock status
            let bg_color = if !self.level_manager.can_play_level(level_id) {
                Color::new(0.2, 0.2, 0.2, 0.8) // Locked
            } else if Some(level_id) == self.hover_level {
                Color::new(0.3, 0.8, 0.3, 0.8) // Hover
            } else if level_id == self.selected_level {
                Color::new(0.2, 0.6, 0.2, 0.8) // Selected
            } else {
                Color::new(0.1, 0.4, 0.1, 0.8) // Normal
            };

            draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg_color);
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, GREEN);

            // Level number
            draw_text(
                &level_id.to_string(),
                rect.x + 10.0,
                rect.y + 30.0,
                24.0,
                WHITE,
            );

            // PERBAIKAN: Ganti emoji bintang dengan ASCII
            let stars = self.level_manager.get_level_stars(level_id);
            for s in 0..stars {
                draw_text(
                    "*", // Ganti dari "★" ke "*"
                    rect.x + 10.0 + s as f32 * 15.0,
                    rect.y + 55.0,
                    16.0,
                    YELLOW,
                );
            }

            // PERBAIKAN: Ganti emoji gembok dengan ASCII
            if !self.level_manager.can_play_level(level_id) {
                draw_text(
                    "[L]", // Ganti dari "🔒" ke "[L]" (Locked)
                    rect.x + rect.w - 40.0,
                    rect.y + 25.0,
                    16.0,
                    RED,
                );
            }

            // Move count
            if let Some(level) = self.level_manager.get_level(level_id) {
                draw_text(
                    &format!("{} moves", level.moves),
                    rect.x + 10.0,
                    rect.y + rect.h - 10.0,
                    12.0,
                    LIGHTGRAY,
                );
            }
        }

        // Navigation hints
        draw_text(
            "<- Prev Page | -> Next Page", // Ganti panah ASCII
            screen_width() / 2.0 - 150.0,
            screen_height() - 50.0,
            24.0,
            DARKGREEN,
        );

        // Level info panel
        if let Some(level_id) = self.hover_level {
            if let Some(level) = self.level_manager.get_level(level_id) {
                self.draw_level_info(level);
            }
        }
    }

    fn draw_level_info(&self, level: &crate::level::Level) {
        let panel_x = 50.0;
        let panel_y = screen_height() / 2.0 - 150.0;

        draw_rectangle(
            panel_x,
            panel_y,
            300.0,
            300.0,
            Color::new(0.0, 0.0, 0.0, 0.9),
        );

        draw_text(&level.name, panel_x + 10.0, panel_y + 30.0, 24.0, GREEN);

        draw_text(
            &level.description,
            panel_x + 10.0,
            panel_y + 60.0,
            16.0,
            WHITE,
        );

        draw_text(
            &format!("Moves: {}", level.moves),
            panel_x + 10.0,
            panel_y + 90.0,
            18.0,
            YELLOW,
        );

        draw_text(
            "Objectives:",
            panel_x + 10.0,
            panel_y + 120.0,
            18.0,
            LIGHTGRAY,
        );

        for (i, obj) in level.objectives.iter().enumerate() {
            let obj_text = match obj {
                crate::level::LevelObjective::Score(s) => format!("Score: {}", s),
                crate::level::LevelObjective::CollectGems { gem_type, count } => {
                    let gem_display = match gem_type {
                        '💎' => "O", // Ruby
                        '💠' => "#", // Sapphire
                        '💚' => "@", // Emerald
                        '💛' => "$", // Topaz
                        '💜' => "%", // Amethyst
                        _ => "?",
                    };
                    format!("Collect {} '{}': {}", count, gem_display, count)
                }
                crate::level::LevelObjective::ClearGems(c) => format!("Clear {} gems", c),
                crate::level::LevelObjective::Combo(c) => format!("Combo x{}", c),
                crate::level::LevelObjective::SpecialGems(s) => format!("Create {} special", s),
            };

            draw_text(
                &obj_text,
                panel_x + 20.0,
                panel_y + 150.0 + i as f32 * 20.0,
                14.0,
                WHITE,
            );
        }

        let stars = self.level_manager.get_level_stars(level.id);
        let stars_display = format!("Stars: {}/3", stars);
        draw_text(
            &stars_display,
            panel_x + 10.0,
            panel_y + 260.0,
            18.0,
            YELLOW,
        );
    }
}
