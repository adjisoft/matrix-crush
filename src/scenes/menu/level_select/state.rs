use macroquad::prelude::*;

use crate::game::progression::LevelManager;
use crate::systems::save::SaveData;

pub(super) const GLITCH_CHARSET: &[char] = &[
    '0', '1', '#', '@', '!', '%', '/', '\\', '|', '-', '+', '=', '~',
];

pub(super) const COMING_SOON_DURATION: f32 = 0.8;

#[derive(Clone, Copy, Debug)]
pub enum LevelSelectEvent {
    Playable(u32),
    Locked(u32),
    ComingSoon(u32),
    Portal,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum NodeState {
    Playable,
    Locked,
    ComingSoon,
}

pub(super) struct NodeVisual {
    pub level_id: u32,
    pub pos: Vec2,
    pub state: NodeState,
    pub is_hover: bool,
    pub is_selected: bool,
    pub stars: u32,
}

pub struct LevelSelection {
    pub level_manager: LevelManager,
    pub scroll_offset: f32,
    pub selected_level: u32,
    pub hover_level: Option<u32>,
    pub(super) click_event: Option<LevelSelectEvent>,
    pub(super) portal_hover: bool,
    pub(super) ambient_glitch_timer: f32,
    pub(super) ambient_glitch_intensity: f32,
    pub(super) glitch_state: Vec<(usize, f32)>,
    pub(super) glitch_timer: f32,
    pub(super) coming_soon_timer: f32,
    pub(super) coming_soon_pos: Vec2,
    pub(super) coming_soon_level: u32,
    pub(super) coming_soon_seed: u32,
}

impl LevelSelection {
    pub fn new() -> Self {
        LevelSelection {
            level_manager: LevelManager::new(),
            scroll_offset: 0.0,
            selected_level: 1,
            hover_level: None,
            click_event: None,
            portal_hover: false,
            glitch_state: vec![(0, 0.0); 25],
            glitch_timer: 0.0,
            ambient_glitch_timer: 0.0,
            ambient_glitch_intensity: 0.0,
            coming_soon_timer: 0.0,
            coming_soon_pos: Vec2::ZERO,
            coming_soon_level: 0,
            coming_soon_seed: 1,
        }
    }

    pub(super) fn max_playable() -> u32 {
        5
    }

    pub fn take_event(&mut self) -> Option<LevelSelectEvent> {
        self.click_event.take()
    }

    pub fn update(&mut self, dt: f32, save: Option<&SaveData>) {
        let max_unlocked = if let Some(s) = save {
            s.max_unlocked_level
        } else {
            self.level_manager.max_unlocked_level
        };

        self.glitch_timer += dt;
        if self.ambient_glitch_timer > 0.0 {
            self.ambient_glitch_timer = (self.ambient_glitch_timer - dt).max(0.0);
        }
        if self.ambient_glitch_timer <= 0.0 {
            let pulse = rand::gen_range(0.0, 1.0);
            if pulse > 0.985 {
                self.ambient_glitch_timer = rand::gen_range(0.12, 0.3);
                self.ambient_glitch_intensity = rand::gen_range(0.2, 0.6);
            } else {
                self.ambient_glitch_intensity = 0.0;
            }
        }
        if self.coming_soon_timer > 0.0 {
            self.coming_soon_timer = (self.coming_soon_timer - dt).max(0.0);
        }

        for i in 5..25_usize {
            self.glitch_state[i].1 += dt;
            let rate = 0.08 + (i as f32 * 0.013) % 0.12;
            if self.glitch_state[i].1 > rate {
                self.glitch_state[i].1 = 0.0;
                let seed = ((get_time() as u32).wrapping_mul(1337))
                    .wrapping_add((i as u32).wrapping_mul(2654435761));
                self.glitch_state[i].0 = (seed as usize) % GLITCH_CHARSET.len();
            }
        }

        let (cell_w, cell_h, start_x, start_y) = self.grid_layout();
        let radius = self.node_radius(cell_w, cell_h);
        let (mouse_x, mouse_y) = mouse_position();
        let mouse = vec2(mouse_x, mouse_y);

        self.hover_level = None;
        self.portal_hover = false;

        let portal_rect = self.portal_rect(screen_width(), screen_height());
        if portal_rect.contains(mouse) {
            self.portal_hover = true;
            if is_mouse_button_pressed(MouseButton::Left) {
                self.click_event = Some(LevelSelectEvent::Portal);
                return;
            }
        }

        for i in 0..25_u32 {
            let level_id = i + 1;
            let pos = self.node_center(i, cell_w, cell_h, start_x, start_y);

            if mouse.distance(pos) <= radius {
                self.hover_level = Some(level_id);

                if is_mouse_button_pressed(MouseButton::Left) {
                    let playable = level_id <= Self::max_playable() && level_id <= max_unlocked;
                    let coming_soon = level_id > Self::max_playable();

                    if playable {
                        self.selected_level = level_id;
                        self.click_event = Some(LevelSelectEvent::Playable(level_id));
                    } else if coming_soon {
                        self.start_coming_soon_fx(level_id, pos);
                        self.click_event = Some(LevelSelectEvent::ComingSoon(level_id));
                    } else {
                        self.click_event = Some(LevelSelectEvent::Locked(level_id));
                    }
                }
                break;
            }
        }
    }

    fn start_coming_soon_fx(&mut self, level_id: u32, pos: Vec2) {
        self.coming_soon_timer = COMING_SOON_DURATION;
        self.coming_soon_pos = pos;
        self.coming_soon_level = level_id;
        self.coming_soon_seed = self
            .coming_soon_seed
            .wrapping_mul(1664525)
            .wrapping_add(1013904223);
    }
}
