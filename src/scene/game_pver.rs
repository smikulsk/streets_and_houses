use crate::game::GameStatistics;
use crate::scene::prelude::*;

#[derive(Debug, Clone)]
pub struct GameOverScene {
    statistics: GameStatistics,
    retry_button_bounding_box: Rect,
}

impl GameOverScene {
    pub fn new(statistics: GameStatistics) -> Self {
        Self {
            statistics,
            retry_button_bounding_box: Rect::default(),
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
        draw_text(
            ctx,
            quad_ctx,
            width / 2.0 - 50.0,
            height / 2.0 - 60.0,
            &format!("Player2 points: {}", self.statistics.player2_points),
        )?;
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
        )?;

        graphics::present(ctx, quad_ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _quad_ctx: &mut ggez::miniquad::GraphicsContext,
        _button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Option<Transition> {
        let point = Point2::new(x, y);
        self.retry_button_bounding_box.contains(point).then(|| {
            let game = MainMenuScene::new();
            Transition::ToMainMenu(Box::new(game))
        })
    }
}
