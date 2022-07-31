use collision::bottom_collision;
use macroquad::prelude::*;
mod components;
use components::*;
mod draw;
mod spawner;
use draw::*;
mod input;
use input::*;
mod collision;

pub fn xy_idx(x: f32, y: f32) -> usize {
    (y as usize * WELL_WIDTH as usize) + x as usize
}

fn update(gs: &mut GameState) {
    if bottom_collision(&gs.current, &gs.current.pos) {
        debug!("ding ding");
        let points = gs.current.relative_points(&gs.current.pos);

        for p in points.iter() {
            gs.placed_blocks[xy_idx(p.x, p.y)] = Some(Block {
                color: gs.current.color,
            });
        }

        gs.current = gs.next.drain(0..1).collect::<Vec<Tetromino3>>()[0];
        gs.next.push(spawner::spawn_tetromino(&gs.tetrominos));
    }

    let t = &gs.current;
    let new_pos = t.pos + vec2(0.0, -1.0);

    if !collision::well_collision(&gs.current, &new_pos) {
        let time = get_time();

        if time - gs.last_update < UPDATE_TIMEOUT {
            return;
        }
        gs.last_update = time;

        gs.current.pos = new_pos;
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
