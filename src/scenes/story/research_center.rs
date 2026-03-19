// research_center.rs -- Abandoned Research Center (WIP)

use macroquad::prelude::*;
use crate::systems::audio::AudioManager;

const CHATTER_LINES: [&str; 8] = [
    "We remember the collider's hum.",
    "Neurons asleep. Pathways severed.",
    "The portal flickers. The lab waits.",
    "Upgrade protocols: offline.",
    "Unlock keys: corrupted.",
    "Matter search: pending.",
    "Stability is a myth.",
    "Do you hear us?",
];

const GLITCH_FACES: [&str; 3] = ["(vc%)", "($>@)", "( * )"];

pub struct ResearchCenterView {
    time: f32,
    chatter_timer: f32,
    chatter_index: usize,
    back_hover: bool,
    glitch_seed: u32,
    lhc_jump_timer: f32,
    lhc_jump_cooldown: f32,
    lhc_flash: f32,
    lhc_glitch: f32,
}

impl ResearchCenterView {
    pub fn new() -> Self {
        ResearchCenterView {
            time: 0.0,
            chatter_timer: 0.0,
            chatter_index: 0,
            back_hover: false,
            glitch_seed: 1337,
            lhc_jump_timer: 0.0,
            lhc_jump_cooldown: 2.5,
            lhc_flash: 0.0,
            lhc_glitch: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, audio: &AudioManager) -> bool {
        self.time += dt;
        self.chatter_timer += dt;
        if self.chatter_timer > 2.4 {
            self.chatter_timer = 0.0;
            self.chatter_index = (self.chatter_index + 1) % CHATTER_LINES.len();
        }
        self.glitch_seed = self.glitch_seed.wrapping_mul(1664525).wrapping_add(1013904223);

        if self.lhc_jump_cooldown > 0.0 {
            self.lhc_jump_cooldown = (self.lhc_jump_cooldown - dt).max(0.0);
        }
        if self.lhc_jump_timer > 0.0 {
            self.lhc_jump_timer = (self.lhc_jump_timer - dt).max(0.0);
        }
        if self.lhc_flash > 0.0 {
            self.lhc_flash = (self.lhc_flash - dt * 2.6).max(0.0);
        }
        if self.lhc_glitch > 0.0 {
            self.lhc_glitch = (self.lhc_glitch - dt * 1.8).max(0.0);
        }

        if self.lhc_jump_cooldown <= 0.0 {
            let trigger = rand::gen_range(0.0, 1.0);
            if trigger > 0.97 {
                self.trigger_lhc_jump(audio);
            }
        }

        let (mx, my) = mouse_position();
        let back_rect = self.back_button_rect();
        self.back_hover = back_rect.contains(vec2(mx, my));

        if is_key_pressed(KeyCode::Escape) {
            return true;
        }
        if self.back_hover && is_mouse_button_pressed(MouseButton::Left) {
            return true;
        }
        false
    }

    pub fn draw(&self) {
        let sw = screen_width();
        let sh = screen_height();
        let t = self.time;

        let jitter = if self.lhc_glitch > 0.0 {
            vec2(
                (t * 40.0).sin() * self.lhc_glitch * 3.0,
                (t * 37.0).cos() * self.lhc_glitch * 2.5,
            )
        } else {
            vec2(0.0, 0.0)
        };

        clear_background(Color::new(0.0, 0.0, 0.0, 1.0));
        self.draw_lab_backdrop(sw, sh, t);
        self.draw_dead_neurons(sw, sh, t);
        self.draw_lhc(sw, sh, t, jitter);
        self.draw_glitch_entities(sw, sh, t);
        self.draw_wip_panel(sw, sh, t);
        self.draw_back_button(t);
        self.draw_jump_overlay(sw, sh, t);
    }

    fn back_button_rect(&self) -> Rect {
        let sh = screen_height();
        let w = 160.0;
        let h = 40.0;
        Rect::new(24.0, sh - h - 18.0, w, h)
    }

    fn draw_lab_backdrop(&self, sw: f32, _sh: f32, t: f32) {
        let title = "ABANDONED RESEARCH CENTER";
        let tw = measure_text(title, None, 32, 1.0).width;
        draw_text(title, sw / 2.0 - tw / 2.0, 54.0, 32.0,
            Color::new(0.7, 0.1, 1.0, 0.95));

        let subtitle = "STATUS: UNDER DEVELOPMENT";
        let swt = measure_text(subtitle, None, 18, 1.0).width;
        draw_text(subtitle, sw / 2.0 - swt / 2.0, 80.0, 18.0,
            Color::new(0.7, 0.4, 1.0, 0.7));

        for i in 0..6 {
            let y = 120.0 + i as f32 * 60.0 + (t * 12.0 + i as f32).sin() * 2.0;
            draw_line(0.0, y, sw, y, 1.0, Color::new(0.1, 0.0, 0.15, 0.25));
        }
    }

    fn draw_dead_neurons(&self, sw: f32, sh: f32, t: f32) {
        let base_x = sw * 0.12;
        let base_y = sh * 0.28;
        for i in 0..12 {
            let cx = base_x + (i % 4) as f32 * 70.0 + (t * 0.8 + i as f32).sin() * 6.0;
            let cy = base_y + (i / 4) as f32 * 80.0 + (t * 0.7 + i as f32).cos() * 6.0;
            let flicker = (t * 3.0 + i as f32).sin().abs();
            draw_circle(cx, cy, 6.0 + flicker * 1.5, Color::new(0.2, 0.2, 0.25, 0.5));

            if i % 2 == 0 && i + 1 < 12 {
                let nx = base_x + ((i + 1) % 4) as f32 * 70.0;
                let ny = base_y + ((i + 1) / 4) as f32 * 80.0;
                draw_line(cx, cy, nx, ny, 1.0, Color::new(0.2, 0.2, 0.3, 0.2));
            }
        }
    }

    fn draw_lhc(&self, sw: f32, sh: f32, t: f32, jitter: Vec2) {
        let center = vec2(sw * 0.68, sh * 0.55) + jitter;
        let radius = sh * 0.22;
        let segments = 72;

        for i in 0..segments {
            if i % 9 == 0 || i % 13 == 0 { continue; }
            let a0 = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let a1 = a0 + 0.08;
            let p0 = vec2(center.x + a0.cos() * radius, center.y + a0.sin() * radius);
            let p1 = vec2(center.x + a1.cos() * radius, center.y + a1.sin() * radius);
            draw_line(p0.x, p0.y, p1.x, p1.y, 3.0, Color::new(0.4, 0.0, 0.6, 0.7));
        }

        let inner = radius * 0.7;
        draw_circle_lines(center.x, center.y, inner, 2.0, Color::new(0.2, 0.0, 0.3, 0.6));

        for i in 0..6 {
            let phase = (t * 4.0 + i as f32 * 0.9).sin();
            if phase > 0.7 {
                let angle = (i as f32 * 1.2 + t) % std::f32::consts::TAU;
                let sx = center.x + angle.cos() * (radius - 10.0);
                let sy = center.y + angle.sin() * (radius - 10.0);
                draw_line(sx - 8.0, sy, sx + 8.0, sy, 2.0, Color::new(0.9, 0.4, 1.0, 0.6));
            }
        }

        let label = "LHC: DAMAGED";
        let lw = measure_text(label, None, 18, 1.0).width;
        draw_text(label, center.x - lw / 2.0, center.y + radius + 30.0, 18.0,
            Color::new(0.8, 0.2, 1.0, 0.8));
    }

    fn draw_glitch_entities(&self, sw: f32, sh: f32, t: f32) {
        let faces = GLITCH_FACES;
        let positions = [
            vec2(sw * 0.25, sh * 0.62),
            vec2(sw * 0.5, sh * 0.32),
            vec2(sw * 0.82, sh * 0.36),
        ];

        for (i, pos) in positions.iter().enumerate() {
            let jitter = vec2((t * 8.0 + i as f32).sin() * 2.0, (t * 7.0 + i as f32).cos() * 2.0);
            let face = faces[i % faces.len()];
            draw_text(face, pos.x + jitter.x, pos.y + jitter.y, 26.0,
                Color::new(0.8, 0.2, 1.0, 0.8));
        }

        let chatter = CHATTER_LINES[self.chatter_index];
        let cw = measure_text(chatter, None, 18, 1.0).width;
        let seed_bump = (self.glitch_seed as f32 * 0.000001).sin();
        let flicker = (t * 5.0 + seed_bump).sin().abs() * 0.5 + 0.5;
        draw_text(chatter, sw / 2.0 - cw / 2.0, sh * 0.78, 18.0,
            Color::new(0.7, 0.3, 1.0, 0.5 + 0.4 * flicker));
    }

    fn draw_jump_overlay(&self, sw: f32, sh: f32, t: f32) {
        if self.lhc_flash <= 0.0 && self.lhc_jump_timer <= 0.0 {
            return;
        }

        let flash = self.lhc_flash.min(1.0);
        if flash > 0.0 {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(1.0, 1.0, 1.0, 0.15 * flash));
        }

        if self.lhc_jump_timer > 0.0 {
            for i in 0..6 {
                let y = (t * 120.0 + i as f32 * 40.0) % sh;
                let h = 6.0 + (i as f32 * 0.6);
                draw_rectangle(0.0, y, sw, h, Color::new(0.8, 0.2, 1.0, 0.18));
            }

            let msg = "LHC ANOMALY DETECTED";
            let mw = measure_text(msg, None, 24, 1.0).width;
            draw_text(msg, sw / 2.0 - mw / 2.0, sh * 0.15, 24.0,
                Color::new(0.9, 0.2, 1.0, 0.9));
        }
    }

    fn trigger_lhc_jump(&mut self, audio: &AudioManager) {
        self.lhc_jump_timer = 0.45;
        self.lhc_flash = 1.0;
        self.lhc_glitch = 1.0;
        self.lhc_jump_cooldown = rand::gen_range(2.8, 6.0);

        let choices = [
            "glitch_distort",
            "glitch_portal",
            "glitch_shoots",
            "glitch_reverse",
            "glitch_reverse2",
            "glitch_super",
            "glitch_void",
            "collider",
        ];
        let idx = rand::gen_range(0, choices.len() as i32) as usize;
        audio.play_sound(choices[idx]);
    }

    fn draw_wip_panel(&self, sw: f32, sh: f32, t: f32) {
        let panel_w = 360.0;
        let panel_h = 180.0;
        let panel_x = sw - panel_w - 30.0;
        let panel_y = sh - panel_h - 40.0;

        let pulse = (t * 2.0).sin() * 0.1 + 0.9;
        draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::new(0.03, 0.0, 0.05, 0.9));
        draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 1.5, Color::new(0.6, 0.1, 0.8, 0.6 * pulse));

        let title = "LAB SYSTEMS (WIP)";
        draw_text(title, panel_x + 16.0, panel_y + 28.0, 18.0, Color::new(0.8, 0.3, 1.0, 0.85));

        let items = [
            "UPGRADE POWERUP (WIP)",
            "UNLOCK POWERUP (WIP)",
            "LHC MATTER SEARCH (WIP)",
        ];

        for (i, item) in items.iter().enumerate() {
            let y = panel_y + 60.0 + i as f32 * 34.0;
            draw_rectangle(panel_x + 12.0, y - 18.0, panel_w - 24.0, 28.0,
                Color::new(0.08, 0.0, 0.1, 0.6));
            draw_rectangle_lines(panel_x + 12.0, y - 18.0, panel_w - 24.0, 28.0, 1.0,
                Color::new(0.3, 0.1, 0.4, 0.6));
            draw_text(item, panel_x + 22.0, y, 16.0, Color::new(0.5, 0.3, 0.6, 0.8));
        }
    }

    fn draw_back_button(&self, t: f32) {
        let rect = self.back_button_rect();
        let pulse = (t * 2.2).sin() * 0.12 + 0.88;
        let color = if self.back_hover {
            Color::new(0.0, 0.9, 0.6, 0.9)
        } else {
            Color::new(0.0, 0.6, 0.4, 0.7)
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.1, 0.05, 0.8));
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.5, Color::new(color.r, color.g, color.b, pulse));

        let label = "BACK";
        let lw = measure_text(label, None, 20, 1.0).width;
        draw_text(label, rect.x + rect.w / 2.0 - lw / 2.0, rect.y + rect.h - 12.0, 20.0, color);
    }
}
