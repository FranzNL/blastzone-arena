use bevy::prelude::*;
use crate::components::SoundEvent;
use crate::game_assets::GameAssets;
use crate::states::GameState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>()
            .add_systems(OnEnter(GameState::MainMenu), start_music)
            .add_systems(OnExit(GameState::MainMenu), stop_music)
            .add_systems(OnEnter(GameState::Playing), start_music)
            .add_systems(OnExit(GameState::Playing), stop_music)
            .add_systems(Update, play_sound_effects.run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::GameOver), start_gameover_sound)
            .add_systems(OnExit(GameState::GameOver), stop_music);
    }
}

#[derive(Component)]
struct MusicMarker;

fn start_music(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        AudioPlayer(assets.mus_main.clone()),
        PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new(0.4)),
        MusicMarker,
    ));
}

fn stop_music(mut commands: Commands, query: Query<Entity, With<MusicMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

fn start_gameover_sound(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        AudioPlayer(assets.snd_gameover.clone()),
        PlaybackSettings::ONCE.with_volume(bevy::audio::Volume::new(0.5)),
        MusicMarker,
    ));
}

fn play_sound_effects(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut events: EventReader<SoundEvent>,
) {
    for event in events.read() {
        let handle = match event {
            SoundEvent::BombPlace => assets.snd_bomb_place.clone(),
            SoundEvent::Explosion => assets.snd_explosion.clone(),
            SoundEvent::PowerUp   => assets.snd_powerup.clone(),
            SoundEvent::PlayerHit => assets.snd_player_hit.clone(),
            SoundEvent::Win       => assets.snd_win.clone(),
        };
        commands.spawn((
            AudioPlayer(handle),
            PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::new(0.5)),
        ));
    }
}
