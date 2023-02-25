use bevy::prelude::*;

use super::{a_star::GridCoord, DebugGrid};

#[derive(Component)]
pub struct GridAgent {
    pub path: Option<Vec<GridCoord>>,
    pub speed: f32,
    pub error_margin: f32, // how much further from the waypoint it can go
}

pub fn follow_path(
    time: Res<Time>,
    mut agents: Query<(&mut Transform, &mut GridAgent)>,
    grid: Query<&DebugGrid>,
) {
    let grid = grid.single();

    for (mut transform, mut agent) in agents.iter_mut() {
        let error_margin = agent.error_margin;

        if let Some(path) = &mut agent.path {
            let next_waypoint = path.get(0);

            if next_waypoint == None {
                agent.path = None;
            } else if let Some(next_waypoint) = next_waypoint {
                let target_pos =
                    grid.to_screen_coords(next_waypoint.0 as usize, next_waypoint.1 as usize);

                if reached_waypoint(&transform.translation, &target_pos, error_margin) {
                    path.remove(0);
                    return;
                }

                let direction =
                    direction_towards(&grid, next_waypoint, &transform.translation) * agent.speed;

                transform.translation = transform.translation
                    + Vec3::new(direction.x, direction.y, 0.) * time.delta_seconds();
            }
        }
    }
}

fn reached_waypoint(current: &Vec3, target: &Vec2, error_margin: f32) -> bool {
    current.x > target.x - error_margin
        && current.x < target.x + error_margin
        && current.y > target.y - error_margin
        && current.y < target.y + error_margin
}

fn direction_towards(grid: &DebugGrid, next_waypoint: &(i32, i32), current_pos: &Vec3) -> Vec2 {
    let direction = grid.to_screen_coords(next_waypoint.0 as usize, next_waypoint.1 as usize)
        - Vec2::new(current_pos.x, current_pos.y);

    direction.normalize()
}
