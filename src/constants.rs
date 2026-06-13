pub const TILE: f32 = 32.0;
pub const GRID_COLS: u32 = 13;
pub const GRID_ROWS: u32 = 11;

pub const WINDOW_W: f32 = GRID_COLS as f32 * TILE; // 416
pub const WINDOW_H: f32 = GRID_ROWS as f32 * TILE + HUD_HEIGHT; // 352 + 32 = 384
pub const HUD_HEIGHT: f32 = 32.0;

pub const H_SPEED: f32 = 3.0 * TILE; // cells per second (96 px/s)
pub const ENEMY_SPEED: f32 = 2.0 * TILE;

pub const BOMB_FUSE: f32 = 2.5;
pub const BLAST_TIME: f32 = 0.9;
pub const CHAIN_DELAY: f32 = 0.15; // seconds before a chain-triggered bomb detonates
pub const BASE_RANGE: u32 = 2;
pub const BASE_BOMBS: u32 = 1;
pub const SHIELD_DURATION: f32 = 5.0;
pub const DEATH_PAUSE_DURATION: f32 = 3.0;

pub const LEVEL_TIME: f32 = 120.0;
