use macroquad::prelude::{get_time, is_key_down, is_key_pressed, vec2, KeyCode};

use super::{collision::can_translate, srs, GameState, UPDATE_TIMEOUT, WELL_WIDTH};

pub fn input(gs: &mut GameState) {
    let time = get_time();
    if is_key_pressed(KeyCode::Left) {
        let new_pos = vec2(gs.current.pos.x - 1.0, gs.current.pos.y);
        if can_translate(&gs.current, &gs.placed_blocks, &new_pos) {
            gs.current.pos = new_pos;
        }
    }
    if is_key_pressed(KeyCode::Right) {
        let new_pos = vec2(gs.current.pos.x + 1.0, gs.current.pos.y);
        if can_translate(&gs.current, &gs.placed_blocks, &new_pos) {
            gs.current.pos = new_pos;
        }
    }
    if is_key_pressed(KeyCode::Up) {
        srs::rotate(&mut gs.current, &gs.placed_blocks);
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
