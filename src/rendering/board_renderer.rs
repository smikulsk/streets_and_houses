use super::prelude::*;
use super::*;

#[derive(Debug)]
pub struct BoardRenderer {
    spritebatch_empty: graphics::spritebatch::SpriteBatch,
    spritebatch_path_h: graphics::spritebatch::SpriteBatch,
    spritebatch_path_v: graphics::spritebatch::SpriteBatch,
    spritebatch_street_h: graphics::spritebatch::SpriteBatch,
    spritebatch_street_v: graphics::spritebatch::SpriteBatch,
    spritebatch_a: graphics::spritebatch::SpriteBatch,
    spritebatch_b: graphics::spritebatch::SpriteBatch,
    spritebatch_joints: Vec<graphics::spritebatch::SpriteBatch>,
    board: Board,
    player: Player,
    wall_bounding_boxes: Vec<Vec<Rect>>,
    draw_footer: bool,
}

impl BoardRenderer {
    pub fn new(
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        player: Player,
        board: Board,
        draw_footer: bool,
    ) -> GameResult<BoardRenderer> {
        let image_empty = graphics::Image::new(ctx, quad_ctx, "roads/empty.png")?;
        let image_path_h = graphics::Image::new(ctx, quad_ctx, "roads/path_h.png")?;
        let image_path_v = graphics::Image::new(ctx, quad_ctx, "roads/path_v.png")?;
        let image_street_h = graphics::Image::new(ctx, quad_ctx, "roads/street_h.png")?;
        let image_street_v = graphics::Image::new(ctx, quad_ctx, "roads/street_v.png")?;
        let image_a = graphics::Image::new(ctx, quad_ctx, "buildings/textured_red.png")?;
        let image_b = graphics::Image::new(ctx, quad_ctx, "buildings/textured_green.png")?;
        let batch_empty = graphics::spritebatch::SpriteBatch::new(image_empty);
        let batch_path_h = graphics::spritebatch::SpriteBatch::new(image_path_h);
        let batch_path_v = graphics::spritebatch::SpriteBatch::new(image_path_v);
        let batch_street_h = graphics::spritebatch::SpriteBatch::new(image_street_h);
        let batch_street_v = graphics::spritebatch::SpriteBatch::new(image_street_v);
        let batch_a = graphics::spritebatch::SpriteBatch::new(image_a);
        let batch_b = graphics::spritebatch::SpriteBatch::new(image_b);
        let bacth_joints = generate_joint_spritesheets(ctx, quad_ctx)?;
        let wall_bounding_boxes =
            vec![vec![Rect::default(); board.width + 1]; 2 * board.height + 1];

        let s = BoardRenderer {
            spritebatch_empty: batch_empty,
            spritebatch_path_h: batch_path_h,
            spritebatch_path_v: batch_path_v,
            spritebatch_street_h: batch_street_h,
            spritebatch_street_v: batch_street_v,
            spritebatch_a: batch_a,
            spritebatch_b: batch_b,
            spritebatch_joints: bacth_joints,
            board,
            player,
            wall_bounding_boxes,
            draw_footer,
        };
        Ok(s)
    }

    fn get_tile_size(&self, quad_ctx: &mut miniquad::Context, footer_height: f32) -> (f32, f32) {
        let (w, h) = quad_ctx.display().screen_size();

        let tile_size_x = IMAGE_WIDTH * w
            / ((IMAGE_WIDTH + V_STREET_WIDTH) * self.board.width as f32 + V_STREET_WIDTH);
        let tile_size_y = IMAGE_HEIGHT * (h - footer_height)
            / ((IMAGE_HEIGHT + H_STREET_HEIGHT) * self.board.height as f32 + H_STREET_HEIGHT);

        if tile_size_x > tile_size_y {
            return (tile_size_y, tile_size_y);
        }
        (tile_size_x, tile_size_x)
    }

    fn get_translation(
        &self,
        quad_ctx: &mut miniquad::Context,
        footer_height: f32,
        tile_size: (f32, f32),
    ) -> (f32, f32) {
        let (w, h) = quad_ctx.display().screen_size();

        (
            (w - ((IMAGE_WIDTH + V_STREET_WIDTH) * self.board.width as f32 + V_STREET_WIDTH)
                / IMAGE_WIDTH
                * tile_size.0)
                / 2.0,
            (h - footer_height
                - ((IMAGE_HEIGHT + H_STREET_HEIGHT) * self.board.height as f32 + H_STREET_HEIGHT)
                    / IMAGE_WIDTH
                    * tile_size.1)
                / 2.0,
        )
    }

    fn add_cell_sprite(
        &mut self,
        tile_size: (f32, f32),
        translation: (f32, f32),
        row: usize,
        col: usize,
    ) {
        let r = row as f32;
        let c = col as f32;
        let p = graphics::DrawParam::new()
            .dest(Point2::new(
                tile_size.0 * V_STREET_WIDTH / IMAGE_WIDTH
                    + (IMAGE_WIDTH + V_STREET_WIDTH) * c * tile_size.0 / IMAGE_WIDTH
                    + translation.0,
                ((IMAGE_HEIGHT + H_STREET_HEIGHT) * r / 2.0 / IMAGE_HEIGHT
                    + H_STREET_HEIGHT / IMAGE_HEIGHT)
                    * tile_size.1
                    + translation.1,
            ))
            .scale(Vector2::new(
                tile_size.0 / IMAGE_WIDTH,
                tile_size.1 / IMAGE_HEIGHT,
            ));

        if row < 2 * self.board.height {
            if let Some(player) = self.board.cells[row / 2][col].owner {
                match player {
                    game::Player::Player1 => self.spritebatch_a.add(p),
                    game::Player::Player2 | game::Player::CPU => self.spritebatch_b.add(p),
                };
            } else {
                self.spritebatch_empty.add(p);
            }
        }
    }

    fn add_wall_sprite(
        &mut self,
        tile_size: (f32, f32),
        row: usize,
        col: usize,
        dest: Point2<f32>,
    ) {
        let p = graphics::DrawParam::new().dest(dest).scale(Vector2::new(
            tile_size.0 / IMAGE_WIDTH,
            tile_size.1 / IMAGE_HEIGHT,
        ));
        if row % 2 == 0 {
            if self.board.walls[row][col].is_clicked {
                self.spritebatch_street_h.add(p);
            } else {
                self.spritebatch_path_h.add(p);
            }
        } else if self.board.walls[row][col].is_clicked {
            self.spritebatch_street_v.add(p);
        } else {
            self.spritebatch_path_v.add(p);
        }
    }

    fn add_joint_sprite(
        &mut self,
        tile_size: (f32, f32),
        row: usize,
        col: usize,
        dest: Point2<f32>,
    ) {
        let p = graphics::DrawParam::new().dest(dest).scale(Vector2::new(
            tile_size.0 / IMAGE_WIDTH,
            tile_size.1 / IMAGE_HEIGHT,
        ));
        let joint_idx = self.board.joints[row / 2][col].get_joint_mask();
        self.spritebatch_joints[joint_idx].add(p);
    }

    fn draw_footer(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<Rect, ggez::GameError> {
        if !self.draw_footer {
            return Ok(Rect::default());
        }

        let (width, height) = graphics::drawable_size(quad_ctx);
        let footer_rect = draw_text(
            ctx,
            quad_ctx,
            width / 2.0,
            height - 15.0,
            &format!(
                "========================= {:?} =========================",
                self.player
            ),
        )?;
        Ok(Rect::new(
            footer_rect.x,
            footer_rect.y,
            footer_rect.w,
            footer_rect.h + 15.0,
        ))
    }

    pub fn get_wall_bounding_boxes(&self) -> Vec<Vec<Rect>> {
        self.wall_bounding_boxes.clone()
    }

    pub fn set_board(&mut self, board: &Board) {
        self.board = board.clone();
    }
}

impl Renderer for BoardRenderer {
    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, graphics::Color::BLACK);

        let footer_rect = self.draw_footer(ctx, quad_ctx)?;

        let tile_size = self.get_tile_size(quad_ctx, footer_rect.h);
        let translation = self.get_translation(quad_ctx, footer_rect.h, tile_size);

        for row in 0..2 * self.board.height + 1 {
            if row % 2 == 0 {
                for col in 0..self.board.width {
                    let dest =
                        get_horizontal_wall_sprite_destination(tile_size, translation, row, col);
                    self.add_wall_sprite(tile_size, row, col, dest);
                    self.wall_bounding_boxes[row][col] =
                        get_horizontal_wall_sprite_bounding_box(tile_size, translation, row, col);

                    self.add_cell_sprite(tile_size, translation, row, col);
                }
            } else {
                for col in 0..self.board.width + 1 {
                    let mut dest =
                        get_vertical_wall_sprite_destination(tile_size, translation, row, col);
                    self.add_wall_sprite(tile_size, row, col, dest);
                    self.wall_bounding_boxes[row][col] =
                        get_vertical_wall_sprite_bounding_box(tile_size, translation, row, col);

                    dest.y -= H_STREET_HEIGHT * tile_size.1 / IMAGE_HEIGHT;
                    self.add_joint_sprite(tile_size, row, col, dest);

                    if row == 2 * self.board.height - 1 {
                        dest.y += (IMAGE_HEIGHT + H_STREET_HEIGHT) * tile_size.1 / IMAGE_HEIGHT;
                        self.add_joint_sprite(tile_size, row + 1, col, dest);
                    }
                }
            }
        }

        for spritebatch in [
            &mut self.spritebatch_empty,
            &mut self.spritebatch_path_h,
            &mut self.spritebatch_path_v,
            &mut self.spritebatch_street_h,
            &mut self.spritebatch_street_v,
            &mut self.spritebatch_a,
            &mut self.spritebatch_b,
        ] {
            graphics::draw(ctx, quad_ctx, spritebatch, graphics::DrawParam::new())?;
            spritebatch.clear();
        }

        for spritebatch in &mut self.spritebatch_joints {
            graphics::draw(ctx, quad_ctx, spritebatch, graphics::DrawParam::new())?;
            spritebatch.clear();
        }

        graphics::present(ctx, quad_ctx)?;

        Ok(())
    }
}

fn generate_joint_spritesheets(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::GraphicsContext,
) -> GameResult<Vec<graphics::spritebatch::SpriteBatch>> {
    let images = (0..16)
        .map(|n| format!("crossroads/{}.png", get_bit_mask(n)))
        .map(|path| graphics::Image::new(ctx, quad_ctx, path))
        .collect::<GameResult<Vec<_>>>();

    Ok(images?
        .iter()
        .map(|image| graphics::spritebatch::SpriteBatch::new(image.clone()))
        .collect::<Vec<_>>())
}

fn get_horizontal_wall_sprite_destination(
    tile_size: (f32, f32),
    translation: (f32, f32),
    row: usize,
    col: usize,
) -> Point2<f32> {
    let r = row as f32;
    let c = col as f32;

    Point2::new(
        V_STREET_WIDTH * tile_size.0 / IMAGE_WIDTH
            + (IMAGE_WIDTH + V_STREET_WIDTH) * c * tile_size.0 / IMAGE_WIDTH
            + translation.0,
        (IMAGE_HEIGHT + H_STREET_HEIGHT) * r * tile_size.1 / 2.0 / IMAGE_HEIGHT + translation.1,
    )
}

fn get_horizontal_wall_sprite_bounding_box(
    tile_size: (f32, f32),
    translation: (f32, f32),
    row: usize,
    col: usize,
) -> Rect {
    let r = row as f32;
    let c = col as f32;
    Rect::new(
        ((IMAGE_WIDTH + V_STREET_WIDTH) * c + V_STREET_WIDTH) * tile_size.0 / IMAGE_WIDTH
            + translation.0,
        (IMAGE_HEIGHT + H_STREET_HEIGHT) * r * tile_size.1 / 2.0 / IMAGE_HEIGHT + translation.1,
        tile_size.0,
        tile_size.1 * H_STREET_HEIGHT / IMAGE_HEIGHT,
    )
}

fn get_vertical_wall_sprite_destination(
    tile_size: (f32, f32),
    translation: (f32, f32),
    row: usize,
    col: usize,
) -> Point2<f32> {
    let r = row as f32;
    let c = col as f32;

    Point2::new(
        (IMAGE_WIDTH + V_STREET_WIDTH) * c * tile_size.0 / IMAGE_WIDTH + translation.0,
        H_STREET_HEIGHT * tile_size.1 / IMAGE_HEIGHT
            + (IMAGE_HEIGHT + H_STREET_HEIGHT) * (r - 1.0) * tile_size.1 / 2.0 / IMAGE_HEIGHT
            + translation.1,
    )
}

fn get_vertical_wall_sprite_bounding_box(
    tile_size: (f32, f32),
    translation: (f32, f32),
    row: usize,
    col: usize,
) -> Rect {
    let r = row as f32;
    let c = col as f32;
    Rect::new(
        (IMAGE_WIDTH + V_STREET_WIDTH) * c * tile_size.0 / IMAGE_WIDTH + translation.0,
        H_STREET_HEIGHT * tile_size.1 / IMAGE_HEIGHT
            + (IMAGE_HEIGHT + H_STREET_HEIGHT) * (r - 1.0) * tile_size.1 / 2.0 / IMAGE_HEIGHT
            + translation.1,
        tile_size.0 * V_STREET_WIDTH / IMAGE_WIDTH,
        tile_size.1,
    )
}

fn get_bit_mask(n: usize) -> String {
    [8, 4, 2, 1]
        .iter()
        .map(|pow_of_2| u8::from(n & pow_of_2 > 0).to_string())
        .fold(String::new(), |acc, x| acc + &x)
}

#[cfg(test)]
mod get_bit_mask_tests {
    use super::*;

    #[test]
    fn joint_masks_are_generated_correctly() {
        let expected_masks = [
            "0000".to_string(),
            "0001".to_string(),
            "0010".to_string(),
            "0011".to_string(),
            "0100".to_string(),
            "0101".to_string(),
            "0110".to_string(),
            "0111".to_string(),
            "1000".to_string(),
            "1001".to_string(),
            "1010".to_string(),
            "1011".to_string(),
            "1100".to_string(),
            "1101".to_string(),
            "1110".to_string(),
            "1111".to_string(),
        ];
        for (n, expected_mask) in expected_masks.iter().enumerate() {
            assert_eq!(expected_mask, &get_bit_mask(n));
        }
    }
}
