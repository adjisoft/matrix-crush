use macroquad::prelude::*;

pub const GEM_1: char = '@';
pub const GEM_2: char = '#';
pub const GEM_3: char = '%';
pub const GEM_4: char = '*';
pub const GEM_5: char = '$';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerType {
    XBomb,
    VSweep,
    GlitchBomb,
    AntimatterBomb,
    VoidBomb,
}

pub const BOMB_GEM: char = 'X';
pub const SWEEP_GEM: char = 'V';
pub const GLITCH_GEM: char = 'G';
pub const ANTIMATTER_GEM: char = 'A';
pub const VOID_GEM: char = 'O';

pub const GEM_CHARS: [char; 5] = [GEM_1, GEM_2, GEM_3, GEM_4, GEM_5];

pub fn get_gem_color(gem: char) -> Color {
    match gem {
        GEM_1 => Color::new(1.0, 0.3, 0.3, 1.0),
        GEM_2 => Color::new(0.3, 0.6, 1.0, 1.0),
        GEM_3 => Color::new(0.3, 1.0, 0.3, 1.0),
        GEM_4 => Color::new(1.0, 1.0, 0.3, 1.0),
        GEM_5 => Color::new(1.0, 0.5, 1.0, 1.0),
        BOMB_GEM => Color::new(1.0, 0.0, 0.0, 1.0),
        SWEEP_GEM => Color::new(0.0, 1.0, 1.0, 1.0),
        GLITCH_GEM => Color::new(0.0, 1.0, 0.5, 1.0),
        ANTIMATTER_GEM => Color::new(0.8, 0.0, 1.0, 1.0),
        VOID_GEM => Color::new(0.3, 0.0, 0.3, 1.0),
        _ => WHITE,
    }
}

pub fn get_power_type(gem: char) -> Option<PowerType> {
    match gem {
        BOMB_GEM => Some(PowerType::XBomb),
        SWEEP_GEM => Some(PowerType::VSweep),
        GLITCH_GEM => Some(PowerType::GlitchBomb),
        ANTIMATTER_GEM => Some(PowerType::AntimatterBomb),
        VOID_GEM => Some(PowerType::VoidBomb),
        _ => None,
    }
}

pub fn is_special_gem(gem: char) -> bool {
    get_power_type(gem).is_some()
}

pub fn create_special_gem(match_count: usize) -> Option<char> {
    if match_count >= 5 {
        Some(BOMB_GEM) // 5 match = bomb
    } else if match_count >= 4 {
        Some(SWEEP_GEM) // 4 match = sweep
    } else {
        None
    }
}
