use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::components::MainCamera;
use crate::constants::*;
use crate::states::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, letterbox.run_if(not(in_state(GameState::Loading))));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: WINDOW_W, height: WINDOW_H },
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(WINDOW_W * 0.5, -(WINDOW_H * 0.5), 100.0),
        MainCamera,
    ));
}

fn letterbox(windows: Query<&Window>, mut camera_q: Query<&mut Camera, With<MainCamera>>) {
    use bevy::render::camera::Viewport;
    let Ok(window) = windows.get_single() else { return; };
    let Ok(mut cam) = camera_q.get_single_mut() else { return; };

    let pw = window.physical_width() as f32;
    let ph = window.physical_height() as f32;
    if pw == 0.0 || ph == 0.0 { return; }

    let game_aspect = WINDOW_W / WINDOW_H;
    let win_aspect  = pw / ph;

    let (vw, vh, vx, vy) = if win_aspect > game_aspect {
        let vh = ph;
        let vw = (vh * game_aspect).round();
        let vx = ((pw - vw) / 2.0).round();
        (vw as u32, vh as u32, vx as u32, 0u32)
    } else {
        let vw = pw;
        let vh = (vw / game_aspect).round();
        let vy = ((ph - vh) / 2.0).round();
        (vw as u32, vh as u32, 0u32, vy as u32)
    };

    cam.viewport = Some(Viewport {
        physical_position: UVec2::new(vx, vy),
        physical_size: UVec2::new(vw, vh),
        ..default()
    });
}
