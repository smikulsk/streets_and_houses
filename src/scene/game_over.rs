use crate::game::Difficulty;
use crate::game::GameStatistics;
use crate::scene::prelude::*;

#[derive(Debug)]
pub struct GameOverScene {
    statistics: GameStatistics,
    retry_button_bounding_box: Rect,
    is_one_player_game: bool,
    difficulty: Difficulty,
    width: usize,
    height: usize,
    image_background: graphics::Image,
    image_player_1_points: graphics::Image,
    image_player_2_points: graphics::Image,
    image_cpu_points: graphics::Image,
    image_points_1: Vec<graphics::Image>,
    image_points_2: Vec<graphics::Image>,
    image_player_1_wins: graphics::Image,
    image_player_2_wins: graphics::Image,
    image_cpu_wins: graphics::Image,
    image_tie: graphics::Image,
    image_restart_game: graphics::Image,
}

impl GameOverScene {
    pub fn new(
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        statistics: GameStatistics,
        game_mode: &GameMode,
        difficulty: Difficulty,
        width: usize,
        height: usize,
    ) -> GameResult<Self> {
        let (points_1, points_2, is_one_player_game) = match game_mode {
            GameMode::OnePlayer(_) => (statistics.player1_points, statistics.cpu_points, true),
            GameMode::TwoPlayer => (statistics.player1_points, statistics.player2_points, false),
        };

        let image_background = graphics::Image::new(ctx, quad_ctx, "ui/game_over.png")?;
        let image_player_1_points = graphics::Image::new(ctx, quad_ctx, "ui/player_1_points.png")?;
        let image_player_2_points = graphics::Image::new(ctx, quad_ctx, "ui/player_2_points.png")?;
        let image_cpu_points = graphics::Image::new(ctx, quad_ctx, "ui/CPU_points.png")?;
        let image_points_1 = convert_points_to_list_of_images(ctx, quad_ctx, points_1)?;
        let image_points_2 = convert_points_to_list_of_images(ctx, quad_ctx, points_2)?;
        let image_player_1_wins = graphics::Image::new(ctx, quad_ctx, "ui/player_1_wins.png")?;
        let image_player_2_wins = graphics::Image::new(ctx, quad_ctx, "ui/player_2_wins.png")?;
        let image_cpu_wins = graphics::Image::new(ctx, quad_ctx, "ui/CPU_wins.png")?;
        let image_tie = graphics::Image::new(ctx, quad_ctx, "ui/tie.png")?;
        let image_start_game = graphics::Image::new(ctx, quad_ctx, "ui/start_game.png")?;

        Ok(Self {
            statistics,
            retry_button_bounding_box: Rect::default(),
            is_one_player_game,
            difficulty,
            width,
            height,
            image_background,
            image_player_1_points,
            image_player_2_points,
            image_cpu_points,
            image_points_1,
            image_points_2,
            image_player_1_wins,
            image_player_2_wins,
            image_cpu_wins,
            image_tie,
            image_restart_game: image_start_game,
        })
    }

    fn draw_background(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
        scene_scale: (f32, f32),
        translation: (f32, f32),
    ) -> Result<(), ggez::GameError> {
        graphics::draw(
            ctx,
            quad_ctx,
            &self.image_background,
            graphics::DrawParam::new()
                .dest(Point2::new(translation.0, translation.1))
                .scale(Vector2::new(scene_scale.0, scene_scale.1)),
        )?;
        Ok(())
    }

    fn draw_game_result(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
        scene_scale: (f32, f32),
        translation: (f32, f32),
    ) -> Result<(), ggez::GameError> {
        if let Some(player) = self.statistics.winner {
            match player {
                game::Player::Player1 => graphics::draw(
                    ctx,
                    quad_ctx,
                    &self.image_player_1_wins,
                    graphics::DrawParam::new()
                        .dest(Point2::new(
                            GAME_OVER_PLAYER_1_WINS_X * scene_scale.0 + translation.0,
                            GAME_OVER_PLAYER_1_WINS_Y * scene_scale.1 + translation.1,
                        ))
                        .scale(Vector2::new(scene_scale.0, scene_scale.1)),
                )?,
                game::Player::Player2 => graphics::draw(
                    ctx,
                    quad_ctx,
                    &self.image_player_2_wins,
                    graphics::DrawParam::new()
                        .dest(Point2::new(
                            GAME_OVER_PLAYER_2_WINS_X * scene_scale.0 + translation.0,
                            GAME_OVER_PLAYER_2_WINS_Y * scene_scale.1 + translation.1,
                        ))
                        .scale(Vector2::new(scene_scale.0, scene_scale.1)),
                )?,
                game::Player::CPU => graphics::draw(
                    ctx,
                    quad_ctx,
                    &self.image_cpu_wins,
                    graphics::DrawParam::new()
                        .dest(Point2::new(
                            GAME_OVER_CPU_WINS_X * scene_scale.0 + translation.0,
                            GAME_OVER_CPU_WINS_Y * scene_scale.1 + translation.1,
                        ))
                        .scale(Vector2::new(scene_scale.0, scene_scale.1)),
                )?,
            }
        } else {
            graphics::draw(
                ctx,
                quad_ctx,
                &self.image_tie,
                graphics::DrawParam::new()
                    .dest(Point2::new(
                        GAME_OVER_TIE_X * scene_scale.0 + translation.0,
                        GAME_OVER_TIE_Y * scene_scale.1 + translation.1,
                    ))
                    .scale(Vector2::new(scene_scale.0, scene_scale.1)),
            )?
        }
        Ok(())
    }

    fn draw_first_player_points(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
        scene_scale: (f32, f32),
        translation: (f32, f32),
    ) -> Result<(), ggez::GameError> {
        graphics::draw(
            ctx,
            quad_ctx,
            &self.image_player_1_points,
            graphics::DrawParam::new()
                .dest(Point2::new(
                    GAME_OVER_PLAYER_1_POINTS_X * scene_scale.0 + translation.0,
                    GAME_OVER_PLAYER_1_POINTS_Y * scene_scale.1 + translation.1,
                ))
                .scale(Vector2::new(scene_scale.0, scene_scale.1)),
        )?;
        for (idx, image) in self.image_points_1.iter().enumerate() {
            graphics::draw(
                ctx,
                quad_ctx,
                image,
                graphics::DrawParam::new()
                    .dest(Point2::new(
                        (GAME_OVER_POINTS_1_X + 2.0 * idx as f32 * MAIN_MENU_DIGIT_WIDTH / 3.0)
                            * scene_scale.0
                            + translation.0,
                        GAME_OVER_POINTS_1_Y * scene_scale.1 + translation.1,
                    ))
                    .scale(Vector2::new(
                        MAIN_MENU_DIGIT_WIDTH / image.width() as f32 * scene_scale.0,
                        MAIN_MENU_DIGIT_HEIGHT / image.height() as f32 * scene_scale.1,
                    )),
            )?;
        }
        Ok(())
    }

    fn draw_second_player_points(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
        scene_scale: (f32, f32),
        translation: (f32, f32),
    ) -> Result<(), ggez::GameError> {
        if self.is_one_player_game {
            graphics::draw(
                ctx,
                quad_ctx,
                &self.image_cpu_points,
                graphics::DrawParam::new()
                    .dest(Point2::new(
                        GAME_OVER_CPU_POINTS_X * scene_scale.0 + translation.0,
                        GAME_OVER_CPU_POINTS_Y * scene_scale.1 + translation.1,
                    ))
                    .scale(Vector2::new(scene_scale.0, scene_scale.1)),
            )?;
        } else {
            graphics::draw(
                ctx,
                quad_ctx,
                &self.image_player_2_points,
                graphics::DrawParam::new()
                    .dest(Point2::new(
                        GAME_OVER_PLAYER_2_POINTS_X * scene_scale.0 + translation.0,
                        GAME_OVER_PLAYER_2_POINTS_Y * scene_scale.1 + translation.1,
                    ))
                    .scale(Vector2::new(scene_scale.0, scene_scale.1)),
            )?;
        }

        for (idx, image) in self.image_points_2.iter().enumerate() {
            graphics::draw(
                ctx,
                quad_ctx,
                image,
                graphics::DrawParam::new()
                    .dest(Point2::new(
                        (GAME_OVER_POINTS_2_X + 2.0 * idx as f32 * MAIN_MENU_DIGIT_WIDTH / 3.0)
                            * scene_scale.0
                            + translation.0,
                        GAME_OVER_POINTS_2_Y * scene_scale.1 + translation.1,
                    ))
                    .scale(Vector2::new(
                        MAIN_MENU_DIGIT_WIDTH / image.width() as f32 * scene_scale.0,
                        MAIN_MENU_DIGIT_HEIGHT / image.height() as f32 * scene_scale.1,
                    )),
            )?;
        }
        Ok(())
    }

    fn draw_retry_button(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
        scene_scale: (f32, f32),
        translation: (f32, f32),
    ) -> Result<(), ggez::GameError> {
        graphics::draw(
            ctx,
            quad_ctx,
            &self.image_restart_game,
            graphics::DrawParam::new()
                .dest(Point2::new(
                    GAME_OVER_START_BUTTON_X * scene_scale.0 + translation.0,
                    GAME_OVER_START_BUTTON_Y * scene_scale.1 + translation.1,
                ))
                .scale(Vector2::new(
                    GAME_OVER_START_BUTTON_WIDTH / self.image_restart_game.width() as f32
                        * scene_scale.0,
                    GAME_OVER_START_BUTTON_HEIGHT / self.image_restart_game.height() as f32
                        * scene_scale.1,
                )),
        )?;
        self.retry_button_bounding_box = graphics::Rect::new(
            translation.0 + GAME_OVER_START_BUTTON_X * scene_scale.0,
            translation.1 + GAME_OVER_START_BUTTON_Y * scene_scale.1,
            GAME_OVER_START_BUTTON_WIDTH * scene_scale.0,
            GAME_OVER_START_BUTTON_HEIGHT * scene_scale.1,
        );
        Ok(())
    }
}

impl Scene for GameOverScene {
    type State = GameOverState;

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

        self.draw_background(ctx, quad_ctx, scene_scale, translation)?;
        self.draw_game_result(ctx, quad_ctx, scene_scale, translation)?;
        self.draw_first_player_points(ctx, quad_ctx, scene_scale, translation)?;
        self.draw_second_player_points(ctx, quad_ctx, scene_scale, translation)?;
        self.draw_retry_button(ctx, quad_ctx, scene_scale, translation)?;

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
        self.retry_button_bounding_box.contains(point).then(|| {
            let game = MainMenuScene::from(
                ctx,
                quad_ctx,
                self.width,
                self.height,
                self.is_one_player_game,
                self.difficulty,
            );
            Transition::ToMainMenu(Box::new(game.expect("scene has been created")))
        })
    }
}

fn convert_points_to_list_of_images(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::Context,
    points: usize,
) -> GameResult<Vec<graphics::Image>> {
    points
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .map(|d| graphics::Image::new(ctx, quad_ctx, format!("ui/{:?}.png", d)))
        .collect::<GameResult<Vec<_>>>()
}
