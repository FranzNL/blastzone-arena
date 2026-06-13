use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::game_assets::GameAssets;
use crate::states::{DeathPauseTimer, GameState};

const OVERLAY_MAX_ALPHA: f32 = 0.85;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiHandles>()
            // Playing HUD
            .add_systems(OnEnter(GameState::Playing), setup_hud)
            .add_systems(Update, update_hud.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_ui)
            // Main menu
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_ui)
            .add_systems(Update, (main_menu_input, animate_blink).run_if(in_state(GameState::MainMenu)))
            // Game over
            .add_systems(OnEnter(GameState::GameOver), setup_gameover)
            .add_systems(OnExit(GameState::GameOver), cleanup_ui)
            .add_systems(Update, gameover_input.run_if(in_state(GameState::GameOver)))
            // Death pause
            .add_systems(OnEnter(GameState::DeathPause), setup_death_screen)
            .add_systems(Update, (death_pause_tick, animate_death_fade).run_if(in_state(GameState::DeathPause)))
            .add_systems(OnExit(GameState::DeathPause), cleanup_ui)
            // Level complete
            .add_systems(OnEnter(GameState::LevelComplete), setup_level_complete)
            .add_systems(Update, level_complete_tick.run_if(in_state(GameState::LevelComplete)))
            .add_systems(OnExit(GameState::LevelComplete), cleanup_ui);
    }
}

// World-space HUD strip is at the very top of the camera view.
// Camera looks at (WINDOW_W*0.5, -WINDOW_H*0.5). Top of view = y = 0.
const HUD_Y: f32 = -(HUD_HEIGHT * 0.5); // center of HUD strip

fn setup_hud(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ui_handles: ResMut<UiHandles>,
    data: Res<GameData>,
) {
    let fsize = 14.0f32;
    let font = assets.font.clone();

    let lives_e = commands.spawn((
        Text2d::new(format!("Lives: {}", data.lives)),
        TextFont { font: font.clone(), font_size: fsize, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(60.0, HUD_Y, 10.0),
        UiEntity,
    )).id();
    ui_handles.lives_text = Some(lives_e);

    let bombs_e = commands.spawn((
        Text2d::new("Bombs: 1"),
        TextFont { font: font.clone(), font_size: fsize, ..default() },
        TextColor(Color::srgb(1.0, 0.8, 0.2)),
        Transform::from_xyz(180.0, HUD_Y, 10.0),
        UiEntity,
    )).id();
    ui_handles.bombs_text = Some(bombs_e);

    let range_e = commands.spawn((
        Text2d::new("Range: 2"),
        TextFont { font: font.clone(), font_size: fsize, ..default() },
        TextColor(Color::srgb(1.0, 0.4, 0.4)),
        Transform::from_xyz(300.0, HUD_Y, 10.0),
        UiEntity,
    )).id();
    ui_handles.range_text = Some(range_e);

    let time_e = commands.spawn((
        Text2d::new(format!("Time: {}", data.time as i32)),
        TextFont { font: font.clone(), font_size: fsize, ..default() },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Transform::from_xyz(WINDOW_W - 50.0, HUD_Y, 10.0),
        UiEntity,
    )).id();
    ui_handles.time_text = Some(time_e);
}

fn update_hud(
    data: Res<GameData>,
    player_q: Query<&Player>,
    ui_handles: Res<UiHandles>,
    mut text_q: Query<&mut Text2d, With<UiEntity>>,
) {
    let (max_bombs, range) = player_q.get_single()
        .map(|p| (p.max_bombs, p.range))
        .unwrap_or((1, 2));

    if let Some(e) = ui_handles.lives_text {
        if let Ok(mut t) = text_q.get_mut(e) {
            t.0 = format!("Lives: {}", data.lives.max(0));
        }
    }
    if let Some(e) = ui_handles.bombs_text {
        if let Ok(mut t) = text_q.get_mut(e) {
            t.0 = format!("Bombs: {}", max_bombs);
        }
    }
    if let Some(e) = ui_handles.range_text {
        if let Ok(mut t) = text_q.get_mut(e) {
            t.0 = format!("Range: {}", range);
        }
    }
    if let Some(e) = ui_handles.time_text {
        if let Ok(mut t) = text_q.get_mut(e) {
            t.0 = format!("Time: {}", data.time as i32);
        }
    }
}

fn cleanup_ui(mut commands: Commands, q: Query<Entity, With<UiEntity>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

// ── Main Menu ────────────────────────────────────────────────────────
fn setup_main_menu(mut commands: Commands, assets: Res<GameAssets>) {
    let cx = WINDOW_W * 0.5;
    // Top edge of the second-from-bottom floor row — characters stand here
    let floor_top = -WINDOW_H + TILE * 3.0; // -288

    // Dark green background
    commands.spawn((
        Sprite {
            color: Color::srgb(0.02, 0.09, 0.06),
            custom_size: Some(Vec2::new(WINDOW_W, WINDOW_H)),
            ..default()
        },
        Transform::from_xyz(cx, -WINDOW_H * 0.5, -1.0),
        UiEntity,
    ));

    // Bottom tile rows: wall + two floor rows
    for col in 0..GRID_COLS {
        let x = col as f32 * TILE + TILE * 0.5;
        commands.spawn((
            Sprite { image: assets.themes[0].wall.clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
            Transform::from_xyz(x, -WINDOW_H + TILE * 0.5, 0.0),
            UiEntity,
        ));
        for row in 1..=2u32 {
            commands.spawn((
                Sprite { image: assets.themes[0].floor.clone(), custom_size: Some(Vec2::splat(TILE)), ..default() },
                Transform::from_xyz(x, -WINDOW_H + TILE * (0.5 + row as f32), 0.0),
                UiEntity,
            ));
        }
    }

    // Player (left, 80 px, feet on floor_top)
    commands.spawn((
        Sprite { image: assets.themes[0].player_down[0].clone(), custom_size: Some(Vec2::splat(80.0)), ..default() },
        Transform::from_xyz(72.0, floor_top + 40.0, 1.0),
        UiEntity,
    ));

    // Bomb left of center, resting on lower floor row
    commands.spawn((
        Sprite { image: assets.themes[0].bomb[2].clone(), custom_size: Some(Vec2::splat(44.0)), ..default() },
        Transform::from_xyz(cx - 72.0, -WINDOW_H + TILE * 2.0 + 22.0, 1.0),
        UiEntity,
    ));

    // Explosion floating center-right (above enemy)
    commands.spawn((
        Sprite { image: assets.themes[0].exp_center.clone(), custom_size: Some(Vec2::splat(68.0)), ..default() },
        Transform::from_xyz(cx + 88.0, floor_top + 80.0, 2.0),
        UiEntity,
    ));

    // Enemy (right, 64 px)
    commands.spawn((
        Sprite { image: assets.themes[0].enemy[0].clone(), custom_size: Some(Vec2::splat(64.0)), ..default() },
        Transform::from_xyz(WINDOW_W - 68.0, floor_top + 32.0, 1.0),
        UiEntity,
    ));

    // Bomb right of center
    commands.spawn((
        Sprite { image: assets.themes[0].bomb[1].clone(), custom_size: Some(Vec2::splat(40.0)), ..default() },
        Transform::from_xyz(cx + 60.0, -WINDOW_H + TILE * 2.0 + 20.0, 1.0),
        UiEntity,
    ));

    // Title: "BLAST" (orange) + "ZONE" (green) as a single text line
    commands.spawn((
        Text2d::new("BLAST"),
        TextFont { font: assets.font.clone(), font_size: 56.0, ..default() },
        TextColor(Color::srgb(1.0, 0.54, 0.08)),
        Transform::from_xyz(cx, -80.0, 10.0),
        UiEntity,
    )).with_children(|p| {
        p.spawn((
            TextSpan::new("ZONE"),
            TextFont { font: assets.font.clone(), font_size: 56.0, ..default() },
            TextColor(Color::srgb(0.36, 0.76, 0.28)),
        ));
    });

    // "ARENA" subtitle with wide spacing
    commands.spawn((
        Text2d::new("A  R  E  N  A"),
        TextFont { font: assets.font.clone(), font_size: 26.0, ..default() },
        TextColor(Color::srgb(0.90, 0.96, 1.0)),
        Transform::from_xyz(cx, -150.0, 10.0),
        UiEntity,
    ));

    // Blinking "PRESS ENTER"
    commands.spawn((
        Text2d::new("PRESS  ENTER"),
        TextFont { font: assets.font.clone(), font_size: 20.0, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(cx, -210.0, 10.0),
        BlinkText,
        UiEntity,
    ));

    // Controls hint — sits on the floor tiles, below the character sprites
    commands.spawn((
        Text2d::new("WASD / Arrows: Move     Space / X: Bomb"),
        TextFont { font: assets.font.clone(), font_size: 11.0, ..default() },
        TextColor(Color::srgba(0.75, 0.80, 0.75, 0.70)),
        Transform::from_xyz(cx, -WINDOW_H + TILE * 1.5, 5.0),
        UiEntity,
    ));

    // Version — subtle, bottom-right corner
    commands.spawn((
        Text2d::new(format!("v{}", env!("CARGO_PKG_VERSION"))),
        TextFont { font: assets.font.clone(), font_size: 11.0, ..default() },
        TextColor(Color::srgba(0.9, 0.95, 1.0, 0.40)),
        Transform::from_xyz(WINDOW_W - 24.0, -WINDOW_H + TILE * 0.5, 5.0),
        UiEntity,
    ));
}

fn animate_blink(
    time: Res<Time>,
    mut timer: Local<f32>,
    mut q: Query<&mut Visibility, With<BlinkText>>,
) {
    *timer += time.delta_secs();
    let visible = (*timer % 1.15) < 0.65;
    for mut vis in q.iter_mut() {
        *vis = if visible { Visibility::Visible } else { Visibility::Hidden };
    }
}

fn main_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut next: ResMut<NextState<GameState>>,
    mut data: ResMut<GameData>,
) {
    if keys.just_pressed(KeyCode::Enter)
        || keys.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
    {
        data.lives = 3;
        data.level = 1;
        data.time = LEVEL_TIME;
        next.set(GameState::Playing);
    }
}

// ── Game Over ────────────────────────────────────────────────────────
fn setup_gameover(mut commands: Commands, assets: Res<GameAssets>) {
    let cx = WINDOW_W * 0.5;
    let cy = -(WINDOW_H * 0.5);
    commands.spawn((
        Text2d::new("GAME OVER"),
        TextFont { font: assets.font.clone(), font_size: 48.0, ..default() },
        TextColor(Color::srgb(1.0, 0.2, 0.2)),
        Transform::from_xyz(cx, cy + 80.0, 10.0),
        UiEntity,
    ));
    commands.spawn((
        Text2d::new("Press ENTER to try again"),
        TextFont { font: assets.font.clone(), font_size: 20.0, ..default() },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Transform::from_xyz(cx, cy - 40.0, 10.0),
        UiEntity,
    ));
}

fn gameover_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut next: ResMut<NextState<GameState>>,
    mut data: ResMut<GameData>,
) {
    if keys.just_pressed(KeyCode::Enter)
        || keys.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
    {
        data.lives = 3;
        data.level = 1;
        data.time = LEVEL_TIME;
        next.set(GameState::Playing);
    }
}

// ── Death Pause ──────────────────────────────────────────────────────
fn setup_death_screen(mut commands: Commands, assets: Res<GameAssets>, data: Res<GameData>) {
    let cx = WINDOW_W * 0.5;
    let cy = -(WINDOW_H * 0.5);

    // Full-screen fade overlay, starts fully transparent
    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(WINDOW_W, WINDOW_H)),
            ..default()
        },
        Transform::from_xyz(cx, cy, 8.0),
        FadeOverlay,
        UiEntity,
    ));

    commands.spawn((
        Text2d::new("YOU DIED"),
        TextFont { font: assets.font.clone(), font_size: 36.0, ..default() },
        TextColor(Color::srgb(1.0, 0.3, 0.3)),
        Transform::from_xyz(cx, cy + 60.0, 10.0),
        UiEntity,
    ));
    commands.spawn((
        Text2d::new(format!("Lives remaining: {}", data.lives.max(0))),
        TextFont { font: assets.font.clone(), font_size: 20.0, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(cx, cy, 10.0),
        UiEntity,
    ));
}

fn animate_death_fade(
    timer: Res<DeathPauseTimer>,
    mut overlay_q: Query<&mut Sprite, With<FadeOverlay>>,
) {
    if let Ok(mut sprite) = overlay_q.get_single_mut() {
        let progress = 1.0 - (timer.0 / DEATH_PAUSE_DURATION).clamp(0.0, 1.0);
        sprite.color = Color::srgba(0.0, 0.0, 0.0, progress * OVERLAY_MAX_ALPHA);
    }
}

fn death_pause_tick(
    time: Res<Time>,
    mut timer: ResMut<DeathPauseTimer>,
    mut data: ResMut<GameData>,
    mut next: ResMut<NextState<GameState>>,
) {
    timer.0 -= time.delta_secs();
    if timer.0 <= 0.0 {
        data.time = LEVEL_TIME;
        if data.lives <= 0 {
            next.set(GameState::GameOver);
        } else {
            next.set(GameState::Playing);
        }
    }
}

// ── Level Complete ───────────────────────────────────────────────────
fn setup_level_complete(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut data: ResMut<GameData>,
) {
    data.level += 1;
    if data.level > 4 { data.level = 1; }
    data.time = LEVEL_TIME;

    let cx = WINDOW_W * 0.5;
    let cy = -(WINDOW_H * 0.5);
    commands.spawn((
        Text2d::new("ARENA CLEARED!"),
        TextFont { font: assets.font.clone(), font_size: 36.0, ..default() },
        TextColor(Color::srgb(0.2, 1.0, 0.4)),
        Transform::from_xyz(cx, cy + 40.0, 10.0),
        UiEntity,
    ));
    commands.spawn((
        Text2d::new(format!("Next: Level {}", data.level)),
        TextFont { font: assets.font.clone(), font_size: 24.0, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(cx, cy - 20.0, 10.0),
        UiEntity,
    ));
}

fn level_complete_tick(
    time: Res<Time>,
    mut timer: Local<f32>,
    mut next: ResMut<NextState<GameState>>,
) {
    *timer += time.delta_secs();
    if *timer >= 3.0 {
        *timer = 0.0;
        next.set(GameState::Playing);
    }
}
