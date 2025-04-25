use std::io;

use crate::game::Board;
use crate::scene::prelude::*;

#[derive(Debug, Clone)]
pub struct MainMenuScene {}

impl MainMenuScene {
    pub fn new() -> Self {
        Self {}
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
        let text = graphics::Text::new("Click to start the game!");
        let text_width = text.width(ctx);
        let text_height = text.height(ctx);
        let dest_point = graphics::DrawParam::new().dest(Point2::new(
            (width - text_width) / 2.0,
            (height - text_height) / 2.0,
        ));
        graphics::draw(ctx, quad_ctx, &text, dest_point)?;

        graphics::present(ctx, quad_ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Option<Transition> {
        let board = init_board();
        match board {
            Ok(board) => {
                let game = PlayingScene::new(ctx, quad_ctx, board).expect("board was initialized");
                Some(Transition::ToPlaying(Box::new(game)))
            }
            Err(_) => {
                println!("Error initializing the game board.");
                None
            }
        }
    }
}

fn get_number_pair_from_input() -> io::Result<(usize, usize)> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    if let Some((a, b)) = buffer.split_once(',') {
        let a = a.trim().parse::<usize>().expect("this is a number");
        let b = b.trim().parse::<usize>().expect("this is a number");
        return Ok((a, b));
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        "Wrong imput format. Should be 'number,number'",
    ))
}

fn init_board() -> io::Result<Board> {
    println!("Provide width and height delimited with comma");
    let (width, height) = get_number_pair_from_input()?;
    Ok(Board::new(width, height))
}
