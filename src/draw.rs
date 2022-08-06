use macroquad::prelude::{
    clear_background, draw_circle, draw_line, draw_rectangle, draw_rectangle_lines, draw_text,
    vec2, Vec2, BLACK, BLUE, GRAY, GREEN, PINK, RED, WHITE,
};

use crate::components::{GAME_HEIGHT, GAME_WIDTH};

use super::{
    Block, GameState, Score, Tetromino, TetrominoType, WELL_CELL, WELL_CELL_GAP, WELL_HEIGHT,
    WELL_WIDTH,
};

pub fn draw_well(offset: Vec2, scl: f32) {
    for ht in 0..WELL_HEIGHT {
        for wt in 0..WELL_WIDTH {
            draw_rectangle(
                (offset.x + wt as f32) * scl,
                (offset.y + ht as f32) * scl,
                (WELL_CELL - WELL_CELL_GAP) * scl,
                (WELL_CELL - WELL_CELL_GAP) * scl,
                WHITE,
            );
        }
    }
}

pub fn draw_tetromino(
    offset: Vec2,
    scl: f32,
    current: &Tetromino,
    pos: &Vec2,
    ghost: bool,
    debug: &bool,
) {
    let x = pos.x;
    let y = pos.y;
    let w = (WELL_CELL - WELL_CELL_GAP) * scl;
    let color = if ghost {
        current.ghost_color
    } else {
        current.color
    };

    match current.kind {
        TetrominoType::I | TetrominoType::O => {
            for r in 0..4 {
                for c in 0..4 {
                    let dx = x + r as f32;
                    let dy = WELL_HEIGHT as f32 - (y + c as f32);
                    if current.mat4.row(r)[c] == 1.0 && dx >= 0.0 && dy >= 0.0 {
                        draw_rectangle(
                            (offset.x + dx as f32) * scl,
                            (offset.y + dy as f32) * scl,
                            w,
                            w,
                            color,
                        );
                    } else {
                        if *debug {
                            draw_rectangle(
                                (offset.x + dx as f32) * scl,
                                (offset.y + dy as f32) * scl,
                                w,
                                w,
                                PINK,
                            );
                        }
                    }
                }
            }
        }
        _ => {
            for r in 0..3 {
                for c in 0..3 {
                    let dx = x + r as f32;
                    let dy = WELL_HEIGHT as f32 - (y + c as f32);
                    if current.mat.row(r)[c] == 1.0 && dx >= 0.0 && dy >= 0.0 {
                        draw_rectangle(
                            (offset.x + dx as f32) * scl,
                            (offset.y + dy as f32) * scl,
                            w,
                            w,
                            color,
                        );
                    } else {
                        if *debug {
                            draw_rectangle(
                                (offset.x + dx as f32) * scl,
                                (offset.y + dy as f32) * scl,
                                w,
                                w,
                                PINK,
                            );
                        }
                    }
                }
            }
        }
    }

    if *debug {
        let points = current.relative_points(&current.pos);
        for p in points.iter() {
            draw_circle(
                (offset.x + p.x as f32) * scl,
                (offset.y + p.y as f32) * scl,
                0.2 * scl,
                RED,
            )
        }
    }
}

fn draw_hold(offset: Vec2, scl: f32, hold: &Option<Tetromino>) {
    match hold {
        Some(hold) => {
            let scl_delta = 2.0;
            let ui_x = WELL_WIDTH as f32 * (WELL_CELL - WELL_CELL_GAP) as f32 * scl + 20.0;

            draw_text("Hold", ui_x, 130.0, 1. * scl, WHITE);
            draw_rectangle_lines(ui_x - 10.0, 105.0, 80.0, 80.0, 4.0, WHITE);
            let y_dis = WELL_HEIGHT as f32 - 13.0;
            let pos = vec2((WELL_WIDTH as f32 + 1.0) * scl_delta, y_dis);
            draw_tetromino(offset, scl / scl_delta, hold, &pos, false, &false);
        }
        _ => {}
    }
}

fn draw_next(offset: Vec2, scl: f32, next: &Vec<Tetromino>) {
    let scl_delta = 2.0;
    let ui_x = WELL_WIDTH as f32 * (WELL_CELL - WELL_CELL_GAP) as f32 * scl + 20.0;

    draw_text("Next", ui_x, 230.0, 1. * scl, WHITE);
    draw_rectangle_lines(ui_x - 10.0, 205.0, 80.0, 360.0, 4.0, WHITE);
    for (i, t) in next.iter().enumerate() {
        let y_dis = WELL_HEIGHT as f32 - 20.0 - (4.0 * i as f32);
        let pos = vec2((WELL_WIDTH as f32 + 1.0) * scl_delta, y_dis);
        draw_tetromino(offset, scl / scl_delta, t, &pos, false, &false);
        if i >= 5 {
            break;
        }
    }
}

fn draw_placed(offset: Vec2, scl: f32, placed: &Vec<Option<Block>>, debug: &bool) {
    let w = (WELL_CELL - WELL_CELL_GAP) * scl;
    for (idx, block) in placed.iter().enumerate() {
        match block {
            Some(block) => {
                let color = if *debug { GRAY } else { block.color };
                let x = idx as usize % WELL_WIDTH;
                let y = idx as usize / WELL_WIDTH;

                draw_rectangle(
                    (offset.x + x as f32) * scl,
                    (offset.y + y as f32) * scl,
                    w,
                    w,
                    color,
                );
            }
            _ => {}
        }
    }
}

fn draw_score(scl: f32, score: &Score) {
    let level_txt = &format!("Level {}", score.level).to_string();
    let score_txt = &format!("Score {}", score.val).to_string();
    let lines_txt = &format!("Lines {}", score.lines).to_string();
    let ui_x = WELL_WIDTH as f32 * (WELL_CELL - WELL_CELL_GAP) as f32 * scl + 20.0;

    draw_text(level_txt, ui_x, 30.0, 1.25 * scl, WHITE);
    draw_text(score_txt, ui_x, 60.0, 1.25 * scl, WHITE);
    draw_text(lines_txt, ui_x, 90.0, 1.25 * scl, WHITE);

    if score.topout {
        draw_text("Game Over", ui_x + 40.0, 90.0, 1.25 * scl, WHITE);
    }
}

pub fn draw(gs: &GameState) {
    clear_background(BLACK);

    let offset = vec2(
        GAME_WIDTH as f32 / 2.0 - WELL_WIDTH as f32 / 2.0,
        GAME_HEIGHT as f32 / 2.0 - WELL_HEIGHT as f32 / 2.0,
    );
    draw_well(offset, gs.scl);
    draw_tetromino(offset, gs.scl, &gs.current, &gs.ghost.pos, true, &gs.debug);
    draw_tetromino(
        offset,
        gs.scl,
        &gs.current,
        &gs.current.pos,
        false,
        &gs.debug,
    );
    draw_placed(offset, gs.scl, &gs.placed_blocks, &gs.debug);
    // draw_hold(gs.scl, &gs.hold);
    // draw_next(gs.scl, &gs.next);
    // draw_score(gs.scl, &gs.score);

    if gs.debug {
        draw_text(
            format!("{} {}", gs.current.pos.x, gs.current.pos.y).as_str(),
            gs.current.pos.x + 20.0,
            gs.current.pos.y - WELL_HEIGHT as f32 - 20.0,
            1.25 * gs.scl,
            BLUE,
        );
    }
}
