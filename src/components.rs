use super::spawner;
use macroquad::{
    prelude::{vec2, Color, Mat3, Mat4, Vec2},
    rand::srand,
};
use std::slice::Iter;

const TETROMINO_SEED: u64 = 492750437;

pub const SCREEN_WIDTH: f32 = 300.0;
pub const SCREEN_HEIGHT: f32 = 1200.0;
pub const UNITS: f32 = 22.0; // scale 1:22

pub const WELL_WIDTH: usize = 10;
pub const WELL_HEIGHT: usize = 22;

pub const WELL_CELL_GAP: f32 = 0.05;
pub const WELL_CELL: f32 = 1.0;

// timers in seconds
pub const UPDATE_TIMER: f64 = 0.5;

#[derive(Copy, Clone)]
pub enum TetrominoType {
    J,
    L,
    S,
    T,
    Z,
    // O,
}

pub struct Tetromino4 {
    pub pos: Vec2,
    pub rot_index: usize,
    pub mat: Mat4,
}
#[derive(Clone)]
pub struct Tetromino3 {
    pub pos: Vec2,
    pub rot_index: usize,
    pub mat: Mat3,
    pub width: i32,
    pub tetromino_type: TetrominoType,
    pub color: Color,
}

#[derive(Clone)]
pub struct Block {
    pub color: Color,
}

pub struct GameState {
    pub scl: f32,
    pub placed_blocks: Vec<Option<Block>>,
    pub next: Vec<Tetromino3>,
    pub current: Tetromino3,
    pub tetrominos: Vec<Tetromino3>,
    pub last_update: f64,
}

pub fn get_game_state() -> GameState {
    let tetrominos = spawner::tetromino_set();
    srand(TETROMINO_SEED);
    let next = spawner::random_tetrominos(&tetrominos, 10);
    let mut current = spawner::random_tetromino(&tetrominos);
    current.pos = vec2(f32::floor((5 - current.width) as f32), 22.0);
    GameState {
        scl: 0.0,
        placed_blocks: vec![None; (WELL_WIDTH * WELL_HEIGHT) as usize],
        next,
        tetrominos,
        current,
        last_update: 0.0,
    }
}
