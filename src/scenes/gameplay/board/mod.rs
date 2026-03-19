mod state;

pub mod draw;
pub mod effects;
pub mod input;
pub mod logic;
pub mod ui;

pub use crate::game::board::{GRID_HEIGHT, GRID_WIDTH};
pub use state::{
    board_offset_x, board_offset_y, cell_size, dialog_message, dialog_variant_count, Board,
    DialogKind, CYAN, ANON_FACES,
};
