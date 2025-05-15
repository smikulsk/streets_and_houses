use crate::scene::Scene;

use super::marker::*;

#[derive(Debug)]
/// Represents the possible transitions between scenes in the game.
pub enum Transition {
    ToMainMenu(Box<dyn Scene<State = MainMenuState>>),
    ToPlaying(Box<dyn Scene<State = PlayingState>>),
    ToPreparePlayer(Box<dyn Scene<State = PreparePlayerState>>),
    //ToPaused(Box<dyn Scene<State = PausedState>>),
    ToGameOver(Box<dyn Scene<State = GameOverState>>),
    //ToSettings(Box<dyn Scene<State = SettingsState>>),
    ToTitleScreen(Box<dyn Scene<State = TitleScreenState>>),
}
