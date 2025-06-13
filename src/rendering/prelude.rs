pub use super::board_renderer::*;
pub use super::button_renderer::*;
pub use super::constants::*;
pub use super::player_data_renderer::*;
pub use super::ui::*;

use super::*;

#[derive(Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub fn draw_text(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::Context,
    mid_x: f32,
    mid_y: f32,
    caption: &str,
) -> Result<Rect, ggez::GameError> {
    let text = graphics::Text::new(caption);

    let text_height = text.height(ctx);
    let text_width = text.width(ctx);
    let x = mid_x - text_width / 2.0;
    let y = mid_y - text_height / 2.0;
    let dest_point = graphics::DrawParam::new().dest(Point2::new(x, y));

    graphics::draw(ctx, quad_ctx, &text, dest_point)?;

    Ok(Rect::new(x, y, text_width, text_height))
}

pub fn get_scene_orientation(quad_ctx: &mut miniquad::Context) -> Orientation {
    let (w, h) = quad_ctx.display().screen_size();

    if w > h {
        Orientation::Horizontal
    } else {
        Orientation::Vertical
    }
}
