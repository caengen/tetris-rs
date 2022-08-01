use macroquad::prelude::{
    get_time, is_key_down, is_key_pressed, mat3, vec2, vec3, KeyCode, Mat3, Quat,
};

use crate::collision::right_block_collision;

use super::{collision, GameState, UPDATE_TIMEOUT, WELL_WIDTH};

pub fn mat3_clockwise_rot(m_a: &Mat3) -> Mat3 {
    let col = m_a.to_cols_array();
    let m_b = Mat3::from_cols_array(&[
        col[2], col[5], col[8], col[1], col[4], col[7], col[0], col[3], col[6],
    ]);

    m_b
}
/*
pub fn mat3_counter_clockwise_rot(m_a: &Mat3) -> Mat3 {
    let col = m_a.to_cols_array();
    let m_b = Mat3::from_cols_array(&[
        col[6], col[3], col[0], col[7], col[4], col[1], col[8], col[5], col[2],
    ]);

    m_b
}
 */
pub fn input(gs: &mut GameState) {
    let time = get_time();
    if is_key_pressed(KeyCode::Left) {
        let new_pos = vec2(gs.current.pos.x - 1.0, gs.current.pos.y);
        if !collision::well_collision(&gs.current, &new_pos)
            && !collision::left_block_collision(&gs.placed_blocks, &gs.current, &gs.current.pos)
        {
            gs.current.pos = new_pos;
        }
    }
    if is_key_pressed(KeyCode::Right) {
        let new_pos = vec2(gs.current.pos.x + 1.0, gs.current.pos.y);
        if !collision::well_collision(&gs.current, &new_pos)
            && !collision::right_block_collision(&gs.placed_blocks, &gs.current, &gs.current.pos)
        {
            gs.current.pos = new_pos;
        }
    }
    if is_key_pressed(KeyCode::Up) {
        gs.current.mat = mat3_clockwise_rot(&gs.current.mat)
    }
    if is_key_down(KeyCode::Down) {
        if time - gs.last_update < (UPDATE_TIMEOUT / 5.0) {
            return;
        }
        gs.last_update = time;

        let t = &gs.current;
        let new_pos = t.pos + vec2(0.0, -1.0);
        gs.current.pos = new_pos;
    }
    if is_key_pressed(KeyCode::G) {
        gs.debug = !gs.debug;
    }
}
