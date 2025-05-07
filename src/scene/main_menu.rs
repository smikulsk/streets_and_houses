use crate::game::ai::GreadyAlgorithmPlayer;
use crate::game::Board;
use crate::scene::prelude::*;

#[derive(Debug, Clone)]
pub struct MainMenuScene {
    width: usize,
    height: usize,
    one_player_bounding_box: Rect,
    two_player_bounding_box: Rect,
    width_decr_button_bounding_box: Rect,
    width_incr_button_bounding_box: Rect,
    height_decr_button_bounding_box: Rect,
    height_incr_button_bounding_box: Rect,
    start_button_bounding_box: Rect,
    one_player_game: bool,
}

impl MainMenuScene {
    pub fn new() -> Self {
        Self {
            width: 3,
            height: 3,
            one_player_bounding_box: Rect::default(),
            two_player_bounding_box: Rect::default(),
            width_decr_button_bounding_box: Rect::default(),
            width_incr_button_bounding_box: Rect::default(),
            height_decr_button_bounding_box: Rect::default(),
            height_incr_button_bounding_box: Rect::default(),
            start_button_bounding_box: Rect::default(),
            one_player_game: true,
        }
    }

    pub fn from(width: usize, height: usize, one_player_game: bool) -> Self {
        Self {
            width,
            height,
            one_player_bounding_box: Rect::default(),
            two_player_bounding_box: Rect::default(),
            width_decr_button_bounding_box: Rect::default(),
            width_incr_button_bounding_box: Rect::default(),
            height_decr_button_bounding_box: Rect::default(),
            height_incr_button_bounding_box: Rect::default(),
            start_button_bounding_box: Rect::default(),
            one_player_game,
        }
    }
}

impl Default for MainMenuScene {
    fn default() -> Self {
        Self::new()
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
        graphics::clear(ctx, quad_ctx, graphics::Color::BLACK);

        let (width, height) = graphics::drawable_size(quad_ctx);

        self.one_player_bounding_box = draw_button(
            ctx,
            quad_ctx,
            width / 2.0 - 65.0,
            height / 2.0 - 120.0,
            "One player",
            self.one_player_game,
        )?;

        self.two_player_bounding_box = draw_button(
            ctx,
            quad_ctx,
            width / 2.0 + 65.0,
            height / 2.0 - 120.0,
            "Two players",
            !self.one_player_game,
        )?;

        draw_text(
            ctx,
            quad_ctx,
            width / 2.0 - 50.0,
            height / 2.0 - 60.0,
            "Width:",
        )?;
        self.width_decr_button_bounding_box =
            draw_button(ctx, quad_ctx, width / 2.0, height / 2.0 - 60.0, "-", false)?;
        draw_text(
            ctx,
            quad_ctx,
            width / 2.0 + 30.0,
            height / 2.0 - 60.0,
            &self.width.to_string(),
        )?;
        self.width_incr_button_bounding_box =
            draw_button(ctx, quad_ctx, width / 2.0 + 60.0, height / 2.0 - 60.0, "+", false)?;
        draw_text(
            ctx,
            quad_ctx,
            width / 2.0 - 50.0,
            height / 2.0 - 20.0,
            "Height:",
        )?;
        self.height_decr_button_bounding_box =
            draw_button(ctx, quad_ctx, width / 2.0, height / 2.0 - 20.0, "-", false)?;
        draw_text(
            ctx,
            quad_ctx,
            width / 2.0 + 30.0,
            height / 2.0 - 20.0,
            &self.height.to_string(),
        )?;
        self.height_incr_button_bounding_box = draw_button(
            ctx,
            quad_ctx,
            width / 2.0 + 60.0,
            height / 2.0 - 20.0,
            "+",
            false,
        )?;
        self.start_button_bounding_box = draw_button(
            ctx,
            quad_ctx,
            width / 2.0,
            height / 2.0 + 40.0,
            "Click to start the game!",
            false,
        )?;

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
        self.height_decr_button_bounding_box
            .contains(point)
            .then(|| {
                self.height = std::cmp::max(self.height - 1, 1);
            });

        self.height_incr_button_bounding_box
            .contains(point)
            .then(|| {
                self.height = std::cmp::min(self.height + 1, 9);
            });

        self.width_decr_button_bounding_box
            .contains(point)
            .then(|| {
                self.width = std::cmp::max(self.width - 1, 1);
            });

        self.width_incr_button_bounding_box
            .contains(point)
            .then(|| {
                self.width = std::cmp::min(self.width + 1, 9);
            });

        self.one_player_bounding_box.contains(point).then(|| {
            self.one_player_game = true;
        });

        self.two_player_bounding_box.contains(point).then(|| {
            self.one_player_game = false;
        });

        self.start_button_bounding_box.contains(point).then(|| {
            let ai_player = GreadyAlgorithmPlayer::default();
            let game_mode = if self.one_player_game {
                GameMode::OnePlayer(Box::new(ai_player))
            } else {
                GameMode::TwoPlayer
            };
            let game = PreparePlayerScene::new(
                game::Player::Player1,
                &Board::new(self.width, self.height),
                &game_mode,
            );
            Transition::ToPreparePlayer(Box::new(game))
        })
    }
}
