use crate::components::{ENTRY_DELAY, PIXELS_PER_UNIT};

use super::{
    Block, GameState, Score, Tetromino, TetrominoType, DARK, GAME_HEIGHT, GAME_WIDTH, LIGHT,
    WELL_CELL, WELL_CELL_GAP, WELL_HEIGHT, WELL_WIDTH,
};
use macroquad::prelude::{
    clear_background, draw_circle, draw_line, draw_rectangle, draw_rectangle_lines, draw_text,
    draw_texture_ex, measure_text, vec2, DrawTextureParams, Rect, Texture2D, Vec2, BLUE, GRAY,
    PINK, RED,
};

pub fn draw_well(offset: Vec2, scl: f32) {
    for ht in 0..WELL_HEIGHT {
        for wt in 0..WELL_WIDTH {
            draw_rectangle(
                (offset.x + wt as f32) * scl,
                (offset.y + ht as f32) * scl,
                (WELL_CELL - WELL_CELL_GAP) * scl,
                (WELL_CELL - WELL_CELL_GAP) * scl,
                DARK,
            );
        }
    }
}

pub fn draw_block(
    scl: f32,
    textures: &Texture2D,
    x: f32,
    y: f32,
    kind: TetrominoType,
    is_ghost: bool,
) {
    let mut atlas_x = match kind {
        TetrominoType::J => 0.0,
        TetrominoType::L => 16.0,
        TetrominoType::S => 32.0,
        TetrominoType::T => 48.0,
        TetrominoType::Z => 64.0,
        TetrominoType::I => 80.0,
        TetrominoType::O => 96.0,
    };
    if is_ghost {
        atlas_x = 112.0;
    }
    draw_texture_ex(
        *textures,
        x,
        y,
        LIGHT,
        DrawTextureParams {
            dest_size: Some(vec2(1.0 * scl, 1.0 * scl)),
            source: Some(Rect::new(atlas_x, 0.0, 16.0, 16.0)),
            ..Default::default()
        },
    );
}

pub fn draw_tetromino(
    textures: &Texture2D,
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

    match current.kind {
        TetrominoType::I | TetrominoType::O => {
            for r in 0..4 {
                for c in 0..4 {
                    let dx = x + r as f32;
                    let dy = WELL_HEIGHT as f32 - (y + c as f32);
                    if current.mat4.row(r)[c] == 1.0 && dx >= 0.0 && dy >= 0.0 {
                        draw_block(
                            scl,
                            textures,
                            (offset.x + dx as f32) * scl,
                            (offset.y + dy as f32) * scl,
                            current.kind,
                            ghost,
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
                        draw_block(
                            scl,
                            textures,
                            (offset.x + dx as f32) * scl,
                            (offset.y + dy as f32) * scl,
                            current.kind,
                            ghost,
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

fn draw_visual_only_tetromino(
    scl: f32,
    textures: &Texture2D,
    game_pos: &Vec2,
    tetromino: &Tetromino,
) {
    let points = tetromino.relative_points(game_pos);
    for p in points.iter() {
        draw_block(scl, textures, p.x * scl, p.y * scl, tetromino.kind, false);
    }
}

fn draw_hold(textures: &Texture2D, scl: f32, hold: &Option<Tetromino>) {
    let font_size = 1.5 * scl;
    let text = &"Hold".to_string();
    let text_measure = measure_text(text, None, font_size as _, 1.0);
    let x = (5.0 - text_measure.width / scl) * scl;
    draw_text("Hold", x, 2.0 * scl, font_size, LIGHT);
    let pos = vec2(5.0 - text_measure.width / scl, WELL_HEIGHT as f32 - 5.0);
    draw_border(textures, scl, vec2(2.0, 1.0), 4.0, 4.0);

    match hold {
        Some(hold) => {
            let offset = if hold.width == 4 {
                vec2(-0.5, 0.5)
            } else {
                vec2(0.0, -0.5)
            };
            draw_tetromino(textures, offset, scl, hold, &pos, false, &false);
        }
        _ => {}
    }
}

fn draw_next(textures: &Texture2D, scl: f32, next: &Vec<Tetromino>) {
    let font_size = 1.5 * scl;
    let text = &"Next".to_string();
    let text_measure = measure_text(text, None, font_size as _, 1.0);
    let x = GAME_WIDTH as f32 - 5.0 - text_measure.width / scl;
    draw_text("Next", x * scl, 2.0 * scl, font_size, LIGHT);

    for (i, t) in next.iter().enumerate() {
        let y_dis = GAME_HEIGHT as f32 - 13.0 - (3.0 * i as f32);
        let pos = vec2(x, y_dis);

        let offset = if t.width == 4 {
            vec2(-0.5, 0.5)
        } else {
            vec2(0.0, -0.5)
        };

        draw_tetromino(textures, offset, scl, t, &pos, false, &false);
        if i >= 3 {
            break;
        }
    }
    draw_border(textures, scl, vec2(GAME_WIDTH as f32 - 8.0, 1.0), 4.0, 13.0);
}

fn draw_placed(
    textures: &Texture2D,
    offset: Vec2,
    scl: f32,
    placed: &Vec<Option<Block>>,
    debug: &bool,
) {
    let w = (WELL_CELL - WELL_CELL_GAP) * scl;
    for (idx, block) in placed.iter().enumerate() {
        match block {
            Some(block) => {
                let color = if *debug { GRAY } else { block.color };
                let x = idx as usize % WELL_WIDTH;
                let y = idx as usize / WELL_WIDTH;

                draw_block(
                    scl,
                    textures,
                    (offset.x + x as f32) * scl,
                    (offset.y + y as f32) * scl,
                    block.kind,
                    false,
                );
            }
            _ => {}
        }
    }
}

fn draw_score(textures: &Texture2D, scl: f32, score: &Score) {
    let font_size = 2.0 * scl;

    let lines_txt = &format!("Lines {}", score.lines).to_string();
    let text_measure = measure_text(lines_txt, None, font_size as _, 1.0);
    let x = (GAME_WIDTH as f32 - 2.0 - text_measure.width / scl) * scl;
    let y_1 = (GAME_HEIGHT as f32 - 7.0) * scl;
    let y_2 = (GAME_HEIGHT as f32 - 9.0) * scl;
    let y_3 = (GAME_HEIGHT as f32 - 11.0) * scl;

    let level_txt = &format!("Level {}", score.level).to_string();
    let score_txt = &"Score".to_string();

    draw_text(level_txt, x, y_2, font_size, LIGHT);
    draw_text(lines_txt, x, y_3, font_size, LIGHT);
    draw_text(score_txt, x, y_1, font_size, LIGHT);
    draw_text(
        &format!("{}", score.val),
        x,
        y_1 + 1.5 * scl,
        font_size,
        LIGHT,
    );
    let b_pos = vec2((GAME_WIDTH as f32 - 8.5), 17.0);
    draw_border(textures, scl, b_pos, 7.0, 8.0);
    // if score.topout {
    //     draw_text("Game Over", ui_x + 40.0, 90.0, 1.25 * scl, LIGHT);
    // }
}

fn draw_border(textures: &Texture2D, scl: f32, pos: Vec2, w: f32, h: f32) {
    draw_rectangle_lines(
        (pos.x - 0.5) * scl,
        (pos.y - 0.5) * scl,
        (w + 1.0) * scl,
        (h + 1.0) * scl,
        1.0 * scl,
        LIGHT,
    );
    draw_texture_ex(
        *textures,
        (pos.x - 1.0) * scl,
        (pos.y - 1.0) * scl,
        LIGHT,
        DrawTextureParams {
            dest_size: Some(vec2(1.0 * scl, 1.0 * scl)),
            source: Some(Rect::new(0.0, 16.0, 16.0, 16.0)),
            ..Default::default()
        },
    );
    for i in 0..(w as i32) {
        draw_texture_ex(
            *textures,
            (pos.x + i as f32) * scl,
            (pos.y - 1.0) * scl,
            LIGHT,
            DrawTextureParams {
                dest_size: Some(vec2(1.0 * scl, 1.0 * scl)),
                source: Some(Rect::new(16.0, 16.0, 16.0, 16.0)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            *textures,
            (pos.x + i as f32) * scl,
            (pos.y + h) * scl,
            LIGHT,
            DrawTextureParams {
                dest_size: Some(vec2(1.0 * scl, 1.0 * scl)),
                source: Some(Rect::new(16.0, 16.0, 16.0, 16.0)),
                rotation: 180.0_f32.to_radians(),
                ..Default::default()
            },
        );
    }
    draw_texture_ex(
        *textures,
        (pos.x + w) * scl,
        (pos.y - 1.0) * scl,
        LIGHT,
        DrawTextureParams {
            dest_size: Some(vec2(1.0 * scl, 1.0 * scl)),
            source: Some(Rect::new(0.0, 16.0, 16.0, 16.0)),
            rotation: 90.0_f32.to_radians(),
            ..Default::default()
        },
    );
    for i in 0..(h as i32) {
        draw_texture_ex(
            *textures,
            (pos.x + w) * scl,
            (pos.y + i as f32) * scl,
            LIGHT,
            DrawTextureParams {
                dest_size: Some(vec2(1.0 * scl, 1.0 * scl)),
                source: Some(Rect::new(16.0, 16.0, 16.0, 16.0)),
                rotation: 90.0_f32.to_radians(),
                ..Default::default()
            },
        );
        draw_texture_ex(
            *textures,
            (pos.x - 1.0) * scl,
            (pos.y + i as f32) * scl,
            LIGHT,
            DrawTextureParams {
                dest_size: Some(vec2(1.0 * scl, 1.0 * scl)),
                source: Some(Rect::new(16.0, 16.0, 16.0, 16.0)),
                rotation: -90.0_f32.to_radians(),
                ..Default::default()
            },
        );
    }
    draw_texture_ex(
        *textures,
        (pos.x - 1.0) * scl,
        (pos.y + h) * scl,
        LIGHT,
        DrawTextureParams {
            dest_size: Some(vec2(1.0 * scl, 1.0 * scl)),
            source: Some(Rect::new(0.0, 16.0, 16.0, 16.0)),
            rotation: -90.0_f32.to_radians(),

            ..Default::default()
        },
    );
    draw_texture_ex(
        *textures,
        (pos.x + w) * scl,
        (pos.y + h) * scl,
        LIGHT,
        DrawTextureParams {
            dest_size: Some(vec2(1.0 * scl, 1.0 * scl)),
            source: Some(Rect::new(0.0, 16.0, 16.0, 16.0)),
            rotation: 180.0_f32.to_radians(),
            ..Default::default()
        },
    );
}

pub fn draw(gs: &GameState) {
    clear_background(DARK);

    let offset = vec2(
        GAME_WIDTH as f32 / 2.0 - WELL_WIDTH as f32 / 2.0,
        GAME_HEIGHT as f32 / 2.0 - WELL_HEIGHT as f32 / 2.0,
    );
    draw_well(offset, gs.scl);
    draw_border(
        &gs.textures,
        gs.scl,
        offset,
        WELL_WIDTH as f32,
        WELL_HEIGHT as f32,
    );
    draw_placed(&gs.textures, offset, gs.scl, &gs.placed_blocks, &gs.debug);

    draw_hold(&gs.textures, gs.scl, &gs.hold);
    draw_next(&gs.textures, gs.scl, &gs.next);

    draw_score(&gs.textures, gs.scl, &gs.score);
    let entered = gs.current.entry_timer >= ENTRY_DELAY;
    if entered {
        draw_tetromino(
            &gs.textures,
            offset,
            gs.scl,
            &gs.current,
            &gs.ghost.pos,
            true,
            &gs.debug,
        );
        draw_tetromino(
            &gs.textures,
            offset,
            gs.scl,
            &gs.current,
            &gs.current.pos,
            false,
            &gs.debug,
        );
    } else {
        draw_visual_only_tetromino(
            gs.scl,
            &gs.textures,
            &vec2(
                f32::floor(GAME_WIDTH as f32 / 2.0 - f32::floor(gs.current.width as f32 / 2.0)),
                19.0,
            ),
            &gs.current,
        );
    }

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
