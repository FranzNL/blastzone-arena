use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::game_assets::GameAssets;
use crate::level::cell_world;
use crate::states::{DeathPauseTimer, GameState};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_enemies, enemy_player_collision, check_win)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn update_enemies(
    time: Res<Time>,
    mut enemy_q: Query<(&mut Transform, &mut GridPos, &mut Enemy, &mut Sprite), Without<Wall>>,
    wall_q: Query<&GridPos, (With<Wall>, Without<Enemy>)>,
    block_q: Query<&GridPos, (With<Block>, Without<Enemy>)>,
    bomb_q: Query<&GridPos, (With<Bomb>, Without<Enemy>)>,
    assets: Res<GameAssets>,
    current_theme: Res<CurrentTheme>,
) {
    let dt = time.delta_secs();
    let theme = &assets.themes[current_theme.0];

    // Snapshot occupied cells (walls, blocks, bombs) for pathfinding
    let walls: Vec<(i32,i32)> = wall_q.iter().map(|g| (g.col, g.row)).collect();
    let blocks: Vec<(i32,i32)> = block_q.iter().map(|g| (g.col, g.row)).collect();
    let bombs: Vec<(i32,i32)> = bomb_q.iter().map(|g| (g.col, g.row)).collect();

    for (mut transform, mut gpos, mut enemy, mut sprite) in enemy_q.iter_mut() {
        enemy.anim_timer += dt;
        let frame = (enemy.anim_timer * 4.0) as usize % 2;
        sprite.image = theme.enemy[frame].clone();

        enemy.move_timer -= dt;
        if enemy.move_timer > 0.0 {
            // Interpolate towards target cell center
            let target = cell_world(gpos.col, gpos.row);
            let pos = transform.translation.truncate();
            let delta = Vec2::new(target.x, target.y) - pos;
            let step = ENEMY_SPEED * dt;
            if delta.length() < step {
                transform.translation.x = target.x;
                transform.translation.y = target.y;
            } else {
                let dir = delta.normalize();
                transform.translation.x += dir.x * step;
                transform.translation.y += dir.y * step;
            }
            continue;
        }

        // Snap to cell and pick next direction
        let wp = cell_world(gpos.col, gpos.row);
        transform.translation.x = wp.x;
        transform.translation.y = wp.y;

        let cur_dir = enemy.dir;
        let candidates = [
            cur_dir,                              // prefer current dir
            IVec2::new(-cur_dir.y, cur_dir.x),   // turn left
            IVec2::new(cur_dir.y, -cur_dir.x),   // turn right
            -cur_dir,                             // reverse
        ];

        let mut moved = false;
        for dir in candidates {
            let nc = gpos.col + dir.x;
            let nr = gpos.row + dir.y;
            if nc < 0 || nr < 0 || nc >= GRID_COLS as i32 || nr >= GRID_ROWS as i32 { continue; }
            let blocked = walls.iter().any(|&(c,r)| c==nc && r==nr)
                || blocks.iter().any(|&(c,r)| c==nc && r==nr)
                || bombs.iter().any(|&(c,r)| c==nc && r==nr);
            if !blocked {
                gpos.col = nc;
                gpos.row = nr;
                enemy.dir = dir;
                enemy.move_timer = TILE / ENEMY_SPEED;
                moved = true;
                break;
            }
        }
        if !moved {
            enemy.move_timer = 0.1; // stuck — retry soon
        }
    }
}

fn enemy_player_collision(
    enemy_q: Query<&Transform, With<Enemy>>,
    player_q: Query<(&Transform, &Player)>,
    mut sound_events: EventWriter<SoundEvent>,
    mut data: ResMut<GameData>,
    mut next: ResMut<NextState<GameState>>,
    mut death_timer: ResMut<DeathPauseTimer>,
) {
    let Ok((ptransform, player)) = player_q.get_single() else { return; };
    if player.hit_timer > 0.0 || player.shield_timer > 0.0 { return; }

    let pp = ptransform.translation.truncate();
    for etransform in enemy_q.iter() {
        let ep = etransform.translation.truncate();
        if (ep - pp).abs().cmple(Vec2::splat(TILE * 0.45)).all() {
            sound_events.send(SoundEvent::PlayerHit);
            data.lives -= 1;
            death_timer.0 = DEATH_PAUSE_DURATION;
            next.set(GameState::DeathPause);
            return;
        }
    }
}

fn check_win(
    enemy_q: Query<&Enemy>,
    mut next: ResMut<NextState<GameState>>,
    mut sound_events: EventWriter<SoundEvent>,
    mut fired: Local<bool>,
) {
    if enemy_q.is_empty() && !*fired {
        *fired = true;
        sound_events.send(SoundEvent::Win);
        next.set(GameState::LevelComplete);
    }
    if !enemy_q.is_empty() {
        *fired = false;
    }
}

