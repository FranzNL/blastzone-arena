use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::states::{DeathPauseTimer, GameState};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (collect_powerups, time_countdown).run_if(in_state(GameState::Playing)),
        );
    }
}

fn collect_powerups(
    mut commands: Commands,
    pu_q: Query<(Entity, &GridPos, &PowerUp)>,
    mut player_q: Query<(&GridPos, &mut Player)>,
    mut sound_events: EventWriter<SoundEvent>,
) {
    let Ok((pgpos, mut player)) = player_q.get_single_mut() else { return; };

    for (entity, gpos, pu) in pu_q.iter() {
        if gpos.col == pgpos.col && gpos.row == pgpos.row {
            match pu.0 {
                PowerUpKind::Fire   => player.range += 1,
                PowerUpKind::Bomb   => player.max_bombs += 1,
                PowerUpKind::Speed  => player.speed = (player.speed + TILE).min(H_SPEED * 2.0),
                PowerUpKind::Shield => player.shield_timer = SHIELD_DURATION,
            }
            sound_events.send(SoundEvent::PowerUp);
            commands.entity(entity).despawn();
        }
    }
}

fn time_countdown(
    time: Res<Time>,
    mut data: ResMut<GameData>,
    player_q: Query<(), With<Player>>,
    mut next: ResMut<NextState<GameState>>,
    mut death_timer: ResMut<DeathPauseTimer>,
    mut sound_events: EventWriter<SoundEvent>,
) {
    if player_q.get_single().is_ok() {
        data.time -= time.delta_secs();
        if data.time <= 0.0 {
            data.time = 0.0;
            sound_events.send(SoundEvent::PlayerHit);
            data.lives -= 1;
            death_timer.0 = DEATH_PAUSE_DURATION;
            next.set(GameState::DeathPause);
        }
    }
}
