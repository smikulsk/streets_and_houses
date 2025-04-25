use crate::game::Board;
use crate::scene::prelude::*;

#[derive(Debug, Clone)]
pub struct MainMenuScene {
    width: usize,
    height: usize,
    width_button_bounding_box: Rect,
    height_button_bounding_box: Rect,
    start_button_bounding_box: Rect,
}

impl MainMenuScene {
    pub fn new() -> Self {
        Self {
            width: 3,
            height: 3,
            width_button_bounding_box: Rect::default(),
            height_button_bounding_box: Rect::default(),
            start_button_bounding_box: Rect::default(),
        }
    }
}

impl Default for MainMenuScene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene for MainMenuScene {
    type State = MainMenuState;

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

        draw_text(
            ctx,
            quad_ctx,
            width / 2.0 - 50.0,
            height / 2.0 - 60.0,
            "Width:",
        )?;
        self.width_button_bounding_box = draw_button(
            ctx,
            quad_ctx,
            width / 2.0,
            height / 2.0 - 60.0,
            &self.width.to_string(),
        )?;
        draw_text(
            ctx,
            quad_ctx,
            width / 2.0 - 50.0,
            height / 2.0 - 20.0,
            "Height:",
        )?;
        self.height_button_bounding_box = draw_button(
            ctx,
            quad_ctx,
            width / 2.0,
            height / 2.0 - 20.0,
            &self.height.to_string(),
        )?;
        self.start_button_bounding_box = draw_button(
            ctx,
            quad_ctx,
            width / 2.0,
            height / 2.0 + 40.0,
            "Click to start the game!",
        )?;

        graphics::present(ctx, quad_ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Option<Transition> {
        let point = Point2::new(x, y);
        self.height_button_bounding_box.contains(point).then(|| {
            button
                .eq(&event::MouseButton::Left)
                .then(|| self.height = std::cmp::min(self.height + 1, 9));
            button
                .eq(&event::MouseButton::Right)
                .then(|| self.height = std::cmp::max(self.height - 1, 1));
        });

        self.width_button_bounding_box.contains(point).then(|| {
            button
                .eq(&event::MouseButton::Left)
                .then(|| self.width = std::cmp::min(self.width + 1, 9));
            button
                .eq(&event::MouseButton::Right)
                .then(|| self.width = std::cmp::max(self.width - 1, 1));
        });

        self.start_button_bounding_box.contains(point).then(|| {
            let game = PlayingScene::new(ctx, quad_ctx, Board::new(self.width, self.height))
                .expect("board was initialized");
            Transition::ToPlaying(Box::new(game))
        })
    }
}
