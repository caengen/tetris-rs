use macroquad::{
    prelude::{const_mat3, vec2, Color, Mat3, Vec2, BLUE, GREEN, ORANGE, PURPLE, RED},
    rand,
};

use crate::components::{TetrominoType, WELL_HEIGHT};

use super::{Tetromino3, Tetromino4};

// 3x3 tetrominos
const J: Mat3 = const_mat3!([1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0]);
const L: Mat3 = const_mat3!([0.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0]);
const S: Mat3 = const_mat3!([0.0, 1.0, 1.0], [1.0, 1.0, 0.0], [0.0, 0.0, 0.0]);
const T: Mat3 = const_mat3!([0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0]);
const Z: Mat3 = const_mat3!([1.0, 1.0, 0.0], [0.0, 1.0, 1.0], [0.0, 0.0, 0.0]);

pub fn tetromino_color(t: &TetrominoType) -> Color {
    match t {
        TetrominoType::J => BLUE,
        TetrominoType::L => ORANGE,
        TetrominoType::S => GREEN,
        TetrominoType::T => PURPLE,
        TetrominoType::Z => RED,
    }
}

pub fn tetromino_set() -> Vec<Tetromino3> {
    let mut tetrominos = Vec::new();
    let mats: Vec<(TetrominoType, Mat3)> = vec![
        (TetrominoType::J, J),
        (TetrominoType::L, L),
        (TetrominoType::S, S),
        (TetrominoType::T, T),
        (TetrominoType::Z, Z),
    ];
    // let types = vec![TetrominoType]

    for (t, mat) in mats.iter() {
        let width = 3;
        tetrominos.push(Tetromino3 {
            pos: vec2(f32::floor(5.0 - width as f32 / 2.0), 15.0),
            rot_index: 0,
            mat: *mat,
            width,
            tetromino_type: *t,
            color: tetromino_color(t),
        });
    }

    tetrominos
}

pub fn random_tetrominos(tetrominos: &Vec<Tetromino3>, amount: usize) -> Vec<Tetromino3> {
    let mut rand_tetrominos = Vec::new();
    let len = tetrominos.len();
    for i in 0..amount {
        rand_tetrominos.push(tetrominos[rand::gen_range(0, len)].clone());
    }

    rand_tetrominos
}

fn random_tetromino(tetrominos: &Vec<Tetromino3>) -> Tetromino3 {
    random_tetrominos(tetrominos, 1)[0].clone()
}

pub fn spawn_tetromino(tetrominos: &Vec<Tetromino3>) -> Tetromino3 {
    let mut t = random_tetromino(tetrominos);

    t
}
