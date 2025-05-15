pub use super::contants::*;
pub use super::game_over::*;
pub use super::main_menu::*;
pub use super::playing::*;
pub use super::prepare_player::*;
pub use super::title_screen::*;
pub use super::*;

pub fn get_scene_scale(quad_ctx: &mut miniquad::Context) -> (f32, f32) {
    let (w, h) = quad_ctx.display().screen_size();

    let scene_scale_x = w / SCENE_WIDTH;
    let scene_scale_y = h / SCENE_HEIGHT;

    if scene_scale_x > scene_scale_y {
        return (scene_scale_y, scene_scale_y);
    }
    (scene_scale_x, scene_scale_x)
}

pub fn get_scene_translation(
    quad_ctx: &mut miniquad::Context,
    scene_scale: (f32, f32),
) -> (f32, f32) {
    let (w, h) = quad_ctx.display().screen_size();

    (
        (w - SCENE_WIDTH * scene_scale.0) / 2.0,
        (h - SCENE_HEIGHT * scene_scale.1) / 2.0,
    )
}

pub fn draw_bounding_rect(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::Context,
    bounding_rect: Rect,
) -> Result<(), ggez::GameError> {
    let dest_point = graphics::DrawParam::new().dest(Point2::new(0.0, 0.0));
    let rect = graphics::Mesh::new_rectangle(
        ctx,
        quad_ctx,
        graphics::DrawMode::stroke(1.0),
        bounding_rect,
        graphics::Color::WHITE,
    )?;
    graphics::draw(ctx, quad_ctx, &rect, dest_point)?;
    Ok(())
}

pub fn draw_selection_rect(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::Context,
    bounding_rect: Rect,
) -> GameResult {
    let dest_point = graphics::DrawParam::new().dest(Point2::new(bounding_rect.x, bounding_rect.y));
    let rect = graphics::Mesh::new_rectangle(
        ctx,
        quad_ctx,
        graphics::DrawMode::stroke(1.0),
        graphics::Rect::new(-15.0, -15.0, bounding_rect.w + 30.0, bounding_rect.h + 30.0),
        graphics::Color::WHITE,
    )?;

    graphics::draw(ctx, quad_ctx, &rect, dest_point)?;
    Ok(())
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
