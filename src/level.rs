use bevy::prelude::*;
use image::GenericImageView;
use crate::components::*;
use crate::constants::*;
use crate::game_assets::GameAssets;
use crate::states::GameState;


const LEVEL1: &[u8] = include_bytes!("../assets/levels/lvl1.png");
const LEVEL2: &[u8] = include_bytes!("../assets/levels/lvl2.png");
const LEVEL3: &[u8] = include_bytes!("../assets/levels/lvl3.png");
const LEVEL4: &[u8] = include_bytes!("../assets/levels/lvl4.png");

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (cleanup_level_entities, apply_deferred, spawn_level).chain(),
        );
    }
}

fn cleanup_level_entities(mut commands: Commands, q: Query<Entity, With<LevelEntity>>) {
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}

fn level_bytes(lvl: u32) -> &'static [u8] {
    match lvl {
        1 => LEVEL1,
        2 => LEVEL2,
        3 => LEVEL3,
        _ => LEVEL4,
    }
}

// World position for center of cell (col, row), with origin at top-left inside camera view.
// Camera is centered at (WINDOW_W*0.5, -WINDOW_H*0.5), looking at world with +Y up.
// HUD is at the top (y near 0), arena below.
pub fn cell_world(col: i32, row: i32) -> Vec2 {
    let x = col as f32 * TILE + TILE * 0.5;
    // row 0 is top of arena, which starts below HUD
    let y = -(HUD_HEIGHT + row as f32 * TILE + TILE * 0.5);
    Vec2::new(x, y)
}

fn random_pu_kind(col: u32, row: u32) -> PowerUpKind {
    // Deterministic pseudo-random based on position
    match (col * 7 + row * 13) % 4 {
        0 => PowerUpKind::Fire,
        1 => PowerUpKind::Bomb,
        2 => PowerUpKind::Speed,
        _ => PowerUpKind::Shield,
    }
}

pub fn spawn_level(
    mut commands: Commands,
    assets: Res<GameAssets>,
    data: Res<GameData>,
    mut current_theme: ResMut<CurrentTheme>,
) {
    current_theme.0 = (data.level as usize - 1) % 3;
    let theme = &assets.themes[current_theme.0];

    let bytes = level_bytes(data.level);
    let img = image::load_from_memory(bytes).expect("level png invalid");

    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            let rgba = img.get_pixel(col, row);
            let (r, g, b) = (rgba[0], rgba[1], rgba[2]);
            let wpos = cell_world(col as i32, row as i32);
            let gpos = GridPos { col: col as i32, row: row as i32 };

            match (r, g, b) {
                (0, 0, 0) => {
                    // Indestructible wall
                    commands.spawn((
                        Sprite { image: theme.wall.clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
                        Transform::from_xyz(wpos.x, wpos.y, 0.0),
                        gpos,
                        Wall,
                        LevelEntity,
                    ));
                }
                (255, 255, 255) | (0, 200, 0) | (255, 0, 0) => {
                    // Floor (and spawn tiles, which are just floor)
                    commands.spawn((
                        Sprite { image: theme.floor.clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
                        Transform::from_xyz(wpos.x, wpos.y, 0.0),
                        gpos,
                        LevelEntity,
                    ));

                    if (r, g, b) == (0, 200, 0) {
                        // Player spawn
                        commands.spawn((
                            Sprite { image: theme.player_down[0].clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
                            Transform::from_xyz(wpos.x, wpos.y, 3.0),
                            gpos,
                            Player::default(),
                            Facing::Down,
                            Velocity::default(),
                            AnimTimer(0.0),
                            LevelEntity,
                        ));
                    } else if (r, g, b) == (255, 0, 0) {
                        // Enemy spawn
                        commands.spawn((
                            Sprite { image: theme.enemy[0].clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
                            Transform::from_xyz(wpos.x, wpos.y, 2.0),
                            gpos,
                            Enemy { dir: IVec2::new(1, 0), move_timer: 0.0, anim_timer: 0.0 },
                            LevelEntity,
                        ));
                    }
                }
                (150, 75, 0) => {
                    // Destructible block (no hidden power-up)
                    commands.spawn((
                        Sprite { image: theme.floor.clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
                        Transform::from_xyz(wpos.x, wpos.y, 0.0),
                        gpos,
                        LevelEntity,
                    ));
                    commands.spawn((
                        Sprite { image: theme.block.clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
                        Transform::from_xyz(wpos.x, wpos.y, 1.0),
                        gpos,
                        Block { powerup: None },
                        LevelEntity,
                    ));
                }
                (255, 255, 0) => {
                    // Block hiding a power-up
                    commands.spawn((
                        Sprite { image: theme.floor.clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
                        Transform::from_xyz(wpos.x, wpos.y, 0.0),
                        gpos,
                        LevelEntity,
                    ));
                    commands.spawn((
                        Sprite { image: theme.block.clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
                        Transform::from_xyz(wpos.x, wpos.y, 1.0),
                        gpos,
                        Block { powerup: Some(random_pu_kind(col, row)) },
                        LevelEntity,
                    ));
                }
                _ => {
                    // Unknown pixel → plain floor
                    commands.spawn((
                        Sprite { image: theme.floor.clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
                        Transform::from_xyz(wpos.x, wpos.y, 0.0),
                        gpos,
                        LevelEntity,
                    ));
                }
            }
        }
    }
}
