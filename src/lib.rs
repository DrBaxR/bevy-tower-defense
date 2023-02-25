use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use cursor::*;
use grid::{a_star::GridCoord, DebugGrid, DebugNode, agent::GridAgent};

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

    // TODO: grid does not spawn at correct coordinates (slightly to left-down: check assets/full_size.map)
    // spawn agents
    spawn_enemy(&mut commands, &grid, (2, 2), target);
    spawn_enemy(&mut commands, &grid, (2, 10), target);
    spawn_enemy(&mut commands, &grid, (15, 19), target);
    spawn_enemy(&mut commands, &grid, (19, 2), target);
}

