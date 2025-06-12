use ggez::timer;

use super::*;
use crate::game::GameMode;
use crate::game::Player;
use crate::rendering::prelude::*;
use crate::rendering::Renderer;
use crate::scene::prelude::*;

#[derive(Debug)]
pub struct PlayingScene {
    board_renderer: BoardRenderer,
    first_player_renderer: PlayerDataRenderer,
    second_player_renderer: PlayerDataRenderer,
    board: game::Board,
    player: game::Player,
    wall_bounding_boxes: Vec<Vec<Rect>>,
    game_mode: game::GameMode,
    difficulty: game::Difficulty,
    already_drawn: bool,
    deferred_transition: Option<Transition>,
}

impl PlayingScene {
    pub fn new(
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        player: Player,
        board: Board,
        game_mode: game::GameMode,
        difficulty: game::Difficulty,
    ) -> GameResult<PlayingScene> {
        let wall_bounding_boxes =
            vec![vec![Rect::default(); board.width + 1]; 2 * board.height + 1];

        let board_renderer = BoardRenderer::new(ctx, quad_ctx, player, board.clone(), false)?;
        let first_player_renderer = PlayerDataRenderer::new(
            ctx,
            quad_ctx,
            Player::Player1,
            board.statistics.player1_points,
            player == Player::Player1,
        )?;
        let second_player_renderer = match game_mode {
            GameMode::OnePlayer(_) => {
                PlayerDataRenderer::new(ctx, quad_ctx, Player::CPU, board.statistics.cpu_points, player == Player::CPU)?
            }
            GameMode::TwoPlayer => PlayerDataRenderer::new(
                ctx,
                quad_ctx,
                Player::Player2,
                board.statistics.player2_points,
                player == Player::Player2)?,
        };

        let s = PlayingScene {
            board_renderer,
            first_player_renderer,
            second_player_renderer,
            board,
            player,
            wall_bounding_boxes,
            game_mode,
            difficulty,
            already_drawn: false,
            deferred_transition: None,
        };
        Ok(s)
    }

    fn click_wall(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut event::GraphicsContext,
        row: usize,
        col: usize,
    ) {
        match self.board.click_wall(row, col, self.player) {
            Ok(additional_move) => {
                self.board_renderer.set_board(&self.board);
                self.update_points(ctx, quad_ctx);

                if !additional_move {
                    let new_player = match self.player {
                        game::Player::Player1 => match &self.game_mode {
                            GameMode::OnePlayer(_) => game::Player::CPU,
                            GameMode::TwoPlayer => game::Player::Player2,
                        },
                        game::Player::Player2 | game::Player::CPU => game::Player::Player1,
                    };

                    if !self.board.all_is_clicked() {
                        let game = PlayingScene::new(
                            ctx,
                            quad_ctx,
                            new_player,
                            self.board.clone(),
                            self.game_mode.clone(),
                            self.difficulty,
                        )
                        .expect("board was initialized");

                        self.deferred_transition = Some(Transition::ToPlaying(Box::new(game)));
                    }
                }
                self.already_drawn = false;
            }
            Err(error) => println!("Error occurred: '{}'", error),
        }
    }

    fn update_points(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::Context) {
        self.first_player_renderer
            .set_points(ctx, quad_ctx, self.board.statistics.player1_points)
            .expect("Player 1 points can be set in the renderer");
        match &self.game_mode {
            GameMode::OnePlayer(_) => self
                .second_player_renderer
                .set_points(ctx, quad_ctx, self.board.statistics.cpu_points)
                .expect("CPU points can be set in the renderer"),
            GameMode::TwoPlayer => self
                .second_player_renderer
                .set_points(ctx, quad_ctx, self.board.statistics.player2_points)
                .expect("Player 2 points can be set in the renderer"),
        };
    }
}

impl Scene for PlayingScene {
    type State = PlayingState;

    fn update(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut event::GraphicsContext,
    ) -> Result<Option<Transition>, ggez::GameError> {
        if self.already_drawn && timer::ticks(ctx) % PLAYING_TICK_COUNT == 0 {
            if let Some(transition) = self.deferred_transition.take() {
                return Ok(Some(transition));
            }

            if self.board.all_is_clicked() {
                let game_statistics = self.board.get_statistics();
                let game = GameOverScene::new(
                    ctx,
                    quad_ctx,
                    game_statistics,
                    &self.game_mode,
                    self.difficulty,
                    self.board.width,
                    self.board.height,
                )
                .expect("scene has been created");
                return Ok(Some(Transition::ToGameOver(Box::new(game))));
            }

            if self.player == Player::CPU {
                if let Some((row, col)) = match &self.game_mode {
                    GameMode::OnePlayer(move_generator) => move_generator.next_move(&self.board),
                    GameMode::TwoPlayer => None,
                } {
                    self.click_wall(ctx, quad_ctx, row, col);
                }
            }
        }
        Ok(None)
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        self.board_renderer.draw(ctx, quad_ctx)?;
        self.first_player_renderer.set_orientation(quad_ctx);
        self.first_player_renderer.draw(ctx, quad_ctx)?;
        self.second_player_renderer.set_orientation(quad_ctx);
        self.second_player_renderer.draw(ctx, quad_ctx)?;

        self.wall_bounding_boxes = self.board_renderer.get_wall_bounding_boxes();
        self.already_drawn = true;

        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        _button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Option<Transition> {
        if self.player == Player::CPU || self.deferred_transition.is_some() {
            return None;
        }

        for row in 0..2 * self.board.height + 1 {
            let max_col = if row % 2 > 0 {
                self.board.width + 1
            } else {
                self.board.width
            };
            for col in 0..max_col {
                let rect = self.wall_bounding_boxes[row][col];
                if rect.contains(Point2::new(x, y)) {
                    self.click_wall(ctx, quad_ctx, row, col);
                    return None;
                }
            }
        }
        None
    }
}
