use macroquad::prelude::*;

use super::Particle;

pub struct SweepEffect {
    pub pos: Vec2,
    pub direction: f32,
    pub progress: f32,
    pub width: f32,
    pub travel: f32,
    pub color: Color,
    pub lifetime: f32,
    pub particles: Vec<Particle>,
    pub jitter_phase: f32,
}

impl SweepEffect {
    pub fn new(x: f32, y: f32, direction: f32, color: Color, travel: f32) -> Self {
        let mut particles = Vec::new();

        for _ in 0..5 {
            let angle = direction + rand::gen_range(-0.5, 0.5);
            let speed = rand::gen_range(200.0, 400.0);

            particles.push(Particle {
                pos: vec2(x, y),
                vel: vec2(angle.cos() * speed, angle.sin() * speed),
                color,
                char: '✦',
                lifetime: 0.8,
                max_lifetime: 0.8,
                size: rand::gen_range(10.0, 16.0),
                rotation: rand::gen_range(-0.2, 0.2),
                scale: 1.0,
                glow: true,
            });
        }

        SweepEffect {
            pos: vec2(x, y),
            direction,
            progress: 0.0,
            width: 160.0,
            travel,
            color,
            lifetime: 0.9,
            particles,
            jitter_phase: rand::gen_range(0.0, std::f32::consts::PI * 2.0),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.progress += dt * 2.2;
        self.lifetime -= dt;

        for particle in &mut self.particles {
            particle.update(dt);
        }
        self.particles.retain(|p| p.is_alive());
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0 || !self.particles.is_empty()
    }

    pub fn draw(&self, offset: Vec2) {
        let base_alpha = (self.lifetime / 0.9).clamp(0.0, 1.0) * 0.8;
        let mut color = self.color;

        let jitter = (get_time() as f32 * 10.0 + self.jitter_phase).sin() * 0.06;
        let dir_angle = self.direction + jitter;
        let dir_vec = vec2(dir_angle.cos(), dir_angle.sin());

        let t = self.progress.clamp(0.0, 1.0);
        let eased = t * t * (3.0 - 2.0 * t);
        let travel_offset = (eased - 0.5) * self.travel;

        let center = self.pos + dir_vec * travel_offset;
        let start = center - dir_vec * (self.width * 0.5);
        let end = center + dir_vec * (self.width * 0.5);

        let thickness = 28.0 * (1.0 - t * 0.4);

        let segments = 5;
        for i in 0..segments {
            let t0 = i as f32 / segments as f32;
            let t1 = (i as f32 + 1.0) / segments as f32;
            let seg_start = start.lerp(end, t0);
            let seg_end = start.lerp(end, t1);
            let seg_alpha = base_alpha * (1.0 - t0);
            let seg_width = thickness * (1.0 - t0 * 0.4);

            color.a = seg_alpha;
            draw_line(
                seg_start.x + offset.x,
                seg_start.y + offset.y,
                seg_end.x + offset.x,
                seg_end.y + offset.y,
                seg_width,
                color,
            );

            let mut glow = color;
            glow.a = seg_alpha * 0.35;
            draw_line(
                seg_start.x + offset.x,
                seg_start.y + offset.y,
                seg_end.x + offset.x,
                seg_end.y + offset.y,
                seg_width * 1.6,
                glow,
            );
        }

        for particle in &self.particles {
            particle.draw(offset);
        }
    }
}
