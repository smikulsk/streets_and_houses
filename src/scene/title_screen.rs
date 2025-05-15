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

        let tile_size = self.get_tile_size(quad_ctx);

        let translation = get_scene_translation(quad_ctx, tile_size);

        let dst = Point2::new(translation.0, translation.1);

        graphics::draw(
            ctx,
            quad_ctx,
            &self.image,
            DrawParam::new()
                .dest(dst)
                .scale(Vector2::new(tile_size.0, tile_size.1)),
        )?;

        self.start_button_bounding_box = graphics::Rect::new(
            translation.0 + START_BUTTON_X * tile_size.0,
            translation.1 + START_BUTTON_Y * tile_size.1,
            START_BUTTON_WIDTH * tile_size.0,
            START_BUTTON_HEIGHT * tile_size.1,
        );

        let dest_point = graphics::DrawParam::new().dest(Point2::new(0.0, 0.0));

        let rect = graphics::Mesh::new_rectangle(
            ctx,
            quad_ctx,
            graphics::DrawMode::stroke(1.0),
            self.start_button_bounding_box,
            graphics::Color::WHITE,
        )?;

        graphics::draw(ctx, quad_ctx, &rect, dest_point)?;

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

        self.start_button_bounding_box.contains(point).then(|| {
            let game = MainMenuScene::new();

            Transition::ToMainMenu(Box::new(game))
        })
    }
}