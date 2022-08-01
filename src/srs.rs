use macroquad::prelude::{Mat3, Mat4};

use super::{Block, Tetromino, TetrominoType};

// super rotation system
fn mat3_clockwise_rot(m_a: &Mat3) -> Mat3 {
    let col = m_a.to_cols_array();
    let m_b = Mat3::from_cols_array(&[
        col[2], col[5], col[8], col[1], col[4], col[7], col[0], col[3], col[6],
    ]);

    m_b
}

/*
fn mat3_counter_clockwise_rot(m_a: &Mat3) -> Mat3 {
    let col = m_a.to_cols_array();
    let m_b = Mat3::from_cols_array(&[
        col[6], col[3], col[0], col[7], col[4], col[1], col[8], col[5], col[2],
    ]);

    m_b
}
 */

fn mat4_clockwise_rot(m_a: &Mat4) -> Mat4 {
    let col = m_a.to_cols_array();
    let m_b = Mat4::from_cols_array(&[
        col[12], col[13], col[14], col[15], col[8], col[9], col[10], col[11], col[4], col[5],
        col[6], col[7], col[0], col[1], col[2], col[3],
    ]);

    m_b
}

pub fn rotate(tetromino: &mut Tetromino, placed: &Vec<Option<Block>>) {
    match tetromino.kind {
        TetrominoType::I => tetromino.mat4 = tetromino.mat4.transpose(),
        TetrominoType::O => {}
        _ => tetromino.mat = mat3_clockwise_rot(&tetromino.mat),
    }
}
