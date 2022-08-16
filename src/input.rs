use crate::{
    components::{
        get_game_state, get_level_gravity_max, GameMode, GameModeMenuSelection, Ghost,
        AUTO_SHIFT_DELAY, HARD_DROP_GRAVITY, LOCK_DELAY, SOFT_DROP_GRAVITY,
    },
    spawner::{drain_next, reset_transform},
};

use super::{
    collision::can_translate_horizontally, srs, Block, GameState, Tetromino, AUTO_SHIFT_TIMEOUT,
    WELL_WIDTH,
};
use macroquad::prelude::{get_time, is_key_down, is_key_pressed, is_key_released, vec2, KeyCode};

fn move_left(tetromino: &mut Tetromino, placed: &Vec<Option<Block>>, ghost: &mut Ghost) {
    let new_pos = vec2(tetromino.pos.x - 1.0, tetromino.pos.y);
    if can_translate_horizontally(&tetromino, placed, &new_pos) {
        tetromino.pos = new_pos;
        ghost.pos.x = new_pos.x;
        ghost.dirty = true;
        // if tetromino.locking {
        //     tetromino.lock_timer = 0.0;
        // }
    }
}

fn move_right(tetromino: &mut Tetromino, placed: &Vec<Option<Block>>, ghost: &mut Ghost) {
    let new_pos = vec2(tetromino.pos.x + 1.0, tetromino.pos.y);
    if can_translate_horizontally(&tetromino, placed, &new_pos) {
        tetromino.pos = new_pos;
        ghost.pos.x = new_pos.x;
        ghost.dirty = true;
        // if tetromino.locking {
        //     tetromino.lock_timer = 0.0;
        // }
    }
}

pub fn input(gs: &mut GameState) {
    match gs.game_mode {
        GameMode::Play => play_input(gs),
        GameMode::Pause => pause_input(gs),
        GameMode::Title | GameMode::GameTypeMenu => menu_input(gs),
        _ => {}
    }
}

fn menu_input(gs: &mut GameState) {
    match gs.game_mode {
        GameMode::Title => {
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                gs.game_mode = GameMode::GameTypeMenu;
            }
        }
        GameMode::GameTypeMenu => {
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Down) {
                if gs.menu_selection.sub_menu == GameModeMenuSelection::GameType {
                    gs.menu_selection.sub_menu = GameModeMenuSelection::MusicType;
                } else {
                    gs.menu_selection.sub_menu = GameModeMenuSelection::GameType;
                }
            }
            if is_key_pressed(KeyCode::Left) {
                match gs.menu_selection.sub_menu {
                    GameModeMenuSelection::GameType => {
                        gs.menu_selection.game_type =
                            usize::min(0, gs.menu_selection.game_type - 1);
                    }
                    GameModeMenuSelection::MusicType => {
                        gs.menu_selection.music_type =
                            usize::min(0, gs.menu_selection.music_type - 1);
                    }
                }
            }
            if is_key_pressed(KeyCode::Right) {
                match gs.menu_selection.sub_menu {
                    GameModeMenuSelection::GameType => {
                        gs.menu_selection.game_type =
                            usize::min(0, gs.menu_selection.game_type + 1);
                    }
                    GameModeMenuSelection::MusicType => {
                        gs.menu_selection.music_type =
                            usize::min(0, gs.menu_selection.music_type + 1);
                    }
                }
            }
            if is_key_pressed(KeyCode::Enter) {
                gs.game_mode = GameMode::Play;
            }
        }
        _ => {}
    }
}

fn pause_input(gs: &mut GameState) {
    if is_key_pressed(KeyCode::P) {
        gs.game_mode = GameMode::Play;
    }
}

pub fn play_input(gs: &mut GameState) {
    let time = get_time();

    if is_key_released(KeyCode::Left) {
        gs.key_info.auto_shift = (None, time);
        gs.key_info.auto_shift_start = 0.;
    }
    if is_key_down(KeyCode::Left) {
        if gs.key_info.auto_shift_start == 0. {
            gs.key_info.auto_shift_start = time;
            move_left(&mut gs.current, &gs.placed_blocks, &mut gs.ghost);
            gs.key_info.auto_shift = (Some(KeyCode::Left), time);
        }

        if time - gs.key_info.auto_shift_start > AUTO_SHIFT_DELAY {
            let (key, last_move) = gs.key_info.auto_shift;
            match key {
                Some(k) => {
                    if k == KeyCode::Left && time - last_move > AUTO_SHIFT_TIMEOUT {
                        move_left(&mut gs.current, &gs.placed_blocks, &mut gs.ghost);
                        gs.key_info.auto_shift.1 = time;
                    } else if k == KeyCode::Right {
                        move_left(&mut gs.current, &gs.placed_blocks, &mut gs.ghost);
                        gs.key_info.auto_shift = (Some(KeyCode::Left), time);
                    }
                }
                _ => {
                    move_left(&mut gs.current, &gs.placed_blocks, &mut gs.ghost);
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
            move_right(&mut gs.current, &gs.placed_blocks, &mut gs.ghost);
            gs.key_info.auto_shift = (Some(KeyCode::Right), time);
        }

        if time - gs.key_info.auto_shift_start > AUTO_SHIFT_DELAY {
            let (key, last_move) = gs.key_info.auto_shift;
            match key {
                Some(k) => {
                    if k == KeyCode::Right && time - last_move > AUTO_SHIFT_TIMEOUT {
                        move_right(&mut gs.current, &gs.placed_blocks, &mut gs.ghost);
                        gs.key_info.auto_shift.1 = time;
                    } else if k == KeyCode::Left {
                        move_right(&mut gs.current, &gs.placed_blocks, &mut gs.ghost);
                        gs.key_info.auto_shift = (Some(KeyCode::Right), time);
                    }
                }
                _ => {
                    move_right(&mut gs.current, &gs.placed_blocks, &mut gs.ghost);
                    gs.key_info.auto_shift = (Some(KeyCode::Right), time);
                }
            }
        }
    }
    if is_key_pressed(KeyCode::Up) {
        srs::rotate(&mut gs.current, &gs.placed_blocks, &mut gs.ghost);
    }
    if is_key_down(KeyCode::Down) {
        gs.gravity.max = SOFT_DROP_GRAVITY;
    }
    if is_key_released(KeyCode::Down) {
        gs.gravity.max = get_level_gravity_max(gs.score.level);
    }
    if is_key_pressed(KeyCode::Space) && !gs.ghost.dirty {
        gs.current.sonic_lock = true;
        gs.current.pos = gs.ghost.pos
    }
    if is_key_pressed(KeyCode::R) {
        let textures = gs.textures;
        let font = gs.font;
        *gs = get_game_state(GameMode::Play);
        gs.textures = textures;
        gs.font = font;
    }
    if is_key_pressed(KeyCode::C) {
        if gs.current.held {
            return;
        }

        match gs.hold {
            Some(hold) => {
                let mut temp = gs.current;
                gs.current = hold;
                gs.current.held = true;
                reset_transform(&mut temp);
                gs.hold = Some(temp);
            }
            None => {
                let mut hold = gs.current;
                reset_transform(&mut hold);
                gs.hold = Some(hold);
                gs.current = drain_next(gs);
                gs.current.held = true;
            }
        }
        gs.ghost.dirty = true;
    }
    if is_key_pressed(KeyCode::P) {
        gs.game_mode = GameMode::Pause;
    }
    if is_key_pressed(KeyCode::G) {
        gs.debug = !gs.debug;
    }
}
