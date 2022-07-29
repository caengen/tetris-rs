use macroquad::prelude::{clear_background, draw_rectangle, draw_text, Vec2, BLACK, BLUE, WHITE};

use crate::components::{GameState, Tetromino3, WELL_CELL, WELL_CELL_GAP, WELL_HEIGHT, WELL_WIDTH};

pub fn draw_well(scl: f32) {
    for ht in 0..WELL_HEIGHT {
        for wt in 0..WELL_WIDTH {
            draw_rectangle(
                wt as f32 * scl,
                ht as f32 * scl,
                (WELL_CELL - WELL_CELL_GAP) * scl,
                (WELL_CELL - WELL_CELL_GAP) * scl,
                WHITE,
            );
        }
    }
}

pub fn draw_tetromino(scl: f32, current: &Tetromino3) {
    let x = current.pos.x;
    let y = current.pos.y;
    let w = (WELL_CELL - WELL_CELL_GAP) * scl;
    for r in 0..3 {
        for c in 0..3 {
            let dx = x + r as f32;
            let dy = WELL_HEIGHT as f32 - (y + c as f32);
            if current.mat.row(r)[c] == 1.0 && dx >= 0.0 && dy >= 0.0 {
                draw_rectangle(dx as f32 * scl, dy as f32 * scl, w, w, current.color);
            }
        }
    }
}

pub fn draw(gs: &GameState) {
    clear_background(BLACK);

    draw_well(gs.scl);
    draw_tetromino(gs.scl, &gs.current);

    draw_text(
        format!("{} {}", gs.current.pos.x, gs.current.pos.y).as_str(),
        240.0,
        290.0,
        15.0,
        BLUE,
    );
}
