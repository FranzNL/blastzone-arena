use bevy::prelude::*;

// ── Grid position (integer cell) ─────────────────────────────────────
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPos {
    pub col: i32,
    pub row: i32,
}

// ── Facing direction ─────────────────────────────────────────────────
#[derive(Component, Clone, Copy, PartialEq, Eq, Default)]
pub enum Facing {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

// ── Player ───────────────────────────────────────────────────────────
#[derive(Component)]
pub struct Player {
    pub range: u32,
    pub max_bombs: u32,
    pub speed: f32,
    pub shield_timer: f32,
    pub active_bombs: u32,
    pub hit_timer: f32,
}

impl Default for Player {
    fn default() -> Self {
        use crate::constants::*;
        Self {
            range: BASE_RANGE,
            max_bombs: BASE_BOMBS,
            speed: H_SPEED,
            shield_timer: 0.0,
            active_bombs: 0,
            hit_timer: 0.0,
        }
    }
}

// ── Bomb ─────────────────────────────────────────────────────────────
#[derive(Component)]
pub struct Bomb {
    pub fuse: f32,
    pub owner: Entity,
    pub range: u32,
}

// ── Blast segment ────────────────────────────────────────────────────
#[derive(Component, Clone, Copy)]
pub enum BlastDir {
    Center,
    ArmH,
    ArmV,
    TipUp,
    TipDown,
    TipLeft,
    TipRight,
}

#[derive(Component)]
pub struct Blast {
    pub life: f32,
}

// ── Tiles ────────────────────────────────────────────────────────────
#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Block {
    pub powerup: Option<PowerUpKind>,
}

// ── Power-ups ─────────────────────────────────────────────────────────
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum PowerUpKind {
    Fire,
    Bomb,
    Speed,
    Shield,
}

#[derive(Component)]
pub struct PowerUp(pub PowerUpKind);

// ── Enemy ─────────────────────────────────────────────────────────────
#[derive(Component)]
pub struct Enemy {
    pub dir: IVec2,
    pub move_timer: f32,
    pub anim_timer: f32,
}

// ── Animation ────────────────────────────────────────────────────────
#[derive(Component)]
pub struct AnimTimer(pub f32);

// ── Velocity (world-space pixels/sec) ───────────────────────────────
#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// ── Marker for cleanup ───────────────────────────────────────────────
#[derive(Component)]
pub struct LevelEntity;

#[derive(Component)]
pub struct UiEntity;

// ── Camera marker ────────────────────────────────────────────────────
#[derive(Component)]
pub struct MainCamera;

// ── Death fade overlay ───────────────────────────────────────────────
#[derive(Component)]
pub struct FadeOverlay;

// ── Blinking text (main menu) ────────────────────────────────────────
#[derive(Component)]
pub struct BlinkText;

// ── Sound Events ─────────────────────────────────────────────────────
#[derive(Event, Clone, Copy, PartialEq)]
pub enum SoundEvent {
    BombPlace,
    Explosion,
    PowerUp,
    PlayerHit,
    Win,
}

// ── Active visual theme (index into GameAssets::themes) ─────────────
#[derive(Resource, Default)]
pub struct CurrentTheme(pub usize);

// ── Game state data ──────────────────────────────────────────────────
#[derive(Resource, Default)]
pub struct GameData {
    pub lives: i32,
    pub level: u32,
    pub time: f32,
}

// ── UI references ────────────────────────────────────────────────────
#[derive(Resource, Default)]
pub struct UiHandles {
    pub lives_text: Option<Entity>,
    pub bombs_text: Option<Entity>,
    pub range_text: Option<Entity>,
    pub time_text: Option<Entity>,
}

// ── Virtual (touch) input ────────────────────────────────────────────
#[derive(Resource, Default)]
pub struct VirtualInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub bomb: bool,
    pub bomb_just: bool,
}
