use bevy::prelude::*;

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    DeathPause,
    LevelComplete,
    GameOver,
}

#[derive(Resource, Default)]
pub struct DeathPauseTimer(pub f32);
