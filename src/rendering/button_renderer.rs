use super::*;
use crate::scene::prelude::*;

#[derive(Debug)]
pub struct ButtonRenderer {
    image: graphics::Image,
    pos_x: f32,
    pos_y: f32,
    bounding_rect : Rect,
}

impl ButtonRenderer {
    pub fn new(
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        image_path: &str,
        pos_x: f32,
        pos_y: f32,
    ) -> GameResult<Self> {
        let image = graphics::Image::new(ctx, quad_ctx, image_path)?;
        Ok(Self {
            image,
            pos_x,
            pos_y,
            bounding_rect : Rect::default(),
        })
    }

    pub fn get_bouding_rect(&self) -> Rect {
        self.bounding_rect
    }
}

impl Renderer for ButtonRenderer {
    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        let scene_scale = get_scene_scale(quad_ctx);
        let (w, _) = quad_ctx.screen_size();

        self.bounding_rect = Rect::new(
            w - (self.pos_x + self.image.width() as f32) * scene_scale.0,
            self.pos_y * scene_scale.1,
            self.image.width() as f32 * scene_scale.0,
            self.image.height() as f32 * scene_scale.1,
        );
        graphics::draw(
            ctx,
            quad_ctx,
            &self.image,
            graphics::DrawParam::new()
                .dest(Point2::new(self.bounding_rect.x, self.bounding_rect.y))
                .scale(Vector2::new(scene_scale.0, scene_scale.1)),
        )
    }
}
