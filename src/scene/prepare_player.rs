use std::time::Duration;

use ggez::graphics::{DrawParam, Point2};

use crate::game::Player;
use crate::scene::prelude::*;

#[derive(Debug)]
pub struct PreparePlayerScene {
    player: Player,
    board: Board,
    game_mode: GameMode,
    start_time: Option<Duration>,
    image_player1: graphics::Image,
    image_player2: graphics::Image,
    image_cpu: graphics::Image,
}

impl PreparePlayerScene {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::event::GraphicsContext,
        player: Player,
        board: &Board,
        game_mode: &GameMode,
    ) -> Self {
        let image_player1 = graphics::Image::new(ctx, quad_ctx, "ui/player_1_turn.png")
            .expect("image is available");
        let image_player2 = graphics::Image::new(ctx, quad_ctx, "ui/player_2_turn.png")
            .expect("image is available");
        let image_cpu =
            graphics::Image::new(ctx, quad_ctx, "ui/cpu_turn.png").expect("image is available");

        Self {
            player,
            board: board.clone(),
            game_mode: game_mode.clone(),
            start_time: None,
            image_player1,
            image_player2,
            image_cpu,
        }
    }

    fn get_tile_size(&self, quad_ctx: &mut miniquad::Context) -> (f32, f32) {
        let (w, h) = quad_ctx.display().screen_size();

        let tile_size_x = w / SCENE_WIDTH;
        let tile_size_y = h / SCENE_HEIGHT;

        if tile_size_x > tile_size_y {
            return (tile_size_y, tile_size_y);
        }
        (tile_size_x, tile_size_x)
    }
}

impl Scene for PreparePlayerScene {
    type State = PreparePlayerState;

    fn update(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::event::GraphicsContext,
    ) -> Result<Option<Transition>, ggez::GameError> {
        if self.start_time.is_none() {
            self.start_time = Some(get_time());
        }
        if let Some(start_time) = self.start_time {
            if start_time.abs_diff(get_time()) >= Duration::from_secs_f32(PREPARE_PLAYER_DURATION) {
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
        }
        Ok(None)
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, graphics::Color::BLACK);

        let tile_size = self.get_tile_size(quad_ctx);
        let translation = get_scene_translation(quad_ctx, tile_size);

        let dst = Point2::new(translation.0, translation.1);

        match self.player {
            Player::Player1 => graphics::draw(
                ctx,
                quad_ctx,
                &self.image_player1,
                DrawParam::new()
                    .dest(dst)
                    .scale(Vector2::new(tile_size.0, tile_size.1)),
            )?,
            Player::Player2 => graphics::draw(
                ctx,
                quad_ctx,
                &self.image_player2,
                DrawParam::new()
                    .dest(dst)
                    .scale(Vector2::new(tile_size.0, tile_size.1)),
            )?,
            Player::CPU => graphics::draw(
                ctx,
                quad_ctx,
                &self.image_cpu,
                DrawParam::new()
                    .dest(dst)
                    .scale(Vector2::new(tile_size.0, tile_size.1)),
            )?,
        }

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

fn get_time() -> Duration {
    ggez::timer::f64_to_duration(ggez::timer::time())
}
