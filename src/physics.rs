use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::level::cell_world;
use crate::states::GameState;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_player.run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn move_player(
    time: Res<Time>,
    walls: Query<&GridPos, (With<Wall>, Without<Player>)>,
    blocks: Query<&GridPos, (With<Block>, Without<Player>)>,
    bombs: Query<(&GridPos, &Bomb), Without<Player>>,
    mut player_q: Query<(Entity, &mut Transform, &mut GridPos, &mut Velocity, &Player)>,
) {
    let dt = time.delta_secs();

    // Snapshot obstacle cells into vecs to avoid borrow conflicts
    let wall_cells: Vec<(i32, i32)> = walls.iter().map(|g| (g.col, g.row)).collect();
    let block_cells: Vec<(i32, i32)> = blocks.iter().map(|g| (g.col, g.row)).collect();
    let bomb_cells: Vec<((i32, i32), Entity)> = bombs.iter().map(|(g, b)| ((g.col, g.row), b.owner)).collect();

    for (entity, mut transform, mut gpos, vel, _player) in player_q.iter_mut() {
        let new_x = transform.translation.x + vel.x * dt;
        let new_y = transform.translation.y + vel.y * dt;
        let margin = TILE * 0.35;

        let resolved_x = resolve_axis(
            new_x, transform.translation.y,
            vel.x, margin, true,
            entity, &wall_cells, &block_cells, &bomb_cells,
        );
        transform.translation.x = resolved_x;

        let resolved_y = resolve_axis(
            transform.translation.x, new_y,
            vel.y, margin, false,
            entity, &wall_cells, &block_cells, &bomb_cells,
        );
        transform.translation.y = resolved_y;

        // Update grid position from world position
        let snap_col = ((transform.translation.x - TILE * 0.5) / TILE).round() as i32;
        let snap_row = (-(transform.translation.y + HUD_HEIGHT + TILE * 0.5) / TILE).round() as i32;
        gpos.col = snap_col.clamp(0, GRID_COLS as i32 - 1);
        gpos.row = snap_row.clamp(0, GRID_ROWS as i32 - 1);
    }
}

fn resolve_axis(
    px: f32, py: f32,
    vel: f32, margin: f32, is_x: bool,
    entity: Entity,
    walls: &[(i32, i32)],
    blocks: &[(i32, i32)],
    bombs: &[((i32, i32), Entity)],
) -> f32 {
    if vel == 0.0 { return if is_x { px } else { py }; }

    let half = TILE * 0.5 - 2.0;
    let (primary, secondary) = if is_x { (px, py) } else { (py, px) };
    let probes = [secondary - margin, secondary + margin];

    let mut result = primary;
    for &perp in &probes {
        let (world_x, world_y) = if is_x { (primary, perp) } else { (perp, primary) };

        // Accurate cell for the perpendicular (non-moving) axis probe
        let col_perp = (world_x / TILE).floor() as i32;
        let row_perp = ((-world_y - HUD_HEIGHT) / TILE).floor() as i32;

        // Leading edge along the moving axis determines which cell to check
        let (check_col, check_row) = if is_x {
            let lead = if vel > 0.0 { world_x + half } else { world_x - half };
            ((lead / TILE).floor() as i32, row_perp)
        } else {
            let lead = if vel > 0.0 { world_y + half } else { world_y - half };
            (col_perp, ((-lead - HUD_HEIGHT) / TILE).floor() as i32)
        };

        if is_blocked(check_col, check_row, entity, walls, blocks, bombs) {
            let target_world = cell_world(check_col, check_row);
            result = if is_x {
                if vel > 0.0 {
                    (target_world.x - TILE * 0.5 - half).min(result)
                } else {
                    (target_world.x + TILE * 0.5 + half).max(result)
                }
            } else {
                if vel < 0.0 {
                    (target_world.y + TILE * 0.5 + half).max(result)
                } else {
                    (target_world.y - TILE * 0.5 - half).min(result)
                }
            };
        }
    }
    result
}

fn is_blocked(
    col: i32, row: i32,
    player_entity: Entity,
    walls: &[(i32, i32)],
    blocks: &[(i32, i32)],
    bombs: &[((i32, i32), Entity)],
) -> bool {
    if col < 0 || row < 0 || col >= GRID_COLS as i32 || row >= GRID_ROWS as i32 {
        return true;
    }
    if walls.iter().any(|&(c, r)| c == col && r == row) { return true; }
    if blocks.iter().any(|&(c, r)| c == col && r == row) { return true; }
    for &((c, r), owner) in bombs {
        if c == col && r == row {
            if owner == player_entity { return false; } // can step off own bomb
            return true;
        }
    }
    false
}
