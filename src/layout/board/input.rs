use super::*;
use crate::audio::AudioManager;

impl Board {
    pub fn swap_gems(
        &mut self,
        pos1: (usize, usize),
        pos2: (usize, usize),
        audio: &AudioManager,
    ) -> bool {
        if self.is_animating || self.level_complete || self.level_failed {
            return false;
        }

        if self.level_mode {
            if let Some(session) = &mut self.level_session {
                if !session.use_move() {
                    self.level_failed = true;
                    return false;
                }
            }
        }

        let temp = self.grid[pos1.1][pos1.0];
        self.grid[pos1.1][pos1.0] = self.grid[pos2.1][pos2.0];
        self.grid[pos2.1][pos2.0] = temp;

        let mut is_special_swap = false;
        if is_special_gem(self.grid[pos1.1][pos1.0]) {
            self.special_gems_to_process.push(pos1);
            is_special_swap = true;
        }
        if is_special_gem(self.grid[pos2.1][pos2.0]) {
            self.special_gems_to_process.push(pos2);
            is_special_swap = true;
        }

        if is_special_swap || self.check_for_matches() {
            audio.play_sound("select");
            true
        } else {
            let temp = self.grid[pos1.1][pos1.0];
            self.grid[pos1.1][pos1.0] = self.grid[pos2.1][pos2.0];
            self.grid[pos2.1][pos2.0] = temp;

            self.create_error_effect(pos1, pos2);
            audio.play_sound("not_match");

            if self.level_mode {
                if let Some(session) = &mut self.level_session {
                    session.moves_left += 1;
                }
            }

            false
        }
    }

    pub fn handle_click(&mut self, mouse_x: f32, mouse_y: f32, audio: &AudioManager) -> bool {
        if self.is_animating {
            return false;
        }

        let grid_x = ((mouse_x - BOARD_OFFSET_X) / CELL_SIZE) as usize;
        let grid_y = ((mouse_y - BOARD_OFFSET_Y) / CELL_SIZE) as usize;

        if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
            if let Some(selected) = self.selected {
                if selected == (grid_x, grid_y) && is_special_gem(self.grid[grid_y][grid_x]) {
                    // Double click to activate special gem
                    self.special_gems_to_process.push((grid_x, grid_y));
                    self.selected = None;
                    audio.play_sound("select");
                    return true;
                }

                let dx = (grid_x as i32 - selected.0 as i32).abs();
                let dy = (grid_y as i32 - selected.1 as i32).abs();

                if (dx == 1 && dy == 0) || (dx == 0 && dy == 1) {
                    let result = self.swap_gems(selected, (grid_x, grid_y), audio);
                    self.selected = None;
                    return result;
                } else {
                    self.selected = Some((grid_x, grid_y));
                    audio.play_sound("select");
                }
            } else {
                self.selected = Some((grid_x, grid_y));
                audio.play_sound("select");
            }
        }
        false
    }
}
