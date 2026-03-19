// story/runner.rs
// Visual Novel-style dialogue runner for Matrix Crushed!

use crate::game::story::dialogue::*;
use macroquad::prelude::*;

const TYPEWRITER_SPEED: f32 = 40.0; // chars/sec
const GLITCH_CHARS: &[char] = &['#', '@', '!', '%', '&', '0', '1', '/', '\\', '|'];

pub struct StoryRunner {
    pub scene: &'static StoryScene,
    pub line_idx: usize,
    pub char_progress: f32,  // typewriter
    pub done: bool,
    pub chosen_route: Option<u8>,
    pub hover_choice: Option<usize>,
    pub glitch_offset: f32,
    pub glitch_seed: u32,
    pub flash_timer: f32,
    pub skip_held: f32,
    pub choice_phase: bool,  // showing choices (paused typewriter)
    pub beep_cooldown: f32,
    pub beep_pending: bool,
}

impl StoryRunner {
    pub fn new(scene: &'static StoryScene) -> Self {
        StoryRunner {
            scene,
            line_idx: 0,
            char_progress: 0.0,
            done: false,
            chosen_route: None,
            hover_choice: None,
            glitch_offset: 0.0,
            glitch_seed: 42,
            flash_timer: 0.0,
            skip_held: 0.0,
            choice_phase: false,
            beep_cooldown: 0.0,
            beep_pending: false,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn chosen_route(&self) -> Option<u8> {
        self.chosen_route
    }

    pub fn take_beep(&mut self) -> bool {
        let pending = self.beep_pending;
        self.beep_pending = false;
        pending
    }

    pub fn update(&mut self, dt: f32) {
        if self.done { return; }

        self.glitch_offset += dt * 8.0;
        self.flash_timer = (self.flash_timer + dt * 3.0) % (std::f32::consts::TAU);
        if self.beep_cooldown > 0.0 {
            self.beep_cooldown = (self.beep_cooldown - dt).max(0.0);
        }

        let lines = self.scene.lines;
        if self.line_idx >= lines.len() {
            // All lines done — show choice or finish
            if let Some(choice) = self.scene.choice {
                if self.chosen_route.is_none() {
                    self.choice_phase = true;

                    let (mx, my) = mouse_position();
                    let sw = screen_width();
                    let sh = screen_height();
                    let box_y = sh * 0.55;
                    let choice_start_y = box_y + 160.0;

                    self.hover_choice = None;
                    for (i, _opt) in choice.options.iter().enumerate() {
                        let cy = choice_start_y + i as f32 * 48.0;
                        let cx = sw / 2.0 - 180.0;
                        if mx >= cx && mx <= cx + 360.0 && my >= cy - 20.0 && my <= cy + 10.0 {
                            self.hover_choice = Some(i);
                        }
                    }

                    if is_mouse_button_pressed(MouseButton::Left) {
                        if let Some(hi) = self.hover_choice {
                            self.chosen_route = Some(choice.options[hi].route);
                            self.done = true;
                        }
                    }
                    // keyboard selection
                    if is_key_pressed(KeyCode::Key1) && choice.options.len() >= 1 {
                        self.chosen_route = Some(choice.options[0].route);
                        self.done = true;
                    }
                    if is_key_pressed(KeyCode::Key2) && choice.options.len() >= 2 {
                        self.chosen_route = Some(choice.options[1].route);
                        self.done = true;
                    }
                    if is_key_pressed(KeyCode::Key3) && choice.options.len() >= 3 {
                        self.chosen_route = Some(choice.options[2].route);
                        self.done = true;
                    }
                    return;
                }
            }
            self.done = true;
            return;
        }

        let current_line = &lines[self.line_idx];
        let total_chars = current_line.text.chars().count() as f32;
        let prev_progress = self.char_progress;

        // Progress typewriter
        let speed = TYPEWRITER_SPEED * (1.0 + current_line.glitch * 0.5);
        if self.char_progress < total_chars {
            self.char_progress += speed * dt;
        }

        if self.char_progress > prev_progress && self.beep_cooldown <= 0.0 {
            self.beep_pending = true;
            self.beep_cooldown = 0.04;
        }

        // Advance on Space / Enter / Click
        let advance = is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Enter)
            || is_mouse_button_pressed(MouseButton::Left);

        if advance {
            if self.char_progress < total_chars {
                // Skip to end of current line
                self.char_progress = total_chars;
            } else {
                // Advance to next line
                self.line_idx += 1;
                self.char_progress = 0.0;
            }
        }

        // Fast-skip on held Space
        if is_key_down(KeyCode::Space) {
            self.skip_held += dt;
            if self.skip_held > 0.5 {
                if self.char_progress < total_chars {
                    self.char_progress = total_chars;
                } else {
                    self.line_idx += 1;
                    self.char_progress = 0.0;
                    self.skip_held = 0.4;
                }
            }
        } else {
            self.skip_held = 0.0;
        }

        // Update glitch seed
        self.glitch_seed = self.glitch_seed.wrapping_mul(1664525).wrapping_add(1013904223);
    }

    pub fn draw(&self) {
        let sw = screen_width();
        let sh = screen_height();
        let t = get_time() as f32;

        // ── Backdrop ──────────────────────────────────────────────────────────
        let box_h = sh * 0.38;
        let box_y = sh - box_h - 20.0;
        let box_x = 30.0;
        let box_w = sw - 60.0;

        // Semi-transparent dark panel
        draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.0, 0.0, 0.0, 0.88));

        // Border glow — alternates colour based on speaker
        let lines = self.scene.lines;
        let line = if self.line_idx < lines.len() {
            Some(&lines[self.line_idx])
        } else {
            lines.last()
        };

        let (border_color, face_color) = if let Some(l) = line {
            match l.speaker {
                "SYSTEM" => (Color::new(0.0, 1.0, 0.1, 0.9), Color::new(0.0, 1.0, 0.1, 1.0)),
                "GLITCH" => (Color::new(0.8, 0.0, 1.0, 0.9), Color::new(0.8, 0.0, 1.0, 1.0)),
                "VOID"   => (Color::new(0.3, 0.3, 0.3, 0.9), Color::new(0.5, 0.5, 0.5, 1.0)),
                _        => (Color::new(0.0, 0.6, 1.0, 0.9), Color::new(0.0, 0.8, 1.0, 1.0)),
            }
        } else {
            (Color::new(0.0, 1.0, 0.1, 0.9), Color::new(0.0, 1.0, 0.1, 1.0))
        };

        // Animated border
        let pulse = (t * 2.5).sin() * 0.15 + 0.85;
        let border_c = Color::new(border_color.r, border_color.g, border_color.b, pulse);
        draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, border_c);

        // Scanlines overlay
        let mut scan_y = box_y;
        while scan_y < box_y + box_h {
            draw_line(box_x, scan_y, box_x + box_w, scan_y, 1.0,
                Color::new(0.0, 0.0, 0.0, 0.12));
            scan_y += 4.0;
        }

        if let Some(current_line) = line {
            let glitch = current_line.glitch;

            // ── Face box ──────────────────────────────────────────────────────
            let face_box_sz = 70.0;
            let face_box_x  = box_x + 20.0;
            let face_box_y  = box_y + 14.0;
            draw_rectangle(face_box_x, face_box_y, face_box_sz, face_box_sz,
                Color::new(0.0, 0.08, 0.0, 1.0));
            draw_rectangle_lines(face_box_x, face_box_y, face_box_sz, face_box_sz, 1.5, face_color);

            // Face with optional glitch displacement
            let face_dx = if glitch > 0.3 {
                ((self.glitch_seed % 7) as f32 - 3.0) * glitch * 2.0
            } else { 0.0 };
            draw_text(
                current_line.face,
                face_box_x + 4.0 + face_dx,
                face_box_y + face_box_sz / 2.0 + 8.0,
                26.0,
                face_color,
            );

            // ── Speaker name ──────────────────────────────────────────────────
            let name_x = face_box_x + face_box_sz + 16.0;
            let name_y = box_y + 36.0;
            draw_text(current_line.speaker, name_x, name_y, 24.0,
                Color::new(border_color.r, border_color.g, border_color.b, 1.0));

            // Separator line
            draw_line(name_x, name_y + 6.0, box_x + box_w - 20.0, name_y + 6.0, 1.0,
                Color::new(border_color.r, border_color.g, border_color.b, 0.4));

            // ── Typewriter text ───────────────────────────────────────────────
            let text_x = name_x;
            let text_y = name_y + 30.0;
            let max_w  = box_w - face_box_sz - 70.0;
            let total_chars_count = current_line.text.chars().count();
            let chars_to_show = (self.char_progress as usize).min(total_chars_count);
            let visible_text: String = current_line.text.chars().take(chars_to_show).collect();

            // Glitch effect: corrupt a char near the typewriter cursor
            if glitch > 0.3 && chars_to_show < total_chars_count {
                let seed = self.glitch_seed as usize;
                let mut chars: Vec<char> = visible_text.chars().collect();
                if !chars.is_empty() {
                    // corrupt_pos is always < chars.len() — safe
                    let corrupt_pos = chars.len() - 1;
                    let noise_char = GLITCH_CHARS[seed % GLITCH_CHARS.len()];
                    chars[corrupt_pos] = noise_char;
                }
                let display: String = chars.into_iter().collect();
                self.draw_wrapped_text(&display, text_x, text_y, max_w, 22.0,
                    Color::new(1.0, 0.4, 1.0, 0.9));
            } else {
                self.draw_wrapped_text(&visible_text, text_x, text_y, max_w, 22.0, WHITE);
            }


            // ── Cursor blink ──────────────────────────────────────────────────
            let cursor_vis = (t * 4.0).sin() > 0.0;
            let all_done = chars_to_show >= total_chars_count;
            if !all_done && cursor_vis {
                let cx = text_x + measure_text(&visible_text, None, 22, 1.0).width;
                draw_text("_", cx, text_y, 22.0, face_color);
            }

            // ── Progress indicator ────────────────────────────────────────────
            let total = self.scene.lines.len();
            let done  = self.line_idx.min(total);
            let prog_w = box_w - 40.0;
            let prog_y = box_y + box_h - 14.0;
            draw_rectangle(box_x + 20.0, prog_y, prog_w, 3.0,
                Color::new(0.1, 0.1, 0.1, 0.8));
            let fill = prog_w * (done as f32 / total as f32);
            draw_rectangle(box_x + 20.0, prog_y, fill, 3.0, border_c);

            // ── Hint (advance) ────────────────────────────────────────────────
            if all_done && !self.choice_phase {
                let hint = "[ SPACE / CLICK to continue ]";
                let hw = measure_text(hint, None, 16, 1.0).width;
                let alpha = self.flash_timer.sin().abs() * 0.6 + 0.4;
                draw_text(hint, box_x + box_w - hw - 16.0, box_y + box_h - 6.0,
                    16.0, Color::new(0.0, 0.8, 0.8, alpha));
            }
        }

        // ── Choice overlay ────────────────────────────────────────────────────
        if self.choice_phase {
            if let Some(choice) = self.scene.choice {
                let choice_box_x = box_x + 40.0;
                let choice_box_y = box_y + 100.0;
                let choice_box_w = box_w - 80.0;

                draw_text("CHOOSE YOUR PATH:", choice_box_x, choice_box_y, 22.0,
                    Color::new(0.0, 1.0, 0.8, 1.0));

                for (i, opt) in choice.options.iter().enumerate() {
                    let cy = choice_box_y + 28.0 + i as f32 * 48.0;
                    let is_hover = self.hover_choice == Some(i);

                    let bg_alpha = if is_hover { 0.35 } else { 0.15 };
                    draw_rectangle(choice_box_x, cy - 18.0, choice_box_w, 36.0,
                        Color::new(0.0, 0.3, 0.1, bg_alpha));

                    let border_a = if is_hover { 0.9 } else { 0.3 };
                    draw_rectangle_lines(choice_box_x, cy - 18.0, choice_box_w, 36.0, 1.5,
                        Color::new(0.0, 1.0, 0.3, border_a));

                    let prefix = format!("{}  {}", i + 1, opt.label);
                    let tc = if is_hover { Color::new(0.0, 1.0, 0.3, 1.0) }
                        else { Color::new(0.5, 0.9, 0.5, 0.9) };
                    draw_text(&prefix, choice_box_x + 14.0, cy + 6.0, 20.0, tc);
                }

                let key_hint = "[ 1 / 2 / 3 to choose ]";
                let kw = measure_text(key_hint, None, 16, 1.0).width;
                draw_text(key_hint,
                    box_x + box_w - kw - 16.0,
                    box_y + box_h - 6.0,
                    16.0,
                    Color::new(0.0, 0.7, 0.7, 0.7));
            }
        }
    }

    /// Word-wrapped draw_text helper
    fn draw_wrapped_text(&self, text: &str, x: f32, y: f32, max_w: f32, size: f32, color: Color) {
        let mut line_y = y;
        let mut line = String::new();
        for word in text.split_whitespace() {
            let test = if line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", line, word)
            };
            if measure_text(&test, None, size as u16, 1.0).width > max_w {
                draw_text(&line, x, line_y, size, color);
                line_y += size + 4.0;
                line = word.to_string();
            } else {
                line = test;
            }
        }
        if !line.is_empty() {
            draw_text(&line, x, line_y, size, color);
        }
    }
}
