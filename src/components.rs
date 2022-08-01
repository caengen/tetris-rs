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
pub const UPDATE_TIMEOUT: f64 = 0.5;

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
#[derive(Clone, Copy)]
pub struct Tetromino3 {
    pub pos: Vec2,
    pub rot_index: usize,
    pub mat: Mat3,
    pub width: i32,
    pub tetromino_type: TetrominoType,
    pub color: Color,
}

impl Tetromino3 {
    pub fn relative_points(self: &Self, pos: &Vec2) -> Vec<Vec2> {
        let mut points = Vec::new();
        let x = pos.x;
        let y = pos.y;
        for r in 0..3 {
            for c in 0..3 {
                if self.mat.row(r)[c] == 1.0 {
                    let dx = x + r as f32;
                    let dy = WELL_HEIGHT as f32 - (y + c as f32);
                    points.push(vec2(dx, dy))
                }
            }
        }

        points
    }
}

#[derive(Clone, Copy)]
pub struct Block {
    pub color: Color,
}

pub struct GameState {
    pub debug: bool,
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
    let mut current = spawner::spawn_tetromino(&tetrominos);
    GameState {
        debug: false,
        scl: 0.0,
        placed_blocks: vec![None; (WELL_WIDTH * WELL_HEIGHT) as usize],
        next,
        tetrominos,
        current,
        last_update: 0.0,
    }
}
