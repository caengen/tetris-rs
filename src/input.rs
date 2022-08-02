use crate::components::AUTO_SHIFT_DELAY;

use super::{
    collision::can_translate, srs, Block, GameState, Tetromino, AUTO_SHIFT_TIMOUT, UPDATE_DELAY,
    WELL_WIDTH,
};
use macroquad::prelude::{get_time, is_key_down, is_key_pressed, is_key_released, vec2, KeyCode};

fn move_left(tetromino: &mut Tetromino, placed: &Vec<Option<Block>>) {
    let new_pos = vec2(tetromino.pos.x - 1.0, tetromino.pos.y);
    if can_translate(&tetromino, placed, &new_pos) {
        tetromino.pos = new_pos;
    }
}

fn move_right(tetromino: &mut Tetromino, placed: &Vec<Option<Block>>) {
    let new_pos = vec2(tetromino.pos.x + 1.0, tetromino.pos.y);
    if can_translate(&tetromino, placed, &new_pos) {
        tetromino.pos = new_pos;
    }
}

pub fn input(gs: &mut GameState) {
    let time = get_time();

    if is_key_released(KeyCode::Left) {
        gs.key_info.auto_shift = (None, time);
        gs.key_info.auto_shift_start = 0.;
    }
    if is_key_down(KeyCode::Left) {
        if gs.key_info.auto_shift_start == 0. {
            gs.key_info.auto_shift_start = time;
            move_left(&mut gs.current, &gs.placed_blocks);
            gs.key_info.auto_shift = (Some(KeyCode::Left), time);
        }

        if time - gs.key_info.auto_shift_start > AUTO_SHIFT_DELAY {
            let (key, last_move) = gs.key_info.auto_shift;
            match key {
                Some(k) => {
                    if k == KeyCode::Left && time - last_move > AUTO_SHIFT_TIMOUT {
                        move_left(&mut gs.current, &gs.placed_blocks);
                        gs.key_info.auto_shift.1 = time;
                    } else if k == KeyCode::Right {
                        move_left(&mut gs.current, &gs.placed_blocks);
                        gs.key_info.auto_shift = (Some(KeyCode::Left), time);
                    }
                }
                _ => {
                    move_left(&mut gs.current, &gs.placed_blocks);
                    gs.key_info.auto_shift = (Some(KeyCode::Left), time);
                }
            }
        }
    }

    if is_key_released(KeyCode::Right) {
        gs.key_info.auto_shift = (None, time);
        gs.key_info.auto_shift_start = 0.;
    }
    if is_key_down(KeyCode::Right) {
        if gs.key_info.auto_shift_start == 0. {
            gs.key_info.auto_shift_start = time;
            move_right(&mut gs.current, &gs.placed_blocks);
            gs.key_info.auto_shift = (Some(KeyCode::Right), time);
        }

        if time - gs.key_info.auto_shift_start < AUTO_SHIFT_DELAY {
            let (key, last_move) = gs.key_info.auto_shift;
            match key {
                Some(k) => {
                    if k == KeyCode::Right && time - last_move > AUTO_SHIFT_TIMOUT {
                        move_right(&mut gs.current, &gs.placed_blocks);
                        gs.key_info.auto_shift.1 = time;
                    } else if k == KeyCode::Left {
                        move_right(&mut gs.current, &gs.placed_blocks);
                        gs.key_info.auto_shift = (Some(KeyCode::Right), time);
                    }
                }
                _ => {
                    move_right(&mut gs.current, &gs.placed_blocks);
                    gs.key_info.auto_shift = (Some(KeyCode::Right), time);
                }
            }
        }
    }
    if is_key_pressed(KeyCode::Up) {
        srs::rotate(&mut gs.current, &gs.placed_blocks);
    }
    if is_key_down(KeyCode::Down) {
        if time - gs.last_update < (UPDATE_DELAY / 5.0) {
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
