use bevy::prelude::*;
use crate::states::GameState;

#[derive(Default)]
pub struct ThemeAssets {
    pub floor: Handle<Image>,
    pub wall: Handle<Image>,
    pub block: Handle<Image>,
    pub bomb: [Handle<Image>; 3],
    pub exp_center: Handle<Image>,
    pub exp_arm: [Handle<Image>; 2],
    pub exp_tip: [Handle<Image>; 4],
    pub player_down: [Handle<Image>; 2],
    pub player_up: [Handle<Image>; 2],
    pub player_side: [Handle<Image>; 2],
    pub enemy: [Handle<Image>; 2],
    pub pu_fire: Handle<Image>,
    pub pu_bomb: Handle<Image>,
    pub pu_speed: Handle<Image>,
    pub pu_shield: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct GameAssets {
    // Index 0 = grassland, 1 = desert, 2 = ice
    pub themes: [ThemeAssets; 3],

    pub font: Handle<Font>,

    pub snd_bomb_place: Handle<AudioSource>,
    pub snd_explosion:  Handle<AudioSource>,
    pub snd_powerup:    Handle<AudioSource>,
    pub snd_player_hit: Handle<AudioSource>,
    pub snd_win:        Handle<AudioSource>,
    pub snd_gameover:   Handle<AudioSource>,
    pub mus_main:       Handle<AudioSource>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(Update, check_assets_ready.run_if(in_state(GameState::Loading)));
    }
}

fn load_theme(server: &AssetServer, name: &str) -> ThemeAssets {
    let sp = |file: &str| format!("sprites/{}/{}", name, file);
    ThemeAssets {
        floor:  server.load(sp("floor.png")),
        wall:   server.load(sp("wall.png")),
        block:  server.load(sp("block.png")),
        bomb:   [server.load(sp("bomb1.png")), server.load(sp("bomb2.png")), server.load(sp("bomb3.png"))],
        exp_center: server.load(sp("exp_center.png")),
        exp_arm:    [server.load(sp("exp_arm_h.png")), server.load(sp("exp_arm_v.png"))],
        exp_tip:    [
            server.load(sp("exp_tip_u.png")),
            server.load(sp("exp_tip_d.png")),
            server.load(sp("exp_tip_l.png")),
            server.load(sp("exp_tip_r.png")),
        ],
        player_down:  [server.load(sp("char_down1.png")), server.load(sp("char_down2.png"))],
        player_up:    [server.load(sp("char_up1.png")),   server.load(sp("char_up2.png"))],
        player_side:  [server.load(sp("char_left1.png")), server.load(sp("char_left2.png"))],
        enemy:    [server.load(sp("enemy1.png")), server.load(sp("enemy2.png"))],
        pu_fire:   server.load(sp("pu_fire.png")),
        pu_bomb:   server.load(sp("pu_bomb.png")),
        pu_speed:  server.load(sp("pu_speed.png")),
        pu_shield: server.load(sp("pu_shield.png")),
    }
}

fn load_assets(mut assets: ResMut<GameAssets>, server: Res<AssetServer>) {
    assets.themes = [
        load_theme(&server, "grassland"),
        load_theme(&server, "desert"),
        load_theme(&server, "ice"),
    ];

    assets.font = server.load("fonts/font.ttf");

    assets.snd_bomb_place = server.load("audio/bomb_place.wav");
    assets.snd_explosion  = server.load("audio/explosion.wav");
    assets.snd_powerup    = server.load("audio/powerup.wav");
    assets.snd_player_hit = server.load("audio/player_hit.wav");
    assets.snd_win        = server.load("audio/win.wav");
    assets.snd_gameover   = server.load("audio/gameover.wav");
    assets.mus_main       = server.load("audio/music_main.wav");
}

fn check_assets_ready(
    mut next: ResMut<NextState<GameState>>,
    assets: Res<GameAssets>,
    server: Res<AssetServer>,
) {
    use bevy::asset::LoadState;
    let key_images: &[&Handle<Image>] = &[
        &assets.themes[0].floor,
        &assets.themes[0].player_down[0],
        &assets.themes[0].wall,
    ];
    let key_audio: &[&Handle<AudioSource>] = &[
        &assets.mus_main,
        &assets.snd_explosion,
    ];
    let images_ready = key_images.iter().all(|h| {
        matches!(server.get_load_state(h.id()), Some(LoadState::Loaded))
    });
    let audio_ready = key_audio.iter().all(|h| {
        matches!(server.get_load_state(h.id()), Some(LoadState::Loaded))
    });
    if images_ready && audio_ready {
        next.set(GameState::MainMenu);
    }
}
