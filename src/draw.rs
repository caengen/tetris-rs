use macroquad::prelude::{clear_background, draw_rectangle, Vec2, BLACK, WHITE};

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

pub fn draw_current(scl: f32, current: &Tetromino3) {
    let x = current.pos.x;
    let y = current.pos.y;
    let w = (WELL_CELL - WELL_CELL_GAP) * scl;
    for r in 0..3 {
        for c in 0..3 {
            if current.mat.row(r)[c] == 1.0
                && (x as usize + r) < WELL_WIDTH
                && (y as usize + c) < WELL_HEIGHT
            {
                draw_rectangle(
                    (x as usize + r) as f32 * scl,
                    (y as usize + c) as f32 * scl,
                    w,
                    w,
                    current.color,
                );
            }
        }
    }
}

pub fn draw(gs: &GameState) {
    clear_background(BLACK);

    draw_well(gs.scl);
    draw_current(gs.scl, &gs.current);
}
