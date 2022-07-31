use crate::xy_idx;

use super::{Block, WELL_HEIGHT};

use super::{Tetromino3, WELL_WIDTH};
use macroquad::prelude::{debug, Vec2};

pub fn well_collision(tetromino: &Tetromino3, pos: &Vec2) -> bool {
    let points = tetromino.relative_points(pos);

    for p in points.iter() {
        if p.x < 0.0 || p.x > ((WELL_WIDTH - 1) as f32) || p.y < 0.0 {
            return true;
        }
    }

    false
}

pub fn bottom_collision(tetromino: &Tetromino3, pos: &Vec2) -> bool {
    let points = tetromino.relative_points(pos);
    points.iter().any(|p| {
        debug!("{}", p.to_string());
        p.y == (WELL_HEIGHT - 1) as f32
    })
}

pub fn block_collision(placed: &Vec<Option<Block>>, tetromino: &Tetromino3, pos: &Vec2) -> bool {
    let points = tetromino.relative_points(pos);
    points.iter().any(|p| {
        let idx = xy_idx(p.x, p.y + 1.0);

        match placed[idx] {
            Some(_) => return true,
            _ => return false,
        }
    })
}
