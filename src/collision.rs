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

pub fn right_block_collision(
    placed: &Vec<Option<Block>>,
    tetromino: &Tetromino3,
    pos: &Vec2,
) -> bool {
    let points = tetromino.relative_points(pos);
    let mut collision = false;
    for p in points.iter() {
        if p.x < WELL_WIDTH as f32 && p.x + 1.0 < WELL_WIDTH as f32 {
            let right_idx = xy_idx(p.x + 1.0, p.y);
            match placed[right_idx] {
                Some(_) => {
                    collision = true;
                }
                _ => {}
            }
        }
    }

    collision
}

pub fn left_block_collision(
    placed: &Vec<Option<Block>>,
    tetromino: &Tetromino3,
    pos: &Vec2,
) -> bool {
    let points = tetromino.relative_points(pos);
    let mut collision = false;
    for p in points.iter() {
        if p.x > 0.0 && p.x - 1.0 > 0.0 {
            let left_idx = xy_idx(p.x - 1.0, p.y);
            match placed[left_idx] {
                Some(_) => {
                    collision = true;
                }
                _ => {}
            }
        }
    }

    collision
}

pub fn bottom_collision(tetromino: &Tetromino3, pos: &Vec2) -> bool {
    let points = tetromino.relative_points(pos);
    points.iter().any(|p| p.y == (WELL_HEIGHT - 1) as f32)
}

pub fn vertical_block_collision(
    placed: &Vec<Option<Block>>,
    tetromino: &Tetromino3,
    pos: &Vec2,
) -> bool {
    let points = tetromino.relative_points(pos);
    points.iter().any(|p| {
        let idx = xy_idx(p.x, p.y + 1.0);

        match placed[idx] {
            Some(_) => return true,
            _ => return false,
        }
    })
}

pub fn completed_lines(placed: &Vec<Option<Block>>) -> Vec<usize> {
    let mut completed = Vec::new();
    for y in 0..WELL_HEIGHT {
        let mut complete = true;
        for x in 0..WELL_WIDTH {
            let idx = xy_idx(x as f32, y as f32);
            match placed[idx] {
                Some(_) => continue,
                None => complete = false,
            }
        }
        if complete {
            completed.push(y);
        }
    }

    completed
}
