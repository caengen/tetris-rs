use macroquad::{
    prelude::{
        const_mat3, const_mat4, debug, vec2, Color, Mat3, Mat4, Vec2, BLUE, GREEN, ORANGE, PURPLE,
        RED, SKYBLUE, YELLOW,
    },
    rand,
};

use crate::components::{GameState, Gravity, WELL_WIDTH};

use super::{xy_idx, Block, Tetromino, TetrominoType, WELL_HEIGHT};

const J: Mat3 = const_mat3!([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 0.0]);
const L: Mat3 = const_mat3!([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0, 1.0]);
const S: Mat3 = const_mat3!([0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 1.0]);
const T: Mat3 = const_mat3!([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0, 0.0]);
const Z: Mat3 = const_mat3!([0.0, 0.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 0.0]);
const I: Mat4 = const_mat4!(
    [0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0],
    [1.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.0]
);
const O: Mat4 = const_mat4!(
    [0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 1.0, 0.0],
    [0.0, 1.0, 1.0, 0.0]
);

pub fn tetromino_color(t: &TetrominoType) -> (Color, Color) {
    match t {
        TetrominoType::J => (
            Color::from_rgba(0, 0, 255, 255),
            Color::from_rgba(0, 0, 255, 50),
        ),
        TetrominoType::L => (
            Color::from_rgba(255, 165, 0, 255),
            Color::from_rgba(255, 165, 0, 50),
        ),
        TetrominoType::S => (
            Color::from_rgba(0, 255, 0, 255),
            Color::from_rgba(0, 255, 0, 50),
        ),
        TetrominoType::T => (
            Color::from_rgba(128, 0, 128, 255),
            Color::from_rgba(128, 0, 128, 50),
        ),
        TetrominoType::Z => (
            Color::from_rgba(255, 0, 0, 255),
            Color::from_rgba(255, 0, 0, 50),
        ),
        TetrominoType::I => (
            Color::from_rgba(0, 255, 255, 255),
            Color::from_rgba(0, 255, 255, 50),
        ),
        TetrominoType::O => (
            Color::from_rgba(255, 255, 0, 255),
            Color::from_rgba(255, 255, 0, 50),
        ),
    }
}

fn basic_mats() -> Vec<(TetrominoType, Mat3)> {
    return vec![
        (TetrominoType::J, J),
        (TetrominoType::L, L),
        (TetrominoType::S, S),
        (TetrominoType::T, T),
        (TetrominoType::Z, Z),
    ];
}

fn special_mats() -> Vec<(TetrominoType, Mat4)> {
    vec![(TetrominoType::I, I), (TetrominoType::O, O)]
}

pub fn tetromino_set() -> Vec<Tetromino> {
    let mut tetrominos = Vec::new();
    let mats = basic_mats();
    let mats2 = special_mats();
    for (t, mat) in mats.iter() {
        let width = 3;
        let pos = vec2(
            f32::floor(5.0 - width as f32 / 2.0),
            f32::floor(WELL_HEIGHT as f32 - width as f32 / 2.0) + 1.0,
        );
        let (color, ghost_color) = tetromino_color(t);
        tetrominos.push(Tetromino {
            pos,
            spawn_pos: pos,
            rot_index: 0,
            mat: *mat,
            mat4: Mat4::ZERO,
            width,
            kind: *t,
            color,
            ghost_color,
            lock_timer: 0.0,
            locking: false,
            sonic_lock: false,
            held: false,
        });
    }
    for (t, mat) in mats2.iter() {
        let width = 4;
        let pos = vec2(
            f32::floor(5.0 - width as f32 / 2.0),
            f32::floor(WELL_HEIGHT as f32 - width as f32 / 2.0) + 1.0,
        );
        let (color, ghost_color) = tetromino_color(t);
        tetrominos.push(Tetromino {
            pos,
            spawn_pos: pos,
            rot_index: 0,
            mat: Mat3::ZERO,
            mat4: *mat,
            width,
            kind: *t,
            color,
            ghost_color,
            lock_timer: 0.0,
            locking: false,
            sonic_lock: false,
            held: false,
        });
    }

    tetrominos
}

pub fn random_tetrominos(tetrominos: &Vec<Tetromino>, amount: usize) -> Vec<Tetromino> {
    let mut rand_tetrominos = Vec::new();
    let len = tetrominos.len();
    for _ in 0..amount {
        rand_tetrominos.push(tetrominos[rand::gen_range(0, len)].clone());
    }

    rand_tetrominos
}

fn random_tetromino(tetrominos: &Vec<Tetromino>) -> Tetromino {
    random_tetrominos(tetrominos, 1)[0].clone()
}

pub fn spawn_tetromino(tetrominos: &Vec<Tetromino>) -> Tetromino {
    let t = random_tetromino(tetrominos);

    t
}

pub fn despawn_blocks(placed: &mut Vec<Option<Block>>, lines: &Vec<usize>) {
    for y in lines.iter() {
        debug!("despawning line {}", y);
        for x in 0..WELL_WIDTH {
            let idx = xy_idx(x as f32, *y as f32);
            placed[idx] = None;
        }
    }
}

pub fn drain_next(gs: &mut GameState) -> Tetromino {
    let tetromino = gs.next.drain(0..1).collect::<Vec<Tetromino>>()[0];
    gs.next.push(spawn_tetromino(&gs.tetrominos));
    gs.ghost.dirty = true;
    gs.gravity.meter = 0.0;

    tetromino
}

pub fn reset_transform(tetromino: &mut Tetromino) {
    let pos = vec2(
        f32::floor(5.0 - tetromino.width as f32 / 2.0),
        f32::floor(WELL_HEIGHT as f32 - tetromino.width as f32 / 2.0) + 1.0,
    );
    match tetromino.kind {
        TetrominoType::I | TetrominoType::O => {
            let mats = special_mats();
            for m in mats.iter() {
                if tetromino.kind == m.0 {
                    tetromino.mat4 = m.1;
                    tetromino.rot_index = 0;
                    tetromino.pos = pos;
                    return;
                }
            }
        }
        _ => {
            let mats = basic_mats();
            for m in mats.iter() {
                if tetromino.kind == m.0 {
                    tetromino.mat = m.1;
                    tetromino.rot_index = 0;
                    tetromino.pos = pos;
                    return;
                }
            }
        }
    }
}
