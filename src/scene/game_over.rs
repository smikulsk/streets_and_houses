use crate::game::GameStatistics;
use crate::scene::prelude::*;

#[derive(Debug)]
pub struct GameOverScene {
    statistics: GameStatistics,
    retry_button_bounding_box: Rect,
    is_one_player_game: bool,
    width: usize,
    height: usize,
}

impl GameOverScene {
    pub fn new(
        statistics: GameStatistics,
        game_mode: &GameMode,
        width: usize,
        height: usize,
    ) -> Self {
        let is_one_player_game = match game_mode {
            GameMode::OnePlayer(_) => true,
            GameMode::TwoPlayer => false,
        };
        Self {
            statistics,
            retry_button_bounding_box: Rect::default(),
            is_one_player_game,
            width,
            height,
        }
    }
}

impl Scene for GameOverScene {
    type State = GameOverState;

    fn update(
        &mut self,
        _ctx: &mut ggez::Context,
        _quad_ctx: &mut ggez::event::GraphicsContext,
    ) -> Result<Option<Transition>, ggez::GameError> {
        Ok(None)
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, graphics::Color::BLACK);

        let (width, height) = graphics::drawable_size(quad_ctx);

        let result_str = if let Some(player) = self.statistics.winner {
            &format!("{:?} wins!!!", player)
        } else {
            "It is a tie!!!"
        };

        draw_text(
            ctx,
            quad_ctx,
            width / 2.0 - 50.0,
            height / 2.0 - 100.0,
            &format!("Player1 points: {}", self.statistics.player1_points),
        )?;
        if self.is_one_player_game {
            draw_text(
                ctx,
                quad_ctx,
                width / 2.0 - 50.0,
                height / 2.0 - 60.0,
                &format!("CPU points: {}", self.statistics.cpu_points),
            )?;
        } else {
            draw_text(
                ctx,
                quad_ctx,
                width / 2.0 - 50.0,
                height / 2.0 - 60.0,
                &format!("Player2 points: {}", self.statistics.player2_points),
            )?;
        }
        draw_text(
            ctx,
            quad_ctx,
            width / 2.0 - 50.0,
            height / 2.0 - 20.0,
            result_str,
        )?;
        self.retry_button_bounding_box = draw_button(
            ctx,
            quad_ctx,
            width / 2.0,
            height / 2.0 + 40.0,
            "Click to restart the game!",
            false,
        )?;

        graphics::present(ctx, quad_ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        _button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Option<Transition> {
        let point = Point2::new(x, y);
        self.retry_button_bounding_box.contains(point).then(|| {
            let game = MainMenuScene::from(
                ctx,
                quad_ctx,
                self.width,
                self.height,
                self.is_one_player_game,
            );
            Transition::ToMainMenu(Box::new(game.expect("scene has been created")))
        })
    }
}
