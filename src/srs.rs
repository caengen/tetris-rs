/*
 Super Rotation System (SRS)
 Tetris Guideline Standard Compliant
*/
use super::{collision::can_translate, rel_xy_idx, Block, Ghost, Tetromino, TetrominoType};
use macroquad::prelude::{const_vec2, debug, vec2, Mat3, Mat4, Vec2};

// geez these mat3s are ugly
fn mat3_clockwise_rot(mat: &Mat3) -> Mat3 {
    let col = mat.to_cols_array();
    Mat3::from_cols_array(&[
        col[2], col[5], col[8], col[1], col[4], col[7], col[0], col[3], col[6],
    ])
}

fn mat3_counter_clockwise_rot(m_a: &Mat3) -> Mat3 {
    let col = m_a.to_cols_array();
    let m_b = Mat3::from_cols_array(&[
        col[6], col[3], col[0], col[7], col[4], col[1], col[8], col[5], col[2],
    ]);

    m_b
}

// why did i use these crazy af structs for this...
fn mat4_clockwise_rot(mat: &Mat4) -> Mat4 {
    let col = mat.to_cols_array();
    Mat4::from_cols_array(&[
        col[3], col[7], col[11], col[15], col[2], col[6], col[10], col[14], col[1], col[5], col[9],
        col[13], col[0], col[4], col[8], col[12],
    ])
}

fn mat4_counter_clockwise_rot(mat: &Mat4) -> Mat4 {
    let col = mat.to_cols_array();
    Mat4::from_cols_array(&[
        col[12], col[8], col[4], col[0], col[13], col[9], col[5], col[1], col[14], col[10], col[6],
        col[2], col[15], col[11], col[7], col[3],
    ])
}

pub fn rotate(
    clockwise: bool,
    tetromino: &mut Tetromino,
    placed: &Vec<Option<Block>>,
    ghost: &mut Ghost,
) {
    match tetromino.kind {
        TetrominoType::I => {
            let mut new_tetromino = tetromino.clone();
            new_tetromino.mat4 = if clockwise {
                mat4_clockwise_rot(&tetromino.mat4)
            } else {
                mat4_counter_clockwise_rot(&tetromino.mat4)
            };
            if can_translate(&new_tetromino, placed, &new_tetromino.pos) {
                tetromino.mat4 = new_tetromino.mat4;
                tetromino.rot_index = (tetromino.rot_index + 1) % 4;
                ghost.dirty = true;
                if tetromino.locking {
                    tetromino.lock_counter = 0;
                }
            } else {
                let res = mat3_super_kick(&I_KICKS, &new_tetromino, placed);
                match res {
                    Ok(new_pos) => {
                        tetromino.mat4 = new_tetromino.mat4;
                        tetromino.pos = new_pos;
                        tetromino.rot_index = (tetromino.rot_index + 1) % 4;
                        ghost.dirty = true;
                        if tetromino.locking {
                            tetromino.lock_counter = 0;
                        }
                    }
                    Err(str) => {
                        debug!("{}", str);
                    }
                }
            }
        }
        TetrominoType::O => {}
        _ => {
            let mut new_tetromino = tetromino.clone();
            new_tetromino.mat = if clockwise {
                mat3_clockwise_rot(&tetromino.mat)
            } else {
                mat3_counter_clockwise_rot(&tetromino.mat)
            };
            // test 1
            if can_translate(&new_tetromino, placed, &new_tetromino.pos) {
                tetromino.mat = new_tetromino.mat;
                tetromino.rot_index = (tetromino.rot_index + 1) % 4;
                ghost.dirty = true;
                if tetromino.locking {
                    tetromino.lock_counter = 0;
                }
            } else {
                let res = mat3_super_kick(&KICKS, &new_tetromino, placed);
                match res {
                    Ok(new_pos) => {
                        tetromino.mat = new_tetromino.mat;
                        tetromino.pos = new_pos;
                        tetromino.rot_index = (tetromino.rot_index + 1) % 4;
                        ghost.dirty = true;
                        if tetromino.locking {
                            tetromino.lock_counter = 0;
                        }
                    }
                    Err(str) => {
                        debug!("{}", str);
                    }
                }
            }
        }
    }
}

const KICKS: [Vec2; 16] = [
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
const I_KICKS: [Vec2; 16] = [
    const_vec2!([-2., 0.]),
    const_vec2!([1., 0.]),
    const_vec2!([-2., -1.]),
    const_vec2!([1., 2.]),
    const_vec2!([-1., 0.]),
    const_vec2!([2., 0.]),
    const_vec2!([-1., 2.]),
    const_vec2!([2., -1.]),
    const_vec2!([2., 0.]),
    const_vec2!([-1., 0.]),
    const_vec2!([2., 1.]),
    const_vec2!([-1., -2.]),
    const_vec2!([1., 0.]),
    const_vec2!([-2., 0.]),
    const_vec2!([1., -2.]),
    const_vec2!([-2., 1.]),
];
pub fn mat3_super_kick(
    kicks: &[Vec2],
    tetromino: &Tetromino,
    placed: &Vec<Option<Block>>,
) -> Result<Vec2, &'static str> {
    for x in 0..4 {
        let idx = rel_xy_idx(x as f32, tetromino.rot_index as f32, 4.0);
        let new_pos = vec2(
            &tetromino.pos.x + kicks[idx].x,
            &tetromino.pos.y + kicks[idx].y,
        );
        if can_translate(tetromino, placed, &new_pos) {
            return Ok(new_pos);
        }
    }

    Err("Failed to kick")
}
