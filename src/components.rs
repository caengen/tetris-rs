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
    I,
    O,
}

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub pos: Vec2,
    pub spawn_pos: Vec2,
    pub rot_index: usize,
    pub mat: Mat3,
    pub mat4: Mat4,
    pub width: i32,
    pub kind: TetrominoType,
    pub color: Color,
}

impl Tetromino {
    pub fn relative_points(self: &Self, pos: &Vec2) -> Vec<Vec2> {
        let mut points = Vec::new();
        let x = pos.x;
        let y = pos.y;
        match self.kind {
            TetrominoType::I | TetrominoType::O => {
                for r in 0..4 {
                    for c in 0..4 {
                        if self.mat4.row(r)[c] == 1.0 {
                            let dx = x + r as f32;
                            let dy = WELL_HEIGHT as f32 - (y + c as f32);
                            points.push(vec2(dx, dy))
                        }
                    }
                }
            }
            _ => {
                for r in 0..3 {
                    for c in 0..3 {
                        if self.mat.row(r)[c] == 1.0 {
                            let dx = x + r as f32;
                            let dy = WELL_HEIGHT as f32 - (y + c as f32);
                            points.push(vec2(dx, dy))
                        }
                    }
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

pub struct Score {
    pub level: usize,
    pub val: usize,
    pub topout: bool,
}

pub struct GameState {
    pub debug: bool,
    pub scl: f32,
    pub placed_blocks: Vec<Option<Block>>,
    pub next: Vec<Tetromino>,
    pub current: Tetromino,
    pub tetrominos: Vec<Tetromino>,
    pub last_update: f64,
    pub score: Score,
}

pub fn get_game_state() -> GameState {
    let tetrominos = spawner::tetromino_set();
    srand(TETROMINO_SEED);
    let next = spawner::random_tetrominos(&tetrominos, 10);
    let current = spawner::spawn_tetromino(&tetrominos);
    GameState {
        debug: false,
        scl: 0.0,
        placed_blocks: vec![None; (WELL_WIDTH * WELL_HEIGHT) as usize],
        next,
        tetrominos,
        current,
        last_update: 0.0,
        score: Score {
            level: 1,
            val: 0,
            topout: false,
        },
    }
}
