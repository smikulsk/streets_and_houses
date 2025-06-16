extern crate good_web_game as ggez;

use ggez::event;
use ggez::graphics::Rect;
use ggez::miniquad;
use ggez::timer::time;
use ggez::Context;
use ggez::GameResult;

use game::controller::*;
use state::prelude::*;

pub mod ai;
pub mod file;
pub mod game;
pub mod rendering;
pub mod scene;
pub mod state;

pub struct GameManager {
    current_game: Box<dyn GameInstance>,
}

impl GameManager {
    pub fn new(ctx: &mut ggez::Context, quad_ctx: &mut ggez::event::GraphicsContext) -> Self {
        Self {
            current_game: Box::new(Game::<TitleScreenState>::new(ctx, quad_ctx)),
        }
    }

    pub fn handle_transition(&mut self, transition: Transition) {
        self.current_game = self.current_game.transition(transition);
    }
}

impl event::EventHandler<ggez::GameError> for GameManager {
    fn update(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
    ) -> GameResult {
        if let Ok(Some(transition)) = self.current_game.update(ctx, quad_ctx) {
            self.handle_transition(transition);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        self.current_game.draw(ctx, quad_ctx)
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if let Some(transition) = self
            .current_game
            .mouse_button_up_event(ctx, quad_ctx, button, x, y)
        {
            self.handle_transition(transition);
        }
    }

    fn resize_event(
        &mut self,
        ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
        width: f32,
        height: f32,
    ) {
        ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height))
            .expect("Failed to set screen coordinates");
    }
}

pub fn main() -> GameResult {
    init_random();
    ggez::start(
        ggez::conf::Conf::default()
            .cache(Some(include_bytes!("resources.tar")))
            .window_width(1400)
            .window_height(800)
            .window_title("Streets'n'Houses".to_string())
            .window_resizable(true),
        |context, quad_ctx| Box::new(GameManager::new(context, quad_ctx)),
    )
}

fn init_random() {
    let time = time() * 10_000_000.0;
    quad_rand::srand(time as u64);
}
