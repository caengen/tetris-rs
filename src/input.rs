use macroquad::prelude::{get_last_key_pressed, mat3, vec3, KeyCode, Mat3, Quat};

use super::{GameState, WELL_WIDTH};

pub fn mat3_clockwise_rot(m_a: &Mat3) -> Mat3 {
    let col = m_a.to_cols_array();
    let m_b = Mat3::from_cols_array(&[
        col[2], col[5], col[8], col[1], col[4], col[7], col[0], col[3], col[6],
    ]);

    m_b
}
pub fn mat3_counter_clockwise_rot(m_a: &Mat3) -> Mat3 {
    let col = m_a.to_cols_array();
    let m_b = Mat3::from_cols_array(&[
        col[6], col[3], col[0], col[7], col[4], col[1], col[8], col[5], col[2],
    ]);

    m_b
}

pub fn input(gs: &mut GameState) {
    let key = get_last_key_pressed();
    let q = Quat::from_axis_angle(vec3(1.0, 0.0, 0.0), f32::to_radians(90.0));
    // const clockwise: Mat3 = Quat {

    // }
    //     [f32::cos(90.0), -f32::sin(90.0), 0.0],
    //     [f32::sin(90.0), f32::cos(90.0), 0.0],
    //     [0.0, 0.0, 1.0]
    // );

    match key {
        Some(KeyCode::Left) => {
            if gs.current.pos.x > 0.0 {
                gs.current.pos.x -= 1.0;
            }
        }
        Some(KeyCode::Right) => {
            if gs.current.pos.x < WELL_WIDTH as f32 {
                gs.current.pos.x += 1.0;
            }
        }
        Some(KeyCode::Up) => gs.current.mat = mat3_counter_clockwise_rot(&gs.current.mat),
        Some(KeyCode::Down) => gs.current.mat = mat3_clockwise_rot(&gs.current.mat),
        _ => {}
    }
}
