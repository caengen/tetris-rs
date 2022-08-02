use macroquad::prelude::{const_vec2, debug, vec2, Mat3, Mat4, Vec2};

use crate::{collision::can_translate, rel_xy_idx, xy_idx};

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
        TetrominoType::I => {
            tetromino.mat4 = tetromino.mat4.transpose();
            tetromino.rot_index = (tetromino.rot_index + 1) % 4;
        }
        TetrominoType::O => {}
        _ => {
            let mut new_tetromino = tetromino.clone();
            new_tetromino.mat = mat3_clockwise_rot(&tetromino.mat);
            // test 1
            if can_translate(&new_tetromino, placed, &new_tetromino.pos) {
                tetromino.mat = new_tetromino.mat;
                tetromino.rot_index = (tetromino.rot_index + 1) % 4;
            } else {
                let res = mat3_super_kick(&new_tetromino, placed);
                match res {
                    Ok(new_pos) => {
                        tetromino.mat = new_tetromino.mat;
                        tetromino.pos = new_pos;
                        tetromino.rot_index = (tetromino.rot_index + 1) % 4;
                    }
                    Err(str) => {
                        debug!("{}", str);
                    }
                }
            }
        }
    }
}

const TRANSLATIONS: [Vec2; 16] = [
    const_vec2!([-1., 0.]),
    const_vec2!([-1., 1.]),
    const_vec2!([0., -2.]),
    const_vec2!([-1., -2.]),
    const_vec2!([1., 0.]),
    const_vec2!([1., -1.]),
    const_vec2!([0., 2.]),
    const_vec2!([1., 2.]),
    const_vec2!([1., 0.]),
    const_vec2!([1., 1.]),
    const_vec2!([0., -2.]),
    const_vec2!([1., -2.]),
    const_vec2!([-1., 0.]),
    const_vec2!([-1., -1.]),
    const_vec2!([0., 2.]),
    const_vec2!([-1., 2.]),
];
pub fn mat3_super_kick(
    tetromino: &Tetromino,
    placed: &Vec<Option<Block>>,
) -> Result<Vec2, &'static str> {
    for x in 0..4 {
        let idx = rel_xy_idx(x as f32, tetromino.rot_index as f32, 4.0);
        let new_pos = vec2(
            &tetromino.pos.x + TRANSLATIONS[idx].x,
            &tetromino.pos.y + TRANSLATIONS[idx].y,
        );
        if can_translate(tetromino, placed, &new_pos) {
            return Ok(new_pos);
        }
    }

    Err("Failed to kick")
}
