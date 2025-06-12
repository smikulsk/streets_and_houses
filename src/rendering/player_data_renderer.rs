use super::prelude::*;
use super::*;
use crate::scene::prelude::*;

#[derive(Debug)]
pub struct PlayerDataRenderer {
    player: Player,
    image_title: graphics::Image,
    image_points: Vec<graphics::Image>,
    orientation: Orientation,
}

impl PlayerDataRenderer {
    pub fn new(
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        player: Player,
        points: usize,
        is_player_turn : bool,
    ) -> GameResult<Self> {
        let image_title = match player {
            Player::Player1 if is_player_turn => graphics::Image::new(ctx, quad_ctx, "ui/player_1_u.png")?,
            Player::Player1  => graphics::Image::new(ctx, quad_ctx, "ui/player_1.png")?,
            Player::Player2 if is_player_turn => graphics::Image::new(ctx, quad_ctx, "ui/player_2_u.png")?,
            Player::Player2 => graphics::Image::new(ctx, quad_ctx, "ui/player_2.png")?,
            Player::CPU if is_player_turn => graphics::Image::new(ctx, quad_ctx, "ui/CPU_u.png")?,
            Player::CPU  => graphics::Image::new(ctx, quad_ctx, "ui/CPU.png")?,
        };
        let image_points = convert_points_to_list_of_images(ctx, quad_ctx, points)?;
        let orientation = get_scene_orientation(quad_ctx);

        Ok(Self {
            player,
            image_title,
            image_points,
            orientation,
        })
    }

    pub fn set_points(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        points: usize,
    ) -> GameResult {
        self.image_points = convert_points_to_list_of_images(ctx, quad_ctx, points)?;
        Ok(())
    }

    pub fn set_orientation(&mut self, quad_ctx: &mut miniquad::GraphicsContext) {
        self.orientation = get_scene_orientation(quad_ctx);
    }

    fn draw_player_points_horizontally(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
        scene_scale: (f32, f32),
        translation: (f32, f32),
    ) -> Result<(), ggez::GameError> {        
        graphics::draw(
            ctx,
            quad_ctx,
            &self.image_title,
            graphics::DrawParam::new()
                .dest(Point2::new(
                    PLAYER_DATA_PANEL_TITLE_X_HORZ * scene_scale.0 + translation.0,
                    PLAYER_DATA_PANEL_TITLE_Y_HORZ * scene_scale.1 + translation.1,
                ))
                .scale(Vector2::new(scene_scale.0, scene_scale.1)),
        )?;
        for (idx, image) in self.image_points.iter().enumerate() {
            graphics::draw(
                ctx,
                quad_ctx,
                image,
                graphics::DrawParam::new()
                    .dest(Point2::new(
                        (PLAYER_DATA_PANEL_POINTS_X_HORZ
                            + 2.0 * idx as f32 * MAIN_MENU_DIGIT_WIDTH / 3.0)
                            * scene_scale.0
                            + translation.0,
                        PLAYER_DATA_PANEL_POINTS_Y_HORZ * scene_scale.1 + translation.1,
                    ))
                    .scale(Vector2::new(
                        MAIN_MENU_DIGIT_WIDTH / image.width() as f32 * scene_scale.0,
                        MAIN_MENU_DIGIT_HEIGHT / image.height() as f32 * scene_scale.1,
                    )),
            )?;
        }
        Ok(())
    }

    fn draw_player_points_vertically(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
        scene_scale: (f32, f32),
        translation: (f32, f32),        
    ) -> Result<(), ggez::GameError> {
        graphics::draw(
            ctx,
            quad_ctx,
            &self.image_title,
            graphics::DrawParam::new()
                .dest(Point2::new(
                    PLAYER_DATA_PANEL_TITLE_X_VERT * scene_scale.0 + translation.0,
                    PLAYER_DATA_PANEL_TITLE_Y_VERT * scene_scale.1 + translation.1,
                ))
                .scale(Vector2::new(scene_scale.0, scene_scale.1)),
        )?;
        for (idx, image) in self.image_points.iter().enumerate() {
            graphics::draw(
                ctx,
                quad_ctx,
                image,
                graphics::DrawParam::new()
                    .dest(Point2::new(
                        (PLAYER_DATA_PANEL_POINTS_X_VERT
                            + 2.0 * idx as f32 * MAIN_MENU_DIGIT_WIDTH / 3.0)
                            * scene_scale.0
                            + translation.0,
                        PLAYER_DATA_PANEL_POINTS_Y_VERT * scene_scale.1 + translation.1,
                    ))
                    .scale(Vector2::new(
                        MAIN_MENU_DIGIT_WIDTH / image.width() as f32 * scene_scale.0,
                        MAIN_MENU_DIGIT_HEIGHT / image.height() as f32 * scene_scale.1,
                    )),
            )?;
        }
        Ok(())
    }

    fn get_translation(
        &mut self,
        quad_ctx: &mut miniquad::Context,
        scene_scale: (f32, f32),
    ) -> (f32, f32) {
        let mut translation = get_scene_translation(quad_ctx, scene_scale);
        let (w, h) = quad_ctx.display().screen_size();

        match self.player {
            Player::Player1 => match self.orientation {
                Orientation::Horizontal => translation.0 = 0.0,
                Orientation::Vertical => translation.1 = 0.0,
            },
            Player::CPU | Player::Player2 => match self.orientation {
                Orientation::Horizontal => translation.0 = w - translation.0,
                Orientation::Vertical => translation.1 = h - translation.1,
            },
        }
        translation
    }
}

impl Renderer for PlayerDataRenderer {
    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        let scene_scale = get_scene_scale(quad_ctx);
        let translation = self.get_translation(quad_ctx, scene_scale);

        match self.orientation {
            Orientation::Horizontal => {
                self.draw_player_points_horizontally(ctx, quad_ctx, scene_scale, translation)?
            }
            Orientation::Vertical => {
                self.draw_player_points_vertically(ctx, quad_ctx, scene_scale, translation)?
            }
        }

        graphics::present(ctx, quad_ctx)?;

        Ok(())
    }
}
