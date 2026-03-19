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
    pub scale: f32,
    pub glow: bool,
}

impl Particle {
    pub fn new(x: f32, y: f32, gem_char: char, gem_color: Color) -> Self {
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let speed = rand::gen_range(150.0, 300.0);

        Particle {
            pos: vec2(x, y),
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color: gem_color,
            char: gem_char,
            lifetime: 1.2,
            max_lifetime: 1.2,
            size: rand::gen_range(18.0, 26.0),
            rotation: rand::gen_range(-0.2, 0.2),
            scale: 1.0,
            glow: false,
        }
    }

    pub fn new_explosion(x: f32, y: f32, gem_char: char, gem_color: Color) -> Self {
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let speed = rand::gen_range(250.0, 500.0);

        Particle {
            pos: vec2(x, y),
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color: gem_color,
            char: gem_char,
            lifetime: 1.5,
            max_lifetime: 1.5,
            size: rand::gen_range(22.0, 34.0),
            rotation: rand::gen_range(-0.3, 0.3),
            scale: 1.2,
            glow: true,
        }
    }

    pub fn new_bomb_effect(x: f32, y: f32, gem_char: char, gem_color: Color) -> Self {
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let speed = rand::gen_range(400.0, 800.0);

        Particle {
            pos: vec2(x, y),
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color: gem_color,
            char: gem_char,
            lifetime: 2.0,
            max_lifetime: 2.0,
            size: rand::gen_range(28.0, 40.0),
            rotation: rand::gen_range(-0.5, 0.5),
            scale: 1.5,
            glow: true,
        }
    }

    pub fn new_sweep_effect(
        x: f32,
        y: f32,
        gem_char: char,
        gem_color: Color,
        direction: f32,
    ) -> Self {
        let angle = direction + rand::gen_range(-0.8, 0.8);
        let speed = rand::gen_range(500.0, 800.0);

        Particle {
            pos: vec2(x, y),
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color: gem_color,
            char: gem_char,
            lifetime: 1.4,
            max_lifetime: 1.4,
            size: rand::gen_range(22.0, 32.0),
            rotation: rand::gen_range(-0.3, 0.3),
            scale: 1.3,
            glow: true,
        }
    }

    pub fn new_error(x: f32, y: f32, gem_char: char, gem_color: Color) -> Self {
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let speed = rand::gen_range(150.0, 250.0);

        let mut particle = Particle {
            pos: vec2(x, y),
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color: gem_color,
            char: gem_char,
            lifetime: 0.8,
            max_lifetime: 0.8,
            size: 24.0,
            rotation: 0.0,
            scale: 1.0,
            glow: true,
        };
        particle.color = Color::new(1.0, 0.1, 0.1, 1.0);
        particle
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.vel.y += 350.0 * dt;

        self.vel *= 0.98;

        self.lifetime -= dt;
        self.rotation += self.vel.x * dt * 0.02;
        self.scale = 1.0 + (self.lifetime / self.max_lifetime) * 0.5;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn draw(&self, offset: Vec2) {
        let alpha = self.lifetime / self.max_lifetime;
        let mut color = self.color;

        color.a = alpha * alpha * 0.9;

        let size = self.size * (0.3 + alpha * 0.7) * self.scale;

        if self.glow && alpha > 0.3 {
            let glow_alpha = alpha * 0.3;
            let glow_size = size * 1.5;
            let mut glow_color = color;
            glow_color.a = glow_alpha;

            draw_text_ex(
                &self.char.to_string(),
                self.pos.x + offset.x - size / 4.0,
                self.pos.y + offset.y,
                TextParams {
                    font_size: glow_size as u16,
                    color: glow_color,
                    rotation: self.rotation,
                    ..Default::default()
                },
            );
        }

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
