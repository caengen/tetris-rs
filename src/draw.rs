use std::collections::HashMap;

use crate::components::{
    GameMode, ScorePopup, ENTRY_DELAY, LINE_CLEAR_DELAY, PIXELS_PER_UNIT, SCORE_TIMEOUT,
};

use super::{
    Block, GameState, Score, Tetromino, TetrominoType, DARK, GAME_HEIGHT, GAME_WIDTH, LIGHT,
    WELL_CELL, WELL_CELL_GAP, WELL_HEIGHT, WELL_WIDTH,
};
use macroquad::{
    prelude::{
        clear_background, debug, draw_circle, draw_line, draw_rectangle, draw_rectangle_lines,
        draw_text, draw_texture_ex, get_fps, measure_text, vec2, DrawTextureParams, Rect,
        Texture2D, Vec2, BLUE, GRAY, PINK, RED,
    },
    text::{draw_text_ex, Font, TextDimensions, TextParams},
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

fn draw_hold(textures: &Texture2D, params: &TextParamsConfig, scl: f32, hold: &Option<Tetromino>) {
    let text = &"HOLD".to_string();
    let (text_params, dims) = params.params_and_dims(text, 1.5);

    let x = (6.0 - dims.width / scl) * scl;
    draw_text_ex(text, x, 2.0 * scl, text_params);
    draw_border(textures, scl, vec2(2.0, 1.0), 4.0, 4.0);

    match hold {
        Some(hold) => {
            draw_visual_only_tetromino(
                scl,
                textures,
                &vec2(
                    4.0 - hold.width as f32 / 2.0,
                    19.0 - f32::floor(hold.width as f32 / 2.0) - 0.5,
                ),
                hold,
            );
        }
        _ => {}
    }
}

fn draw_statistics(
    textures: &Texture2D,
    text_config: &TextParamsConfig,
    scl: f32,
    tetrominos: &Vec<Tetromino>,
    statistics: &HashMap<TetrominoType, usize>,
) {
    let text = &"STATS".to_string();
    let (params, dims) = text_config.params_and_dims(text, 1.5);
    let x = 6.0 - dims.width / scl;
    let y = 10.0;

    draw_text_ex(text, (x + 1.0) * scl, y * scl, params);
    draw_border(textures, scl, vec2(x, y - 1.0), 7.0, 17.0);

    for (i, t) in tetrominos.iter().enumerate() {
        let stat = statistics.get(&t.kind);
        let ty = y as i32 - 3 - (3 * i as i32);
        draw_visual_only_tetromino(scl / 1.25, textures, &vec2(2.5, ty as f32), t);
        match stat {
            Some(stat) => {
                let stat_text = &format!("{:0>3}", stat).to_string();
                let stat_measure =
                    measure_text(&stat_text, Some(params.font), (1.5 * scl) as u16, 1.0);

                draw_text_ex(
                    stat_text,
                    (x + 6.75) * scl - stat_measure.width,
                    (y as f32 + 1.875 + (2.25 * i as f32)) as f32 * scl,
                    params,
                );
            }
            _ => {}
        }
    }
}

fn draw_next(
    textures: &Texture2D,
    text_config: &TextParamsConfig,
    scl: f32,
    next: &Vec<Tetromino>,
) {
    let text = &"NEXT".to_string();
    let (text_params, dims) = text_config.params_and_dims(text, 1.5);

    let x = GAME_WIDTH - 5.5 - dims.width / 2.0 / scl;
    draw_text_ex(text, x * scl, 2.0 * scl, text_params);

    for (i, t) in next.iter().enumerate() {
        let y_dis = GAME_HEIGHT as f32 - 13.0 - (3.0 * i as f32);
        let pos = vec2(x + 0.5, y_dis);

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
    draw_border(textures, scl, vec2(GAME_WIDTH - 8.0, 1.0), 5.0, 13.0);
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

                if *debug {
                    draw_rectangle(
                        (offset.x + x as f32) * scl,
                        (offset.y + y as f32) * scl,
                        w,
                        w,
                        GRAY,
                    );
                } else {
                    draw_block(
                        scl,
                        textures,
                        (offset.x + x as f32) * scl,
                        (offset.y + y as f32) * scl,
                        block.kind,
                        false,
                    );
                }
            }
            _ => {}
        }
    }
}

fn draw_score(textures: &Texture2D, text_config: &TextParamsConfig, scl: f32, score: &Score) {
    let lines_head_y = (GAME_HEIGHT - 12.0) * scl;
    let lines_head = &"LINES".to_string();
    let lines_body = &format!("{:0>3}", score.lines).to_string();
    let (base_params, base_dimensions) = text_config.params_and_dims(lines_head, 1.25);
    // Everything is measured relative to the first heading
    let x = (GAME_WIDTH - 6.0 - base_dimensions.width / 2.0 / scl) * scl;
    draw_text_ex(lines_head, x, lines_head_y, base_params);
    draw_text_ex(lines_body, x, lines_head_y + 1.0 * scl, base_params);

    let level_head_y = (GAME_HEIGHT - 9.5) * scl;
    let level_head = &"LEVEL".to_string();
    let level_body = &format!("{:0>2}", score.level).to_string();
    draw_text_ex(level_head, x, level_head_y, base_params);
    draw_text_ex(level_body, x, level_head_y + 1.0 * scl, base_params);

    let score_head = &"SCORE".to_string();
    let score_head_y = (GAME_HEIGHT - 7.0) * scl;
    draw_text_ex(score_head, x, score_head_y, base_params);
    draw_text_ex(
        &format!("{:0>6}", score.val),
        x,
        score_head_y + 1.0 * scl,
        base_params,
    );

    let border_pos = vec2(GAME_WIDTH - 8.5, 17.0);
    draw_border(textures, scl, border_pos, 7.0, 8.0);

    if score.topout {
        let game_over_text = &"GAME OVER".to_string();
        let (game_over_params, game_over_dimensions) =
            text_config.params_and_dims(game_over_text, 3.0);
        let go_x = (GAME_WIDTH / 2.0) * scl - game_over_dimensions.width / 2.0;
        let go_y = (GAME_HEIGHT / 3.0) * scl;
        draw_text_ex(
            "GAME OVER",
            go_x + 5.0,
            go_y + 5.0,
            TextParams {
                font: game_over_params.font,
                font_size: game_over_params.font_size,
                color: DARK,
                ..Default::default()
            },
        );
        draw_text_ex("GAME OVER", go_x, go_y, game_over_params);
    }
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

fn draw_score_popup(text_config: &TextParamsConfig, scl: f32, well_pos: &Vec2, score: &ScorePopup) {
    let score_text = &format!("{}", score.val).to_string();
    let (params, dims) = text_config.params_and_dims(score_text, 2.0);
    let x = (well_pos.x + (WELL_WIDTH / 2) as f32) * scl - dims.width / 2.0;
    let y = (well_pos.y + (WELL_HEIGHT / 3) as f32) * scl;

    draw_text_ex(
        &score_text,
        x + 0.1 * scl,
        y + 0.1 * scl,
        TextParams {
            font_size: params.font_size,
            font: params.font,
            color: DARK,
            ..Default::default()
        },
    );
    draw_text_ex(&score_text, x, y, params);
}

#[derive(Clone, Copy)]
pub struct TextParamsConfig {
    pub base: TextParams,
    pub scl: f32,
}
impl TextParamsConfig {
    fn params_and_dims(self: Self, text: &String, font_size: f32) -> (TextParams, TextDimensions) {
        (
            TextParams {
                font: self.base.font,
                font_size: (font_size * self.scl) as u16,
                color: self.base.color,
                font_scale: self.base.font_scale,
                font_scale_aspect: self.base.font_scale_aspect,
            },
            measure_text(
                text,
                Some(self.base.font),
                (font_size * self.scl) as u16,
                1.0,
            ),
        )
    }
}

fn draw_title(gs: &GameState, text_config: &TextParamsConfig) {
    let title = &"TETRIS".to_string();
    let (title_params, title_dims) = text_config.params_and_dims(title, 8.0);
    let x = (GAME_WIDTH / 2.0) * gs.scl - title_dims.width / 2.0;
    let y = (GAME_HEIGHT / 3.0) * gs.scl;
    draw_text_ex(title, x, y, title_params);

    let start_text = &"PUSH START".to_string();
    let (start_params, start_dims) = text_config.params_and_dims(start_text, 2.0);
    let x = (GAME_WIDTH / 4.0) * gs.scl - start_dims.width / 2.0;
    let y = (GAME_HEIGHT - GAME_HEIGHT / 3.0) * gs.scl;
    draw_text_ex(&start_text, x, y, start_params);
}

pub fn draw(gs: &GameState) {
    clear_background(DARK);
    let text_config = TextParamsConfig {
        scl: gs.scl,
        base: TextParams {
            font: gs.font,
            font_size: 1,
            color: LIGHT,
            ..Default::default()
        },
    };

    match gs.game_mode {
        GameMode::Title => draw_title(gs, &text_config),
        GameMode::Play | GameMode::Pause => draw_play(gs, &text_config),
        _ => {}
    }
}

pub fn draw_play(gs: &GameState, text_config: &TextParamsConfig) {
    let offset = vec2(
        GAME_WIDTH / 2.0 - WELL_WIDTH as f32 / 2.0,
        GAME_HEIGHT / 2.0 - WELL_HEIGHT as f32 / 2.0,
    );

    // information stuff
    draw_statistics(
        &gs.textures,
        &text_config,
        gs.scl,
        &gs.tetrominos,
        &gs.statistics,
    );

    if gs.debug {
        draw_text(
            format!("{} {}", gs.current.pos.x, gs.current.pos.y).as_str(),
            gs.current.pos.x + 20.0,
            gs.current.pos.y - WELL_HEIGHT as f32 - 20.0,
            1.25 * gs.scl,
            BLUE,
        );
    }
    draw_well(offset, gs.scl);
    draw_border(
        &gs.textures,
        gs.scl,
        offset,
        WELL_WIDTH as f32,
        WELL_HEIGHT as f32,
    );

    if gs.game_mode == GameMode::Pause {
        let text1 = &"GAME".to_string();
        let text2 = &"PAUSED".to_string();
        let text3 = &"\"P\" TO RESUME".to_string();
        let (params1, dims1) = text_config.params_and_dims(text1, 2.0);
        let (params2, dims2) = text_config.params_and_dims(text2, 2.0);
        let (params3, dims3) = text_config.params_and_dims(text3, 1.0);
        draw_text_ex(
            text1,
            (offset.x + WELL_WIDTH as f32 / 2.0) * gs.scl - dims1.width / 2.0,
            (offset.y + WELL_HEIGHT as f32 / 3.0) * gs.scl - dims1.height / 2.0,
            params1,
        );
        draw_text_ex(
            text2,
            (offset.x + WELL_WIDTH as f32 / 2.0) * gs.scl - dims2.width / 2.0,
            (offset.y + WELL_HEIGHT as f32 / 3.0) * gs.scl - dims2.height / 2.0
                + dims1.height
                + 10.0,
            params2,
        );
        draw_text_ex(
            text3,
            (offset.x + WELL_WIDTH as f32 / 2.0) * gs.scl - dims3.width / 2.0,
            (offset.y + WELL_HEIGHT as f32 / 3.0) * gs.scl - dims3.height / 2.0
                + dims1.height
                + dims2.height
                + 10.0,
            params3,
        );
        draw_border(&gs.textures, gs.scl, vec2(2.0, 1.0), 4.0, 4.0);
        draw_border(&gs.textures, gs.scl, vec2(GAME_WIDTH - 8.0, 1.0), 5.0, 13.0);
        return; // not giving the player an advantage by seeing the well
    }
    draw_hold(&gs.textures, &text_config, gs.scl, &gs.hold);
    draw_next(&gs.textures, &text_config, gs.scl, &gs.next);

    // game stuff
    draw_placed(&gs.textures, offset, gs.scl, &gs.placed_blocks, &gs.debug);

    let entered = match &gs.line_clear {
        Some(line_clear) => {
            gs.current.entry_timer >= ENTRY_DELAY && line_clear.counter >= LINE_CLEAR_DELAY
        }
        None => gs.current.entry_timer >= ENTRY_DELAY,
    };

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
                offset.x + 5.0 - f32::ceil(gs.current.width as f32 / 2.0),
                19.0,
            ),
            &gs.current,
        );
    }

    draw_score(&gs.textures, &text_config, gs.scl, &gs.score);
    if gs.last_score.val > 0 && gs.last_score.creation < SCORE_TIMEOUT {
        draw_score_popup(&text_config, gs.scl, &offset, &gs.last_score);
    }

    match &gs.line_clear {
        Some(line_clear) => {
            draw_rectangle(
                offset.x * gs.scl,
                (offset.y + line_clear.y_pos as f32) * gs.scl,
                f32::min(
                    WELL_WIDTH as f32,
                    WELL_WIDTH as f32 * line_clear.counter as f32 * 1.5 / LINE_CLEAR_DELAY as f32,
                ) * gs.scl,
                line_clear.lines.len() as f32 * gs.scl,
                DARK,
            );
        }
        None => {}
    }
}
