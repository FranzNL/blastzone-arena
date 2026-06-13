use bevy::prelude::*;

mod audio;
mod bomb;
mod camera;
mod components;
mod constants;
mod enemies;
mod fullscreen;
mod game_assets;
mod items;
mod level;
mod physics;
mod player;
mod states;
mod touch_controls;
mod ui;

use audio::AudioPlugin;
use bomb::BombPlugin;
use camera::CameraPlugin;
use components::{CurrentTheme, GameData, UiHandles};
use constants::*;
use enemies::EnemiesPlugin;
use fullscreen::FullscreenPlugin;
use game_assets::AssetsPlugin;
use items::ItemsPlugin;
use level::LevelPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use states::{DeathPauseTimer, GameState};
use touch_controls::TouchControlsPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "BlastZone Arena".to_string(),
                        resolution: (WINDOW_W * 2.0, WINDOW_H * 2.0).into(),
                        canvas: Some("#bevy".to_string()),
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.05)))
        .insert_resource(GameData { lives: 3, level: 1, time: LEVEL_TIME })
        .insert_resource(UiHandles::default())
        .insert_resource(DeathPauseTimer::default())
        .insert_resource(CurrentTheme::default())
        .init_state::<GameState>()
        .add_plugins(AssetsPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(BombPlugin)
        .add_plugins(EnemiesPlugin)
        .add_plugins(ItemsPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(FullscreenPlugin)
        .add_plugins(TouchControlsPlugin)
        .add_plugins(UiPlugin)
        .run();
}
