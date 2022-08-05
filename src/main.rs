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
use gravity_system::*;
use spawner::drain_next;
mod srs;

pub fn xy_idx(x: f32, y: f32) -> usize {
    (y as usize * WELL_WIDTH as usize) + x as usize
}

pub fn rel_xy_idx(x: f32, y: f32, w: f32) -> usize {
    (y as usize * w as usize) + x as usize
}

fn update_ghost(gs: &mut GameState) {
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

fn commit_tetromino(gs: &mut GameState) {
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

    gs.current = drain_next(gs);

    let completed_lines = collision::completed_lines(&gs.placed_blocks);
    if completed_lines.len() > 0 {
        spawner::despawn_blocks(&mut gs.placed_blocks, &completed_lines);
        apply_gravity(&mut gs.placed_blocks, &completed_lines);
        gs.score.lines += completed_lines.len();
        if gs.score.lines % 10 == 0 {
            gs.score.level = usize::min(gs.score.level + 1, 30);
        }
        match completed_lines.len() {
            1 => gs.score.val += 1 * WELL_WIDTH,
            2 => gs.score.val += 3 * WELL_WIDTH,
            3 => gs.score.val += 5 * WELL_WIDTH,
            4 => gs.score.val += 8 * WELL_WIDTH,
            _ => {}
        }
    }
}

fn update(gs: &mut GameState) {
    let delta = get_frame_time();
    gs.gravity.meter += 1.0;

    if gs.ghost.dirty {
        update_ghost(gs);
    }

    // only add if locked on previous frame
    if gs.current.locking {
        gs.current.lock_timer += delta;
    }

    let on_surface = should_commit_tetromino(&gs.current, &gs.current.pos, &gs.placed_blocks);
    if on_surface {
        gs.current.locking = true;
    }

    if on_surface
        && (gs.current.sonic_lock || (gs.current.locking && gs.current.lock_timer >= LOCK_DELAY))
    {
        commit_tetromino(gs);
    }

    if !on_surface && gs.gravity.meter >= gs.gravity.max {
        move_downwards(gs);
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
