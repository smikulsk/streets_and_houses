//extern crate cgmath;
extern crate good_web_game as ggez;

use game::Board;
use ggez::cgmath::Vector2;
use ggez::event;
use ggez::graphics;
use ggez::miniquad;

use ggez::timer;
use ggez::{Context, GameResult};

use good_web_game::cgmath::Point2;

pub mod game;

const IMAGE_WIDTH: f32 = 171.0;
const IMAGE_HEIGHT: f32 = 167.0;

struct MainState {
    spritebatch: graphics::spritebatch::SpriteBatch,
    spritebatch_clicked: graphics::spritebatch::SpriteBatch,
    spritebatch_a: graphics::spritebatch::SpriteBatch,
    spritebatch_b: graphics::spritebatch::SpriteBatch,
    board: game::Board,
    player: game::Player,
}

impl MainState {
    fn new(ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, quad_ctx, "tile.png").unwrap();
        let image_clicked = graphics::Image::new(ctx, quad_ctx, "tile_clicked.png").unwrap();
        let image_a = graphics::Image::new(ctx, quad_ctx, "tileA.png").unwrap();
        let image_b = graphics::Image::new(ctx, quad_ctx, "tileB.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);
        let batch_clicked = graphics::spritebatch::SpriteBatch::new(image_clicked);
        let batch_a = graphics::spritebatch::SpriteBatch::new(image_a);
        let batch_b = graphics::spritebatch::SpriteBatch::new(image_b);

        let board = game::Board::new(3, 2);
        let s = MainState {
            spritebatch: batch,
            spritebatch_clicked: batch_clicked,
            spritebatch_a: batch_a,
            spritebatch_b: batch_b,
            board,
            player: game::Player::Player1,
        };
        Ok(s)
    }

    fn get_tile_size(&self, quad_ctx: &mut miniquad::Context) -> f32 {
        let (w, h) = quad_ctx.display().screen_size();

        std::cmp::min(
            w as usize / (6 * self.board.width + 1),
            h as usize / (6 * self.board.height + 1),
        ) as f32
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(
        &mut self,
        ctx: &mut Context,
        _quad_ctx: &mut miniquad::GraphicsContext,
    ) -> GameResult {
        if timer::ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?} ", timer::delta(ctx));
            println!("Average FPS: {}", timer::fps(ctx));
            println!("Current player: {:?}", self.player);

            if self.board.all_is_clicked() {
                let game_statistics = self.board.get_statistics();

                println!("Player 1 points: {}", game_statistics.player1_points);
                println!("Player 2 points: {}", game_statistics.player2_points);
                if let Some(player) = game_statistics.winner {
                    println!("{:?} wins!!!", player);
                } else {
                    println!("It is a tie!!!");
                }
                self.board = Board::new(3, 2);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, graphics::Color::BLACK);

        let tile_size = self.get_tile_size(quad_ctx);

        for row in 0..2 * self.board.height + 1 {
            if row % 2 == 0 {
                for col in 0..self.board.width {
                    let r = row as f32;
                    let c = col as f32;

                    for x in 0..5 {
                        let x = x as f32;
                        let p = graphics::DrawParam::new()
                            .dest(Point2::new(
                                (x + 1.0) * tile_size + 6.0 * c * tile_size,
                                3.0 * r * tile_size,
                            ))
                            .scale(Vector2::new(
                                tile_size / IMAGE_WIDTH,
                                tile_size / IMAGE_HEIGHT,
                            ));
                        if self.board.walls[row][col].is_clicked {
                            self.spritebatch_clicked.add(p);
                        } else {
                            self.spritebatch.add(p);
                        }
                    }

                    let p = graphics::DrawParam::new()
                        .dest(Point2::new(
                            tile_size + 6.0 * c * tile_size,
                            (3.0 * r + 1.0) * tile_size,
                        ))
                        .scale(Vector2::new(
                            5.0 * tile_size / IMAGE_WIDTH,
                            5.0 * tile_size / IMAGE_HEIGHT,
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
            } else {
                for y in 0..5 {
                    for col in 0..self.board.width + 1 {
                        let y = y as f32;
                        let r = row as f32;
                        let c = col as f32;
                        let p = graphics::DrawParam::new()
                            .dest(Point2::new(
                                6.0 * c * tile_size,
                                (y + 1.0) * tile_size + 3.0 * (r - 1.0) * tile_size,
                            ))
                            .scale(Vector2::new(
                                tile_size / IMAGE_WIDTH,
                                tile_size / IMAGE_WIDTH,
                            ));
                        if self.board.walls[row][col].is_clicked {
                            self.spritebatch_clicked.add(p);
                        } else {
                            self.spritebatch.add(p);
                        }
                    }
                }
            }
        }

        graphics::draw(ctx, quad_ctx, &self.spritebatch, graphics::DrawParam::new())?;
        graphics::draw(
            ctx,
            quad_ctx,
            &self.spritebatch_clicked,
            graphics::DrawParam::new(),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &self.spritebatch_a,
            graphics::DrawParam::new(),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &self.spritebatch_b,
            graphics::DrawParam::new(),
        )?;

        self.spritebatch.clear();
        self.spritebatch_clicked.clear();
        self.spritebatch_a.clear();
        self.spritebatch_b.clear();

        graphics::present(ctx, quad_ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        let tile_size = self.get_tile_size(quad_ctx);
        let pos_x = (x / tile_size).floor() as usize;
        let pos_y = (y / tile_size).floor() as usize;

        println!(
            "Mouse button released: {:?}, x: {}, y: {}, pos_x: {}, pos_y: {}",
            button, x, y, pos_x, pos_y
        );

        match pos_y % 6 {
            0 => {
                // horizontal
                if pos_x % 6 > 0 {
                    println!("Trying to click the ({},{}) wall", pos_y / 3, pos_x / 6);
                    match self.board.click_wall(pos_y / 3, pos_x / 6, self.player) {
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
            }
            _ => {
                //vertical
                if pos_x % 6 == 0 {
                    println!(
                        "Trying to click the ({},{}) wall",
                        2 * (pos_y / 6) + 1,
                        pos_x / 6
                    );
                    match self
                        .board
                        .click_wall(2 * (pos_y / 6) + 1, pos_x / 6, self.player)
                    {
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
            }
        }
    }
}

pub fn main() -> GameResult {
    ggez::start(
        ggez::conf::Conf::default().cache(Some(include_bytes!("resources.tar"))),
        |context, quad_ctx| Box::new(MainState::new(context, quad_ctx).unwrap()),
    )
}
