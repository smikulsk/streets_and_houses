use super::*;
use crate::scene::prelude::*;

#[derive(Debug)]
pub struct PlayingScene {
    spritebatch: graphics::spritebatch::SpriteBatch,
    spritebatch_clicked: graphics::spritebatch::SpriteBatch,
    spritebatch_a: graphics::spritebatch::SpriteBatch,
    spritebatch_b: graphics::spritebatch::SpriteBatch,
    board: game::Board,
    player: game::Player,
    wall_bounding_boxes: Vec<Vec<Rect>>,
}

impl PlayingScene {
    pub fn new(
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        board: Board,
    ) -> GameResult<PlayingScene> {
        let image = graphics::Image::new(ctx, quad_ctx, "tile.png").unwrap();
        let image_clicked = graphics::Image::new(ctx, quad_ctx, "tile_clicked.png").unwrap();
        let image_a = graphics::Image::new(ctx, quad_ctx, "tileA.png").unwrap();
        let image_b = graphics::Image::new(ctx, quad_ctx, "tileB.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);
        let batch_clicked = graphics::spritebatch::SpriteBatch::new(image_clicked);
        let batch_a = graphics::spritebatch::SpriteBatch::new(image_a);
        let batch_b = graphics::spritebatch::SpriteBatch::new(image_b);
        let wall_bounding_boxes =
            vec![vec![Rect::default(); board.width + 1]; 2 * board.height + 1];

        let s = PlayingScene {
            spritebatch: batch,
            spritebatch_clicked: batch_clicked,
            spritebatch_a: batch_a,
            spritebatch_b: batch_b,
            board,
            player: game::Player::Player1,
            wall_bounding_boxes,
        };
        Ok(s)
    }

    fn get_tile_size(&self, quad_ctx: &mut miniquad::Context, footer_height : f32) -> (f32, f32) {
        let (w, h) = quad_ctx.display().screen_size();

        (
            (w as usize / (6 * self.board.width + 1)) as f32,
            ((h - footer_height) as usize / (6 * self.board.height + 1)) as f32,
        )
    }

    fn add_cell_sprite(&mut self, tile_size: (f32, f32), row: usize, col: usize) {
        let r = row as f32;
        let c = col as f32;
        let p = graphics::DrawParam::new()
            .dest(Point2::new(
                tile_size.0 + 6.0 * c * tile_size.0,
                (3.0 * r + 1.0) * tile_size.1,
            ))
            .scale(Vector2::new(
                5.0 * tile_size.0 / IMAGE_WIDTH,
                5.0 * tile_size.1 / IMAGE_HEIGHT,
            ));
        if row < 2 * self.board.height {
            if let Some(player) = self.board.cells[row / 2][col].owner {
                match player {
                    game::Player::Player1 => self.spritebatch_a.add(p),
                    game::Player::Player2 => self.spritebatch_b.add(p),
                };
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
        if self.board.walls[row][col].is_clicked {
            self.spritebatch_clicked.add(p);
        } else {
            self.spritebatch.add(p);
        }
    }
    
    fn draw_footer(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::Context) -> Result<Rect, ggez::GameError> {
        let (width, height) = graphics::drawable_size(quad_ctx);
        let footer_rect = draw_text(ctx, quad_ctx, width/2.0, height - 15.0, &format!("========================= {:?} =========================", self.player))?;
        Ok(footer_rect)
    }
}

impl Scene for PlayingScene {
    type State = PlayingState;

    fn update(
        &mut self,
        _ctx: &mut ggez::Context,
        _quad_ctx: &mut event::GraphicsContext,
    ) -> Result<Option<Transition>, ggez::GameError> {        
        Ok(None)
    }
    
    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, graphics::Color::BLACK);
        
        let footer_rect = self.draw_footer(ctx, quad_ctx)?;

        let tile_size = self.get_tile_size(quad_ctx, footer_rect.h);

        for row in 0..2 * self.board.height + 1 {
            if row % 2 == 0 {
                for col in 0..self.board.width {
                    for dest in get_horizontal_wall_sprite_destinations(tile_size, row, col) {
                        self.add_wall_sprite(tile_size, row, col, dest);
                        self.wall_bounding_boxes[row][col] =
                            get_horizontal_wall_sprite_bounding_box(tile_size, row, col);
                    }

                    self.add_cell_sprite(tile_size, row, col);
                }
            } else {
                for col in 0..self.board.width + 1 {
                    for dest in get_vertical_wall_sprite_destinations(tile_size, row, col) {
                        self.add_wall_sprite(tile_size, row, col, dest);
                        self.wall_bounding_boxes[row][col] =
                            get_vertical_wall_sprite_bounding_box(tile_size, row, col);
                    }
                }
            }
        }

        for spritebatch in [
            &mut self.spritebatch,
            &mut self.spritebatch_clicked,
            &mut self.spritebatch_a,
            &mut self.spritebatch_b,
        ] {
            graphics::draw(ctx, quad_ctx, spritebatch, graphics::DrawParam::new())?;
            spritebatch.clear();
        }


        graphics::present(ctx, quad_ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::GraphicsContext,
        _button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Option<Transition> {
        let mut clicked_wall_coords = None;
        for row in 0..2 * self.board.height + 1 {
            let max_col = if row % 2 > 0 {
                self.board.width + 1
            } else {
                self.board.width
            };
            for col in 0..max_col {
                let rect = self.wall_bounding_boxes[row][col];
                if rect.contains(Point2::new(x, y)) {
                    clicked_wall_coords = Some((row, col));
                    break;
                }
            }
            if clicked_wall_coords.is_some() {
                break;
            }
        }

        if let Some((row, col)) = clicked_wall_coords {
            //println!("Trying to click the ({},{}) wall", row, col);
            match self.board.click_wall(row, col, self.player) {
                Ok(additional_move) => {
                    if !additional_move {
                        match self.player {
                            game::Player::Player1 => self.player = game::Player::Player2,
                            game::Player::Player2 => self.player = game::Player::Player1,
                        }
                    }
                }
                Err(error) => println!("Error occured:'{}'", error),
            }
        }
        self.board.all_is_clicked().then(|| {
            let game_statistics = self.board.get_statistics();
            let game = GameOverScene::new(game_statistics);
            Transition::ToGameOver(Box::new(game))
        })
    }
}

fn get_horizontal_wall_sprite_destinations(
    tile_size: (f32, f32),
    row: usize,
    col: usize,
) -> Vec<Point2<f32>> {
    let r = row as f32;
    let c = col as f32;

    (0..5)
        .map(|x| {
            let x = x as f32;
            Point2::new(
                (x + 1.0) * tile_size.0 + 6.0 * c * tile_size.0,
                3.0 * r * tile_size.1,
            )
        })
        .collect::<Vec<_>>()
}

fn get_horizontal_wall_sprite_bounding_box(tile_size: (f32, f32), row: usize, col: usize) -> Rect {
    let r = row as f32;
    let c = col as f32;
    Rect::new(
        (6.0 * c + 1.0) * tile_size.0,
        3.0 * r * tile_size.1,
        5.0 * tile_size.0,
        tile_size.1,
    )
}

fn get_vertical_wall_sprite_destinations(
    tile_size: (f32, f32),
    row: usize,
    col: usize,
) -> Vec<Point2<f32>> {
    let r = row as f32;
    let c = col as f32;

    (0..5)
        .map(|y| {
            let y = y as f32;
            Point2::new(
                6.0 * c * tile_size.0,
                (y + 1.0) * tile_size.1 + 3.0 * (r - 1.0) * tile_size.1,
            )
        })
        .collect::<Vec<_>>()
}

fn get_vertical_wall_sprite_bounding_box(tile_size: (f32, f32), row: usize, col: usize) -> Rect {
    let r = row as f32;
    let c = col as f32;
    Rect::new(
        6.0 * c * tile_size.0,
        (3.0 * r - 2.0) * tile_size.1,
        tile_size.0,
        5.0 * tile_size.1,
    )
}
