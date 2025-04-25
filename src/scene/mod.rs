extern crate good_web_game as ggez;

use crate::game;

use game::Board;

use ggez::cgmath::Point2;
use ggez::cgmath::Vector2;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::miniquad;
use ggez::Context;
use ggez::GameResult;

use std::fmt::Debug;

const IMAGE_WIDTH: f32 = 171.0;
const IMAGE_HEIGHT: f32 = 167.0;

use crate::state::marker::*;
use crate::state::transition::*;

pub mod game_pver;
pub mod main_menu;
pub mod playing;
pub mod prelude;

pub trait Scene: Debug {
    type State: GameStateMarker;

    fn update(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut event::GraphicsContext,
    ) -> Result<Option<Transition>, ggez::GameError>;
    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult;
    fn on_enter(&mut self) {}
    fn on_exit(&mut self) {}
    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Option<Transition>;
}
