extern crate good_web_game as ggez;

use ggez::miniquad;
use ggez::Context;
use ggez::GameResult;

use crate::scene::prelude::*;
use crate::state::prelude::*;

use std::fmt::Debug;

#[derive(Debug)]
pub struct Game<S: GameStateMarker> {
    current_scene: Box<dyn Scene<State = S>>,
}

impl<S: GameStateMarker> Game<S> {
    pub fn update(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::event::GraphicsContext,
    ) -> Result<Option<Transition>, ggez::GameError> {
        self.current_scene.update(ctx, quad_ctx)
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
    ) -> GameResult {
        self.current_scene.draw(ctx, quad_ctx)
    }

    pub fn mouse_button_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Option<Transition> {
        self.current_scene
            .mouse_button_up_event(ctx, quad_ctx, button, x, y)
    }
}

// Specific implementations for each state
impl Game<MainMenuState> {
    pub fn new() -> Self {
        Self {
            current_scene: Box::new(MainMenuScene::new()),
        }
    }

    pub fn transition(self, transition: Transition) -> Box<dyn GameInstance> {
        match transition {
            Transition::ToPreparePlayer(scene) => Box::new(Game {
                current_scene: scene,
            }),
            Transition::ToPlaying(scene) => Box::new(Game {
                current_scene: scene,
            }),
            // Transition::ToSettings(scene) => Box::new(Game {
            //     current_scene: scene,
            // }),
            _ => Box::new(self),
        }
    }
}

impl Default for Game<MainMenuState> {
    fn default() -> Self {
        Self::new()
    }
}

impl Game<PreparePlayerState> {
    pub fn transition(self, transition: Transition) -> Box<dyn GameInstance> {
        match transition {
            Transition::ToPlaying(scene) => Box::new(Game {
                current_scene: scene,
            }),
            // Transition::ToSettings(scene) => Box::new(Game {
            //     current_scene: scene,
            // }),
            _ => Box::new(self),
        }
    }
}

impl Game<PlayingState> {
    pub fn transition(self, transition: Transition) -> Box<dyn GameInstance> {
        match transition {
            // Transition::ToPaused(scene) => Box::new(Game {
            //     current_scene: scene,
            // }),
            Transition::ToPreparePlayer(scene) => Box::new(Game {
                current_scene: scene,
            }),
            Transition::ToGameOver(scene) => Box::new(Game {
                current_scene: scene,
            }),
            _ => Box::new(self),
        }
    }
}

impl Game<TitleScreenState> {
    pub fn new(ctx: &mut ggez::Context, quad_ctx: &mut ggez::event::GraphicsContext) -> Self {
        Self {
            current_scene: Box::new(TitleScreenScene::new(ctx, quad_ctx)),
        }
    }
    pub fn transition(self, transition: Transition) -> Box<dyn GameInstance> {
        match transition {
            Transition::ToMainMenu(scene) => Box::new(Game {
                current_scene: scene,
            }),
            // Transition::ToSettings(scene) => Box::new(Game {
            //     current_scene: scene,
            // }),
            _ => Box::new(self),
        }
    }
}

// Common trait for all game instances
pub trait GameInstance: Debug {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::event::GraphicsContext,
    ) -> Result<Option<Transition>, ggez::GameError>;
    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult;
    fn mouse_button_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Option<Transition>;
    fn transition(&self, transition: Transition) -> Box<dyn GameInstance>;
}

// Implement GameInstance for all Game<S> types
impl<S: GameStateMarker + 'static> GameInstance for Game<S> {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::event::GraphicsContext,
    ) -> Result<Option<Transition>, ggez::GameError> {
        self.update(ctx, quad_ctx)
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        self.draw(ctx, quad_ctx)
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Option<Transition> {
        self.mouse_button_up_event(ctx, quad_ctx, button, x, y)
    }

    fn transition(&self, transition: Transition) -> Box<dyn GameInstance> {
        match transition {
            Transition::ToMainMenu(scene) => Box::new(Game {
                current_scene: scene,
            }),
            Transition::ToPreparePlayer(scene) => Box::new(Game {
                current_scene: scene,
            }),
            Transition::ToPlaying(scene) => Box::new(Game {
                current_scene: scene,
            }),
            Transition::ToGameOver(scene) => Box::new(Game {
                current_scene: scene,
            }),
            Transition::ToTitleScreen(scene) => Box::new(Game {
                current_scene: scene,
            }),
        }
    }
}
