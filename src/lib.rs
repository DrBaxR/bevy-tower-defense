use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use cursor::*;
use grid::{a_star::GridCoord, DebugGrid, DebugNode};

pub mod bullet;
pub mod cursor;
pub mod grid;
pub mod lifetime;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            ..default()
        },
    ));
}

pub fn setup_entities(mut commands: Commands) {
    commands.spawn((
        Name::new("Cursor"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(0., 0., 1.)),
            ..default()
        },
        Cursor,
    ));

    // TODO: uncomment when done with grid testing
    // commands.spawn((
    //     Name::new("Tower"),
    //     SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::WHITE,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(0., 0., 1.).with_scale(Vec3::new(30., 30., 1.)),
    //         ..default()
    //     },
    //     Shooter {
    //         cooldown: Timer::new(Duration::from_millis(50000), TimerMode::Repeating),
    //         target: Vec3::ZERO,
    //     },
    // ));
}

#[derive(Bundle)]
struct EnemyBundle {
    name: Name,
    #[bundle]
    sprite: SpriteBundle,
    agent: GridAgent,
}

impl EnemyBundle {
    fn new(pos: Vec3, path: Option<Vec<GridCoord>>) -> Self {
        Self {
            name: Name::new("Enemy"),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                transform: Transform::from_translation(pos).with_scale(Vec3::new(10., 10., 1.)),
                ..default()
            },
            agent: GridAgent {
                path,
                speed: 50.,
                error_margin: 0.5,
            },
        }
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    grid: &DebugGrid,
    grid_pos: (usize, usize),
    target: GridCoord,
) {
    let agent_pos = grid.to_screen_coords(grid_pos.0, grid_pos.1);
    let agent_pos = Vec3::new(agent_pos.x, agent_pos.y, 10.);

    let grid_pos = (grid_pos.0 as i32, grid_pos.1 as i32);
    let path = grid.find_path(grid_pos, target);
    commands.spawn(EnemyBundle::new(agent_pos, path));
}

pub fn setup_enemy(
    mut commands: Commands,
    grid: Query<&DebugGrid>,
    mut debug_nodes: Query<&mut DebugNode>,
) {
    let grid = grid.single();
    let target = (10, 3);

    // set color of target node
    for mut node in debug_nodes.iter_mut() {
        if node.x == target.0 && node.y == target.1 {
            node.color = Color::GOLD;
        }
    }
    let target = (target.0 as i32, target.1 as i32);

    // spawn agents
    spawn_enemy(&mut commands, &grid, (0, 0), target);
    spawn_enemy(&mut commands, &grid, (0, 10), target);
    spawn_enemy(&mut commands, &grid, (15, 19), target);
    spawn_enemy(&mut commands, &grid, (19, 0), target);
}

// TODO: move this to the grid module
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
    // TODO: cleanup
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
                if transform.translation.x > target_pos.x - error_margin
                    && transform.translation.x < target_pos.x + error_margin
                    && transform.translation.y > target_pos.y - error_margin
                    && transform.translation.y < target_pos.y + error_margin
                {
                    path.remove(0);
                    return;
                }

                // move to the next waypoint
                let mut direction = grid
                    .to_screen_coords(next_waypoint.0 as usize, next_waypoint.1 as usize)
                    - Vec2::new(transform.translation.x, transform.translation.y);
                direction = direction.normalize() * agent.speed;

                transform.translation = transform.translation
                    + Vec3::new(direction.x, direction.y, 0.) * time.delta_seconds();
            }
        }
    }
}
