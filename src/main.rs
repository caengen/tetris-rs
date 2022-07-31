use macroquad::prelude::*;
mod components;
use components::*;
mod draw;
mod spawner;
use draw::*;
mod input;
use input::*;
mod collision;

fn update(gs: &mut GameState) {
    let t = &gs.current;
    let new_pos = t.pos + vec2(0.0, -1.0);

    if !collision::well_collision(&gs.current, &new_pos) {
        let time = get_time();

        if time - gs.last_update < UPDATE_TIMEOUT {
            return;
        }
        gs.last_update = time;

        // gs.current.pos = new_pos;
    }
}

#[macroquad::main("tetris.rs")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut gs = get_game_state();

    loop {
        gs.scl = screen_height() / UNITS;

        input(&mut gs);
        update(&mut gs);
        draw(&gs);

        next_frame().await
    }
}
