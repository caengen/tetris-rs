use collision::should_commit_tetromino;
use macroquad::{prelude::*, window};
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
    while pos.y >= 0.0 {
        if should_commit_tetromino(&gs.current, &pos, &gs.placed_blocks) {
            break;
        }
        pos.y -= 1.0;
    }

    gs.ghost.pos = pos;
}

fn calculate_score(gs: &mut GameState, completed_lines: &Vec<usize>, t_spin: bool) -> usize {
    let n = completed_lines.len();
    gs.score.lines += n;
    gs.score.level = gs.score.lines / 10;
    let mut score = match n {
        1 => 40 * (n + 1),
        2 => 100 * (n + 1),
        3 => 300 * (n + 1),
        // T-Spin 400 * (n + 1),
        4 => 1200 * (n + 1),
        _ => 0,
    };

    if t_spin {
        score += 400 * (n + 1);
    }

    score
}

fn has_block(placed: &Vec<Option<Block>>, x: f32, y: f32) -> bool {
    let idx = xy_idx(x, y);
    let block = placed[idx];
    match block {
        Some(_) => true,
        _ => false,
    }
}

fn commit_tetromino(gs: &mut GameState) {
    if gs.current.pos.cmpeq(gs.current.spawn_pos).all() {
        gs.score.topout = true;
        return;
    }

    // first we place all the blocks on the board
    let points = gs.current.relative_points(&gs.current.pos);
    for p in points.iter() {
        gs.placed_blocks[xy_idx(p.x, p.y)] = Some(Block {
            // color: gs.current.color,
            color: LIGHT,
            kind: gs.current.kind,
        });
    }

    let t_spin_occured = match (&gs.current.kind, &gs.last_input) {
        (TetrominoType::T, FrameInput::Rotate) => {
            // Each t-block rotation has a different range of bounds that we need to check
            // visualise the "keyhole" in blocks that a t-block can fill
            let t_spin_ranges: Vec<(Vec2, Vec2)> = vec![
                (vec2(-1., -1.), vec2(1., 0.)),
                (vec2(0., -1.), vec2(1., 1.)),
                (vec2(-1., 0.), vec2(1., 1.)),
                (vec2(-1., -1.), vec2(1., 0.)),
            ];
            // get the bounds for the current t-block rotation
            let bounds = t_spin_ranges[gs.current.rot_index];

            // For each point in the t-block we can simply check if all orthaognal neighbours contains a block
            // as long as the point is within the bounds of the current rotation. Because all points are already committed
            // some of these neighbours will be from the current t-block, which we allow because it simplifies the logic.
            let nwse = vec![vec2(0., -1.), vec2(-1., 0.), vec2(0., 1.), vec2(1., 0.)];
            let t_spin_occured = points.iter().all(|p| {
                nwse.iter().all(|dp| {
                    let x = p.x + dp.x;
                    let y = p.y + dp.y;
                    let blx = p.x + bounds.0.x;
                    let bly = p.y + bounds.0.y;
                    let bhx = p.x + bounds.1.x;
                    let bhy = p.y + bounds.1.y;

                    // the point is not necessary when checking if the
                    // t-block has filled a "hole" in the board
                    if x < blx || x > bhx || y < bly || y > bhy {
                        return true;
                    }

                    has_block(&gs.placed_blocks, x, y)
                })
            });

            t_spin_occured
        }
        _ => false,
    };

    let stat = gs.statistics.get_mut(&gs.current.kind);
    match stat {
        Some(stat_val) => *stat_val += 1,
        _ => {}
    }

    gs.current = drain_next(gs);
    let completed_lines = collision::completed_lines(&gs.placed_blocks);
    if completed_lines.len() > 0 {
        let score = calculate_score(gs, &completed_lines, t_spin_occured);
        gs.score.val += score;
        gs.last_score = ScorePopup {
            val: score,
            creation: 0,
        };

        gs.line_clear = Some(LineClear {
            y_pos: *completed_lines.iter().min().unwrap(),
            lines: completed_lines.clone(),
            counter: 0,
        });
    }
}

fn remove_lines(placed_blocks: &mut Vec<Option<Block>>, completed_lines: &Vec<usize>) {
    spawner::despawn_blocks(placed_blocks, &completed_lines);
    apply_gravity(placed_blocks, &completed_lines);
}

fn update(gs: &mut GameState) {
    match gs.game_mode {
        GameMode::Play => play_update(gs),
        _ => {}
    }
}

fn play_update(gs: &mut GameState) {
    gs.gravity.meter += 1.0;
    if gs.current.entry_timer < ENTRY_DELAY {
        gs.current.entry_timer += 1;
    }
    if gs.last_score.val > 0 && gs.last_score.creation < SCORE_TIMEOUT {
        gs.last_score.creation += 1;
    }
    match &mut gs.line_clear {
        Some(line_clear) => {
            line_clear.counter += 1;
            if line_clear.counter >= LINE_CLEAR_DELAY {
                remove_lines(&mut gs.placed_blocks, &line_clear.lines);
                gs.line_clear = None;
                gs.ghost.dirty = true;
            }
        }
        None => {}
    }

    // only add if locked on previous frame
    if gs.current.locking {
        gs.current.lock_counter += 1;
        debug!("gs.current.lock_counter={}", gs.current.lock_counter);
    }

    if gs.ghost.dirty {
        update_ghost(gs);
    }

    let on_surface = should_commit_tetromino(&gs.current, &gs.current.pos, &gs.placed_blocks);
    if on_surface {
        gs.current.locking = true;
    }

    if on_surface
        && (gs.current.sonic_lock || (gs.current.locking && gs.current.lock_counter >= LOCK_DELAY))
    {
        debug!("commiting");
        debug!("on_surface={}", on_surface);
        debug!("gs.current.locking={}", gs.current.locking);
        debug!("gs.current.lock_counter={}", gs.current.lock_counter);
        debug!("LOCK_DELAY={}", LOCK_DELAY);
        commit_tetromino(gs);
    }

    if !on_surface && gs.gravity.meter >= gs.gravity.max && gs.current.entry_timer >= ENTRY_DELAY {
        move_downwards(gs);
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Tetris.rs".to_owned(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    let mut gs = get_game_state(GameMode::Title);
    let blocks_texture: Texture2D = load_texture("assets/blocks.png").await.unwrap();
    let font = load_ttf_font("assets/visitor.ttf").await.unwrap();
    gs.textures = blocks_texture;
    gs.font = font;

    loop {
        gs.scl = screen_width() / GAME_WIDTH;

        if !gs.score.topout {
            input(&mut gs);
            update(&mut gs);
        }

        draw(&gs);

        next_frame().await
    }
}
