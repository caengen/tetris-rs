use macroquad::prelude::{debug, get_time, vec2};

use super::{
    components::{WELL_HEIGHT, WELL_WIDTH},
    xy_idx, Block, GameState,
};

pub fn move_downwards(gs: &mut GameState) {
    let t = &gs.current;
    let new_pos = t.pos + vec2(0.0, -1.0);
    gs.last_update = get_time();

    gs.current.pos = new_pos;
    gs.gravity.meter = 0.0;
    if gs.current.locking {
        gs.current.lock_timer = 0.0;
    }
}

pub fn apply_gravity(placed: &mut Vec<Option<Block>>, removed_lines: &Vec<usize>) {
    let y_delta = removed_lines.len();
    debug!("removed lines {}", y_delta);
    let line = removed_lines.iter().min().unwrap();
    debug!("apply gravity above (below) line {}", line);

    for y in (0..*line).rev() {
        for x in 0..WELL_WIDTH {
            let idx = xy_idx(x as f32, y as f32);
            let new_idx = xy_idx(x as f32, (y + y_delta) as f32);
            match placed[idx] {
                Some(b) => {
                    debug!("Moving {}, {} to {}, {}", x, y, x, (y + y_delta));
                    placed[new_idx] = Some(b);
                    placed[idx] = None;
                }
                _ => continue,
            }
        }
    }
}
