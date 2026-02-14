use macroquad::prelude::*;

#[derive(Clone)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub char: char,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: f32,
    pub rotation: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, gem_char: char, gem_color: Color) -> Self {
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let speed = rand::gen_range(100.0, 250.0);

        Particle {
            pos: vec2(x, y),
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color: gem_color,
            char: gem_char,
            lifetime: 1.2,
            max_lifetime: 1.2,
            size: rand::gen_range(16.0, 24.0),
            rotation: rand::gen_range(-0.2, 0.2),
        }
    }

    pub fn new_explosion(x: f32, y: f32, gem_char: char, gem_color: Color) -> Self {
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let speed = rand::gen_range(200.0, 400.0);

        Particle {
            pos: vec2(x, y),
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color: gem_color,
            char: gem_char,
            lifetime: 1.5,
            max_lifetime: 1.5,
            size: rand::gen_range(20.0, 30.0),
            rotation: rand::gen_range(-0.3, 0.3),
        }
    }

    pub fn new_bomb_effect(x: f32, y: f32, gem_char: char, gem_color: Color) -> Self {
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let speed = rand::gen_range(300.0, 600.0);

        Particle {
            pos: vec2(x, y),
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color: gem_color,
            char: gem_char,
            lifetime: 1.8,
            max_lifetime: 1.8,
            size: rand::gen_range(25.0, 35.0),
            rotation: rand::gen_range(-0.5, 0.5),
        }
    }

    pub fn new_sweep_effect(
        x: f32,
        y: f32,
        gem_char: char,
        gem_color: Color,
        direction: f32,
    ) -> Self {
        let angle = direction + rand::gen_range(-0.5, 0.5);
        let speed = rand::gen_range(400.0, 600.0);

        Particle {
            pos: vec2(x, y),
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color: gem_color,
            char: gem_char,
            lifetime: 1.2,
            max_lifetime: 1.2,
            size: rand::gen_range(20.0, 28.0),
            rotation: rand::gen_range(-0.2, 0.2),
        }
    }

    pub fn new_error(x: f32, y: f32, gem_char: char, gem_color: Color) -> Self {
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let speed = rand::gen_range(100.0, 200.0);

        let mut particle = Particle {
            pos: vec2(x, y),
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color: gem_color,
            char: gem_char,
            lifetime: 0.6,
            max_lifetime: 0.6,
            size: 20.0,
            rotation: 0.0,
        };
        particle.color = Color::new(1.0, 0.2, 0.2, 1.0);
        particle
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.vel.y += 300.0 * dt;

        self.vel *= 0.96;

        self.lifetime -= dt;
        self.rotation += self.vel.x * dt * 0.01;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn draw(&self, offset: Vec2) {
        let alpha = self.lifetime / self.max_lifetime;
        let mut color = self.color;
        color.a = alpha * 0.8;

        let size = self.size * (0.5 + alpha * 0.5);

        draw_text_ex(
            &self.char.to_string(),
            self.pos.x + offset.x - size / 4.0,
            self.pos.y + offset.y,
            TextParams {
                font_size: size as u16,
                color,
                rotation: self.rotation,
                ..Default::default()
            },
        );
    }
}

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
    pub bounce: f32,
    pub rotation: f32,
    pub is_special: bool,
}

impl FallingGem {
    pub fn new(
        gem_char: char,
        gem_color: Color,
        target_x: usize,
        target_y: usize,
        start_y: f32,
    ) -> Self {
        let cell_size = 40.0;
        let board_offset_x = 150.0;
        let board_offset_y = 80.0;

        let _target_pos_y = target_y as f32 * cell_size + board_offset_y + cell_size / 2.0 + 8.0;
        let target_pos_x = target_x as f32 * cell_size + board_offset_x + cell_size / 2.0 - 8.0;

        let is_special =
            gem_char == crate::matrix_match::gem::BOMB_GEM || gem_char == crate::matrix_match::gem::SWEEP_GEM;

        FallingGem {
            char: gem_char,
            color: gem_color,
            target_x,
            target_y,
            current_pos: vec2(target_pos_x, start_y),
            start_y,
            progress: 0.0,
            speed: rand::gen_range(1.5, 3.5),
            bounce: 0.0,
            rotation: rand::gen_range(-0.1, 0.1),
            is_special,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let cell_size = 40.0;
        let board_offset_y = 80.0;
        let target_y = self.target_y as f32 * cell_size + board_offset_y + cell_size / 2.0 + 8.0;

        self.progress += dt * self.speed;

        if self.progress < 1.0 {
            let t = self.progress;

            if t < 0.6 {
                let fall_t = t / 0.6;
                self.current_pos.y = self.start_y + (target_y - self.start_y) * fall_t * fall_t;
            } else {
                let bounce_t = (t - 0.6) / 0.4;
                self.bounce =
                    (bounce_t * std::f32::consts::PI * 2.0).sin() * 8.0 * (1.0 - bounce_t);
                self.current_pos.y = target_y - self.bounce;
            }

            if self.is_special {
                self.rotation = (t * 15.0).sin() * 0.2;
            } else {
                self.rotation = (t * 10.0).sin() * 0.1;
            }
        } else {
            self.current_pos.y = target_y;
            self.rotation = 0.0;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.progress >= 1.0
    }

    pub fn draw(&self, offset: Vec2) {
        let mut color = self.color;

        if self.progress < 1.0 {
            color.a = 0.8 + (self.progress * 0.2);
        }

        if self.is_special && self.progress >= 0.9 {
            let pulse = (get_time() as f32 * 5.0).sin().abs() * 0.3 + 0.7;
            color = Color::new(color.r * pulse, color.g * pulse, color.b * pulse, color.a);
        }

        let _stretch = 1.0 + (self.progress * 2.0).sin().abs() * 0.1;

        draw_text_ex(
            &self.char.to_string(),
            self.current_pos.x + offset.x,
            self.current_pos.y + offset.y,
            TextParams {
                font_size: if self.is_special { 28 } else { 24 },
                color,
                rotation: self.rotation,
                ..Default::default()
            },
        );

        if self.progress < 0.7 {
            let trail_alpha = if self.is_special { 0.25 } else { 0.15 } * (1.0 - self.progress);
            let mut trail_color = self.color;
            trail_color.a = trail_alpha;

            let trail_count = if self.is_special { 6 } else { 3 };

            for i in 1..trail_count {
                let trail_y = self.current_pos.y - (i as f32 * 6.0) * (1.0 - self.progress);
                let trail_x = self.current_pos.x + (i as f32 * 3.0) * (1.0 - self.progress).sin();

                draw_text_ex(
                    &self.char.to_string(),
                    trail_x + offset.x,
                    trail_y + offset.y,
                    TextParams {
                        font_size: if self.is_special { 20 } else { 16 },
                        color: trail_color,
                        rotation: self.rotation * 0.5,
                        ..Default::default()
                    },
                );
            }
        }
    }
}

pub struct SweepEffect {
    pub pos: Vec2,
    pub direction: f32,
    pub progress: f32,
    pub width: f32,
    pub color: Color,
    pub lifetime: f32,
}

impl SweepEffect {
    pub fn new(x: f32, y: f32, direction: f32, color: Color) -> Self {
        SweepEffect {
            pos: vec2(x, y),
            direction,
            progress: 0.0,
            width: 100.0,
            color,
            lifetime: 0.8,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.progress += dt * 2.0;
        self.lifetime -= dt;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn draw(&self, offset: Vec2) {
        let alpha = (self.lifetime / 0.8) * 0.5;
        let mut color = self.color;
        color.a = alpha;

        let dir_vec = vec2(self.direction.cos(), self.direction.sin());
        let perp = vec2(-dir_vec.y, dir_vec.x);

        let start = self.pos + dir_vec * (self.progress - 0.5) * self.width;
        let end = self.pos + dir_vec * (self.progress + 0.5) * self.width;

        let width = 20.0 * (1.0 - self.progress.abs() * 0.5);

        let p1 = start + perp * width;
        let p2 = start - perp * width;
        let p3 = end - perp * width;
        let p4 = end + perp * width;

        let p1_vec = vec2(p1.x + offset.x, p1.y + offset.y);
        let p2_vec = vec2(p2.x + offset.x, p2.y + offset.y);
        let p3_vec = vec2(p3.x + offset.x, p3.y + offset.y);
        let p4_vec = vec2(p4.x + offset.x, p4.y + offset.y);

        draw_line(p1_vec.x, p1_vec.y, p2_vec.x, p2_vec.y, 2.0, color);
        draw_line(p2_vec.x, p2_vec.y, p3_vec.x, p3_vec.y, 2.0, color);
        draw_line(p3_vec.x, p3_vec.y, p4_vec.x, p4_vec.y, 2.0, color);
        draw_line(p4_vec.x, p4_vec.y, p1_vec.x, p1_vec.y, 2.0, color);

        draw_triangle(p1_vec, p2_vec, p3_vec, color);
        draw_triangle(p1_vec, p3_vec, p4_vec, color);
    }
}

pub struct BombEffect {
    pub pos: Vec2,
    pub radius: f32,
    pub progress: f32,
    pub color: Color,
    pub lifetime: f32,
    pub rings: Vec<BombRing>,
}

pub struct BombRing {
    pub radius: f32,
    pub progress: f32,
    pub speed: f32,
}

impl BombEffect {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        let mut rings = Vec::new();
        for i in 0..3 {
            rings.push(BombRing {
                radius: 20.0 + i as f32 * 15.0,
                progress: i as f32 * 0.2,
                speed: 2.0 + i as f32 * 0.5,
            });
        }

        BombEffect {
            pos: vec2(x, y),
            radius: 0.0,
            progress: 0.0,
            color,
            lifetime: 1.0,
            rings,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.progress += dt * 3.0;
        self.lifetime -= dt;

        for ring in &mut self.rings {
            ring.progress += dt * ring.speed;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn draw(&self, offset: Vec2) {
        let alpha = self.lifetime;
        let pos_x = self.pos.x + offset.x;
        let pos_y = self.pos.y + offset.y;

        for ring in &self.rings {
            let ring_alpha = alpha * (1.0 - (ring.progress % 1.0));
            let radius = ring.radius * (1.0 + (ring.progress % 1.0) * 2.0);

            let mut color = self.color;
            color.a = ring_alpha * 0.3;

            draw_circle(pos_x, pos_y, radius, color);

            color.a = ring_alpha * 0.8;
            draw_circle_lines(pos_x, pos_y, radius, 2.0, color);
        }
    }
}
