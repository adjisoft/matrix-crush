use macroquad::prelude::*;

pub const GEM_1: char = '@';
pub const GEM_2: char = '#';
pub const GEM_3: char = '%';
pub const GEM_4: char = '*';
pub const GEM_5: char = '$';
pub const BOMB_GEM: char = 'X';
pub const SWEEP_GEM: char = 'V';

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
        _ => WHITE,
    }
}

pub fn is_special_gem(gem: char) -> bool {
    gem == BOMB_GEM || gem == SWEEP_GEM
}

pub fn create_special_gem(match_count: usize) -> Option<char> {
    if match_count >= 5 {
        Some(BOMB_GEM) // + match = bomb
    } else if match_count >= 4 {
        Some(SWEEP_GEM) //4 match = sweep
    } else {
        None
    }
}
