use macroquad::prelude::*;

#[derive(Clone)]
pub struct FallingGem {
    pub char: char,
    pub color: Color,
    pub target_x: usize,
    pub target_y: usize,
    pub current_pos: Vec2,
    pub start_y: f32,
    pub progress: f32,
    pub speed: f32,
    pub rotation: f32,
    pub is_special: bool,
    pub trail_alpha: f32,
    pub cell_size: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub target_pos_x: f32,
    pub squash: f32,
}

impl FallingGem {
    pub fn new(
        gem_char: char,
        gem_color: Color,
        target_x: usize,
        target_y: usize,
        start_y: f32,
        cell_size: f32,
        offset_x: f32,
        offset_y: f32,
    ) -> Self {
        let target_pos_x = target_x as f32 * cell_size + offset_x + cell_size / 2.0 - 8.0;

        let is_special = gem_char == crate::matrix_match::gem::BOMB_GEM
            || gem_char == crate::matrix_match::gem::SWEEP_GEM;

        FallingGem {
            char: gem_char,
            color: gem_color,
            target_x,
            target_y,
            current_pos: vec2(target_pos_x, start_y),
            start_y,
            progress: 0.0,
            speed: rand::gen_range(2.4, 3.8),
            rotation: rand::gen_range(-0.1, 0.1),
            is_special,
            trail_alpha: 0.0,
            cell_size,
            offset_x,
            offset_y,
            target_pos_x,
            squash: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let target_y =
            self.target_y as f32 * self.cell_size + self.offset_y + self.cell_size / 2.0 + 8.0;

        self.progress = (self.progress + dt * self.speed).min(1.0);
        let t = self.progress;

        let overshoot = 1.4;
        let t1 = t - 1.0;
        let eased = if t < 1.0 {
            1.0 + (overshoot + 1.0) * t1 * t1 * t1 + overshoot * t1 * t1
        } else {
            1.0
        };

        let sway = (t * std::f32::consts::PI * 2.0).sin() * 6.0 * (1.0 - t);
        self.current_pos.x = self.target_pos_x + sway;
        self.current_pos.y = self.start_y + (target_y - self.start_y) * eased;

        let land_t = ((t - 0.85) / 0.15).clamp(0.0, 1.0);
        self.squash = (land_t * std::f32::consts::PI).sin() * 0.2;

        self.trail_alpha = (t * 2.0).sin().abs() * 0.3;

        if self.is_special {
            self.rotation = sway * 0.02 + (t * 18.0).sin() * 0.25;
        } else {
            self.rotation = sway * 0.015 + (t * 12.0).sin() * 0.12;
        }

        if self.progress >= 1.0 {
            self.current_pos.x = self.target_pos_x;
            self.current_pos.y = target_y;
            self.rotation = 0.0;
            self.trail_alpha = 0.0;
            self.squash = 0.0;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.progress >= 1.0
    }

    pub fn draw(&self, offset: Vec2) {
        let mut color = self.color;

        if self.progress < 1.0 {
            color.a = 0.9 + (self.progress * 0.1);
        }

        if self.is_special {
            let pulse = (get_time() as f32 * 8.0).sin() * 0.2 + 0.8;
            color = Color::new(
                (color.r * pulse).min(1.0),
                (color.g * pulse).min(1.0),
                (color.b * pulse).min(1.0),
                color.a,
            );
        }

        if self.progress < 0.85 && self.trail_alpha > 0.0 {
            let trail_count = if self.is_special { 8 } else { 4 };

            for i in 1..trail_count {
                let trail_factor = i as f32 / trail_count as f32;
                let trail_alpha = self.trail_alpha * (1.0 - trail_factor) * 0.4;
                let trail_y =
                    self.current_pos.y - (i as f32 * 8.0) * (1.0 - self.progress);
                let trail_x =
                    self.current_pos.x + (i as f32 * 4.0) * self.progress.sin();

                let mut trail_color = self.color;
                trail_color.a = trail_alpha;

                draw_text_ex(
                    &self.char.to_string(),
                    trail_x + offset.x,
                    trail_y + offset.y,
                    TextParams {
                        font_size: (if self.is_special { 22 } else { 18 }) as u16,
                        color: trail_color,
                        rotation: self.rotation * 0.7,
                        ..Default::default()
                    },
                );
            }
        }

        let base_size = if self.is_special { 30.0 } else { 26.0 };
        let size = base_size * (1.0 + self.squash);

        draw_text_ex(
            &self.char.to_string(),
            self.current_pos.x + offset.x,
            self.current_pos.y + offset.y + self.squash * 4.0,
            TextParams {
                font_size: size as u16,
                color,
                rotation: self.rotation,
                ..Default::default()
            },
        );
    }
}
