use super::spawner;
use macroquad::prelude::{color_u8, vec2, Color, KeyCode, Mat3, Mat4, Texture2D, Vec2};
use std::collections::HashMap;

//colors
pub const DARK: Color = color_u8!(49, 47, 40, 255);
pub const LIGHT: Color = color_u8!(218, 216, 209, 255);

const TETROMINO_SEED: u64 = 792164921846;

pub const SCREEN_WIDTH: i32 = 512;
pub const SCREEN_HEIGHT: i32 = 480;
pub const GAME_WIDTH: i32 = 32;
pub const GAME_HEIGHT: i32 = 30;
pub const PIXELS_PER_UNIT: i32 = 16;

pub const WELL_WIDTH: usize = 10;
pub const WELL_HEIGHT: usize = 22;

pub const WELL_CELL_GAP: f32 = 1.0 / PIXELS_PER_UNIT as f32;
pub const WELL_CELL: f32 = 1.0;

// timers in frames
pub const AUTO_SHIFT_TIMEOUT: f64 = 0.05;
pub const AUTO_SHIFT_DELAY: f64 = 0.3;
pub const LOCK_DELAY: f32 = 0.5;
pub const HARD_DROP_GRAVITY: f32 = 1.0;
pub const SOFT_DROP_GRAVITY: f32 = 5.0;
pub const ENTRY_DELAY: f32 = 20.0;
pub const SCORE_TIMEOUT: f32 = 60.0;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
    pub ghost_color: Color,
    pub lock_timer: f32,
    pub entry_timer: f32,
    pub locking: bool,
    pub sonic_lock: bool,
    pub held: bool,
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

pub struct Ghost {
    pub pos: Vec2,
    pub dirty: bool,
}

#[derive(Clone, Copy)]
pub struct Block {
    pub color: Color,
    pub kind: TetrominoType,
}

pub struct Score {
    pub level: usize,
    pub lines: usize,
    pub val: usize,
    pub topout: bool,
}

pub struct KeyInfo {
    pub auto_shift_start: f64,
    pub auto_shift: (Option<KeyCode>, f64),
}

pub struct Gravity {
    pub meter: f32,
    pub max: f32,
}

pub struct GameState {
    pub debug: bool,
    pub scl: f32,
    pub placed_blocks: Vec<Option<Block>>,
    pub next: Vec<Tetromino>,
    pub current: Tetromino,
    pub ghost: Ghost,
    pub tetrominos: Vec<Tetromino>,
    pub last_update: f64,
    pub score: Score,
    pub key_info: KeyInfo,
    pub gravity: Gravity,
    pub hold: Option<Tetromino>,
    pub textures: Texture2D,
    pub statistics: HashMap<TetrominoType, usize>,
}

pub fn get_level_gravity_max(level: usize) -> f32 {
    match level {
        0..=8 => 48.0 - (level as f32 * 5.0),
        9 => 6.0,
        10..=12 => 5.0,
        13..=15 => 4.0,
        16..=18 => 3.0,
        19..=28 => 2.0,
        29..=30 => 1.0,
        _ => 1.0,
    }
}

pub fn get_game_state() -> GameState {
    let tetrominos = spawner::tetromino_set();
    let next = spawner::random_tetrominos(&tetrominos, 10);
    let current = spawner::spawn_tetromino(&tetrominos);
    GameState {
        debug: false,
        scl: 0.0,
        placed_blocks: vec![None; (WELL_WIDTH * WELL_HEIGHT) as usize],
        next,
        tetrominos,
        current,
        ghost: Ghost {
            pos: current.pos,
            dirty: true,
        },
        last_update: 0.0,
        score: Score {
            level: 0,
            lines: 0,
            val: 0,
            topout: false,
        },
        key_info: KeyInfo {
            auto_shift_start: 0.0,
            auto_shift: (None, 0.0),
        },
        gravity: Gravity {
            meter: 0.0,
            max: get_level_gravity_max(0),
        },
        hold: None,
        textures: Texture2D::empty(),
        statistics: spawner::statistics(),
    }
}
