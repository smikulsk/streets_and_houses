use ggez::timer;

use super::*;
use crate::game::{ai::*, Player};
use crate::scene::prelude::*;

#[derive(Debug, Clone)]
pub enum GameMode {
    OnePlayer(Box<dyn MoveGenerator>),
    TwoPlayer,
}

#[derive(Debug)]
pub struct PlayingScene {
    spritebatch_empty: graphics::spritebatch::SpriteBatch,
    spritebatch_path_h: graphics::spritebatch::SpriteBatch,
    spritebatch_path_v: graphics::spritebatch::SpriteBatch,
    spritebatch_street_h: graphics::spritebatch::SpriteBatch,
    spritebatch_street_v: graphics::spritebatch::SpriteBatch,
    spritebatch_a: graphics::spritebatch::SpriteBatch,
    spritebatch_b: graphics::spritebatch::SpriteBatch,
    board: game::Board,
    player: game::Player,
    wall_bounding_boxes: Vec<Vec<Rect>>,
    game_mode: GameMode,
    already_drawn: bool,
    deferred_transition: Option<Transition>,
}

impl PlayingScene {
    pub fn new(
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        player: Player,
        board: Board,
        game_mode: GameMode,
    ) -> GameResult<PlayingScene> {
        let image_empty = graphics::Image::new(ctx, quad_ctx, "roads/empty.png")?;
        let image_path_h = graphics::Image::new(ctx, quad_ctx, "roads/path_h.png")?;
        let image_path_v = graphics::Image::new(ctx, quad_ctx, "roads/path_v.png")?;
        let image_street_h = graphics::Image::new(ctx, quad_ctx, "roads/street_h.png")?;
        let image_street_v = graphics::Image::new(ctx, quad_ctx, "roads/street_v.png")?;
        let image_a = graphics::Image::new(ctx, quad_ctx, "buildings/textured_red.png")?;
        let image_b = graphics::Image::new(ctx, quad_ctx, "buildings/textured_green.png")?;
        let batch_empty = graphics::spritebatch::SpriteBatch::new(image_empty);
        let batch_path_h = graphics::spritebatch::SpriteBatch::new(image_path_h);
        let batch_path_v = graphics::spritebatch::SpriteBatch::new(image_path_v);
        let batch_street_h = graphics::spritebatch::SpriteBatch::new(image_street_h);
        let batch_street_v = graphics::spritebatch::SpriteBatch::new(image_street_v);
        let batch_a = graphics::spritebatch::SpriteBatch::new(image_a);
        let batch_b = graphics::spritebatch::SpriteBatch::new(image_b);
        let wall_bounding_boxes =
            vec![vec![Rect::default(); board.width + 1]; 2 * board.height + 1];

        let s = PlayingScene {
            spritebatch_empty: batch_empty,
            spritebatch_path_h: batch_path_h,
            spritebatch_path_v: batch_path_v,
            spritebatch_street_h: batch_street_h,
            spritebatch_street_v: batch_street_v,
            spritebatch_a: batch_a,
            spritebatch_b: batch_b,
            board,
            player,
            wall_bounding_boxes,
            game_mode,
            already_drawn: false,
            deferred_transition: None,
        };
        Ok(s)
    }

    fn get_tile_size(&self, quad_ctx: &mut miniquad::Context, footer_height: f32) -> (f32, f32) {
        let (w, h) = quad_ctx.display().screen_size();

        let tile_size_x = IMAGE_WIDTH * w
            / ((IMAGE_WIDTH + V_STREET_WIDTH) * self.board.width as f32 + V_STREET_WIDTH);
        let tile_size_y = IMAGE_HEIGHT * (h - footer_height)
            / ((IMAGE_HEIGHT + H_STREET_HEIGHT) * self.board.height as f32 + H_STREET_HEIGHT);

        if tile_size_x > tile_size_y {
            return (tile_size_y, tile_size_y);
        }
        (tile_size_x, tile_size_x)
    }

    fn get_translation(
        &self,
        quad_ctx: &mut miniquad::Context,
        footer_height: f32,
        tile_size: (f32, f32),
    ) -> (f32, f32) {
        let (w, h) = quad_ctx.display().screen_size();

        (
            (w - ((IMAGE_WIDTH + V_STREET_WIDTH) * self.board.width as f32 + V_STREET_WIDTH)
                / IMAGE_WIDTH
                * tile_size.0)
                / 2.0,
            (h - footer_height
                - ((IMAGE_HEIGHT + H_STREET_HEIGHT) * self.board.height as f32 + H_STREET_HEIGHT)
                    / IMAGE_WIDTH
                    * tile_size.1)
                / 2.0,
        )
    }

    fn add_cell_sprite(
        &mut self,
        tile_size: (f32, f32),
        translation: (f32, f32),
        row: usize,
        col: usize,
    ) {
        let r = row as f32;
        let c = col as f32;
        let p = graphics::DrawParam::new()
            .dest(Point2::new(
                tile_size.0 * V_STREET_WIDTH / IMAGE_WIDTH
                    + (IMAGE_WIDTH + V_STREET_WIDTH) * c * tile_size.0 / IMAGE_WIDTH
                    + translation.0,
                ((IMAGE_HEIGHT + H_STREET_HEIGHT) * r / 2.0 / IMAGE_HEIGHT
                    + H_STREET_HEIGHT / IMAGE_HEIGHT)
                    * tile_size.1
                    + translation.1,
            ))
            .scale(Vector2::new(
                tile_size.0 / IMAGE_WIDTH,
                tile_size.1 / IMAGE_HEIGHT,
            ));

        if row < 2 * self.board.height {
            if let Some(player) = self.board.cells[row / 2][col].owner {
                match player {
                    game::Player::Player1 => self.spritebatch_a.add(p),
                    game::Player::Player2 | game::Player::CPU => self.spritebatch_b.add(p),
                };
            } else {
                self.spritebatch_empty.add(p);
            }
        }
    }

    fn add_wall_sprite(
        &mut self,
        tile_size: (f32, f32),
        row: usize,
        col: usize,
        dest: Point2<f32>,
    ) {
        let p = graphics::DrawParam::new().dest(dest).scale(Vector2::new(
            tile_size.0 / IMAGE_WIDTH,
            tile_size.1 / IMAGE_HEIGHT,
        ));
        if row % 2 == 0 {
            if self.board.walls[row][col].is_clicked {
                self.spritebatch_street_h.add(p);
            } else {
                self.spritebatch_path_h.add(p);
            }
        } else if self.board.walls[row][col].is_clicked {
            self.spritebatch_street_v.add(p);
        } else {
            self.spritebatch_path_v.add(p);
        }
    }

    fn draw_footer(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<Rect, ggez::GameError> {
        let (width, height) = graphics::drawable_size(quad_ctx);
        let footer_rect = draw_text(
            ctx,
            quad_ctx,
            width / 2.0,
            height - 15.0,
            &format!(
                "========================= {:?} =========================",
                self.player
            ),
        )?;
        Ok(Rect::new(
            footer_rect.x,
            footer_rect.y,
            footer_rect.w,
            footer_rect.h + 15.0,
        ))
    }

    fn click_wall(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut event::GraphicsContext,
        row: usize,
        col: usize,
    ) {
        match self.board.click_wall(row, col, self.player) {
            Ok(additional_move) => {
                if !additional_move {
                    let new_player = match self.player {
                        game::Player::Player1 => match &self.game_mode {
                            GameMode::OnePlayer(_) => game::Player::CPU,
                            GameMode::TwoPlayer => game::Player::Player2,
                        },
                        game::Player::Player2 | game::Player::CPU => game::Player::Player1,
                    };

                    if !self.board.all_is_clicked() {
                        self.deferred_transition = match self.game_mode {
                            GameMode::OnePlayer(_) => {
                                let game = PlayingScene::new(
                                    ctx,
                                    quad_ctx,
                                    new_player,
                                    self.board.clone(),
                                    self.game_mode.clone(),
                                )
                                .expect("board was initialized");

                                Some(Transition::ToPlaying(Box::new(game)))
                            }
                            GameMode::TwoPlayer => {
                                let prepare_player_scene = PreparePlayerScene::new(
                                    ctx,
                                    quad_ctx,
                                    new_player,
                                    &self.board,
                                    &self.game_mode,
                                );
                                Some(Transition::ToPreparePlayer(Box::new(prepare_player_scene)))
                            }
                        }
                    }
                }
                self.already_drawn = false;
            }
            Err(error) => println!("Error occurred: '{}'", error),
        }
    }
}

impl Scene for PlayingScene {
    type State = PlayingState;

    fn update(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut event::GraphicsContext,
    ) -> Result<Option<Transition>, ggez::GameError> {
        if self.already_drawn && timer::ticks(ctx) % PLAYING_TICK_COUNT == 0 {
            if let Some(transition) = self.deferred_transition.take() {
                return Ok(Some(transition));
            }

            if self.board.all_is_clicked() {
                let game_statistics = self.board.get_statistics();
                let game = GameOverScene::new(
                    game_statistics,
                    &self.game_mode,
                    self.board.width,
                    self.board.height,
                );
                return Ok(Some(Transition::ToGameOver(Box::new(game))));
            }

            if self.player == Player::CPU {
                if let Some((row, col)) = match &self.game_mode {
                    GameMode::OnePlayer(move_generator) => move_generator.next_move(&self.board),
                    GameMode::TwoPlayer => None,
                } {
                    self.click_wall(ctx, quad_ctx, row, col);
                }
            }
        }
        Ok(None)
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, graphics::Color::BLACK);

        let footer_rect = self.draw_footer(ctx, quad_ctx)?;

        let tile_size = self.get_tile_size(quad_ctx, footer_rect.h);
        let translation = self.get_translation(quad_ctx, footer_rect.h, tile_size);

        for row in 0..2 * self.board.height + 1 {
            if row % 2 == 0 {
                for col in 0..self.board.width {
                    let dest =
                        get_horizontal_wall_sprite_destination(tile_size, translation, row, col);
                    self.add_wall_sprite(tile_size, row, col, dest);
                    self.wall_bounding_boxes[row][col] =
                        get_horizontal_wall_sprite_bounding_box(tile_size, translation, row, col);

                    self.add_cell_sprite(tile_size, translation, row, col);
                }
            } else {
                for col in 0..self.board.width + 1 {
                    let dest =
                        get_vertical_wall_sprite_destination(tile_size, translation, row, col);
                    self.add_wall_sprite(tile_size, row, col, dest);
                    self.wall_bounding_boxes[row][col] =
                        get_vertical_wall_sprite_bounding_box(tile_size, translation, row, col);
                }
            }
        }

        for spritebatch in [
            &mut self.spritebatch_empty,
            &mut self.spritebatch_path_h,
            &mut self.spritebatch_path_v,
            &mut self.spritebatch_street_h,
            &mut self.spritebatch_street_v,
            &mut self.spritebatch_a,
            &mut self.spritebatch_b,
        ] {
            graphics::draw(ctx, quad_ctx, spritebatch, graphics::DrawParam::new())?;
            spritebatch.clear();
        }

        graphics::present(ctx, quad_ctx)?;

        self.already_drawn = true;

        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        _button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Option<Transition> {
        if self.player == Player::CPU {
            return None;
        }

        for row in 0..2 * self.board.height + 1 {
            let max_col = if row % 2 > 0 {
                self.board.width + 1
            } else {
                self.board.width
            };
            for col in 0..max_col {
                let rect = self.wall_bounding_boxes[row][col];
                if rect.contains(Point2::new(x, y)) {
                    self.click_wall(ctx, quad_ctx, row, col);
                    return None;
                }
            }
        }
        None
    }
}

fn get_horizontal_wall_sprite_destination(
    tile_size: (f32, f32),
    translation: (f32, f32),
    row: usize,
    col: usize,
) -> Point2<f32> {
    let r = row as f32;
    let c = col as f32;

    Point2::new(
        V_STREET_WIDTH * tile_size.0 / IMAGE_WIDTH
            + (IMAGE_WIDTH + V_STREET_WIDTH) * c * tile_size.0 / IMAGE_WIDTH
            + translation.0,
        (IMAGE_HEIGHT + H_STREET_HEIGHT) * r * tile_size.1 / 2.0 / IMAGE_HEIGHT + translation.1,
    )
}

fn get_horizontal_wall_sprite_bounding_box(
    tile_size: (f32, f32),
    translation: (f32, f32),
    row: usize,
    col: usize,
) -> Rect {
    let r = row as f32;
    let c = col as f32;
    Rect::new(
        ((IMAGE_WIDTH + V_STREET_WIDTH) * c + V_STREET_WIDTH) * tile_size.0 / IMAGE_WIDTH
            + translation.0,
        (IMAGE_HEIGHT + H_STREET_HEIGHT) * r * tile_size.1 / 2.0 / IMAGE_HEIGHT + translation.1,
        tile_size.0,
        tile_size.1 * H_STREET_HEIGHT / IMAGE_HEIGHT,
    )
}

fn get_vertical_wall_sprite_destination(
    tile_size: (f32, f32),
    translation: (f32, f32),
    row: usize,
    col: usize,
) -> Point2<f32> {
    let r = row as f32;
    let c = col as f32;

    Point2::new(
        (IMAGE_WIDTH + V_STREET_WIDTH) * c * tile_size.0 / IMAGE_WIDTH + translation.0,
        H_STREET_HEIGHT * tile_size.1 / IMAGE_HEIGHT
            + (IMAGE_HEIGHT + H_STREET_HEIGHT) * (r - 1.0) * tile_size.1 / 2.0 / IMAGE_HEIGHT
            + translation.1,
    )
}

fn get_vertical_wall_sprite_bounding_box(
    tile_size: (f32, f32),
    translation: (f32, f32),
    row: usize,
    col: usize,
) -> Rect {
    let r = row as f32;
    let c = col as f32;
    Rect::new(
        (IMAGE_WIDTH + V_STREET_WIDTH) * c * tile_size.0 / IMAGE_WIDTH + translation.0,
        H_STREET_HEIGHT * tile_size.1 / IMAGE_HEIGHT
            + (IMAGE_HEIGHT + H_STREET_HEIGHT) * (r - 1.0) * tile_size.1 / 2.0 / IMAGE_HEIGHT
            + translation.1,
        tile_size.0 * V_STREET_WIDTH / IMAGE_WIDTH,
        tile_size.1,
    )
}
