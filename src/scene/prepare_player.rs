use std::time::{Duration, Instant};

use crate::game::Player;
use crate::scene::prelude::*;

#[derive(Debug)]
pub struct PreparePlayerScene {
    player: Player,
    board: Board,
    game_mode: GameMode,
    start_time : Instant,
}

impl PreparePlayerScene {
    pub fn new(player: Player, board: &Board, game_mode: &GameMode) -> Self {
        Self {
            player,
            board: board.clone(),
            game_mode: game_mode.clone(),
            start_time :Instant::now()
        }
    }
}

impl Scene for PreparePlayerScene {
    type State = PreparePlayerState;

    fn update(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::event::GraphicsContext,
    ) -> Result<Option<Transition>, ggez::GameError> {
        if self.start_time.elapsed() >= Duration::from_secs_f32(PREPARE_PLAYER_DURATION) {
            let game = PlayingScene::new(
                ctx,
                quad_ctx,
                self.player,
                self.board.clone(),
                self.game_mode.clone(),
            )
            .expect("board was initialized");
            
            return Ok(Some(Transition::ToPlaying(Box::new(game))));
        }
        Ok(None)
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, graphics::Color::BLACK);

        let (width, height) = graphics::drawable_size(quad_ctx);

        draw_text(
            ctx,
            quad_ctx,
            width / 2.0,
            height / 2.0,
            &format!("{:?}'s turn", self.player),
        )?;

        graphics::present(ctx, quad_ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _quad_ctx: &mut ggez::miniquad::GraphicsContext,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Option<Transition> {
        None
    }
}
