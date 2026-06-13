use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::game_assets::GameAssets;
use crate::level::cell_world;
use crate::states::{DeathPauseTimer, GameState};

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DetonateEvent>()
            .add_systems(
                Update,
                (tick_bombs, handle_detonation, tick_blasts, blast_damage)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Event)]
pub struct DetonateEvent {
    pub gpos: GridPos,
    pub range: u32,
}

fn tick_bombs(
    time: Res<Time>,
    mut commands: Commands,
    mut bomb_q: Query<(Entity, &mut Sprite, &mut Bomb, &GridPos)>,
    assets: Res<GameAssets>,
    current_theme: Res<CurrentTheme>,
    mut player_q: Query<&mut Player>,
    blast_q: Query<&GridPos, With<Blast>>,
    mut detonate_events: EventWriter<DetonateEvent>,
) {
    let dt = time.delta_secs();
    let theme = &assets.themes[current_theme.0];
    let fire_cells: Vec<(i32, i32)> = blast_q.iter().map(|gp| (gp.col, gp.row)).collect();

    for (entity, mut sprite, mut bomb, gpos) in bomb_q.iter_mut() {
        bomb.fuse -= dt;

        let fuse_progress = 1.0 - (bomb.fuse / BOMB_FUSE).clamp(0.0, 1.0);
        let frame = (fuse_progress * 3.0) as usize;
        sprite.image = theme.bomb[frame.min(2)].clone();

        let chain = fire_cells.iter().any(|(c, r)| *c == gpos.col && *r == gpos.row);
        if chain && bomb.fuse > CHAIN_DELAY {
            bomb.fuse = CHAIN_DELAY;
        }
        if bomb.fuse <= 0.0 {
            let gp = *gpos;
            let range = bomb.range;
            let owner = bomb.owner;
            commands.entity(entity).despawn();

            if let Ok(mut player) = player_q.get_mut(owner) {
                player.active_bombs = player.active_bombs.saturating_sub(1);
            }

            detonate_events.send(DetonateEvent { gpos: gp, range });
        }
    }
}

fn handle_detonation(
    mut commands: Commands,
    assets: Res<GameAssets>,
    current_theme: Res<CurrentTheme>,
    mut events: EventReader<DetonateEvent>,
    wall_q: Query<&GridPos, With<Wall>>,
    block_q: Query<(Entity, &GridPos, &Block)>,
    mut sound_events: EventWriter<SoundEvent>,
) {
    let theme = &assets.themes[current_theme.0];
    for ev in events.read() {
        let origin = ev.gpos;
        let range = ev.range;

        sound_events.send(SoundEvent::Explosion);

        let wpos = cell_world(origin.col, origin.row);
        commands.spawn((
            Sprite { image: theme.exp_center.clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
            Transform::from_xyz(wpos.x, wpos.y, 4.0),
            GridPos { col: origin.col, row: origin.row },
            Blast { life: BLAST_TIME },
            BlastDir::Center,
            LevelEntity,
        ));

        let dirs: &[(i32, i32, BlastDir, BlastDir)] = &[
            (0, -1, BlastDir::ArmV, BlastDir::TipUp),
            (0,  1, BlastDir::ArmV, BlastDir::TipDown),
            (-1, 0, BlastDir::ArmH, BlastDir::TipLeft),
            (1,  0, BlastDir::ArmH, BlastDir::TipRight),
        ];

        for (dc, dr, arm_dir, tip_dir) in dirs {
            for step in 1..=range as i32 {
                let col = origin.col + dc * step;
                let row = origin.row + dr * step;

                let is_wall = wall_q.iter().any(|gp| gp.col == col && gp.row == row);
                if is_wall { break; }

                let block_hit = block_q.iter().find(|(_, gp, _)| gp.col == col && gp.row == row);
                let is_tip = step == range as i32 || block_hit.is_some();
                let dir = if is_tip { *tip_dir } else { *arm_dir };

                let wp = cell_world(col, row);
                let img = match dir {
                    BlastDir::TipUp    => theme.exp_tip[0].clone(),
                    BlastDir::TipDown  => theme.exp_tip[1].clone(),
                    BlastDir::TipLeft  => theme.exp_tip[2].clone(),
                    BlastDir::TipRight => theme.exp_tip[3].clone(),
                    BlastDir::ArmH     => theme.exp_arm[0].clone(),
                    _                  => theme.exp_arm[1].clone(),
                };

                commands.spawn((
                    Sprite { image: img, custom_size: Some(Vec2::splat(TILE)), ..default() },
                    Transform::from_xyz(wp.x, wp.y, 4.0),
                    GridPos { col, row },
                    Blast { life: BLAST_TIME },
                    dir,
                    LevelEntity,
                ));

                if let Some((block_entity, _, block)) = block_hit {
                    let pu_kind = block.powerup;
                    commands.entity(block_entity).despawn();

                    if let Some(kind) = pu_kind {
                        let pu_img = match kind {
                            PowerUpKind::Fire   => theme.pu_fire.clone(),
                            PowerUpKind::Bomb   => theme.pu_bomb.clone(),
                            PowerUpKind::Speed  => theme.pu_speed.clone(),
                            PowerUpKind::Shield => theme.pu_shield.clone(),
                        };
                        commands.spawn((
                            Sprite { image: pu_img, custom_size: Some(Vec2::splat(TILE)), ..default() },
                            Transform::from_xyz(wp.x, wp.y, 2.0),
                            GridPos { col, row },
                            PowerUp(kind),
                            LevelEntity,
                        ));
                    }
                    break;
                }
            }
        }
    }
}

fn tick_blasts(
    time: Res<Time>,
    mut commands: Commands,
    mut blast_q: Query<(Entity, &mut Blast)>,
) {
    let dt = time.delta_secs();
    for (entity, mut blast) in blast_q.iter_mut() {
        blast.life -= dt;
        if blast.life <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn blast_damage(
    blast_q: Query<&GridPos, With<Blast>>,
    player_q: Query<(&GridPos, &Player)>,
    enemy_q: Query<(Entity, &GridPos), With<Enemy>>,
    mut commands: Commands,
    mut sound_events: EventWriter<SoundEvent>,
    mut data: ResMut<GameData>,
    mut next: ResMut<NextState<GameState>>,
    mut death_timer: ResMut<DeathPauseTimer>,
) {
    let blast_cells: Vec<(i32, i32)> = blast_q.iter().map(|gp| (gp.col, gp.row)).collect();
    if blast_cells.is_empty() { return; }

    for (entity, gpos) in enemy_q.iter() {
        if blast_cells.iter().any(|(c, r)| *c == gpos.col && *r == gpos.row) {
            commands.entity(entity).despawn();
        }
    }

    if let Ok((gpos, player)) = player_q.get_single() {
        if player.hit_timer > 0.0 || player.shield_timer > 0.0 { return; }
        if blast_cells.iter().any(|(c, r)| *c == gpos.col && *r == gpos.row) {
            sound_events.send(SoundEvent::PlayerHit);
            data.lives -= 1;
            death_timer.0 = DEATH_PAUSE_DURATION;
            next.set(GameState::DeathPause);
        }
    }
}
