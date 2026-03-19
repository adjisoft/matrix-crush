use macroquad::prelude::*;

use super::LevelSelection;

impl LevelSelection {
    pub(super) fn grid_layout(&self) -> (f32, f32, f32, f32) {
        let sw = screen_width();
        let sh = screen_height();
        let grid_w = sw * 0.75;
        let grid_h = sh * 0.62;
        let cell_w = grid_w / 5.0;
        let cell_h = grid_h / 5.0;
        let start_x = (sw - grid_w) / 2.0;
        let start_y = sh * 0.22;
        (cell_w, cell_h, start_x, start_y)
    }

    pub(super) fn node_center(
        &self,
        index: u32,
        cell_w: f32,
        cell_h: f32,
        start_x: f32,
        start_y: f32,
    ) -> Vec2 {
        let col = index % 5;
        let row = index / 5;
        let base = vec2(
            start_x + col as f32 * cell_w + cell_w * 0.5,
            start_y + row as f32 * cell_h + cell_h * 0.5,
        );
        let jitter = self.node_offset(index as usize);
        base + jitter
    }

    pub(super) fn node_offset(&self, idx: usize) -> Vec2 {
        let fx = (idx as f32 * 12.9898).sin();
        let fy = (idx as f32 * 78.233).cos();
        vec2(fx * 6.0, fy * 6.0)
    }

    pub(super) fn node_radius(&self, cell_w: f32, cell_h: f32) -> f32 {
        (cell_w.min(cell_h) * 0.24).clamp(14.0, 26.0)
    }

    pub(super) fn portal_rect(&self, sw: f32, sh: f32) -> Rect {
        let size = (sw.min(sh) * 0.22).clamp(150.0, 220.0);
        let x = sw - size - 30.0;
        let y = sh - size - 40.0;
        Rect::new(x, y, size, size)
    }
}
