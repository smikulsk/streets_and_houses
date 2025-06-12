use crate::ai::prelude::*;
use crate::game::Difficulty;
use crate::rendering::ui::{RadioButton, SceneTransformation};
use crate::scene::prelude::*;

#[derive(Debug)]
pub struct MainMenuScene {
    width: usize,
    height: usize,
    one_player_bounding_box: Rect,
    two_players_bounding_box: Rect,
    easy_difficulty_bounding_box: Rect,
    medium_difficulty_bounding_box: Rect,
    hard_difficulty_bounding_box: Rect,
    width_decr_button_bounding_box: Rect,
    width_incr_button_bounding_box: Rect,
    height_decr_button_bounding_box: Rect,
    height_incr_button_bounding_box: Rect,
    start_button_bounding_box: Rect,
    one_player_game: bool,
    difficulty: game::Difficulty,
    image_background: graphics::Image,
    image_plus: graphics::Image,
    image_minus: graphics::Image,
    image_width: graphics::Image,
    image_height: graphics::Image,
    image_one_player: graphics::Image,
    image_two_players: graphics::Image,
    image_disabled_checked_radio: graphics::Image,
    image_disabled_unchecked_radio: graphics::Image,
    image_easy: graphics::Image,
    image_medium: graphics::Image,
    image_hard: graphics::Image,
    image_unchecked_radio: graphics::Image,
    image_checked_radio: graphics::Image,
    spritebatch_plus: graphics::spritebatch::SpriteBatch,
    spritebatch_minus: graphics::spritebatch::SpriteBatch,
    spritebatch_width: graphics::spritebatch::SpriteBatch,
    spritebatch_height: graphics::spritebatch::SpriteBatch,
}

impl MainMenuScene {
    pub fn new(ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult<Self> {
        let image_background = graphics::Image::new(ctx, quad_ctx, "ui/settings.png")?;
        let image_plus = graphics::Image::new(ctx, quad_ctx, "ui/plus.png")?;
        let image_minus = graphics::Image::new(ctx, quad_ctx, "ui/minus.png")?;
        let image_width =
            graphics::Image::new(ctx, quad_ctx, format!("ui/{:?}.png", DEFAULT_BOARD_WIDTH))?;
        let image_height =
            graphics::Image::new(ctx, quad_ctx, format!("ui/{:?}.png", DEFAULT_BOARD_HEIGHT))?;
        let image_one_player = graphics::Image::new(ctx, quad_ctx, "ui/1_player.png")?;
        let image_two_players = graphics::Image::new(ctx, quad_ctx, "ui/2_players.png")?;
        let image_disabled_checked_radio =
            graphics::Image::new(ctx, quad_ctx, "ui/disabled_checked_radio.png")?;
        let image_disabled_unchecked_radio =
            graphics::Image::new(ctx, quad_ctx, "ui/disabled_radio.png")?;
        let image_easy = graphics::Image::new(ctx, quad_ctx, "ui/easy.png")?;
        let image_medium = graphics::Image::new(ctx, quad_ctx, "ui/medium.png")?;
        let image_hard = graphics::Image::new(ctx, quad_ctx, "ui/hard.png")?;
        let image_unchecked_radio = graphics::Image::new(ctx, quad_ctx, "ui/unchecked_radio.png")?;
        let image_checked_radio = graphics::Image::new(ctx, quad_ctx, "ui/checked_radio.png")?;
        let batch_plus = graphics::spritebatch::SpriteBatch::new(image_plus.clone());
        let batch_minus = graphics::spritebatch::SpriteBatch::new(image_minus.clone());
        let batch_width = graphics::spritebatch::SpriteBatch::new(image_width.clone());
        let batch_height = graphics::spritebatch::SpriteBatch::new(image_height.clone());

        let s = MainMenuScene {
            width: DEFAULT_BOARD_WIDTH,
            height: DEFAULT_BOARD_HEIGHT,
            one_player_bounding_box: Rect::default(),
            two_players_bounding_box: Rect::default(),
            easy_difficulty_bounding_box: Rect::default(),
            medium_difficulty_bounding_box: Rect::default(),
            hard_difficulty_bounding_box: Rect::default(),
            width_decr_button_bounding_box: Rect::default(),
            width_incr_button_bounding_box: Rect::default(),
            height_decr_button_bounding_box: Rect::default(),
            height_incr_button_bounding_box: Rect::default(),
            start_button_bounding_box: Rect::default(),
            one_player_game: true,
            difficulty: Difficulty::Medium,
            image_background,
            image_plus,
            image_minus,
            image_width,
            image_height,
            image_one_player,
            image_two_players,
            image_disabled_checked_radio,
            image_disabled_unchecked_radio,
            image_easy,
            image_medium,
            image_hard,
            image_unchecked_radio,
            image_checked_radio,
            spritebatch_plus: batch_plus,
            spritebatch_minus: batch_minus,
            spritebatch_width: batch_width,
            spritebatch_height: batch_height,
        };
        Ok(s)
    }

    pub fn from(
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        width: usize,
        height: usize,
        one_player_game: bool,
        difficulty: Difficulty,
    ) -> GameResult<Self> {
        let image_background = graphics::Image::new(ctx, quad_ctx, "ui/settings.png")?;
        let image_plus = graphics::Image::new(ctx, quad_ctx, "ui/plus.png")?;
        let image_minus = graphics::Image::new(ctx, quad_ctx, "ui/minus.png")?;
        let image_width = graphics::Image::new(ctx, quad_ctx, format!("ui/{:?}.png", width))?;
        let image_height = graphics::Image::new(ctx, quad_ctx, format!("ui/{:?}.png", height))?;
        let image_one_player = graphics::Image::new(ctx, quad_ctx, "ui/1_player.png")?;
        let image_two_players = graphics::Image::new(ctx, quad_ctx, "ui/2_players.png")?;
        let image_disabled_checked_radio =
            graphics::Image::new(ctx, quad_ctx, "ui/disabled_checked_radio.png")?;
        let image_disabled_unchecked_radio =
            graphics::Image::new(ctx, quad_ctx, "ui/disabled_radio.png")?;
        let image_easy = graphics::Image::new(ctx, quad_ctx, "ui/easy.png")?;
        let image_medium = graphics::Image::new(ctx, quad_ctx, "ui/medium.png")?;
        let image_hard = graphics::Image::new(ctx, quad_ctx, "ui/hard.png")?;
        let image_unchecked_radio = graphics::Image::new(ctx, quad_ctx, "ui/unchecked_radio.png")?;
        let image_checked_radio = graphics::Image::new(ctx, quad_ctx, "ui/checked_radio.png")?;
        let batch_plus = graphics::spritebatch::SpriteBatch::new(image_plus.clone());
        let batch_minus = graphics::spritebatch::SpriteBatch::new(image_minus.clone());
        let batch_width = graphics::spritebatch::SpriteBatch::new(image_width.clone());
        let batch_height = graphics::spritebatch::SpriteBatch::new(image_height.clone());

        let s = Self {
            width,
            height,
            one_player_bounding_box: Rect::default(),
            two_players_bounding_box: Rect::default(),
            easy_difficulty_bounding_box: Rect::default(),
            medium_difficulty_bounding_box: Rect::default(),
            hard_difficulty_bounding_box: Rect::default(),
            width_decr_button_bounding_box: Rect::default(),
            width_incr_button_bounding_box: Rect::default(),
            height_decr_button_bounding_box: Rect::default(),
            height_incr_button_bounding_box: Rect::default(),
            start_button_bounding_box: Rect::default(),
            one_player_game,
            difficulty,
            image_background,
            image_plus,
            image_minus,
            image_width,
            image_height,
            image_one_player,
            image_two_players,
            image_disabled_checked_radio,
            image_disabled_unchecked_radio,
            image_easy,
            image_medium,
            image_hard,
            image_unchecked_radio,
            image_checked_radio,
            spritebatch_plus: batch_plus,
            spritebatch_minus: batch_minus,
            spritebatch_width: batch_width,
            spritebatch_height: batch_height,
        };
        Ok(s)
    }

    fn add_decr_button_sprite(
        &mut self,
        scene_scale: (f32, f32),
        translation: (f32, f32),
        x: f32,
        y: f32,
    ) -> Rect {
        let p = graphics::DrawParam::new()
            .dest(Point2::new(
                x * scene_scale.0 + translation.0,
                y * scene_scale.1 + translation.1,
            ))
            .scale(Vector2::new(
                MAIN_MENU_DECR_BUTTON_WIDTH / self.image_minus.width() as f32 * scene_scale.0,
                MAIN_MENU_DECR_BUTTON_HEIGHT / self.image_minus.height() as f32 * scene_scale.1,
            ));
        self.spritebatch_minus.add(p);
        Rect::new(
            x * scene_scale.0 + translation.0,
            y * scene_scale.1 + translation.1,
            MAIN_MENU_DECR_BUTTON_WIDTH * scene_scale.0,
            MAIN_MENU_DECR_BUTTON_HEIGHT * scene_scale.1,
        )
    }

    fn add_incr_button_sprite(
        &mut self,
        scene_scale: (f32, f32),
        translation: (f32, f32),
        x: f32,
        y: f32,
    ) -> Rect {
        let p = graphics::DrawParam::new()
            .dest(Point2::new(
                x * scene_scale.0 + translation.0,
                y * scene_scale.1 + translation.1,
            ))
            .scale(Vector2::new(
                MAIN_MENU_INCR_BUTTON_WIDTH / self.image_plus.width() as f32 * scene_scale.0,
                MAIN_MENU_INCR_BUTTON_HEIGHT / self.image_plus.height() as f32 * scene_scale.1,
            ));

        self.spritebatch_plus.add(p);
        Rect::new(
            x * scene_scale.0 + translation.0,
            y * scene_scale.1 + translation.1,
            MAIN_MENU_INCR_BUTTON_WIDTH * scene_scale.0,
            MAIN_MENU_INCR_BUTTON_HEIGHT * scene_scale.1,
        )
    }

    fn get_radio_button_image(&self, difficulty: Difficulty) -> &graphics::Image {
        if self.one_player_game {
            if self.difficulty == difficulty {
                return &self.image_checked_radio;
            } else {
                return &self.image_unchecked_radio;
            }
        } else if self.difficulty == difficulty {
            return &self.image_disabled_checked_radio;
        }

        &self.image_disabled_unchecked_radio
    }

    fn draw_difficulty_radio(
        &self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        difficulty: Difficulty,
        button: RadioButton,
        transformation: SceneTransformation,
    ) -> Rect {
        let image_radio = self.get_radio_button_image(difficulty);
        graphics::draw(
            ctx,
            quad_ctx,
            image_radio,
            graphics::DrawParam::new()
                .dest(Point2::new(
                    button.radio_pos.0 * transformation.scene_scale.0 + transformation.translation.0,
                    button.radio_pos.1 * transformation.scene_scale.1 + transformation.translation.1,
                ))
                .scale(Vector2::new(transformation.scene_scale.0, transformation.scene_scale.1)),
        )
        .expect("draw radio button");
        let bounding_box = Rect::new(
            button.radio_pos.0 * transformation.scene_scale.0 + transformation.translation.0,
            button.radio_pos.1 * transformation.scene_scale.1 + transformation.translation.1,
            image_radio.width() as f32 * transformation.scene_scale.0,
            image_radio.height() as f32 * transformation.scene_scale.1,
        );
        graphics::draw(
            ctx,
            quad_ctx,
            button.label_image,
            graphics::DrawParam::new()
                .dest(Point2::new(
                    button.label_pos.0 * transformation.scene_scale.0 + transformation.translation.0,
                    button.label_pos.1 * transformation.scene_scale.1 + transformation.translation.1,
                ))
                .scale(Vector2::new(transformation.scene_scale.0, transformation.scene_scale.1)),
        )
        .expect("draw label");
        bounding_box
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
        graphics::clear(ctx, quad_ctx, graphics::Color::from_rgb_u32(MAIN_MENU_BGCOLOR));

        let scene_scale = get_scene_scale(quad_ctx);
        let translation = get_scene_translation(quad_ctx, scene_scale);

        graphics::draw(
            ctx,
            quad_ctx,
            &self.image_background,
            graphics::DrawParam::new()
                .dest(Point2::new(translation.0, translation.1))
                .scale(Vector2::new(scene_scale.0, scene_scale.1)),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.image_one_player,
            graphics::DrawParam::new()
                .dest(Point2::new(
                    MAIN_MENU_ONE_PLAYER_X * scene_scale.0 + translation.0,
                    MAIN_MENU_ONE_PLAYER_Y * scene_scale.1 + translation.1,
                ))
                .scale(Vector2::new(scene_scale.0, scene_scale.1)),
        )?;

        self.one_player_bounding_box = Rect::new(
            MAIN_MENU_ONE_PLAYER_X * scene_scale.0 + translation.0,
            MAIN_MENU_ONE_PLAYER_Y * scene_scale.1 + translation.1,
            self.image_one_player.width() as f32 * scene_scale.0,
            self.image_one_player.height() as f32 * scene_scale.1,
        );

        graphics::draw(
            ctx,
            quad_ctx,
            &self.image_two_players,
            graphics::DrawParam::new()
                .dest(Point2::new(
                    MAIN_MENU_TWO_PLAYERS_X * scene_scale.0 + translation.0,
                    MAIN_MENU_TWO_PLAYERS_Y * scene_scale.1 + translation.1,
                ))
                .scale(Vector2::new(scene_scale.0, scene_scale.1)),
        )?;

        self.two_players_bounding_box = Rect::new(
            MAIN_MENU_TWO_PLAYERS_X * scene_scale.0 + translation.0,
            MAIN_MENU_TWO_PLAYERS_Y * scene_scale.1 + translation.1,
            self.image_two_players.width() as f32 * scene_scale.0,
            self.image_two_players.height() as f32 * scene_scale.1,
        );

        if self.one_player_game {
            draw_selection_rect(ctx, quad_ctx, self.one_player_bounding_box, scene_scale)?;
        } else {
            draw_selection_rect(ctx, quad_ctx, self.two_players_bounding_box, scene_scale)?;
        }

        self.width_incr_button_bounding_box = self.add_incr_button_sprite(
            scene_scale,
            translation,
            MAIN_MENU_INCR_BUTTON_X,
            MAIN_MENU_INCR_WIDTH_Y,
        );
        #[cfg(feature = "draw_bounding_rects")]
        draw_bounding_rect(ctx, quad_ctx, self.width_incr_button_bounding_box)?;

        self.height_incr_button_bounding_box = self.add_incr_button_sprite(
            scene_scale,
            translation,
            MAIN_MENU_INCR_BUTTON_X,
            MAIN_MENU_INCR_HEIGHT_Y,
        );
        #[cfg(feature = "draw_bounding_rects")]
        draw_bounding_rect(ctx, quad_ctx, self.height_incr_button_bounding_box)?;

        self.width_decr_button_bounding_box = self.add_decr_button_sprite(
            scene_scale,
            translation,
            MAIN_MENU_DECR_BUTTON_X,
            MAIN_MENU_DECR_WIDTH_Y,
        );
        #[cfg(feature = "draw_bounding_rects")]
        draw_bounding_rect(ctx, quad_ctx, self.width_decr_button_bounding_box)?;

        self.height_decr_button_bounding_box = self.add_decr_button_sprite(
            scene_scale,
            translation,
            MAIN_MENU_DECR_BUTTON_X,
            MAIN_MENU_DECR_HEIGHT_Y,
        );
        #[cfg(feature = "draw_bounding_rects")]
        draw_bounding_rect(ctx, quad_ctx, self.height_decr_button_bounding_box)?;

        let mut p = graphics::DrawParam::new()
            .dest(Point2::new(
                MAIN_MENU_IMAGE_WIDTH_X * scene_scale.0 + translation.0,
                MAIN_MENU_IMAGE_WIDTH_Y * scene_scale.1 + translation.1,
            ))
            .scale(Vector2::new(
                MAIN_MENU_DIGIT_WIDTH / self.image_width.width() as f32 * scene_scale.0,
                MAIN_MENU_DIGIT_HEIGHT / self.image_width.height() as f32 * scene_scale.1,
            ));
        self.spritebatch_width.add(p);

        p = graphics::DrawParam::new()
            .dest(Point2::new(
                MAIN_MENU_IMAGE_HEIGHT_X * scene_scale.0 + translation.0,
                MAIN_MENU_IMAGE_HEIGHT_Y * scene_scale.1 + translation.1,
            ))
            .scale(Vector2::new(
                MAIN_MENU_DIGIT_WIDTH / self.image_height.width() as f32 * scene_scale.0,
                MAIN_MENU_DIGIT_HEIGHT / self.image_height.height() as f32 * scene_scale.1,
            ));
        self.spritebatch_height.add(p);

        #[cfg(feature = "draw_bounding_rects")]
        draw_bounding_rect(ctx, quad_ctx, self.start_button_bounding_box)?;

        for spritebatch in [
            &mut self.spritebatch_minus,
            &mut self.spritebatch_plus,
            &mut self.spritebatch_width,
            &mut self.spritebatch_height,
        ] {
            graphics::draw(ctx, quad_ctx, spritebatch, graphics::DrawParam::new())?;
            spritebatch.clear();
        }

        self.easy_difficulty_bounding_box = self.draw_difficulty_radio(
            ctx,
            quad_ctx,
            Difficulty::Easy,
            RadioButton::new(
                (MAIN_MENU_CHECKBOX_EASY_X, MAIN_MENU_CHECKBOX_EASY_Y),
                (MAIN_MENU_EASY_X, MAIN_MENU_EASY_Y),
                &self.image_easy,
            ),
            SceneTransformation::new(scene_scale, translation),
        );

        self.medium_difficulty_bounding_box = self.draw_difficulty_radio(
            ctx,
            quad_ctx,
            Difficulty::Medium,
            RadioButton::new(
                (MAIN_MENU_CHECKBOX_MEDIUM_X, MAIN_MENU_CHECKBOX_MEDIUM_Y),
                (MAIN_MENU_MEDIUM_X, MAIN_MENU_MEDIUM_Y),
                &self.image_medium,
            ),
            SceneTransformation::new(scene_scale, translation),
        );

        self.hard_difficulty_bounding_box = self.draw_difficulty_radio(
            ctx,
            quad_ctx,
            Difficulty::Hard,
            RadioButton::new(
                (MAIN_MENU_CHECKBOX_HARD_X, MAIN_MENU_CHECKBOX_HARD_Y),
                (MAIN_MENU_HARD_X, MAIN_MENU_HARD_Y),
                &self.image_hard,
            ),
            SceneTransformation::new(scene_scale, translation),
        );

        self.start_button_bounding_box = graphics::Rect::new(
            translation.0 + MAIN_MENU_START_BUTTON_X * scene_scale.0,
            translation.1 + MAIN_MENU_START_BUTTON_Y * scene_scale.1,
            MAIN_MENU_START_BUTTON_WIDTH * scene_scale.0,
            MAIN_MENU_START_BUTTON_HEIGHT * scene_scale.1,
        );

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
        self.height_decr_button_bounding_box
            .contains(point)
            .then(|| {
                self.height = std::cmp::max(self.height - 1, 1);
                self.image_height =
                    graphics::Image::new(ctx, quad_ctx, format!("ui/{:?}.png", self.height))
                        .expect("image can be loaded");
                self.spritebatch_height.set_image(self.image_height.clone());
            });

        self.height_incr_button_bounding_box
            .contains(point)
            .then(|| {
                self.height = std::cmp::min(self.height + 1, 9);
                self.image_height =
                    graphics::Image::new(ctx, quad_ctx, format!("ui/{:?}.png", self.height))
                        .expect("image can be loaded");
                self.spritebatch_height.set_image(self.image_height.clone());
            });

        self.width_decr_button_bounding_box
            .contains(point)
            .then(|| {
                self.width = std::cmp::max(self.width - 1, 1);
                self.image_width =
                    graphics::Image::new(ctx, quad_ctx, format!("ui/{:?}.png", self.width))
                        .expect("image can be loaded");
                self.spritebatch_width.set_image(self.image_width.clone());
            });

        self.width_incr_button_bounding_box
            .contains(point)
            .then(|| {
                self.width = std::cmp::min(self.width + 1, 9);
                self.image_width =
                    graphics::Image::new(ctx, quad_ctx, format!("ui/{:?}.png", self.width))
                        .expect("image can be loaded");
                self.spritebatch_width.set_image(self.image_width.clone());
            });

        self.one_player_bounding_box.contains(point).then(|| {
            self.one_player_game = true;
        });

        self.two_players_bounding_box.contains(point).then(|| {
            self.one_player_game = false;
        });

        self.easy_difficulty_bounding_box.contains(point).then(|| {
            if self.one_player_game {
                self.difficulty = Difficulty::Easy;
            }
        });

        self.medium_difficulty_bounding_box
            .contains(point)
            .then(|| {
                if self.one_player_game {
                    self.difficulty = Difficulty::Medium;
                }
            });

         self.hard_difficulty_bounding_box.contains(point).then(|| {
            if self.one_player_game {
                self.difficulty = Difficulty::Hard;
            }
        });

        self.start_button_bounding_box.contains(point).then(|| {
            let game_mode = if self.one_player_game {
                let ai_player = get_cpu_player(&self.difficulty);
                GameMode::OnePlayer(ai_player)
            } else {
                GameMode::TwoPlayer
            };
            let game = PlayingScene::new(
                ctx,
                quad_ctx,
                game::Player::Player1,
                Board::new(self.width, self.height),
                game_mode,
                self.difficulty,
            )
            .expect("board was initialized");

            Transition::ToPlaying(Box::new(game))            
        })
    }
}
