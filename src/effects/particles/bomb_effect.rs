use macroquad::prelude::*;

use super::Particle;

pub struct BombEffect {
    pub pos: Vec2,
    pub radius: f32,
    pub progress: f32,
    pub color: Color,
    pub lifetime: f32,
    pub rings: Vec<BombRing>,
    pub particles: Vec<Particle>,
}

pub struct BombRing {
    pub radius: f32,
    pub progress: f32,
    pub speed: f32,
}

impl BombEffect {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        let mut rings = Vec::new();
        for i in 0..4 {
            rings.push(BombRing {
                radius: 15.0 + i as f32 * 20.0,
                progress: i as f32 * 0.15,
                speed: 2.5 + i as f32 * 0.8,
            });
        }

        let mut particles = Vec::new();
        for _ in 0..12 {
            let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
            let speed = rand::gen_range(100.0, 300.0);

            particles.push(Particle {
                pos: vec2(x, y),
                vel: vec2(angle.cos() * speed, angle.sin() * speed),
                color,
                char: '•',
                lifetime: 1.2,
                max_lifetime: 1.2,
                size: rand::gen_range(8.0, 14.0),
                rotation: rand::gen_range(-0.1, 0.1),
                scale: 1.0,
                glow: true,
            });
        }

        BombEffect {
            pos: vec2(x, y),
            radius: 0.0,
            progress: 0.0,
            color,
            lifetime: 1.2,
            rings,
            particles,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.progress += dt * 4.0;
        self.lifetime -= dt;

        for ring in &mut self.rings {
            ring.progress += dt * ring.speed;
        }

        for particle in &mut self.particles {
            particle.update(dt);
        }
        self.particles.retain(|p| p.is_alive());
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0 || !self.particles.is_empty()
    }

    pub fn draw(&self, offset: Vec2) {
        let alpha = self.lifetime * 0.8;
        let pos_x = self.pos.x + offset.x;
        let pos_y = self.pos.y + offset.y;

        for ring in &self.rings {
            let ring_alpha = alpha * (1.0 - (ring.progress % 1.0));
            let radius = ring.radius * (1.0 + (ring.progress % 1.0) * 2.5);

            let mut glow_color = self.color;
            glow_color.a = ring_alpha * 0.2;
            draw_circle(pos_x, pos_y, radius * 1.2, glow_color);

            let mut ring_color = self.color;
            ring_color.a = ring_alpha * 0.8;
            draw_circle_lines(pos_x, pos_y, radius, 3.0, ring_color);
        }

        let center_alpha = alpha * 0.5;
        let mut center_color = self.color;
        center_color.a = center_alpha;
        draw_circle(
            pos_x,
            pos_y,
            30.0 * (1.0 + self.progress.sin() * 0.2),
            center_color,
        );

        for particle in &self.particles {
            particle.draw(offset);
        }
    }
}
