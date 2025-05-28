extern crate good_web_game as ggez;

use crate::game;

use game::Board;
use game::Player;

use ggez::cgmath::Point2;
use ggez::cgmath::Vector2;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::miniquad;
use ggez::Context;
use ggez::GameResult;

pub mod board_renderer;
pub mod constants;
pub mod prelude;
pub mod ui;

pub trait Renderer {
    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult;
}
