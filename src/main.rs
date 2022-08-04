use collision::should_commit_tetromino;
use macroquad::prelude::*;
mod components;
use components::*;
mod draw;
mod spawner;
use draw::*;
mod input;
use input::*;
mod collision;
mod gravity_system;
mod srs;

pub fn xy_idx(x: f32, y: f32) -> usize {
    (y as usize * WELL_WIDTH as usize) + x as usize
}

pub fn rel_xy_idx(x: f32, y: f32, w: f32) -> usize {
    (y as usize * w as usize) + x as usize
}

fn update(gs: &mut GameState) {
    let time = get_time();
    let delta = get_frame_time();

    if gs.current.locking {
        gs.current.lock_timer += delta;
    }
    gs.gravity.meter += delta;

    let on_ground = should_commit_tetromino(&gs.current, &gs.current.pos, &gs.placed_blocks);
    if on_ground {
        gs.current.locking = true;
    }
    if on_ground && gs.current.locking && gs.current.lock_timer >= LOCK_DELAY {
        if gs.current.pos.cmpeq(gs.current.spawn_pos).all() {
            gs.score.topout = true;
            return;
        }
        let points = gs.current.relative_points(&gs.current.pos);

        for p in points.iter() {
            gs.placed_blocks[xy_idx(p.x, p.y)] = Some(Block {
                color: gs.current.color,
            });
        }

        gs.current = gs.next.drain(0..1).collect::<Vec<Tetromino>>()[0];
        gs.next.push(spawner::spawn_tetromino(&gs.tetrominos));
        gs.ghost.dirty = true;

        let completed_lines = collision::completed_lines(&gs.placed_blocks);
        if completed_lines.len() > 0 {
            spawner::despawn_blocks(&mut gs.placed_blocks, &completed_lines);
            gravity_system::apply_gravity(&mut gs.placed_blocks, &completed_lines);
            match completed_lines.len() {
                1 => gs.score.val += 1,
                2 => gs.score.val += 3,
                3 => gs.score.val += 5,
                4 => gs.score.val += 8,
                _ => {}
            }
        }
    }

    if gs.ghost.dirty {
        gs.ghost.dirty = false;
        let mut pos = gs.current.pos;
        while pos.y >= 0.0 as f32 {
            if should_commit_tetromino(&gs.current, &pos, &gs.placed_blocks) {
                break;
            }
            pos.y -= 1.0;
        }

        debug!("updating ghost {}", pos);
        gs.ghost.pos = pos;
    }

    // move downwards
    if gs.gravity.meter > gs.gravity.max {
        let t = &gs.current;
        let new_pos = t.pos + vec2(0.0, -1.0);
        gs.last_update = time;

        if !on_ground {
            gs.current.pos = new_pos;
            if gs.current.locking {
                gs.current.lock_timer = 0.0;
            }
        }

        gs.gravity.meter = 0.0;
    }
}

#[macroquad::main("tetris.rs")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut gs = get_game_state();

    loop {
        gs.scl = screen_height() / UNITS;

        if !gs.score.topout {
            input(&mut gs);
            update(&mut gs);
        }

        draw(&gs);

        next_frame().await
    }
}
