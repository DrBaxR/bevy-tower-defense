use std::time::Duration;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use bullet::{Shooter, Targetable};
use cursor::*;
use grid::{a_star::GridCoord, agent::GridAgent, DebugGrid};
use health::Damageable;

pub mod bullet;
pub mod cursor;
pub mod grid;
pub mod health;
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
}

#[derive(Bundle)]
struct EnemyBundle {
    name: Name,
    #[bundle]
    sprite: SpriteBundle,
    agent: GridAgent,
    targetable: Targetable,
    damageable: Damageable,
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
            targetable: Targetable,
            damageable: Damageable {
                max_health: 100.,
                health: 100.,
                delta: 0.,
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

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

pub fn constantly_spawn_enemies(
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    mut commands: Commands,
    grid: Query<&DebugGrid>,
) {
    let grid = grid.single();

    if timer.0.tick(time.delta()).just_finished() {
        spawn_enemy(
            &mut commands,
            &grid,
            (0, grid.size_y / 2),
            (grid.size_x as i32 - 1, grid.size_y as i32 / 2),
        )
    }
}

pub fn setup_tower(mut commands: Commands, grid: Query<&DebugGrid>) {
    let grid = grid.single();
    let pos = grid.to_screen_coords(33, 20);

    // TODO: make bullets damage enemies
    commands.spawn((
        Name::new("Tower"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 1.).with_scale(Vec3::new(15., 15., 1.)),
            ..default()
        },
        Shooter {
            cooldown: Timer::new(Duration::from_millis(1000), TimerMode::Repeating),
            target: None,
            range: 300.,
        },
    ));
}
