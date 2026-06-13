use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};
use crate::game_assets::GameAssets;
use crate::states::GameState;

pub struct FullscreenPlugin;

impl Plugin for FullscreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_fullscreen_button)
            .add_systems(Update, (fullscreen_button_system, fullscreen_keyboard));
    }
}

#[derive(Component)]
struct FullscreenBtn;

const BTN_NORMAL:  Color = Color::srgba(0.05, 0.05, 0.05, 0.55);
const BTN_HOVER:   Color = Color::srgba(0.25, 0.25, 0.25, 0.70);
const BTN_PRESSED: Color = Color::srgba(0.80, 0.80, 0.80, 0.80);

fn spawn_fullscreen_button(
    mut commands: Commands,
    assets: Res<GameAssets>,
    existing: Query<(), With<FullscreenBtn>>,
) {
    if !existing.is_empty() { return; }
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(6.0),
            right: Val::Px(6.0),
            width: Val::Px(52.0),
            height: Val::Px(32.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Button,
        BackgroundColor(BTN_NORMAL),
        BorderRadius::all(Val::Px(6.0)),
        FullscreenBtn,
    )).with_children(|p| {
        p.spawn((
            Text::new("[ ]"),
            TextFont { font: assets.font.clone(), font_size: 13.0, ..default() },
            TextColor(Color::WHITE),
        ));
    });
}

fn fullscreen_button_system(
    mut q: Query<(&Interaction, &mut BackgroundColor), With<FullscreenBtn>>,
    mut prev_pressed: Local<bool>,
    mut windows: Query<&mut Window>,
) {
    let Ok((interaction, mut bg)) = q.get_single_mut() else { return; };
    let pressed = *interaction == Interaction::Pressed;
    bg.0 = match *interaction {
        Interaction::Pressed => BTN_PRESSED,
        Interaction::Hovered => BTN_HOVER,
        Interaction::None    => BTN_NORMAL,
    };
    if pressed && !*prev_pressed {
        toggle(&mut windows);
    }
    *prev_pressed = pressed;
}

fn fullscreen_keyboard(keys: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if keys.just_pressed(KeyCode::F11) { toggle(&mut windows); }
}

fn toggle(windows: &mut Query<&mut Window>) {
    let Ok(mut w) = windows.get_single_mut() else { return; };
    w.mode = match w.mode {
        WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
        _ => WindowMode::Windowed,
    };
}
