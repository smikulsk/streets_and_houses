use ggez::graphics::{DrawParam, Point2};

use crate::scene::prelude::*;

#[derive(Debug)]
pub struct TitleScreenScene {
    image: graphics::Image,
    start_button_bounding_box: Rect,
}

impl TitleScreenScene {
    pub fn new(ctx: &mut ggez::Context, quad_ctx: &mut ggez::event::GraphicsContext) -> Self {
        let image =
            graphics::Image::new(ctx, quad_ctx, "ui/title_screen.png").expect("image is available");

        Self {
            image,
            start_button_bounding_box: Rect::default(),
        }
    }    
}

impl Scene for TitleScreenScene {
    type State = TitleScreenState;

    fn update(
        &mut self,
        _ctx: &mut ggez::Context,
        _quad_ctx: &mut ggez::event::GraphicsContext,
    ) -> Result<Option<Transition>, ggez::GameError> {
        Ok(None)
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, graphics::Color::BLACK);

        let scene_scale = get_scene_scale(quad_ctx);

        let translation = get_scene_translation(quad_ctx, scene_scale);

        let dst = Point2::new(translation.0, translation.1);

        graphics::draw(
            ctx,
            quad_ctx,
            &self.image,
            DrawParam::new()
                .dest(dst)
                .scale(Vector2::new(scene_scale.0, scene_scale.1)),
        )?;

        self.start_button_bounding_box = graphics::Rect::new(
            translation.0 + TITLE_SCREEN_START_BUTTON_X * scene_scale.0,
            translation.1 + TITLE_SCREEN_START_BUTTON_Y * scene_scale.1,
            TITLE_SCREEN_START_BUTTON_WIDTH * scene_scale.0,
            TITLE_SCREEN_START_BUTTON_HEIGHT * scene_scale.1,
        );

        #[cfg(feature = "draw_bounding_rects")]
        draw_bounding_rect(ctx, quad_ctx, self.start_button_bounding_box)?;

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

        self.start_button_bounding_box.contains(point).then(|| {
            let game = MainMenuScene::new(ctx, quad_ctx).expect("scene was created");

            Transition::ToMainMenu(Box::new(game))
        })
    }
}
