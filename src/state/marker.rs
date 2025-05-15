pub trait GameStateMarker : std::fmt::Debug {}

// State types
#[derive(Debug, Clone)]
pub struct MainMenuState;
#[derive(Debug, Clone)]
pub struct PreparePlayerState;
#[derive(Debug, Clone)]
pub struct PlayingState;
#[derive(Debug, Clone)]
pub struct PausedState;
#[derive(Debug, Clone)]
pub struct GameOverState;
#[derive(Debug, Clone)]
pub struct SettingsState;
#[derive(Debug, Clone)]
pub struct TitleScreenState;

impl GameStateMarker for MainMenuState {}
impl GameStateMarker for PreparePlayerState {}
impl GameStateMarker for PlayingState {}
impl GameStateMarker for PausedState {}
impl GameStateMarker for GameOverState {}
impl GameStateMarker for SettingsState {}
impl GameStateMarker for TitleScreenState {}
