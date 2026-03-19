use macroquad::prelude::*;

use crate::systems::localization::I18nManager;
use crate::systems::save::SaveData;

use super::state::{NodeState, NodeVisual, COMING_SOON_DURATION, GLITCH_CHARSET};
use super::LevelSelection;

impl LevelSelection {
    pub fn draw(&self, save: Option<&SaveData>, i18n: &I18nManager) {
        let sw = screen_width();
        let sh = screen_height();
        let t = get_time() as f32;

        let max_unlocked = if let Some(s) = save {
            s.max_unlocked_level
        } else {
            self.level_manager.max_unlocked_level
        };

        self.draw_neural_bg(sw, sh, t);
        self.draw_top_bar(sw, sh, t, save, i18n);
        self.draw_void_portal(sw, sh, t, i18n);

        let title = &i18n.t("level_select_title");
        let tw = measure_text(title, None, 32, 1.0).width;
        draw_text(
            title,
            sw / 2.0 - tw / 2.0,
            sh * 0.17,
            32.0,
            Color::new(0.0, 1.0, 0.4, 0.9),
        );

        let act = &i18n.t("level_select_act");
        let aw = measure_text(act, None, 20, 1.0).width;
        draw_text(
            act,
            sw / 2.0 - aw / 2.0,
            sh * 0.20,
            20.0,
            Color::new(0.0, 0.7, 0.8, 0.75),
        );

        let status = &i18n.t("level_select_status");
        let swt = measure_text(status, None, 16, 1.0).width;
        draw_text(
            status,
            sw / 2.0 - swt / 2.0,
            sh * 0.225,
            16.0,
            Color::new(0.2, 0.9, 0.6, 0.7),
        );

        let (cell_w, cell_h, start_x, start_y) = self.grid_layout();
        let glitch_strength = (self.coming_soon_timer / COMING_SOON_DURATION).clamp(0.0, 1.0);
        let ambient = self.ambient_glitch_intensity;
        let glitch_offset = if glitch_strength > 0.0 {
            vec2(
                (t * 60.0).sin() * glitch_strength * 2.5,
                (t * 73.0).cos() * glitch_strength * 2.0,
            )
        } else if ambient > 0.0 {
            vec2(
                (t * 90.0).sin() * ambient * 2.0,
                (t * 83.0).cos() * ambient * 2.0,
            )
        } else {
            vec2(0.0, 0.0)
        };

        let mut nodes: Vec<NodeVisual> = Vec::with_capacity(25);
        for i in 0..25_u32 {
            let level_id = i + 1;
            let pos = self.node_center(i, cell_w, cell_h, start_x, start_y) + glitch_offset;
            let is_hover = self.hover_level == Some(level_id);
            let is_selected = self.selected_level == level_id;
            let is_playable = level_id <= Self::max_playable() && level_id <= max_unlocked;
            let is_coming = level_id > Self::max_playable();
            let state = if is_playable {
                NodeState::Playable
            } else if is_coming {
                NodeState::ComingSoon
            } else {
                NodeState::Locked
            };
            let stars = self.level_manager.get_level_stars(level_id);

            nodes.push(NodeVisual {
                level_id,
                pos,
                state,
                is_hover,
                is_selected,
                stars,
            });
        }

        self.draw_neural_links(&nodes, t);
        for node in &nodes {
            self.draw_neural_node(node, t, i18n);
        }

        if let Some(hov) = self.hover_level {
            let state = if hov <= Self::max_playable() && hov <= max_unlocked {
                NodeState::Playable
            } else if hov > Self::max_playable() {
                NodeState::ComingSoon
            } else {
                NodeState::Locked
            };
            self.draw_info_panel(hov, state, sw, sh, t, i18n);
        }

        self.draw_coming_soon_fx(t, glitch_offset, i18n);
        self.draw_ambient_glitch(sw, sh, t, i18n);

        let hint = &i18n.t("level_select_hint");
        let hw = measure_text(hint, None, 18, 1.0).width;
        draw_text(
            hint,
            sw / 2.0 - hw / 2.0,
            sh - 18.0,
            18.0,
            Color::new(0.0, 0.55, 0.0, 0.7),
        );
    }

    fn draw_void_portal(&self, sw: f32, sh: f32, t: f32, i18n: &I18nManager) {
        let rect = self.portal_rect(sw, sh);
        let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
        let radius = rect.w * 0.42;

        let hover = self.portal_hover;
        let pulse = (t * 3.0).sin() * 0.15 + 0.85;
        let base = if hover {
            Color::new(0.8, 0.2, 1.0, 0.9)
        } else {
            Color::new(0.6, 0.0, 0.8, 0.7)
        };

        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.02, 0.0, 0.04, 0.85),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.5,
            Color::new(base.r, base.g, base.b, 0.6),
        );

        for i in 0..3 {
            let r = radius * (0.55 + i as f32 * 0.18)
                + (t * (1.4 + i as f32 * 0.6)).sin() * 2.5;
            draw_circle_lines(
                center.x,
                center.y,
                r,
                2.0,
                Color::new(base.r, base.g, base.b, 0.5 * pulse),
            );
        }

        let segs = 48;
        for s in 0..segs {
            if s % 5 == 0 {
                continue;
            }
            let a0 = (s as f32 / segs as f32) * std::f32::consts::TAU + t * 0.7;
            let a1 = a0 + 0.12;
            let p0 = vec2(center.x + a0.cos() * radius, center.y + a0.sin() * radius);
            let p1 = vec2(center.x + a1.cos() * radius, center.y + a1.sin() * radius);
            draw_line(p0.x, p0.y, p1.x, p1.y, 2.0, Color::new(0.9, 0.3, 1.0, 0.45));
        }

        for i in 0..4 {
            let phase = (t * 4.0 + i as f32 * 1.3).sin();
            if phase > 0.85 {
                let glitch_x = rect.x + rect.w * 0.1 + rand::gen_range(0.0, rect.w * 0.8);
                let glitch_y = rect.y + rect.h * 0.1 + rand::gen_range(0.0, rect.h * 0.8);
                let glitch_w = rand::gen_range(20.0, 60.0);
                let glitch_h = rand::gen_range(2.0, 6.0);
                draw_rectangle(
                    glitch_x,
                    glitch_y,
                    glitch_w,
                    glitch_h,
                    Color::new(0.9, 0.4, 1.0, 0.4),
                );
            }
        }

        let label = i18n.t("level_select_portal");
        let lw = measure_text(&label, None, 16, 1.0).width;
        draw_text(
            &label,
            center.x - lw / 2.0,
            rect.y + rect.h + 18.0,
            16.0,
            Color::new(base.r, base.g, base.b, 0.7),
        );
    }

    fn draw_neural_bg(&self, sw: f32, sh: f32, t: f32) {
        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.02, 0.05, 1.0));

        for i in 0..40 {
            let x = (i as f32 * 90.0 + t * 10.0) % sw;
            let y = (i as f32 * 55.0 + t * 15.0) % sh;
            draw_circle(x, y, 1.5, Color::new(0.0, 0.4, 0.3, 0.25));
        }
    }

    fn draw_top_bar(&self, sw: f32, _sh: f32, t: f32, save: Option<&SaveData>, _i18n: &I18nManager) {
        let bar_h = 46.0;
        draw_rectangle(0.0, 0.0, sw, bar_h, Color::new(0.0, 0.04, 0.08, 0.9));
        let pulse = (t * 2.0).sin() * 0.5 + 0.5;
        draw_line(0.0, bar_h, sw, bar_h, 2.0, Color::new(0.0, 0.6, 0.5, 0.3 + 0.3 * pulse));

        if let Some(save) = save {
            let total_stars: u32 = save.level_stars.values().sum();
            let info = format!("Stars: {}   Unlocked: {}", total_stars, save.max_unlocked_level);
            draw_text(&info, 20.0, 30.0, 20.0, Color::new(0.0, 0.8, 0.6, 0.8));
        }
    }

    fn draw_neural_links(&self, nodes: &[NodeVisual], _t: f32) {
        for node in nodes {
            let next_id = node.level_id + 1;
            if let Some(next) = nodes.iter().find(|n| n.level_id == next_id) {
                draw_line(
                    node.pos.x,
                    node.pos.y,
                    next.pos.x,
                    next.pos.y,
                    1.0,
                    Color::new(0.0, 0.4, 0.3, 0.4),
                );
            }
        }
    }

    fn draw_neural_node(&self, node: &NodeVisual, t: f32, _i18n: &I18nManager) {
        let base_color = match node.state {
            NodeState::Playable => Color::new(0.0, 1.0, 0.5, 0.9),
            NodeState::Locked => Color::new(0.2, 0.25, 0.3, 0.6),
            NodeState::ComingSoon => Color::new(0.6, 0.2, 0.9, 0.6),
        };

        let radius = 18.0;
        let pulse = (t * 3.0 + node.level_id as f32 * 0.4).sin().abs() * 0.2 + 0.8;
        let outline = if node.is_hover || node.is_selected {
            Color::new(0.0, 1.0, 0.8, 0.9)
        } else {
            Color::new(base_color.r, base_color.g, base_color.b, 0.5)
        };

        draw_circle(node.pos.x, node.pos.y, radius + 4.0, Color::new(0.0, 0.1, 0.1, 0.6));
        draw_circle(node.pos.x, node.pos.y, radius * pulse, base_color);
        draw_circle_lines(node.pos.x, node.pos.y, radius + 2.0, 2.0, outline);

        let label = format!("{}", node.level_id);
        let lw = measure_text(&label, None, 20, 1.0).width;
        draw_text(&label, node.pos.x - lw / 2.0, node.pos.y + 6.0, 20.0, WHITE);

        if node.stars > 0 {
            for i in 0..node.stars {
                draw_text(
                    "*",
                    node.pos.x - 12.0 + i as f32 * 10.0,
                    node.pos.y + 24.0,
                    12.0,
                    YELLOW,
                );
            }
        }
    }

    fn draw_info_panel(
        &self,
        level_id: u32,
        state: NodeState,
        sw: f32,
        sh: f32,
        _t: f32,
        i18n: &I18nManager,
    ) {
        let panel_w = 230.0;
        let panel_h = 90.0;
        let x = sw * 0.08;
        let y = sh * 0.72;

        draw_rectangle(x, y, panel_w, panel_h, Color::new(0.0, 0.06, 0.08, 0.85));
        draw_rectangle_lines(x, y, panel_w, panel_h, 2.0, Color::new(0.0, 0.6, 0.5, 0.6));

        let level_name = self
            .level_manager
            .get_level(level_id)
            .map(|l| l.name.clone())
            .unwrap_or_default();

        let mut sector = i18n.t("level_select_sector");
        sector = sector.replace("{id}", &level_id.to_string());
        sector = sector.replace("{name}", &level_name);
        draw_text(&sector, x + 12.0, y + 26.0, 18.0, GREEN);

        let status = match state {
            NodeState::Playable => i18n.t("level_select_play"),
            NodeState::Locked => i18n.t("level_select_locked"),
            NodeState::ComingSoon => i18n.t("level_select_dev"),
        };
        draw_text(&status, x + 12.0, y + 52.0, 16.0, LIGHTGRAY);
    }

    fn draw_coming_soon_fx(&self, t: f32, jitter: Vec2, i18n: &I18nManager) {
        if self.coming_soon_timer <= 0.0 {
            return;
        }

        let strength = (self.coming_soon_timer / COMING_SOON_DURATION).clamp(0.0, 1.0);
        let pos = self.coming_soon_pos + jitter;
        let mut msg = i18n.t("level_select_coming");
        msg = msg.replace("{id}", &self.coming_soon_level.to_string());
        let w = measure_text(&msg, None, 20, 1.0).width;
        let flicker = (t * 20.0).sin().abs();
        draw_text(
            &msg,
            pos.x - w / 2.0,
            pos.y - 28.0,
            20.0,
            Color::new(0.9, 0.2, 1.0, 0.4 + 0.5 * strength * flicker),
        );
    }

    fn draw_ambient_glitch(&self, sw: f32, sh: f32, t: f32, _i18n: &I18nManager) {
        let ambient = self.ambient_glitch_intensity;
        if ambient <= 0.0 {
            return;
        }

        for i in 0..6 {
            let seed = ((t * 60.0) as u32).wrapping_add(i as u32 * 17);
            let idx = (seed as usize) % GLITCH_CHARSET.len();
            let ch = GLITCH_CHARSET[idx];
            let x = rand::gen_range(0.0, sw);
            let y = rand::gen_range(0.0, sh);
            let alpha = ambient * rand::gen_range(0.3, 0.7);
            draw_text(&ch.to_string(), x, y, 18.0, Color::new(0.6, 1.0, 0.9, alpha));
        }
    }
}
