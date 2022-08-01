use macroquad::{
    prelude::{
        const_mat3, const_mat4, debug, vec2, Color, Mat3, Mat4, Vec2, BLUE, GREEN, ORANGE, PURPLE,
        RED, SKYBLUE, YELLOW,
    },
    rand,
};

use crate::components::WELL_WIDTH;

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

pub fn tetromino_color(t: &TetrominoType) -> Color {
    match t {
        TetrominoType::J => BLUE,
        TetrominoType::L => ORANGE,
        TetrominoType::S => GREEN,
        TetrominoType::T => PURPLE,
        TetrominoType::Z => RED,
        TetrominoType::I => SKYBLUE,
        TetrominoType::O => YELLOW,
    }
}

pub fn tetromino_set() -> Vec<Tetromino> {
    let mut tetrominos = Vec::new();
    let mats: Vec<(TetrominoType, Mat3)> = vec![
        (TetrominoType::J, J),
        (TetrominoType::L, L),
        (TetrominoType::S, S),
        (TetrominoType::T, T),
        (TetrominoType::Z, Z),
    ];
    let mats2: Vec<(TetrominoType, Mat4)> = vec![(TetrominoType::I, I), (TetrominoType::O, O)];

    for (t, mat) in mats.iter() {
        let width = 3;
        tetrominos.push(Tetromino {
            pos: vec2(f32::floor(5.0 - width as f32 / 2.0), 15.0),
            rot_index: 0,
            mat: *mat,
            mat4: Mat4::ZERO,
            width,
            kind: *t,
            color: tetromino_color(t),
        });
    }
    for (t, mat) in mats2.iter() {
        let width = 4;
        tetrominos.push(Tetromino {
            pos: vec2(f32::floor(5.0 - width as f32 / 2.0), 15.0),
            rot_index: 0,
            mat: Mat3::ZERO,
            mat4: *mat,
            width,
            kind: *t,
            color: tetromino_color(t),
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
