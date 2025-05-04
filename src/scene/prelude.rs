pub use super::game_over::*;
pub use super::main_menu::*;
pub use super::playing::*;
pub use super::*;

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

pub fn draw_button(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::Context,
    mid_x: f32,
    mid_y: f32,
    caption: &str,
    is_clicked: bool,
) -> Result<Rect, ggez::GameError> {
    let text_bounding_box = draw_text(ctx, quad_ctx, mid_x, mid_y, caption)?;

    let dest_point =
        graphics::DrawParam::new().dest(Point2::new(text_bounding_box.x, text_bounding_box.y));

    let rect = graphics::Mesh::new_rectangle(
        ctx,
        quad_ctx,
        graphics::DrawMode::stroke(1.0),
        graphics::Rect::new(
            -10.0,
            -10.0,
            text_bounding_box.w + 20.0,
            text_bounding_box.h + 20.0,
        ),
        graphics::Color::WHITE,
    )?;

    graphics::draw(ctx, quad_ctx, &rect, dest_point)?;

    if is_clicked {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            quad_ctx,
            graphics::DrawMode::stroke(1.0),
            graphics::Rect::new(
                -15.0,
                -15.0,
                text_bounding_box.w + 30.0,
                text_bounding_box.h + 30.0,
            ),
            graphics::Color::WHITE,
        )?;

        graphics::draw(ctx, quad_ctx, &rect, dest_point)?;
    }

    Ok(Rect::new(
        text_bounding_box.x - 10.0,
        text_bounding_box.y - 10.0,
        text_bounding_box.w + 20.0,
        text_bounding_box.h + 20.0,
    ))
}
