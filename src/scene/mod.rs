extern crate good_web_game as ggez;

use crate::game;

use game::Board;
use game::GameMode;

use ggez::cgmath::Point2;
use ggez::cgmath::Vector2;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::miniquad;
use ggez::Context;
use ggez::GameResult;

use std::fmt::Debug;

use crate::rendering::board_renderer::*;
use crate::rendering::constants::*;
use crate::state::marker::*;
use crate::state::transition::*;

pub mod game_over;
pub mod main_menu;
pub mod playing;
pub mod prelude;
pub mod prepare_player;
pub mod title_screen;

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
