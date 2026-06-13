use bevy::prelude::*;
use crate::components::VirtualInput;
use crate::game_assets::GameAssets;
use crate::states::GameState;

pub struct TouchControlsPlugin;

impl Plugin for TouchControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VirtualInput>()
            .add_systems(OnEnter(GameState::Playing), setup_touch_buttons)
            .add_systems(OnExit(GameState::Playing), cleanup_touch_buttons)
            .add_systems(
                Update,
                update_virtual_input.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)] struct TouchBtnUp;
#[derive(Component)] struct TouchBtnDown;
#[derive(Component)] struct TouchBtnLeft;
#[derive(Component)] struct TouchBtnRight;
#[derive(Component)] struct TouchBtnBomb;
#[derive(Component)] struct TouchControl;

const BTN_SIZE: f32 = 50.0;
const BTN_GAP: f32 = 6.0;
const BTN_MARGIN: f32 = 8.0;
const BTN_COLOR: Color = Color::srgba(0.05, 0.05, 0.05, 0.55);
const BTN_PRESSED: Color = Color::srgba(0.9, 0.9, 0.9, 0.55);

fn spawn_btn(
    parent: &mut ChildBuilder,
    label: &str,
    font: Handle<Font>,
    left: Option<f32>,
    right: Option<f32>,
    bottom: Option<f32>,
    extra: impl Bundle,
) {
    let mut node = Node {
        position_type: PositionType::Absolute,
        width: Val::Px(BTN_SIZE),
        height: Val::Px(BTN_SIZE),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    if let Some(l) = left    { node.left   = Val::Px(l); }
    if let Some(r) = right   { node.right  = Val::Px(r); }
    if let Some(b) = bottom  { node.bottom = Val::Px(b); }

    parent.spawn((node, Button, BackgroundColor(BTN_COLOR), BorderRadius::all(Val::Px(8.0)), extra))
        .with_children(|p| {
            p.spawn((Text::new(label), TextFont { font, font_size: 18.0, ..default() }, TextColor(Color::WHITE)));
        });
}

fn setup_touch_buttons(mut commands: Commands, assets: Res<GameAssets>) {
    let font = assets.font.clone();
    let lx = BTN_MARGIN;
    let mid_x = BTN_MARGIN + BTN_SIZE + BTN_GAP;

    commands.spawn((
        Node { width: Val::Percent(100.0), height: Val::Percent(100.0), position_type: PositionType::Absolute, ..default() },
        TouchControl,
        PickingBehavior { should_block_lower: false, is_hoverable: false },
    )).with_children(|p| {
        spawn_btn(p, "^", font.clone(), Some(mid_x), None, Some(BTN_MARGIN + BTN_SIZE * 2.0 + BTN_GAP), TouchBtnUp);
        spawn_btn(p, "v", font.clone(), Some(mid_x), None, Some(BTN_MARGIN), TouchBtnDown);
        spawn_btn(p, "<", font.clone(), Some(lx), None, Some(BTN_MARGIN + BTN_SIZE + BTN_GAP * 0.5), TouchBtnLeft);
        spawn_btn(p, ">", font.clone(), Some(mid_x + BTN_SIZE + BTN_GAP), None, Some(BTN_MARGIN + BTN_SIZE + BTN_GAP * 0.5), TouchBtnRight);
        spawn_btn(p, "B", font.clone(), None, Some(BTN_MARGIN), Some(BTN_MARGIN + BTN_SIZE + BTN_GAP * 0.5), TouchBtnBomb);
    });
}

fn cleanup_touch_buttons(mut commands: Commands, q: Query<Entity, With<TouchControl>>) {
    for e in q.iter() { commands.entity(e).despawn_recursive(); }
}

type AnyBtn = Or<(With<TouchBtnUp>, With<TouchBtnDown>, With<TouchBtnLeft>, With<TouchBtnRight>, With<TouchBtnBomb>)>;

fn update_virtual_input(
    mut vi: ResMut<VirtualInput>,
    mut prev_bomb: Local<bool>,
    mut buttons: Query<(
        &Interaction, &mut BackgroundColor,
        Option<&TouchBtnUp>, Option<&TouchBtnDown>,
        Option<&TouchBtnLeft>, Option<&TouchBtnRight>,
        Option<&TouchBtnBomb>,
    ), AnyBtn>,
) {
    vi.up = false; vi.down = false; vi.left = false; vi.right = false; vi.bomb = false;

    for (interaction, mut bg, up, down, left, right, bomb) in &mut buttons {
        let pressed = *interaction == Interaction::Pressed;
        bg.0 = if pressed { BTN_PRESSED } else { BTN_COLOR };
        if up.is_some()    { vi.up    = pressed; }
        if down.is_some()  { vi.down  = pressed; }
        if left.is_some()  { vi.left  = pressed; }
        if right.is_some() { vi.right = pressed; }
        if bomb.is_some()  { vi.bomb  = pressed; }
    }

    vi.bomb_just = vi.bomb && !*prev_bomb;
    *prev_bomb = vi.bomb;
}
