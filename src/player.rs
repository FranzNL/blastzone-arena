use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::game_assets::GameAssets;
use crate::states::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_input, player_animation)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn player_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    vi: Res<VirtualInput>,
    mut commands: Commands,
    mut player_q: Query<(Entity, &mut Velocity, &mut Facing, &mut Player, &GridPos)>,
    bombs: Query<(&GridPos, &Bomb), Without<Player>>,
    assets: Res<GameAssets>,
    current_theme: Res<CurrentTheme>,
    mut sound_events: EventWriter<SoundEvent>,
) {
    let dt = time.delta_secs();
    let Ok((entity, mut vel, mut facing, mut player, gpos)) = player_q.get_single_mut() else {
        return;
    };

    player.hit_timer -= dt;
    player.shield_timer -= dt;
    if player.shield_timer < 0.0 { player.shield_timer = 0.0; }

    // 4-direction input
    let up    = keys.pressed(KeyCode::ArrowUp)    || keys.pressed(KeyCode::KeyW) || vi.up;
    let down  = keys.pressed(KeyCode::ArrowDown)  || keys.pressed(KeyCode::KeyS) || vi.down;
    let left  = keys.pressed(KeyCode::ArrowLeft)  || keys.pressed(KeyCode::KeyA) || vi.left;
    let right = keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) || vi.right;

    let mut dx = 0.0f32;
    let mut dy = 0.0f32;
    if left  { dx -= 1.0; *facing = Facing::Left; }
    if right { dx  = 1.0; *facing = Facing::Right; }
    if up    { dy  = 1.0; *facing = Facing::Up; }
    if down  { dy -= 1.0; *facing = Facing::Down; }

    // Normalize diagonal
    if dx != 0.0 && dy != 0.0 {
        dx *= 0.7071;
        dy *= 0.7071;
    }

    vel.x = dx * player.speed;
    vel.y = dy * player.speed;

    // Bomb placement (Space or X)
    let bomb_just = keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::KeyX)
        || vi.bomb_just;

    if bomb_just && player.active_bombs < player.max_bombs {
        // Place bomb on player's current grid cell
        let col = gpos.col;
        let row = gpos.row;

        // Don't place if a bomb is already there
        let already_bombed = bombs.iter().any(|(bp, _)| bp.col == col && bp.row == row);
        if !already_bombed {
            use crate::level::cell_world;
            let wpos = cell_world(col, row);
            commands.spawn((
                Sprite { image: assets.themes[current_theme.0].bomb[0].clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
                Transform::from_xyz(wpos.x, wpos.y, 1.5),
                GridPos { col, row },
                Bomb {
                    fuse: BOMB_FUSE,
                    owner: entity,
                    range: player.range,
                },
                LevelEntity,
            ));
            player.active_bombs += 1;
            sound_events.send(SoundEvent::BombPlace);
        }
    }

}

fn player_animation(
    time: Res<Time>,
    mut player_q: Query<(&mut Sprite, &mut AnimTimer, &Velocity, &Facing, &Player)>,
    assets: Res<GameAssets>,
    current_theme: Res<CurrentTheme>,
) {
    let Ok((mut sprite, mut timer, vel, facing, player)) = player_q.get_single_mut() else {
        return;
    };

    timer.0 += time.delta_secs() * 8.0;
    let frame = timer.0 as usize % 2;

    let moving = vel.x.abs() > 1.0 || vel.y.abs() > 1.0;

    let theme = &assets.themes[current_theme.0];
    let (imgs, flip_x) = match facing {
        Facing::Down  => (&theme.player_down, false),
        Facing::Up    => (&theme.player_up, false),
        Facing::Left  => (&theme.player_side, false),
        Facing::Right => (&theme.player_side, true),
    };

    sprite.image = if moving { imgs[frame].clone() } else { imgs[0].clone() };
    sprite.flip_x = flip_x;

    // Blink when hit or invulnerable
    if player.hit_timer > 0.0 && (timer.0 as i32 % 4 < 2) {
        sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.3);
    } else if player.shield_timer > 0.0 {
        sprite.color = Color::srgb(0.5, 0.8, 1.0);
    } else {
        sprite.color = Color::WHITE;
    }
}
